/**
 * The per-crate Rust test runner, `scripts/packages/test/index.sh`.
 *
 * The script decides whether a crate holds a Rust test before it starts cargo,
 * and it then reports what cargo did. A wrong answer to the first question
 * drops a crate's suite without a word. A wrong report of the second hides a
 * red run. This suite holds the script to both.
 *
 * Almost every crate keeps its tests with the code and has no `tests`
 * directory, so the reader gave grep the name of a directory that does not
 * exist. BSD grep and GNU grep both answer correctly there, because `-q` gives
 * status 0 for a match even after an error. The reader now names only the
 * directories that exist, so the answer does not depend on the grep.
 *
 * Runs the real script against throwaway crates with a recording `cargo` on the
 * search path, so what is asserted is what cargo was asked to do.
 */

import assert from 'node:assert/strict';
import path from 'node:path';
import test from 'node:test';

import {
  A_TEST,
  NEEDS_BASH,
  NO_TEST,
  checkRunWithNoArgument,
  hugeCrateFiles,
  runCrateScript,
  valueAfter,
} from './lib/crate-script-harness.mjs';
import { repoRoot } from './lib/test-harness.mjs';

const script = path.join(repoRoot, 'scripts/packages/test/index.sh');

/** Runs the real script inside a throwaway crate. */
function runInCrate(options = {}) {
  return runCrateScript({ script, prefix: 'stylex-crate-test-runner-', ...options });
}

/** The cargo subcommand of one recorded invocation, as the script spells it. */
function subcommandOf(invocation) {
  return invocation.slice(1, 3).join(' ');
}

void test(
  'a crate that keeps its tests in src and has no tests directory still runs',
  { skip: NEEDS_BASH },
  () => {
    const { result, invocations } = runInCrate({ files: { 'src/lib.rs': A_TEST } });

    assert.equal(result.status, 0);
    assert.deepEqual(invocations.map(subcommandOf), ['nextest run', 'test --target-dir']);
  }
);

void test('the two cargo calls are the regular run and the doc run', { skip: NEEDS_BASH }, () => {
  const { invocations } = runInCrate({ files: { 'src/lib.rs': A_TEST } });

  assert.equal(invocations.length, 2);
  assert.deepEqual(invocations[0].slice(1, 3), ['nextest', 'run']);
  assert.equal(invocations[1][1], 'test');
  assert.ok(invocations[1].includes('--doc'), 'the second call is the doc run');
});

void test('a crate that keeps its tests in a tests directory runs', { skip: NEEDS_BASH }, () => {
  const { result, invocations } = runInCrate({
    files: { 'src/lib.rs': NO_TEST, 'tests/integration.rs': A_TEST },
  });

  assert.equal(result.status, 0);
  assert.equal(invocations.length, 2);
});

void test('a crate holding both directories runs the suite once', { skip: NEEDS_BASH }, () => {
  const { invocations } = runInCrate({
    files: { 'src/lib.rs': A_TEST, 'tests/integration.rs': A_TEST },
  });

  assert.equal(invocations.length, 2);
});

void test('a crate with no test marker starts no cargo', { skip: NEEDS_BASH }, () => {
  const { result, invocations } = runInCrate({ files: { 'src/lib.rs': NO_TEST } });

  assert.equal(result.status, 0);
  assert.deepEqual(invocations, []);
});

void test('a crate with neither directory starts no cargo', { skip: NEEDS_BASH }, () => {
  const { result, invocations } = runInCrate({ files: { 'Cargo.toml': '[package]\n' } });

  assert.equal(result.status, 0);
  assert.deepEqual(invocations, []);
  assert.equal(result.stderr, '', 'a crate with no source says nothing');
});

void test('an empty src directory starts no cargo', { skip: NEEDS_BASH }, () => {
  const { result, invocations } = runInCrate({ directories: ['src'] });

  assert.equal(result.status, 0);
  assert.deepEqual(invocations, []);
});

void test('a file named tests is not read as the tests directory', { skip: NEEDS_BASH }, () => {
  // The guard tests for a directory rather than for existence, so a crate that
  // happens to hold a file of that name cannot hand grep a path it rejects.
  const { result, invocations } = runInCrate({
    files: { 'src/lib.rs': A_TEST, tests: 'not a directory\n' },
  });

  assert.equal(result.status, 0);
  assert.equal(invocations.length, 2);
  assert.equal(result.stderr, '');
});

