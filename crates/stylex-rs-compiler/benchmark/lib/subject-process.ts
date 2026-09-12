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
 * byte for byte, so the two cannot become different. And the child needs no module
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

import type { StyleXOptions } from '../../dist/index.js';
import { createPairedBenchConfigs, fixtureStylexOptions } from './config.js';
import {
  FIXTURE_CATEGORIES,
  FIXTURE_WEIGHTS,
  parseOptionOverrides,
  SOURCE_MAP_SPELLINGS,
  type SourceMapValue,
} from './fixture-schema.js';
import {
  isRecord,
  requireArray,
  requireBoolean,
  requireOneOf,
  requirePositiveInteger,
  requireNonNegativeNumber,
  requirePositiveNumber,
  requireRecord,
  requireString,
} from './json.js';
import { timeRound } from './round.js';
import type { RoundMeasurer, RuleCounter } from './runner.js';
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

/**
 * What the child is asked to do.
 *
 * The two answers a child gives are different work: a rule count for every
 * fixture, which is the sanity check the parent makes before it times anything,
 * and a timing for one fixture. The task names which, rather than letting an
 * absent field mean one of them, so a request that asks for neither is a
 * request the reader on the other side refuses.
 */
export type WorkerTask =
  | { readonly kind: 'count-rules' }
  | { readonly kind: 'time-fixture'; readonly fixture: string };

export interface WorkerRequest {
  readonly protocol: typeof WORKER_PROTOCOL_VERSION;
  /** Package directory of the subject to load. */
  readonly packageDir: string;
  /** Name of the subject, for the messages the child gives back. */
  readonly label: string;
  /** Fixtures the child works on, exactly as the parent holds them. */
  readonly fixtures: readonly FixtureDescriptor[];
  /** Shared StyleX options, before a fixture's own overrides are applied. */
  readonly stylexOptions: StyleXOptions;
  /** Tinybench time budget for a standard fixture. */
  readonly timeBudgetMs: number;
  readonly task: WorkerTask;
}

/** What one subject answered about one fixture it cannot measure. */
export interface WorkerRefusal {
  readonly refusal: string;
}

export type WorkerCount = number | WorkerRefusal;

export interface WorkerReport {
  readonly protocol: typeof WORKER_PROTOCOL_VERSION;
  /** Rule count per fixture name. The answer to a `count-rules` task. */
  readonly counts?: Record<string, WorkerCount>;
  /** Timing for the fixture the task named. */
  readonly samples?: RawLatencySamples;
  /** What the child said when it could not do the work at all. */
  readonly failure?: string;
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

  if (request.task.kind === 'count-rules') {
    return { protocol: WORKER_PROTOCOL_VERSION, counts: countRules(subject.run, request) };
  }

