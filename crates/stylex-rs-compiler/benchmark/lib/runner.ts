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
 *
 * `requiredSubject` names the one subject the check is a gate for — the
 * revision under measurement. A fixture the *other* subject cannot compile
 * leaves the comparison instead of stopping the run, because a comparison
 * needs both sides and the release leg compares against the last published
 * version, which can be several features behind. It does not leave the run:
 * the subjects that did answer for it are still timed, because the absolute
 * budget is about one subject and a fixture that prices a new feature is
 * exactly the one a published base cannot compile. A fixture the required
 * subject refuses is still a hard failure: that one is a regression in the
 * code under test. A caller that names no required subject keeps the older
 * behaviour, where any refusal stops the run.
 *
 * A run where no fixture at all is comparable by every subject still fails.
 * That is a broken base rather than a manifest question, and a release must
 * not read as a clean comparison when it compared nothing.
 *
 * A fixture that kept one subject is timed alone, so its rounds do not
 * alternate two bindings the way a compared fixture's rounds do. Read its
 * number against an absolute ceiling, not against the paired fixtures around
 * it: a subject that shares no cache with a second binding is, if anything,
 * a little faster, so the ceiling keeps its margin.
 *
 * A fixture may override `dev` for itself (`FixtureDescriptor.dev`). The
 * override is resolved in one place, `fixtureStylexOptions`, and used by
 * both the sanity check and the timed run, so a fixture can never be
 * validated under one configuration and timed under another.
 */

import type { BenchOptions } from 'tinybench';

import type { StyleXOptions } from '../../dist/index.js';
import { fixtureStylexOptions } from './config.js';
import { timeRound } from './round.js';
import { bootstrapMedianRatio, makeSeededRng, roundRatios } from './stats.js';
import type { LoadedSubject } from './subjects.js';
import type {
  BootstrapConfig,
  FixtureDescriptor,
  FixturePairedStats,
  FixtureRawStats,
  FixtureRoundStats,
  RawLatencySamples,
} from './types.js';

/**
 * How a subject's rule count for one fixture is obtained.
 *
 * @throws Error carrying what the subject said when it cannot compile it.
 */
export type RuleCounter = (subject: LoadedSubject, fixture: FixtureDescriptor) => number;

/**
 * The counter a run uses unless the caller hands in another one: ask the
 * subject in this process.
 *
 * Exported because a caller that answers for one subject in a child process
 * must answer for the others the way the runner would, and two spellings of
 * that would be two configurations a fixture could be validated under.
 */
export function countRulesInProcess(stylexOptions: StyleXOptions): RuleCounter {
  return (subject, fixture) => subject.run(fixture, fixtureStylexOptions(fixture, stylexOptions));
}

/** How one round of one fixture is timed, for the subjects in the order given. */
export type RoundMeasurer = (
  fixture: FixtureDescriptor,
  order: readonly LoadedSubject[]
) => Promise<Record<string, RawLatencySamples>>;

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
  /**
   * Label of the subject the sanity check is a gate for. A fixture any other
   * subject cannot measure leaves the run rather than stopping it. Absent, no
   * subject is privileged and every refusal stops the run.
   */
  requiredSubject?: string;
  /**
   * Where the work happens. Both default to this process, which is what every
   * platform but one uses.
   *
   * macOS cannot hold two bindings that both link mimalloc, so the paired entry
   * point hands in a pair that runs each subject in a process of its own. The
   * schedule, the sanity check and the statistics below are the same either
   * way: only the two steps that touch a subject move.
   */
  countRules?: RuleCounter;
  measureRound?: RoundMeasurer;
}

/**
 * One fixture that lost a subject, and the answer that removed it.
 *
 * The fixture stays in the run and the remaining subjects are timed for it.
 * What it lost is the comparison: a ratio needs both sides.
 */