void test('every marker the script names is recognised', { skip: NEEDS_BASH }, () => {
  const markers = {
    'the attribute': '#[test]\nfn a() {}\n',
    'the module gate': '#[cfg(test)]\nmod tests {}\n',
    'the transform helper': 'fn a() { test_transform("x"); }\n',
    'the macro': 'fn a() { test!("x"); }\n',
  };

  for (const [description, source] of Object.entries(markers)) {
    const { invocations } = runInCrate({ files: { 'src/lib.rs': source } });

    assert.equal(invocations.length, 2, `${description} was not recognised`);
  }
});

void test('a marker in a file that is not Rust does not count', { skip: NEEDS_BASH }, () => {
  const { invocations } = runInCrate({
    files: { 'src/lib.rs': NO_TEST, 'src/notes.md': A_TEST, 'src/build.py': A_TEST },
  });

  assert.deepEqual(invocations, []);
});

void test('a marker nested deep under src counts', { skip: NEEDS_BASH }, () => {
  const { invocations } = runInCrate({
    files: { 'src/lib.rs': NO_TEST, 'src/a/b/c/d/e/deep.rs': A_TEST },
  });

  assert.equal(invocations.length, 2);
});

void test('extra arguments reach both cargo calls', { skip: NEEDS_BASH }, () => {
  const { invocations } = runInCrate({
    files: { 'src/lib.rs': A_TEST },
    args: ['--no-capture', 'some_test_name'],
  });

  for (const invocation of invocations) {
    assert.ok(invocation.includes('--no-capture'));
    assert.ok(invocation.includes('some_test_name'));
  }
});

void test('an argument holding a space stays one argument', { skip: NEEDS_BASH }, () => {
  const { invocations } = runInCrate({
    files: { 'src/lib.rs': A_TEST },
    args: ['--filter-expr', 'test(a) + test(b)'],
  });

  assert.ok(invocations[0].includes('test(a) + test(b)'));
});

void test(
  'the target directory is named for the crate and is one argument',
  { skip: NEEDS_BASH },
  () => {
    const { invocations } = runInCrate({ files: { 'src/lib.rs': A_TEST }, name: 'stylex-utils' });
    const target = valueAfter(invocations[0], '--target-dir');

    assert.ok(target.endsWith(path.join('target', 'test-stylex-utils')), target);
  }
);

void test(
  'a crate directory holding characters a path cannot carry is folded',
  { skip: NEEDS_BASH },
  () => {
    const { invocations } = runInCrate({
      files: { 'src/lib.rs': A_TEST },
      name: 'odd name (v2)',
    });
    const target = valueAfter(invocations[0], '--target-dir');

    assert.ok(target.endsWith(path.join('target', 'test-odd_name__v2_')), target);
    assert.equal(invocations[0].filter(argument => argument.includes('test-odd')).length, 1);
  }
);

void test(
  'a crate far larger than any in this repository is still read',
  { skip: NEEDS_BASH },
  () => {
    const { result, invocations } = runInCrate({ files: hugeCrateFiles() });

    assert.equal(result.status, 0);
    assert.equal(invocations.length, 2);
  }
);

// The script runs the regular suite and then the doc suite. Its status was the
// status of the doc run, so a red regular run reported a pass. Every case above
// stubs a cargo that always succeeds, so none of them could see it.

void test('a failing cargo fails the script', { skip: NEEDS_BASH }, () => {
  const { result } = runInCrate({ files: { 'src/lib.rs': A_TEST }, cargoBody: 'exit 101' });

  assert.notEqual(result.status, 0, 'a failing cargo reported a pass');
});

void test('a failing regular run is not hidden by a passing doc run', { skip: NEEDS_BASH }, () => {
  // Fails the first call and passes the second, which is the shape that the
  // missing `set -e` turned into a green run.
  const { result, invocations } = runInCrate({
    files: { 'src/lib.rs': A_TEST },
    cargoBody: 'case " $* " in *" nextest "*) exit 101 ;; esac',
  });

  assert.notEqual(result.status, 0, 'a failing regular run reported a pass');
  assert.equal(invocations.length, 1, 'the doc run started after the regular run failed');
});

void test('a failing doc run still fails the script', { skip: NEEDS_BASH }, () => {
  const { result, invocations } = runInCrate({
    files: { 'src/lib.rs': A_TEST },
    cargoBody: 'case " $* " in *" --doc "*) exit 101 ;; esac',
  });

  assert.notEqual(result.status, 0);
  assert.equal(invocations.length, 2, 'both runs started');
});

void test('a run with no argument still starts cargo, in every bash', { skip: NEEDS_BASH }, () => {
  checkRunWithNoArgument(runInCrate, 2);
});
