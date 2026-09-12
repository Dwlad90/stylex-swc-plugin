/**
 * The contract between the parent that asks for a measurement and the child
 * that makes it.
 *
 * The end-to-end case at the bottom is the one that matters most. The split
 * path is taken on its own only on macOS, and only against a base that links
 * mimalloc, which no leg has yet -- so without a case that asks for it the code
 * would ship untested everywhere it runs.
 */

import { spawnSync } from 'node:child_process';
import fs from 'node:fs';
import path from 'node:path';
import { pathToFileURL } from 'node:url';

import { afterAll, afterEach, describe, expect, test } from 'vitest';

import { findNativeBindings } from '../lib/native-bindings.js';
import {
  callWorker,
  closeWorkerRuns,
  isWorkerRefusal,
  openWorkerRuns,
  readWorkerRequest,
  workerExecArgv,
  WORKER_PROTOCOL_VERSION,
  type WorkerRequest,
} from '../lib/subject-process.js';
import type { FixtureDescriptor } from '../lib/types.js';
import { createTempDirs } from './helpers/temp-dirs.js';

const packageDir = path.resolve(import.meta.dirname, '..', '..');
const benchmarkDir = path.join(packageDir, 'benchmark');
// Every case needs a subject to load, and a clean checkout has no build. A
// missing build is not a fault in this module, so skip rather than fail.
const built = findNativeBindings(packageDir).length > 0;

const temp = createTempDirs();
const runs = openWorkerRuns();

afterEach(() => {
  temp.removeAll();
});

afterAll(() => {
  closeWorkerRuns(runs);
});

/** A fixture the compiler answers for, small enough to time in milliseconds. */
function fixture(overrides: Partial<FixtureDescriptor> = {}): FixtureDescriptor {
  const file = path.join(benchmarkDir, 'perf_fixtures', 'create-basic.js');

  return {
    name: 'probe',
    filePath: file,
    code: fs.readFileSync(file, 'utf8'),
    weight: 'standard',
    category: 'perf',
    batchSize: 1,
    ...overrides,
  };
}

function request(overrides: Partial<WorkerRequest> = {}): WorkerRequest {
  return {
    protocol: WORKER_PROTOCOL_VERSION,
    packageDir,
    label: 'subject',
    fixtures: [fixture()],
    stylexOptions: {
      dev: false,
      treeshakeCompensation: true,
      unstable_moduleResolution: { type: 'haste', rootDir: packageDir },
    },
    // The smallest budget tinybench accepts. These cases prove the two sides
    // agree, not how fast anything is.
    timeBudgetMs: 20,
    task: { kind: 'count-rules' },
    ...overrides,
  };
}

/**
 * The child reads a request the way the parent reads a report: field by field.
 *
 * A request is written by this repository, so a wrong shape means a file the
 * child was not meant to read. What it must never do is measure such a file:
 * an empty fixture compiles to nothing and reads as a very fast subject.
 */
