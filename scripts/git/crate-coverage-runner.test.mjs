/**
 * The per-crate coverage runner, `scripts/packages/test/coverage.sh`.
 *
 * The script decides whether a crate holds a Rust test before it starts cargo.
 * If the answer is wrong, the script reports a pass and measures nothing. This
 * suite holds that decision still, and it also holds the shape of the cargo
 * call, because a wrong flag turns the gate off just as quietly.
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

const script = path.join(repoRoot, 'scripts/packages/test/coverage.sh');

/** The crates that the script must never measure. */
const EXCLUDED = [
  'stylex-evaluator',
  'stylex-logs',
  'stylex-rs-compiler',
  'stylex-state',
  'stylex-test-parser',
  'stylex-transform',
];

/** Runs the real script inside a throwaway crate. */
function runInCrate(options = {}) {
  return runCrateScript({ script, prefix: 'stylex-crate-coverage-runner-', ...options });
}

/** A crate that holds a library and one test, which is the measurable shape. */
const A_MEASURABLE_CRATE = { 'src/lib.rs': A_TEST };

// ── The shape almost every crate has ─────────────────────────────

void test(
  'a crate that keeps its tests in src and has no tests directory is measured',
  { skip: NEEDS_BASH },
  () => {
    // Almost every crate has this shape. The script once gave grep the name of
    // the missing `tests` directory. GNU grep and BSD grep still answered
    // correctly, because `-q` gives status 0 for a match even after an error,
    // but a grep that answers 2 turned the whole measurement off.
    const { result, invocations } = runInCrate({ files: A_MEASURABLE_CRATE });

    assert.equal(result.status, 0);
    assert.equal(invocations.length, 1, 'coverage did not run for a crate that holds a test');
    assert.deepEqual(invocations[0].slice(1, 4), ['+nightly', 'llvm-cov', 'nextest']);
  }
);

// ── Which crates are measured ─────────────────────────────────────────────────

void test(
  'a crate that keeps its tests in a tests directory is measured',
  { skip: NEEDS_BASH },
  () => {
    const { result, invocations } = runInCrate({
      files: { 'src/lib.rs': NO_TEST, 'tests/integration.rs': A_TEST },
    });

    assert.equal(result.status, 0);
    assert.equal(invocations.length, 1);
  }
);