export interface UncomparedFixture {
  readonly fixture: string;
  readonly subject: string;
  /** What that subject did: a refusal sentence, or the rule count it emitted. */
  readonly reason: string;
}

/**
 * What one subject answered about a fixture it cannot measure.
 *
 * Both readings of the same answer, because which one is wanted depends on who
 * the subject is: the run stops with `failure` where the subject is the one
 * under measurement, and reports `uncompared` where it is not.
 */
interface SubjectRefusal {
  readonly subject: string;
  readonly failure: Error;
  readonly uncompared: UncomparedFixture;
}

export interface RunResult {
  fixtures: FixtureRawStats[];
  /** Empty on a run where every subject measured every fixture. */
  uncompared: UncomparedFixture[];
}

export async function runRounds(options: RunOptions): Promise<RunResult> {
  if (options.subjects.length === 0) {
    throw new Error('runRounds requires at least one subject');
  }
  if (options.rounds < 1) {
    throw new Error('runRounds requires rounds >= 1');
  }
  // Stated here rather than left to the plan below, whose own failure is about
  // a base that refuses every fixture and would name none in that message.
  if (options.fixtures.length === 0) {
    throw new Error('runRounds requires at least one fixture');
  }

  const { planned, uncompared } = planFixtures(options);

  const rng = makeSeededRng(options.seed);
  const fixtures: FixtureRawStats[] = [];

  for (const { fixture, subjects } of planned) {
    const roundStats: FixtureRoundStats[] = [];
    const schedule = createBalancedSchedule(subjects, options.rounds, rng);
    const measure = options.measureRound ?? ((one, order) => runSingleRound(one, order, options));
    for (const [round, order] of schedule.entries()) {
      // Normalised here rather than in either measurer, so a fixture that
      // batches cannot be reported per batch by one of them and per transform
      // by the other.
      const perSubject = normaliseRound(await measure(fixture, order), fixture.batchSize);
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
      paired: computePairedStats(subjects, roundStats, options.bootstrap),
    });
  }

  return { fixtures, uncompared };
}

/** One fixture of a run, and the subjects that answered for it. */
interface FixturePlan {
  fixture: FixtureDescriptor;
  /** A non-empty subset of the run's subjects, in the run's own order. */
  subjects: readonly LoadedSubject[];
}

interface RunPlan {
  /** Every fixture that at least one subject answered for, in manifest order. */
  planned: FixturePlan[];
  uncompared: UncomparedFixture[];
}

function planFixtures(options: RunOptions): RunPlan {
  const planned: FixturePlan[] = [];
  const uncompared: UncomparedFixture[] = [];
  let anyCompared = false;

  for (const fixture of options.fixtures) {
    const refusals = subjectRefusals(fixture, options);
    if (refusals.length === 0) {
      planned.push({ fixture, subjects: options.subjects });
      anyCompared = true;
      continue;
    }
    // A refusal by the subject under measurement stops the run: whatever the
    // other subject answered, this one is the code the numbers are about. Where
    // the caller named no subject, no refusal is a manifest question and the
    // first one stops the run.
    const gating =
      options.requiredSubject === undefined
        ? refusals[0]
        : refusals.find(answer => answer.subject === options.requiredSubject);
    if (gating !== undefined) throw gating.failure;
    // Only the first refusal is reported. A second subject saying the same
    // thing about the same fixture adds a line and no information, and the
    // fixture loses its comparison either way.
    uncompared.push(refusals[0]!.uncompared);

    // The subjects that did answer are still timed. The required subject is
    // among them, because a refusal by that one threw above, so the absolute
    // budget still receives a measurement for this fixture.
    const refused = new Set(refusals.map(answer => answer.subject));
    planned.push({
      fixture,
      subjects: options.subjects.filter(subject => !refused.has(subject.descriptor.label)),
    });
  }

  if (!anyCompared) {
    throw new Error(
      'Sanity check failed: no fixture is measurable by every subject — ' +
        uncompared
          .map(fixture => `"${fixture.fixture}" (${fixture.subject}: ${fixture.reason})`)
          .join(', ')
    );
  }

  return { planned, uncompared };
}

