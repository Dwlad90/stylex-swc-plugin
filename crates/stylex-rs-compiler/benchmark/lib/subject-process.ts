/**
 * Measuring one subject in a process of its own.
 *
 * The paired benchmark holds both subjects in one process, which is what lets
 * it time them against each other on one runner. macOS cannot do that when both
 * bindings link mimalloc: the second one ends the process. So on that platform
 * each subject is timed in a child process, and this module is the contract
 * between the parent that asks and the child that answers.
 *
 * The child is given the fixture rather than told where to find it. Two reasons.
 * The fixture the child measures is then the same object the parent selected,
 * byte for byte, so the two cannot drift apart. And the child needs no module
 * that reads the fixture manifest -- which matters more than it sounds, because
 * `lib/types.ts` reads a value off `dist/index.js`, and importing that loads
 * *this* package's binding. A child that read the manifest would hold the
 * candidate binding before it loaded the subject it was asked about, and would
 * end the same way the single process does.
 *
 * The two sides talk through files, not through pipes. `napi` writes to stdout
 * when `DEBUG` names it, and the release workflow sets `DEBUG: napi:*`, so
 * anything parsed out of stdout would be parsed out of a stream something else
 * writes to as well.
 */

import { spawnSync, type SpawnSyncReturns } from 'node:child_process';
import fs from 'node:fs';
import os from 'node:os';
import path from 'node:path';
import { fileURLToPath } from 'node:url';

import { Bench } from 'tinybench';

import type { StyleXOptions } from '../../dist/index.js';
import { createPairedBenchConfigs, fixtureStylexOptions } from './config.js';
import {
  isRecord,
  requireArray,
  requireNonNegativeNumber,
  requirePositiveInteger,
  requirePositiveNumber,
  requireRecord,
  requireString,
} from './json.js';
import type { RoundMeasurer, RuleCounter } from './runner.js';
import { extractLatencySamples } from './stats.js';
import { createSubject, loadSubject, readPackageVersion, type LoadedSubject } from './subjects.js';
import type { FixtureDescriptor, RawLatencySamples } from './types.js';

/**
 * Version of the shape the two sides exchange.
 *
 * The parent and the child are the same checkout, so this cannot drift in
 * normal use. It is here for the run that goes wrong: a report from an older
 * file, or a file that holds something else entirely, is then refused with a
 * sentence rather than read as a measurement.
 */
export const WORKER_PROTOCOL_VERSION = 1 as const;

export interface WorkerRequest {
  protocol: typeof WORKER_PROTOCOL_VERSION;
  /** Package directory of the subject to load. */
  packageDir: string;
  /** Name of the subject, for the messages the child gives back. */
  label: string;
  /** Fixtures the child works on, exactly as the parent holds them. */
  fixtures: readonly FixtureDescriptor[];
  /** Shared StyleX options, before a fixture's own overrides are applied. */
  stylexOptions: StyleXOptions;
  /** Tinybench time budget for a standard fixture. */
  timeBudgetMs: number;
  /**
   * Name of the fixture to time. Absent asks for the rule count of every
   * fixture instead, which is the sanity check the parent runs before it times
   * anything.
   */
  measure?: string;
}

/** What one subject answered about one fixture it cannot measure. */
export interface WorkerRefusal {
  refusal: string;
}

export type WorkerCount = number | WorkerRefusal;

export interface WorkerReport {
  protocol: typeof WORKER_PROTOCOL_VERSION;
  /** Rule count per fixture name. Present for a request that named no fixture. */
  counts?: Record<string, WorkerCount>;
  /** Timing for the fixture the request named. */
  samples?: RawLatencySamples;
  /** What the child said when it could not do the work at all. */
  failure?: string;
}

export function isWorkerRefusal(count: WorkerCount): count is WorkerRefusal {
  return typeof count !== 'number';
}

/**
 * Does what a request asks, in the process that reads it.
 *
 * Loads exactly one subject. The process that calls this must hold no other
 * binding of this compiler, which is the whole reason the work is here.
 */
