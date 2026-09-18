/**
 * The uncovered-region reporter, `scripts/coverage-missing.sh`.
 *
 * The script measures with whatever nightly the machine holds, while continuous
 * integration installs the newest nightly on every run. The region count is a
 * property of that compiler, so an old local nightly makes the script report
 * full coverage against a gate that fails in continuous integration -- which is
 * exactly how fourteen uncovered regions reached a pull request. The script
 * therefore names the compiler it used and warns when a newer one exists, and
 * this suite holds that behaviour still.
 *
 * The real script runs against stubbed `rustc`, `rustup` and `cargo`, so what is
 * asserted is what the script does, not what the machine happens to have.
 */

import assert from 'node:assert/strict';
import { spawnSync } from 'node:child_process';
import path from 'node:path';
import test from 'node:test';

import {
  createWorkspace,
  hermeticEnvironment,
  makeTemporaryDirectory,
  missing,
  pathVariable,
  readLog,
  repoRoot,
  stubPath,
  writeStubs,
  writeText,
} from './lib/test-harness.mjs';

const script = path.join(repoRoot, 'scripts/coverage-missing.sh');

/** The script is bash, and the report it renders is python. */
const NEEDS_BASH = missing('bash', 'python3');

/** A `rustup check` line for a nightly that has nothing newer behind it. */
const UP_TO_DATE =
  'nightly-aarch64-apple-darwin - up to date: 1.100.0-nightly (cea272fa3 2026-09-07)';

/** The same line for a nightly that is behind the one continuous integration installs. */
const BEHIND =
  'nightly-aarch64-apple-darwin - update available: 1.100.0-nightly (f248f4038 2026-09-05) -> 1.100.0-nightly (cea272fa3 2026-09-07)';

/** A coverage export holding no measured file, which is the shortest clean run. */
const EMPTY_EXPORT = '{"data":[{"files":[],"functions":[]}]}';

/**
 * A measured file, named so neither the ignore regex nor an exclude drops it.
 *
 * It is written to disk, because the script refuses a mapping whose positions
 * the source cannot hold: a file the mapping names and the tree cannot open is
 * read as a stale mapping, and the run stops before it reports anything. The
 * file lives outside the repository so no case writes into the tree it runs in.
 */
const MEASURED_FILE = path.join(
  makeTemporaryDirectory('stylex-coverage-missing-source-'),
  'crates/stylex-demo/src/demo.rs'
);

/**
 * Every line carries code and is wider than the widest column `buildExport`
 * writes, so any line a case names is a position this file can hold.
 */
writeText(
  MEASURED_FILE,
  `${Array.from({ length: 24 }, (_, index) => `  let value_${index} = compute(${index});`).join('\n')}\n`
);

/**
 * Builds a coverage export the way llvm-cov writes one.
 *
 * `records` is one entry per compiled instantiation: a mangled name and the
 * execution count of each of the function's regions, in order. Every record of
 * one function describes the same source regions, which is what makes them an
 * instantiation group.
 *
 * The file summary is filled the way llvm-cov fills it -- a group is scored on
 * its best-covered record -- so a test states only the counts and the summary
 * follows from them.
 */
function buildExport({ startLine = 10, records }) {
  const width = records[0].counts.length;
  const regionsOf = counts =>
    counts.map((count, index) => [startLine + index, 3, startLine + index, 20, count, 0, 0, 0]);
  const covered = Math.max(...records.map(record => record.counts.filter(Boolean).length));

  return JSON.stringify({
    data: [
      {
        files: [
          {
            filename: MEASURED_FILE,
            summary: {
              regions: { count: width, covered, notcovered: width - covered },
              functions: { count: 1, covered: 1 },
              lines: { count: width, covered },
            },
          },
        ],
        functions: records.map(record => ({
          name: record.name,
          filenames: [MEASURED_FILE],
          regions: regionsOf(record.counts),
        })),
      },
    ],
  });
}

/**
 * Runs the real script with stubs for the three commands it starts.
 *
 * `rustcVersion` empty stands for a machine with no nightly installed, so the
 * stub fails the way `rustc +nightly` fails there. `rustupCheck` is the line the
 * script reads to decide whether the nightly is behind.
 */
function runScript({
  rustcVersion = 'rustc 1.100.0-nightly (cea272fa3 2026-09-07)',
  rustupCheck = UP_TO_DATE,
  coverageExport = EMPTY_EXPORT,
  args = [],
} = {}) {
  const workspace = createWorkspace('stylex-coverage-missing-');

  // The stub writes the export where the script asked for it, so the report
  // renders from a real file rather than from a missing one.
  const cargoBody = [
    'output=""',
    'previous=""',
    'for argument in "$@"; do',
    '  if [ "$previous" = "--output-path" ]; then output="$argument"; fi',
    '  previous="$argument"',
    'done',
    `[ -n "$output" ] && printf '%s' '${coverageExport}' > "$output"`,
    'exit 0',
  ].join('\n');

  writeStubs(workspace.bin, {
    cargo: { body: cargoBody },
    rustc: rustcVersion === '' ? { body: 'exit 1' } : { body: `printf '%s\\n' '${rustcVersion}'` },
    rustup: { body: `printf '%s\\n' '${rustupCheck}'` },
  });

  const result = spawnSync('bash', [script, ...args], {
    cwd: repoRoot,
    encoding: 'utf8',
    env: hermeticEnvironment({
      [pathVariable]: stubPath(workspace.bin),
      FAKE_COMMAND_LOG: workspace.log,
    }),
  });

  return { result, log: readLog(workspace.log) };
}