describe('readWorkerRequest', () => {
  /** The request as it arrives: through the file the two sides pass. */
  function overTheWire(overrides: Record<string, unknown> = {}): unknown {
    return JSON.parse(JSON.stringify({ ...request(), ...overrides }));
  }

  /** A request whose one fixture carries the overrides given, valid or not. */
  function withOptions(options: Record<string, unknown>): unknown {
    return overTheWire({ fixtures: [{ ...fixture(), options }] });
  }

  test('answers the request the parent wrote', () => {
    const written = request({ task: { kind: 'time-fixture', fixture: 'probe' } });

    expect(readWorkerRequest(JSON.parse(JSON.stringify(written)))).toEqual(written);
  });

  test('refuses a request of another protocol', () => {
    expect(() => readWorkerRequest(overTheWire({ protocol: 99 }))).toThrow(/protocol/);
  });

  test('refuses a task it cannot do', () => {
    expect(() => readWorkerRequest(overTheWire({ task: { kind: 'guess' } }))).toThrow(
      /task.kind is unsupported/
    );
    expect(() => readWorkerRequest(overTheWire({ task: { kind: 'time-fixture' } }))).toThrow(
      /task.fixture/
    );
  });

  test('names the field of a fixture that is not what the protocol says', () => {
    expect(() => readWorkerRequest(overTheWire({ fixtures: [{ ...fixture(), code: 7 }] }))).toThrow(
      /fixtures\[0\].code/
    );
    expect(() =>
      readWorkerRequest(overTheWire({ fixtures: [{ ...fixture(), weight: 'quick' }] }))
    ).toThrow(/fixtures\[0\].weight must be one of standard, heavy/);
    expect(() =>
      readWorkerRequest(overTheWire({ fixtures: [{ ...fixture(), batchSize: 0 }] }))
    ).toThrow(/fixtures\[0\].batchSize/);
  });

  test('refuses shared options of another shape', () => {
    expect(() => readWorkerRequest(overTheWire({ stylexOptions: { dev: false } }))).toThrow(
      /unstable_moduleResolution/
    );
    expect(() =>
      readWorkerRequest(overTheWire({ stylexOptions: { ...request().stylexOptions, dev: 'no' } }))
    ).toThrow(/stylexOptions.dev must be a boolean/);
  });

  // A fixture keeps the overrides it declares, and an absent one stays absent:
  // an own `dev: undefined` would answer for the shared options that decide the
  // shape where the fixture says nothing.
  // `sourceMap` is the override whose value is not a boolean, and the one the
  // child reads differently from the manifest reader.
  test('keeps a string override and refuses a spelling the compiler has not', () => {
    const declared = { ...fixture(), options: { sourceMap: 'Inline', classNamePrefix: 'x-' } };
    const read = readWorkerRequest(overTheWire({ fixtures: [declared] }));

    expect(read.fixtures[0]?.options).toEqual({ sourceMap: 'Inline', classNamePrefix: 'x-' });
    expect(() => readWorkerRequest(withOptions({ sourceMap: 'inline' }))).toThrow(
      /options.sourceMap must be one of True, False, Inline/
    );
  });

  test('refuses an option no fixture may name', () => {
    expect(() => readWorkerRequest(withOptions({ enableDebugClassNames: 'yes' }))).toThrow(
      /options.enableDebugClassNames must be a boolean/
    );
    expect(() => readWorkerRequest(withOptions({ frobnicate: true }))).toThrow(
      /is not a benchmarkable option/
    );
  });

  test('keeps the overrides a fixture declares and adds none', () => {
    const declared = fixture({ dev: true, options: { enableDebugClassNames: true } });
    const read = readWorkerRequest(overTheWire({ fixtures: [declared] }));

    expect(read.fixtures[0]).toEqual(declared);
    expect(Object.keys(readWorkerRequest(overTheWire()).fixtures[0] ?? {})).not.toContain('dev');
  });
});