export async function runWorkerRequest(request: WorkerRequest): Promise<WorkerReport> {
  const subject = await loadSubject({ label: request.label, packageDir: request.packageDir });

  if (request.measure === undefined) {
    return { protocol: WORKER_PROTOCOL_VERSION, counts: countRules(subject.run, request) };
  }

  const fixture = request.fixtures.find(entry => entry.name === request.measure);
  if (fixture === undefined) {
    return {
      protocol: WORKER_PROTOCOL_VERSION,
      failure: `The request names fixture "${request.measure}", which it does not carry.`,
    };
  }

  return {
    protocol: WORKER_PROTOCOL_VERSION,
    samples: await timeFixture(subject.run, fixture, request),
  };
}

/** The rule count of every fixture, or the sentence the subject refused with. */
function countRules(
  run: (fixture: FixtureDescriptor, options: StyleXOptions) => number,
  request: WorkerRequest
): Record<string, WorkerCount> {
  const counts: Record<string, WorkerCount> = {};

  for (const fixture of request.fixtures) {
    try {
      counts[fixture.name] = run(fixture, fixtureStylexOptions(fixture, request.stylexOptions));
    } catch (error) {
      counts[fixture.name] = { refusal: error instanceof Error ? error.message : String(error) };
    }
  }

  return counts;
}

/** Times one fixture, the way one round of the single-process runner times it. */
async function timeFixture(
  run: (fixture: FixtureDescriptor, options: StyleXOptions) => number,
  fixture: FixtureDescriptor,
  request: WorkerRequest
): Promise<RawLatencySamples> {
  const configs = createPairedBenchConfigs(request.timeBudgetMs);
  // Resolved once rather than per iteration, so a fixture's own overrides do
  // not put an object allocation inside the timed loop.
  const options = fixtureStylexOptions(fixture, request.stylexOptions);
  const bench = new Bench({
    name: `${fixture.name} (round)`,
    ...(fixture.weight === 'heavy' ? configs.heavy : configs.standard),
  });

  bench.add(request.label, () => {
    // Batching lifts sub-millisecond fixtures above timer noise.
    for (let i = 0; i < fixture.batchSize; i++) run(fixture, options);
  });
  await bench.run();

  const task = bench.tasks[0];
  if (task === undefined) throw new Error('tinybench ran no task');

  return extractLatencySamples(task);
}

/** The worker entry point, which the parent starts with `node`. */
const WORKER_ENTRY = path.join(
  path.dirname(fileURLToPath(import.meta.url)),
  '..',
  'bench-worker.ts'
);

/** How much a child may write before the parent stops reading it. */
const WORKER_OUTPUT_LIMIT = 64 * 1024 * 1024;

/** Loader that lets `node` read the TypeScript the whole harness is written in. */
const TYPESCRIPT_LOADER = ['--import', 'tsx/esm'] as const;

/**
 * The flags the child gets.
 *
 * The parent's own flags, and the loader added where the parent does not carry
 * it. Every script that starts this harness names the loader, so a parent
 * usually has it; a test runner starts a test file its own way and does not,
 * and a child without it cannot read the file it is pointed at.
 */
export function workerExecArgv(): string[] {
  const inherited = [...process.execArgv];
  if (inherited.includes(TYPESCRIPT_LOADER[1])) return inherited;

  return [...inherited, ...TYPESCRIPT_LOADER];
}

export interface WorkerRun {
  /** Where the request and the report are written. Removed by `closeWorkerRuns`. */
  directory: string;
  calls: number;
}

/** Opens a directory for the files the two sides pass. */
export function openWorkerRuns(): WorkerRun {
  return { directory: fs.mkdtempSync(path.join(os.tmpdir(), 'stylex-bench-worker-')), calls: 0 };
}

export function closeWorkerRuns(run: WorkerRun): void {
  fs.rmSync(run.directory, { force: true, recursive: true });
}

