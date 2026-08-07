/**
 * Balanced round scheduling and raw-stats emission.
 *
 * The verdict engine treats one *round* as an independent measurement
 * unit and bootstraps the median of per-round ratios. This module
 * produces those rounds and archives them into `raw-stats.v1.json`
 * alongside the historical `output.json`. The budget check and verdict
 * layer consume raw stats, never the human-readable extras.
 *
 * Sanity check: every subject must produce a non-zero StyleX rule count
 * for every fixture *before* timing begins. Existing correctness tests
 * are responsible for intentional output differences — this guard only
 * stops a broken no-op subject from appearing fast.
 */

import { Bench, type BenchOptions } from 'tinybench';

import type { StyleXOptions } from '../../dist/index.js';
import {
  bootstrapMedianRatio,
  extractLatencySamples,
  makeSeededRng,
  roundRatios,
} from './stats.js';
import type { LoadedSubject } from './subjects.js';
import type {
  BootstrapConfig,
  FixtureDescriptor,
  FixturePairedStats,
  FixtureRawStats,
  FixtureRoundStats,
  RawLatencySamples,
} from './types.js';

export interface RunOptions {
  subjects: readonly LoadedSubject[];
  fixtures: readonly FixtureDescriptor[];
  stylexOptions: StyleXOptions;
  /** Number of independent rounds per fixture. */
  rounds: number;
  /** Seed for round-level subject-order permutation. */
  seed: number;
  /** Tinybench options for `weight: 'standard'` fixtures. */
  standardBench: BenchOptions;
  /** Reduced-budget tinybench options for `weight: 'heavy'` fixtures. */
  heavyBench: BenchOptions;
  /**
   * When provided together with exactly two subjects, the runner computes
   * per-fixture ratios and bootstrap confidence bounds and stores them in
   * `FixtureRawStats.paired`. Ignored otherwise.
   */
  bootstrap?: BootstrapConfig;
}

export interface RunResult {
  fixtures: FixtureRawStats[];
}

export async function runRounds(options: RunOptions): Promise<RunResult> {
  if (options.subjects.length === 0) {
    throw new Error('runRounds requires at least one subject');
  }
  if (options.rounds < 1) {
    throw new Error('runRounds requires rounds >= 1');
  }

  sanityCheck(options.subjects, options.fixtures, options.stylexOptions);

  const rng = makeSeededRng(options.seed);
  const fixtures: FixtureRawStats[] = [];

  for (const fixture of options.fixtures) {
    const roundStats: FixtureRoundStats[] = [];
    const schedule = createBalancedSchedule(options.subjects, options.rounds, rng);
    for (const [round, order] of schedule.entries()) {
      const perSubject = await runSingleRound(fixture, order, options);
      roundStats.push({
        round,
        subjectOrder: order.map(subject => subject.descriptor.label),
        perSubject,
      });
    }
    fixtures.push({
      name: fixture.name,
      weight: fixture.weight,
      category: fixture.category,
      batchSize: fixture.batchSize,
      rounds: roundStats,
      paired: computePairedStats(options.subjects, roundStats, options.bootstrap),
    });
  }

  return { fixtures };
}

function sanityCheck(
  subjects: readonly LoadedSubject[],
  fixtures: readonly FixtureDescriptor[],
  stylexOptions: StyleXOptions
): void {
  for (const fixture of fixtures) {
    for (const subject of subjects) {
      const rules = subject.run(fixture, stylexOptions);
      if (!Number.isFinite(rules) || rules <= 0) {
        throw new Error(
          `Sanity check failed: subject "${subject.descriptor.label}" produced ${String(
            rules
          )} StyleX rules for fixture "${fixture.name}"`
        );
      }
    }
  }
}