/** What each subject that cannot measure `fixture` said, in subject order. */
function subjectRefusals(fixture: FixtureDescriptor, options: RunOptions): SubjectRefusal[] {
  const refusals: SubjectRefusal[] = [];

  const count = options.countRules ?? countRulesInProcess(options.stylexOptions);

  for (const subject of options.subjects) {
    const label = subject.descriptor.label;
    let rules: number;
    try {
      rules = count(subject, fixture);
    } catch (error) {
      // A compiler error carries no fixture name, and the CI log for a paired
      // run showed only a stack ending inside the base subject's `transform`.
      // Which fixture and which subject is the whole of the answer here: the
      // base is an older build, so what it cannot compile is a question about
      // the manifest rather than about the change under measurement.
      refusals.push({
        subject: label,
        failure: new Error(refusal(label, fixture, 'could not compile'), { cause: error }),
        uncompared: { fixture: fixture.name, subject: label, reason: sentenceOf(error) },
      });
      continue;
    }
    if (!Number.isFinite(rules) || rules <= 0) {
      const predicate = `produced ${String(rules)} StyleX rules for`;
      refusals.push({
        subject: label,
        failure: new Error(refusal(label, fixture, predicate)),
        uncompared: {
          fixture: fixture.name,
          subject: label,
          reason: `emitted ${String(rules)} StyleX rules`,
        },
      });
    }
  }

  return refusals;
}

/**
 * The first line of what a compiler said, or a stand-in when it said nothing.
 *
 * One line, because a compiler refusal carries a code frame under its sentence
 * and the sentence is the whole of what a reader needs to see beside a dropped
 * fixture. The full error stays on the hard-failure path, where it is the
 * `cause`.
 */
function sentenceOf(error: unknown): string {
  const message = error instanceof Error ? error.message : String(error);
  return message.split('\n')[0]?.trim() || 'refused without a message';
}

/**
 * What a failed sanity check says, in the one shape both of its answers share.
 *
 * `predicate` carries the difference and its own preposition -- a subject that
 * threw "could not compile", one that emitted nothing "produced 0 StyleX rules
 * for" -- so the subject and the fixture a reader needs are named the same way
 * and in the same order either way.
 */
function refusal(label: string, fixture: FixtureDescriptor, predicate: string): string {
  return `Sanity check failed: subject "${label}" ${predicate} fixture "${fixture.name}"`;
}

async function runSingleRound(
  fixture: FixtureDescriptor,
  order: readonly LoadedSubject[],
  options: RunOptions
): Promise<Record<string, RawLatencySamples>> {
  // Resolved once per round rather than per iteration: a fixture's `dev`
  // override must not put an object allocation inside the timed loop.
  const stylexOptions = fixtureStylexOptions(fixture, options.stylexOptions);

  return timeRound(
    fixture,
    { standard: options.standardBench, heavy: options.heavyBench },
    order.map(subject => ({
      label: subject.descriptor.label,
      transform: () => subject.run(fixture, stylexOptions),
    }))
  );
}

/** Every subject's samples for one round, as latency per transform. */
function normaliseRound(
  perSubject: Record<string, RawLatencySamples>,
  batchSize: number
): Record<string, RawLatencySamples> {
  if (batchSize <= 1) return perSubject;

  const scaled: Record<string, RawLatencySamples> = {};
  for (const [label, samples] of Object.entries(perSubject)) {
    scaled[label] = normaliseBatchedSamples(samples, batchSize);
  }

  return scaled;
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
 * Roles are recorded for every fixture two subjects measured; the bootstrap
 * statistics only when a config asks for them.
 *
 * A fixture one subject refused carries no roles, because it carries no
 * comparison. The budget resolves its subject from the fixtures that do.
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