/**
 * Runs one request in a child process and answers what it wrote.
 *
 * Synchronous on purpose. The parent has nothing to do while a subject is
 * timed, and one child at a time is what keeps the measurement worth having:
 * two of them on one runner would compete for the same cores.
 *
 * @throws Error when the child stops without writing a report it can read.
 */
export function callWorker(run: WorkerRun, request: WorkerRequest): WorkerReport {
  run.calls += 1;
  const requestPath = path.join(run.directory, `request-${run.calls}.json`);
  const reportPath = path.join(run.directory, `report-${run.calls}.json`);
  fs.writeFileSync(requestPath, JSON.stringify(request), 'utf8');

  // The parent's own flags, so the child reads TypeScript the same way.
  const result = spawnSync(
    process.execPath,
    [...workerExecArgv(), WORKER_ENTRY, requestPath, reportPath],
    {
      encoding: 'utf8',
      stdio: ['ignore', 'pipe', 'pipe'],
      // What the child says is read only when it fails, but a child that fills
      // the default buffer is killed for saying too much. `napi` writes to
      // stdout whenever `DEBUG` names it, and the release workflow sets
      // `DEBUG: napi:*`, so that is the ordinary case and not a strange one.
      maxBuffer: WORKER_OUTPUT_LIMIT,
    }
  );

  const report = readReport(reportPath);
  if (report === undefined) throw new Error(workerFailure(request, result));
  if (report.failure !== undefined) {
    throw new Error(`Subject "${request.label}" could not be measured: ${report.failure}`);
  }

  fs.rmSync(requestPath, { force: true });
  fs.rmSync(reportPath, { force: true });

  return report;
}

/**
 * The report a child wrote, or nothing when it wrote none this side can read.
 *
 * Built field by field rather than asserted, because the file crosses a process
 * boundary like every other artifact this harness reads. A number that came
 * back as a string would otherwise reach the statistics, where it is a
 * measurement nobody can question.
 */
function readReport(file: string): WorkerReport | undefined {
  let raw: unknown;
  try {
    raw = JSON.parse(fs.readFileSync(file, 'utf8'));
  } catch {
    return undefined;
  }

  if (!isRecord(raw) || raw.protocol !== WORKER_PROTOCOL_VERSION) return undefined;

  const report: WorkerReport = { protocol: WORKER_PROTOCOL_VERSION };
  if (raw.failure !== undefined) report.failure = requireString(raw.failure, 'failure');
  if (raw.counts !== undefined) report.counts = readCounts(raw.counts);
  if (raw.samples !== undefined) report.samples = readSamples(raw.samples);

  return report;
}

function readCounts(value: unknown): Record<string, WorkerCount> {
  const raw = isRecord(value) ? value : {};
  const counts: Record<string, WorkerCount> = {};

  for (const [name, entry] of Object.entries(raw)) {
    counts[name] = isRecord(entry)
      ? { refusal: requireString(entry.refusal, `counts.${name}.refusal`) }
      : requireNonNegativeNumber(entry, `counts.${name}`);
  }

  return counts;
}

function readSamples(value: unknown): RawLatencySamples {
  const raw = requireRecord(value, 'samples');
  // Required rather than defaulted. An empty list reads as a measurement that
  // took no samples, and the bootstrap would carry it as one.
  const samples = requireArray(raw.samples, 'samples.samples');

  return {
    samples: samples.map((sample, index) =>
      requirePositiveNumber(sample, `samples.samples[${String(index)}]`)
    ),
    p50: requirePositiveNumber(raw.p50, 'samples.p50'),
    p95: requirePositiveNumber(raw.p95, 'samples.p95'),
    rme: requireNonNegativeNumber(raw.rme, 'samples.rme'),
    samplesCount: requirePositiveInteger(raw.samplesCount, 'samples.samplesCount'),
    opsPerSec: requirePositiveNumber(raw.opsPerSec, 'samples.opsPerSec'),
  };
}

/**
 * What a child that wrote no report is reported as.
 *
 * Names the signal, because this is the one place a SIGSEGV can still reach:
 * a child holds one binding by design, and a second one would be a fault in
 * this module rather than in the subject.
 */