describe('callWorker', () => {
  test.runIf(built)('answers the rule count of every fixture it is given', () => {
    const report = callWorker(
      runs,
      request({ fixtures: [fixture(), fixture({ name: 'second' })] })
    );
    const counts = report.counts ?? {};

    expect(Object.keys(counts).toSorted()).toEqual(['probe', 'second']);
    for (const count of Object.values(counts)) {
      expect(isWorkerRefusal(count)).toBe(false);
      expect(count).toBeGreaterThan(0);
    }
  });

  test.runIf(built)('times the fixture the request names', () => {
    const report = callWorker(runs, request({ task: { kind: 'time-fixture', fixture: 'probe' } }));

    expect(report.samples?.p50).toBeGreaterThan(0);
    expect(report.samples?.samplesCount).toBeGreaterThan(0);
    expect(report.counts).toBeUndefined();
  });

  // A base that is behind by whole features cannot compile every fixture. The
  // child reports what it said rather than stopping, so the parent decides.
  test.runIf(built)('reports a refusal as a sentence rather than stopping', () => {
    const report = callWorker(runs, request({ fixtures: [fixture({ code: 'const broken = ;' })] }));
    const count = report.counts?.probe;

    expect(count !== undefined && isWorkerRefusal(count)).toBe(true);
  });

  // Its own directory, because a call that throws leaves its two files for
  // `closeWorkerRuns` to sweep, and the shared one holds those.
  test.runIf(built)('leaves no file behind for a call that answered', () => {
    const own = openWorkerRuns();
    try {
      callWorker(own, request({ task: { kind: 'time-fixture', fixture: 'probe' } }));

      expect(fs.readdirSync(own.directory)).toEqual([]);
    } finally {
      closeWorkerRuns(own);
    }
  });

  test('names the subject and what the child said when it cannot load it', () => {
    const absent = path.join(temp.make('bench-worker-'), 'no-such-package');

    expect(() => callWorker(runs, request({ packageDir: absent, label: 'gone' }))).toThrow(
      /gone[\s\S]*entry does not exist/
    );
  });

  // The case the split exists for. A child that ends without a report gives no
  // sentence of its own, so the parent must name how it ended -- and a signal
  // is what a second mimalloc binding would look like here.
  //
  // A directory standing where the report file goes is how the child is left
  // unable to write one. The first call of a run writes `report-1.json`, so
  // making that name a directory takes the report away without touching the
  // request the parent writes beside it.
  test.runIf(built)('names how the process ended when the child writes no report', () => {
    const own = openWorkerRuns();
    try {
      fs.mkdirSync(path.join(own.directory, 'report-1.json'));

      expect(() =>
        callWorker(
          own,
          request({ task: { kind: 'time-fixture', fixture: 'probe' }, label: 'quiet' })
        )
      ).toThrow(/quiet[\s\S]*wrote no measurement[\s\S]*(exit|signal)/);
    } finally {
      closeWorkerRuns(own);
    }
  });

  test.runIf(built)('refuses a fixture name the request does not carry', () => {
    expect(() =>
      callWorker(runs, request({ task: { kind: 'time-fixture', fixture: 'absent' } }))
    ).toThrow(/absent/);
  });
});

describe('bench-worker', () => {
  /** The modules the reader child below loads, in the order it loads them. */
  const readerModules = ['lib/subject-process.js', 'lib/native-bindings.js'];

  /**
   * A path becomes an import specifier only as a `file://` URL. A bare absolute
   * path reads as a path on POSIX, but on Windows its drive letter reads as a
   * URL scheme, and the child stops before it runs.
   */
  function importUrl(file: string): string {
    return pathToFileURL(path.join(benchmarkDir, file)).href;
  }

  /** Source for a child that reports the bindings it holds after it loads the worker. */
  function bindingReaderSource(): string {
    const [worker, bindings] = readerModules.map(file => JSON.stringify(importUrl(file)));

    return (
      `const worker = await import(${worker});` +
      `const bindings = await import(${bindings});` +
      'console.log(JSON.stringify([...bindings.loadedNativeBindings()]));' +
      'void worker;'
    );
  }

  // The property the whole split rests on. `lib/types.ts` reads a value off
  // `dist/index.js`, and importing that loads this package's binding. A child
  // that reached it would hold the candidate binding before it loaded the
  // subject it was asked about, and macOS would end it exactly as the single
  // process ends. An import added without this case would be silent until a
  // release.
  test.runIf(built)('loads no binding of its own before a subject is asked for', () => {
    const reader = bindingReaderSource();

    const result = spawnSync(
      process.execPath,
      [...workerExecArgv(), '--input-type=module', '-e', reader],
      {
        encoding: 'utf8',
      }
    );

    expect(result.status, result.stderr).toBe(0);
    expect(JSON.parse(result.stdout)).toEqual([]);
  });

  // The case above gives the child its source on the command line, where a bad
  // specifier stops it on Windows only. This one holds the rule everywhere.
  test('gives the child a file URL for each module it loads', () => {
    const source = bindingReaderSource();

    for (const file of readerModules) {
      expect(URL.parse(importUrl(file))?.protocol).toBe('file:');
      expect(source).toContain(JSON.stringify(importUrl(file)));
    }
  });

  test('refuses a request whose protocol it does not read', () => {
    const dir = temp.make('bench-worker-protocol-');
    const requestPath = path.join(dir, 'request.json');
    const reportPath = path.join(dir, 'report.json');
    fs.writeFileSync(requestPath, JSON.stringify({ ...request(), protocol: 99 }), 'utf8');

    const result = spawnSync(
      process.execPath,
      [...workerExecArgv(), path.join(benchmarkDir, 'bench-worker.ts'), requestPath, reportPath],
      { encoding: 'utf8' }
    );

    expect(result.status).toBe(1);
    expect(result.stderr).toMatch(/protocol/);
  });

  // The wiring, not the reader: a case that drives a malformed request through
  // a real child fails if `bench-worker.ts` goes back to trusting the file.
  test('refuses a request whose fields are not what the protocol says', () => {
    const dir = temp.make('bench-worker-fields-');
    const requestPath = path.join(dir, 'request.json');
    const reportPath = path.join(dir, 'report.json');
    const broken = { ...request(), fixtures: [{ ...fixture(), code: 7 }] };
    fs.writeFileSync(requestPath, JSON.stringify(broken), 'utf8');

    const result = spawnSync(
      process.execPath,
      [...workerExecArgv(), path.join(benchmarkDir, 'bench-worker.ts'), requestPath, reportPath],
      { encoding: 'utf8' }
    );

    expect(result.status).toBe(1);
    expect(result.stderr).toMatch(/fixtures\[0\].code/);
    // The child says so in the report as well, which is where the parent looks.
    const report: unknown = JSON.parse(fs.readFileSync(reportPath, 'utf8'));
    expect(report).toMatchObject({ failure: expect.stringContaining('code') as unknown });
  });

  test('refuses to run without both file names', () => {
    const result = spawnSync(
      process.execPath,
      [...workerExecArgv(), path.join(benchmarkDir, 'bench-worker.ts')],
      { encoding: 'utf8' }
    );

    expect(result.status).toBe(1);
    expect(result.stderr).toMatch(/request file/);
  });
});