void test('the run names the compiler that measured it', { skip: NEEDS_BASH }, () => {
  const { result } = runScript();

  assert.match(result.stdout, /[=]=> Toolchain: rustc 1\.100\.0-nightly \(cea272fa3 2026-09-07\)/);
  assert.equal(result.status, 0);
});

void test('a nightly behind the newest one is a warning', { skip: NEEDS_BASH }, () => {
  const { result } = runScript({ rustupCheck: BEHIND });

  assert.match(result.stderr, /a newer nightly is available/);
  assert.match(result.stderr, /rustup update nightly/);
  // A warning only: the report still runs and still answers for the run it made.
  assert.equal(result.status, 0);
});

void test('a nightly that is up to date says nothing', { skip: NEEDS_BASH }, () => {
  const { result } = runScript({ rustupCheck: UP_TO_DATE });

  assert.doesNotMatch(result.stderr, /newer nightly/);
  assert.equal(result.status, 0);
});

void test('--skip-toolchain-check asks the network nothing', { skip: NEEDS_BASH }, () => {
  const { result, log } = runScript({ rustupCheck: BEHIND, args: ['--skip-toolchain-check'] });

  assert.doesNotMatch(log, /^rustup check/m, 'the lookup ran although it was turned off');
  assert.doesNotMatch(result.stderr, /newer nightly/);
  assert.match(result.stdout, /[=]=> Toolchain: /, 'the version is named even so');
  assert.equal(result.status, 0);
});

void test(
  'a machine with no nightly is told which command installs one',
  { skip: NEEDS_BASH },
  () => {
    const { result } = runScript({ rustcVersion: '' });

    assert.match(result.stderr, /no nightly toolchain found/);
    assert.match(result.stderr, /rustup toolchain install nightly/);
  }
);

/*
 * What the coverage gate counts.
 *
 * llvm-cov scores a function on its best-covered instantiation, so a region it
 * counts can still have been run -- by a different instantiation. The merge the
 * script does over instantiations reads such a region as covered, and the gate
 * does not, so the script used to fail with a bare count and no location. These
 * cases hold it to naming every region behind a failing gate.
 */

/** Two instantiations of one function, each running the half the other misses. */
const SPLIT_ACROSS_INSTANTIATIONS = buildExport({
  records: [
    { name: '_RNvNtCshash_4demo3foo5Alpha', counts: [7, 0] },
    { name: '_RNvNtCshash_4demo3foo4Beta', counts: [0, 3] },
  ],
});

/** One instantiation, with a region no test reaches. */
const NEVER_RUN = buildExport({
  records: [{ name: '_RNvNtCshash_4demo3foo5Alpha', counts: [7, 0] }],
});

/** One instantiation that runs every region of its function. */
const FULLY_RUN = buildExport({
  records: [{ name: '_RNvNtCshash_4demo3foo5Alpha', counts: [7, 3] }],
});

void test(
  'a region no single instantiation runs is named, not just counted',
  { skip: NEEDS_BASH },
  () => {
    const { result } = runScript({ coverageExport: SPLIT_ACROSS_INSTANTIATIONS });

    assert.match(result.stdout, /Regions the coverage gate counts/);
    assert.match(result.stdout, /fn at line 10: the best instantiation runs 1 of 2 region\(s\)/);
    // Both leaders are named, because the reader has to see that closing the
    // group means one instantiation running what today takes two.
    assert.match(result.stdout, /demo::foo::Alpha misses\n\s+line 11\b/);
    assert.match(result.stdout, /demo::foo::Beta misses\n\s+line 10\b/);
    assert.match(result.stdout, /1 region\(s\) counted against the gate across 1 file\(s\)/);
    assert.equal(result.status, 1, 'the script must fail wherever the gate fails');
  }
);

void test(
  'a region no instantiation runs is reported once, not twice',
  { skip: NEEDS_BASH },
  () => {
    const { result } = runScript({ coverageExport: NEVER_RUN });

    assert.match(result.stdout, /Uncovered regions \(not executed by any test\)/);
    assert.match(result.stdout, /line 11\b/);
    assert.doesNotMatch(
      result.stdout,
      /Regions the coverage gate counts/,
      'a region the first section already named must not be repeated as a gate gap'
    );
    assert.equal(result.status, 1);
  }
);

void test('one instantiation running every region is clean', { skip: NEEDS_BASH }, () => {
  const { result } = runScript({ coverageExport: FULLY_RUN });

  assert.match(result.stdout, /No uncovered regions/);
  assert.doesNotMatch(result.stdout, /Regions the coverage gate counts/);
  assert.equal(result.status, 0);
});