function workerFailure(request: WorkerRequest, result: SpawnSyncReturns<string>): string {
  const ended =
    result.signal === null ? `exit ${String(result.status)}` : `signal ${result.signal}`;
  const said = (result.stderr || result.stdout || '').trim().split('\n').slice(-3).join('\n');

  return (
    `Subject "${request.label}" wrote no measurement: the process that timed it ` +
    `ended with ${ended}.${said === '' ? '' : `\n${said}`}`
  );
}

/** A subject the parent knows only by name and location. */
export interface SplitSubject {
  label: string;
  packageDir: string;
}

export interface SplitRun {
  /** Descriptors for the runner. Their `run` is never called. */
  subjects: LoadedSubject[];
  countRules: RuleCounter;
  measureRound: RoundMeasurer;
  /** Removes the files the two sides passed. */
  close: () => void;
}

/**
 * Prepares a paired run whose subjects never meet in one process.
 *
 * Answers the two steps `runRounds` takes that touch a subject, so everything
 * else about the run -- the balanced schedule, which fixtures are measurable,
 * the paired statistics -- is the code that runs on every other platform.
 *
 * The rule counts are read once, here, because the sanity check the runner
 * makes is synchronous and a child process is the only thing that can answer
 * it. One child per subject, before any timing starts.
 */
export function startSplitRun(input: {
  subjects: readonly SplitSubject[];
  fixtures: readonly FixtureDescriptor[];
  stylexOptions: StyleXOptions;
  timeBudgetMs: number;
}): SplitRun {
  const run = openWorkerRuns();
  const counts = new Map<string, Record<string, WorkerCount>>();

  // The directory is opened before the first child and removed here if one
  // refuses, because the caller has nothing to close until this returns.
  try {
    for (const subject of input.subjects) {
      const report = callWorker(run, {
        protocol: WORKER_PROTOCOL_VERSION,
        packageDir: subject.packageDir,
        label: subject.label,
        fixtures: input.fixtures,
        stylexOptions: input.stylexOptions,
        timeBudgetMs: input.timeBudgetMs,
      });
      counts.set(subject.label, report.counts ?? {});
    }
  } catch (error) {
    closeWorkerRuns(run);
    throw error;
  }

  return {
    subjects: input.subjects.map(subject =>
      createSubject(
        {
          label: subject.label,
          version: readPackageVersion(subject.packageDir),
          resolvedFrom: path.join(subject.packageDir, 'dist/index.js'),
        },
        () => {
          throw new Error(
            `Subject "${subject.label}" is measured in a process of its own and ` +
              'cannot be run in this one.'
          );
        }
      )
    ),

    countRules(subject, fixture) {
      const label = subject.descriptor.label;
      const count = counts.get(label)?.[fixture.name];
      if (count === undefined) {
        throw new Error(`Subject "${label}" said nothing about fixture "${fixture.name}"`);
      }
      if (isWorkerRefusal(count)) throw new Error(count.refusal);

      return count;
    },

    async measureRound(fixture, order) {
      const perSubject: Record<string, RawLatencySamples> = {};

      // One child at a time, in the order the schedule gives, so the position a
      // subject holds in a round is the position it is measured in.
      for (const subject of order) {
        const label = subject.descriptor.label;
        const source = input.subjects.find(entry => entry.label === label);
        if (source === undefined) throw new Error(`No package directory for subject "${label}"`);

        const report = callWorker(run, {
          protocol: WORKER_PROTOCOL_VERSION,
          packageDir: source.packageDir,
          label,
          fixtures: [fixture],
          stylexOptions: input.stylexOptions,
          timeBudgetMs: input.timeBudgetMs,
          measure: fixture.name,
        });
        if (report.samples === undefined) {
          throw new Error(`Subject "${label}" reported no timing for fixture "${fixture.name}"`);
        }
        perSubject[label] = report.samples;
      }

      return perSubject;
    },

    close() {
      closeWorkerRuns(run);
    },
  };
}