void test('a crate holding both directories is measured once', { skip: NEEDS_BASH }, () => {
  const { invocations } = runInCrate({
    files: { 'src/lib.rs': A_TEST, 'tests/integration.rs': A_TEST },
  });

  assert.equal(invocations.length, 1);
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

void test('a crate with a test but no library is not measured', { skip: NEEDS_BASH }, () => {
  // A binary-only crate has no `src/lib.rs`, and llvm-cov has no library to
  // report on.
  const { result, invocations } = runInCrate({ files: { 'src/main.rs': A_TEST } });

  assert.equal(result.status, 0);
  assert.deepEqual(invocations, []);
});

void test('every excluded crate starts no cargo', { skip: NEEDS_BASH }, () => {
  for (const name of EXCLUDED) {
    const { result, invocations } = runInCrate({ files: A_MEASURABLE_CRATE, name });

    assert.equal(result.status, 0, name);
    assert.deepEqual(invocations, [], `${name} is on the exclusion list and must not be measured`);
  }
});

void test(
  'a crate whose name only contains an excluded name is measured',
  { skip: NEEDS_BASH },
  () => {
    // The list matches a whole name. A crate called `stylex-state-index` is not
    // `stylex-state`, and it is on the gate.
    const { invocations } = runInCrate({ files: A_MEASURABLE_CRATE, name: 'stylex-state-index' });

    assert.equal(invocations.length, 1);
  }
);

// ── What the markers are ──────────────────────────────────────────────────────

void test('every marker the script names is recognised', { skip: NEEDS_BASH }, () => {
  const markers = {
    'the attribute': '#[test]\nfn a() {}\n',
    'the transform helper': 'fn a() { test_transform("x"); }\n',
    'the macro': 'fn a() { test!("x"); }\n',
  };

  for (const [description, source] of Object.entries(markers)) {
    const { invocations } = runInCrate({ files: { 'src/lib.rs': source } });

    assert.equal(invocations.length, 1, `${description} was not recognised`);
  }
});

void test('the module gate alone is not a marker for coverage', { skip: NEEDS_BASH }, () => {
  // `index.sh` reads `#[cfg(test)]` as well. This script does not, and a
  // module gate with no test inside it gives llvm-cov nothing to measure.
  const { invocations } = runInCrate({ files: { 'src/lib.rs': '#[cfg(test)]\nmod tests {}\n' } });

  assert.deepEqual(invocations, []);
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

  assert.equal(invocations.length, 1);
});

// ── Paths that a reader can get wrong ─────────────────────────────────────────

void test('a file named tests is not read as the tests directory', { skip: NEEDS_BASH }, () => {
  const { result, invocations } = runInCrate({
    files: { 'src/lib.rs': A_TEST, tests: 'not a directory\n' },
  });

  assert.equal(result.status, 0);
  assert.equal(invocations.length, 1);
  assert.equal(result.stderr, '');
});

void test('the target directory is named for the crate', { skip: NEEDS_BASH }, () => {
  // The name came from `basename` through a pipe. `basename` adds a newline,
  // and `tr` changed that newline into an underscore, so every directory held
  // one more character than the crate name.
  const { targetDir } = runInCrate({ files: A_MEASURABLE_CRATE, name: 'stylex-utils' });

  assert.notEqual(targetDir, '', 'cargo did not run, so the name proves nothing');
  assert.equal(path.basename(targetDir), 'coverage-stylex-utils');
});

void test(
  'a crate directory holding characters a path cannot carry is folded',
  { skip: NEEDS_BASH },
  () => {
    // Each character that is not a letter, a digit, an underscore or a hyphen
    // becomes an underscore. The last one here comes from the bracket, and not
    // from a newline.
    const { result, invocations, targetDir } = runInCrate({
      files: A_MEASURABLE_CRATE,
      name: 'odd name (v2)',
    });

    assert.equal(result.status, 0);
    assert.equal(invocations.length, 1);
    assert.equal(path.basename(targetDir), 'coverage-odd_name__v2_');
  }
);

// ── What cargo is asked to do ─────────────────────────────────────────────────

void test('the gate flags reach cargo', { skip: NEEDS_BASH }, () => {
  const { invocations } = runInCrate({ files: A_MEASURABLE_CRATE });
  const invocation = invocations[0];

  assert.equal(valueAfter(invocation, '--fail-uncovered-lines'), '0');
  assert.equal(valueAfter(invocation, '--fail-uncovered-regions'), '0');
  assert.equal(valueAfter(invocation, '--ignore-filename-regex'), '(tests?|benches?|examples)/');
  assert.ok(invocation.includes('--all-features'));
});

void test('extra arguments reach cargo', { skip: NEEDS_BASH }, () => {
  const { invocations } = runInCrate({
    files: A_MEASURABLE_CRATE,
    args: ['--html', 'some_test_name'],
  });

  assert.ok(invocations[0].includes('--html'));
  assert.ok(invocations[0].includes('some_test_name'));
});

void test('an argument holding a space stays one argument', { skip: NEEDS_BASH }, () => {
  const { invocations } = runInCrate({
    files: A_MEASURABLE_CRATE,
    args: ['--filter-expr', 'test(a) + test(b)'],
  });

  assert.ok(invocations[0].includes('test(a) + test(b)'));
});

void test('a failing cargo fails the script', { skip: NEEDS_BASH }, () => {
  // The script sets `-e`, so a red coverage run must not report a pass.
  const { result } = runInCrate({ files: A_MEASURABLE_CRATE, cargoBody: 'exit 101' });

  assert.notEqual(result.status, 0, 'a failing coverage run reported a pass');
});

// ── Inputs larger than anything the repository holds ──────────────────────────

void test(
  'a crate far larger than any in this repository is still read',
  { skip: NEEDS_BASH },
  () => {
    const { result, invocations } = runInCrate({ files: hugeCrateFiles() });

    assert.equal(result.status, 0);
    assert.equal(invocations.length, 1);
  }
);

void test(
  'a source tree nested far deeper than any crate is still read',
  { skip: NEEDS_BASH },
  () => {
    const deep = Array.from({ length: 100 }, (_unused, index) => `level_${index}`).join('/');
    const { result, invocations } = runInCrate({
      files: { 'src/lib.rs': NO_TEST, [`src/${deep}/leaf.rs`]: A_TEST },
    });

    assert.equal(result.status, 0);
    assert.equal(invocations.length, 1);
  }
);

void test('a run with no argument still starts cargo, in every bash', { skip: NEEDS_BASH }, () => {
  checkRunWithNoArgument(runInCrate, 1);
});