async function runSingleRound(
  fixture: FixtureDescriptor,
  order: readonly LoadedSubject[],
  options: RunOptions
): Promise<Record<string, RawLatencySamples>> {
  const benchOptions = fixture.weight === 'heavy' ? options.heavyBench : options.standardBench;
  const bench = new Bench({
    name: `${fixture.name} (round)`,
    ...benchOptions,
  });

  for (const subject of order) {
    const label = subject.descriptor.label;
    bench.add(label, () => {
      // Batching lifts sub-millisecond fixtures above timer noise.
      for (let i = 0; i < fixture.batchSize; i++) {
        subject.run(fixture, options.stylexOptions);
      }
    });
  }

  await bench.run();

  const perSubject: Record<string, RawLatencySamples> = {};
  for (const task of bench.tasks) {
    const samples = extractLatencySamples(task);
    perSubject[task.name] = normaliseBatchedSamples(samples, fixture.batchSize);
  }
  return perSubject;
}

/**
 * If a fixture batches N transforms per timed operation, divide the
 * observed latency by N so downstream statistics remain "latency per
 * transform". Samples are recomputed together with p50/p95 so both stay
 * consistent.
 */
function normaliseBatchedSamples(samples: RawLatencySamples, batchSize: number): RawLatencySamples {
  if (batchSize <= 1) return samples;
  const scaled = samples.samples.map(sample => sample / batchSize);
  return {
    samples: scaled,
    p50: samples.p50 / batchSize,
    p95: samples.p95 / batchSize,
    rme: samples.rme,
    samplesCount: samples.samplesCount,
    opsPerSec: samples.opsPerSec * batchSize,
  };
}

function permuteSubjects<T>(subjects: readonly T[], rng: () => number): readonly T[] {
  if (subjects.length <= 1) return subjects;
  const copy = [...subjects];
  for (let i = copy.length - 1; i > 0; i--) {
    const j = Math.floor(rng() * (i + 1));
    const tmp = copy[i];
    copy[i] = copy[j] as T;
    copy[j] = tmp as T;
  }
  return copy;
}

/**
 * Randomize each block, then rotate it so every subject occupies every timing
 * position once per complete block. Independent shuffles can leave a paired
 * run split 8/2 and turn warm-cache or temporal drift into an apparent
 * regression; counterbalancing caps that bias without reducing randomness.
 */
function createBalancedSchedule<T>(
  subjects: readonly T[],
  rounds: number,
  rng: () => number
): readonly (readonly T[])[] {
  if (subjects.length <= 1) return Array.from({ length: rounds }, () => subjects);

  const schedule: (readonly T[])[] = [];
  while (schedule.length < rounds) {
    const block = permuteSubjects(subjects, rng);
    for (let offset = 0; offset < block.length && schedule.length < rounds; offset++) {
      schedule.push([...block.slice(offset), ...block.slice(0, offset)]);
    }
  }
  return schedule;
}

/**
 * Roles are recorded for every two-subject run; the bootstrap statistics
 * only when a config asks for them.
 *
 * The two are separable on purpose. `base`/`candidate` are identity — the
 * budget check resolves which subject its ceilings describe from them, and
 * must keep working on a release run that defers ratios and confidence
 * bounds to the verdict engine. Gating the roles on `bootstrap` left every
 * release raw-stats file role-less and failed the budget step outright.
 */
function computePairedStats(
  subjects: readonly LoadedSubject[],
  rounds: readonly FixtureRoundStats[],
  bootstrap: BootstrapConfig | undefined
): FixturePairedStats | undefined {
  if (subjects.length !== 2) return undefined;

  const base = subjects[0]!.descriptor.label;
  const candidate = subjects[1]!.descriptor.label;
  if (bootstrap === undefined) return { base, candidate };

  const basePerRound = rounds.map(round => round.perSubject[base]?.p50 ?? Number.NaN);
  const candidatePerRound = rounds.map(round => round.perSubject[candidate]?.p50 ?? Number.NaN);
  const ratios = roundRatios(basePerRound, candidatePerRound);
  const confidence = bootstrapMedianRatio(ratios, bootstrap);

  return { base, candidate, ratios, confidence };
}