/**
 * The whole path, as the release gate runs it.
 *
 * One package stands in for both subjects. What is proved is the split itself:
 * that two children are asked in the order the schedule gives, that both answer,
 * and that the file the verdict engine reads holds a round for each.
 */
describe('paired run in separate processes', () => {
  test.runIf(built)(
    'measures both subjects and writes the file the verdict engine reads',
    () => {
      const output = path.join(benchmarkDir, 'results', 'revisions-raw-stats.v1.json');
      const before = fs.existsSync(output) ? fs.readFileSync(output, 'utf8') : undefined;

      const result = spawnSync(
        process.execPath,
        [
          ...workerExecArgv(),
          path.join(benchmarkDir, 'bench-revisions.ts'),
          '--base',
          packageDir,
          '--candidate',
          packageDir,
          '--base-label',
          'first',
          '--candidate-label',
          'second',
          '--rounds',
          '2',
          '--time',
          '20',
          '--fixture',
          'Basic create',
          '--separate-processes',
        ],
        { encoding: 'utf8' }
      );

      try {
        expect(result.status, result.stderr).toBe(0);
        expect(result.stdout).toMatch(/process of its own/);

        const raw: unknown = JSON.parse(fs.readFileSync(output, 'utf8'));
        const stats = raw as {
          subjects: { label: string }[];
          fixtures: { rounds: { subjectOrder: string[]; perSubject: Record<string, unknown> }[] }[];
        };

        expect(stats.subjects.map(subject => subject.label)).toEqual(['first', 'second']);
        expect(stats.fixtures.length).toBeGreaterThan(0);
        for (const entry of stats.fixtures) {
          expect(entry.rounds).toHaveLength(2);
          for (const round of entry.rounds) {
            expect(Object.keys(round.perSubject).toSorted()).toEqual(['first', 'second']);
            expect(round.subjectOrder.toSorted()).toEqual(['first', 'second']);
          }
        }
        // Counterbalancing is the reason the split keeps a round boundary at
        // all: a subject that always went first would carry the drift.
        const orders = stats.fixtures[0]?.rounds.map(round => round.subjectOrder.join('>')) ?? [];
        expect(new Set(orders).size).toBe(2);
      } finally {
        // The results file belongs to whoever ran the benchmark last.
        if (before === undefined) fs.rmSync(output, { force: true });
        else fs.writeFileSync(output, before, 'utf8');
      }
    },
    120_000
  );
});