  const wanted = request.task.fixture;
  const fixture = request.fixtures.find(entry => entry.name === wanted);
  if (fixture === undefined) {
    return {
      protocol: WORKER_PROTOCOL_VERSION,
      failure: `The request names fixture "${wanted}", which it does not carry.`,
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

/**
 * Times one fixture, through the same round the single-process runner uses.
 *
 * One subject, because that is all this process holds. `lib/round.ts` decides
 * what a round is, so a subject measured here and a subject measured there are
 * measured alike -- which a ratio between them depends on.
 */
async function timeFixture(
  run: (fixture: FixtureDescriptor, options: StyleXOptions) => number,
  fixture: FixtureDescriptor,
  request: WorkerRequest
): Promise<RawLatencySamples> {
  // Resolved once rather than per iteration, so a fixture's own overrides do
  // not put an object allocation inside the timed loop.
  const options = fixtureStylexOptions(fixture, request.stylexOptions);
  const perSubject = await timeRound(fixture, createPairedBenchConfigs(request.timeBudgetMs), [
    { label: request.label, transform: () => run(fixture, options) },
  ]);

  const samples = perSubject[request.label];
  if (samples === undefined) {
    throw new Error(`The round timed no subject named "${request.label}"`);
  }

  return samples;
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

  // Each field stays absent where the child left it absent, because which
  // fields a report carries is what says which task it answered.
  return {
    protocol: WORKER_PROTOCOL_VERSION,
    ...(raw.failure === undefined ? {} : { failure: requireString(raw.failure, 'failure') }),
    ...(raw.counts === undefined ? {} : { counts: readCounts(raw.counts) }),
    ...(raw.samples === undefined ? {} : { samples: readSamples(raw.samples) }),
  };
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
 * The request a parent wrote, read the way the parent reads a report.
 *
 * Built field by field rather than asserted. The two sides are the same
 * checkout, so a wrong shape here means a file this process was not meant to
 * read -- a request left by an older run, or a path the caller mistyped. A
 * `code` that arrived as a number would otherwise be timed as a fixture that
 * compiles to nothing, and the subject that measured it would read as fast.
 *
 * The rules a fixture is held to are the rules the manifest reader applies, in
 * `lib/fixture-schema.ts`, because a fixture that this side accepted and that
 * side refuses is a fixture measured under conditions nobody declared.
 *
 * @throws Error naming the first field that is not what the protocol says.
 */
export function readWorkerRequest(raw: unknown): WorkerRequest {
  const request = requireRecord(raw, 'request');
  if (request.protocol !== WORKER_PROTOCOL_VERSION) {
    throw new Error(
      `request.protocol must be ${String(WORKER_PROTOCOL_VERSION)}, ` +
        `and it is ${JSON.stringify(request.protocol)}`
    );
  }

  return {
    protocol: WORKER_PROTOCOL_VERSION,
    packageDir: requireString(request.packageDir, 'request.packageDir'),
    label: requireString(request.label, 'request.label'),
    fixtures: requireArray(request.fixtures, 'request.fixtures').map((fixture, index) =>
      readFixture(fixture, `request.fixtures[${String(index)}]`)
    ),
    stylexOptions: readStylexOptions(request.stylexOptions),
    timeBudgetMs: requirePositiveNumber(request.timeBudgetMs, 'request.timeBudgetMs'),
    task: readTask(request.task),
  };
}

function readTask(value: unknown): WorkerTask {
  const task = requireRecord(value, 'request.task');
  if (task.kind === 'count-rules') return { kind: 'count-rules' };
  if (task.kind === 'time-fixture') {
    return { kind: 'time-fixture', fixture: requireString(task.fixture, 'request.task.fixture') };
  }

  throw new Error(`request.task.kind is unsupported: ${JSON.stringify(task.kind)}`);
}

function readFixture(value: unknown, context: string): FixtureDescriptor {
  const fixture = requireRecord(value, context);
  const descriptor: FixtureDescriptor = {
    name: requireString(fixture.name, `${context}.name`),
    filePath: requireString(fixture.filePath, `${context}.filePath`),
    // The source, not a path to it. Empty is a fixture that measures nothing,
    // so it is refused here rather than reported as work that took no time.
    code: requireString(fixture.code, `${context}.code`),
    weight: requireOneOf(fixture.weight, FIXTURE_WEIGHTS, `${context}.weight`),
    category: requireOneOf(fixture.category, FIXTURE_CATEGORIES, `${context}.category`),
    batchSize: requirePositiveInteger(fixture.batchSize, `${context}.batchSize`),
  };

  // Copied only when the parent sent it, the way the manifest parser copies it:
  // an own `dev: undefined` is not the same as an absent `dev`, because the
  // shared options decide the shape where the fixture says nothing.
  if (fixture.dev !== undefined) descriptor.dev = requireBoolean(fixture.dev, `${context}.dev`);
  if (fixture.options !== undefined) {
    descriptor.options = parseOptionOverrides(
      fixture.options,
      `${context}.options`,
      readWireSourceMap
    );
  }

  return descriptor;
}

/**
 * The shared options, which are what `createStylexOptions` returns.
 *
 * Narrowed key by key instead of taken as a whole. A request that carried some
 * other shape would be measured under it while the report still says this run
 * priced the production shape, and nothing about the number would look wrong.
 * A key added to `createStylexOptions` is refused here until it is added here
 * too, and `subject-process.test.ts` measures a real subject, so that arrives
 * as a failing test rather than as a silent measurement.
 */
function readStylexOptions(value: unknown): StyleXOptions {
  const options = requireRecord(value, 'request.stylexOptions');
  const resolution = requireRecord(
    options.unstable_moduleResolution,
    'request.stylexOptions.unstable_moduleResolution'
  );
  if (resolution.type !== 'haste') {
    throw new Error('request.stylexOptions.unstable_moduleResolution.type must be haste');
  }

  return {
    dev: requireBoolean(options.dev, 'request.stylexOptions.dev'),
    treeshakeCompensation: requireBoolean(
      options.treeshakeCompensation,
      'request.stylexOptions.treeshakeCompensation'
    ),
    unstable_moduleResolution: {
      type: 'haste',
      rootDir: requireString(
        resolution.rootDir,
        'request.stylexOptions.unstable_moduleResolution.rootDir'
      ),
    },
  };
}

/**
 * The `sourceMap` setting a request spells, as the addon's own enum names it.
 *
 * The one step of a fixture this side reads differently from the manifest
 * reader. That one looks the value up in the package's `SourceMaps` export;
 * this one cannot, because reading that export loads a binding, and this
 * process must hold no binding but its subject's. So the spelling is held to
 * the settings the compiler accepts, and the value it names is the same string
 * the parent sent -- the enum spells each member as its own name.
 */
function readWireSourceMap(value: unknown, context: string): SourceMapValue {
  const spelling = requireOneOf(value, SOURCE_MAP_SPELLINGS, context);
  if (!isSourceMapValue(spelling)) {
    throw new Error(`${context} must be one of ${SOURCE_MAP_SPELLINGS.join(', ')}`);
  }

  return spelling;
}

/**
 * Whether a spelling is the enum member of the same name.
 *
 * A predicate, where the manifest reader has a lookup. `SourceMaps` is a `const
 * enum` whose members carry their own name as their value, and only a module
 * that imports the package can name one. This process must not, so the check it
 * can make is the name -- which `SOURCE_MAP_SPELLINGS` holds, tied by its type
 * to the settings the compiler accepts.
 */
function isSourceMapValue(spelling: string): spelling is SourceMapValue {
  const accepted: readonly string[] = SOURCE_MAP_SPELLINGS;
  return accepted.includes(spelling);
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

/**
 * One subject's rule count for every fixture, read in a child process.
 *
 * A fixture the subject cannot compile is the reason this exists. In this
 * process the addon draws a code frame on stderr before it throws. That stream
 * is the release log. So a leg that expects the refusal printed a compiler
 * error for a fixture it then reported under `Not compared`, and a good run
 * read as a failed one. The child writes its report to a file, and its stderr
 * is read only when it fails. The refusal then arrives as a value.
 *
 * @throws Error when the child stops without writing a report.
 */
export function countRulesInChild(input: {
  subject: SplitSubject;
  fixtures: readonly FixtureDescriptor[];
  stylexOptions: StyleXOptions;
  timeBudgetMs: number;
}): Record<string, WorkerCount> {
  const run = openWorkerRuns();
  try {
    const report = callWorker(run, {
      protocol: WORKER_PROTOCOL_VERSION,
      packageDir: input.subject.packageDir,
      label: input.subject.label,
      fixtures: input.fixtures,
      stylexOptions: input.stylexOptions,
      timeBudgetMs: input.timeBudgetMs,
      task: { kind: 'count-rules' },
    });

    return report.counts ?? {};
  } finally {
    closeWorkerRuns(run);
  }
}

/**
 * What a child reported about one fixture, as the sanity check needs it.
 *
 * A refusal comes back as the sentence the subject said, thrown here, so the
 * runner cannot tell a child's answer from an answer this process took.
 *
 * @throws Error when the subject refused the fixture, or said nothing about it.
 */
export function readReportedCount(
  counts: Record<string, WorkerCount>,
  label: string,
  fixture: FixtureDescriptor
): number {
  const count = counts[fixture.name];
  if (count === undefined) {
    throw new Error(`Subject "${label}" said nothing about fixture "${fixture.name}"`);
  }
  if (isWorkerRefusal(count)) throw new Error(count.refusal);

  return count;
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
  // Keyed once rather than searched for each round: the runner asks for a
  // subject by the label it carries, and the packages never change.
  const packageDirs = new Map(input.subjects.map(subject => [subject.label, subject.packageDir]));

  // The directory is opened before the first child and removed here if one
  // refuses, because the caller has nothing to close until this returns.
  try {
    for (const subject of input.subjects) {
      counts.set(
        subject.label,
        countRulesInChild({
          subject,
          fixtures: input.fixtures,
          stylexOptions: input.stylexOptions,
          timeBudgetMs: input.timeBudgetMs,
        })
      );
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
      return readReportedCount(counts.get(label) ?? {}, label, fixture);
    },

    async measureRound(fixture, order) {
      const perSubject: Record<string, RawLatencySamples> = {};

      // One child at a time, in the order the schedule gives, so the position a
      // subject holds in a round is the position it is measured in.
      for (const subject of order) {
        const label = subject.descriptor.label;
        const source = packageDirs.get(label);
        if (source === undefined) throw new Error(`No package directory for subject "${label}"`);

        const report = callWorker(run, {
          protocol: WORKER_PROTOCOL_VERSION,
          packageDir: source,
          label,
          fixtures: [fixture],
          stylexOptions: input.stylexOptions,
          timeBudgetMs: input.timeBudgetMs,
          task: { kind: 'time-fixture', fixture: fixture.name },
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
