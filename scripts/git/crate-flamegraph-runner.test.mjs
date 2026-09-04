/**
 * The per-crate flamegraph runner, `scripts/packages/test/flamegraph.sh`.
 *
 * The script is the third reader of `scripts/packages/test/lib/crate.sh`, and
 * it was the one left out when its two siblings were repaired. It kept a copy
 * of the arguments in an array, which bash 3.2 refuses when the array is empty
 * and the script sets `-u`, and it set no options at all, so the status of a
 * red `cargo flamegraph` did not reach the caller.
 *
 * This suite holds both, and it holds the marker decision the script shares
 * with its siblings: the module gate alone is not a reason to profile, because
 * a gate with no test in it gives the profiler nothing to run.
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
  bashInterpreters,
  hugeCrateFiles,
  runCrateScript,
} from './lib/crate-script-harness.mjs';
import { repoRoot } from './lib/test-harness.mjs';

const script = path.join(repoRoot, 'scripts/packages/test/flamegraph.sh');

/** Runs the real script inside a throwaway crate. */
function runInCrate(options = {}) {
  return runCrateScript({ script, prefix: 'stylex-crate-flamegraph-runner-', ...options });
}

// ── The shape almost every crate has ──────────────────────────────────────────

void test(
  'a crate that keeps its tests in src and has no tests directory is profiled',
  { skip: NEEDS_BASH },
  () => {
    const { result, invocations } = runInCrate({ files: { 'src/lib.rs': A_TEST } });

    assert.equal(result.status, 0);
    assert.equal(invocations.length, 1, 'flamegraph did not run for a crate that holds a test');
    assert.deepEqual(invocations[0].slice(1, 4), ['flamegraph', '--root', '--test']);
  }
);

void test(
  'a crate that keeps its tests in a tests directory is profiled',
  { skip: NEEDS_BASH },
  () => {
    const { result, invocations } = runInCrate({
      files: { 'src/lib.rs': NO_TEST, 'tests/integration.rs': A_TEST },
    });

    assert.equal(result.status, 0);
    assert.equal(invocations.length, 1);
  }
);

void test('a crate holding both directories is profiled once', { skip: NEEDS_BASH }, () => {
  const { invocations } = runInCrate({
    files: { 'src/lib.rs': A_TEST, 'tests/integration.rs': A_TEST },
  });

  assert.equal(invocations.length, 1);
});

// ── Which crates are profiled ─────────────────────────────────────────────────

void test('a crate with no test marker starts no cargo', { skip: NEEDS_BASH }, () => {
  const { result, invocations } = runInCrate({ files: { 'src/lib.rs': NO_TEST } });

  assert.equal(result.status, 0);
  assert.deepEqual(invocations, []);
});

void test('the module gate alone is not a reason to profile', { skip: NEEDS_BASH }, () => {
  // This is where the script differs from the test runner beside it. A gate
  // that holds no test gives the profiler nothing to run.
  const { result, invocations } = runInCrate({
    files: { 'src/lib.rs': '#[cfg(test)]\nmod tests {}\n' },
  });

  assert.equal(result.status, 0);
  assert.deepEqual(invocations, [], 'the gate alone started a profile run');
});

void test('every marker the script reads is recognised', { skip: NEEDS_BASH }, () => {
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
  assert.equal(invocations.length, 1);
  assert.equal(result.stderr, '');
});

void test('a marker in a file that is not Rust does not count', { skip: NEEDS_BASH }, () => {
  const { invocations } = runInCrate({
    files: { 'src/lib.rs': NO_TEST, 'src/notes.md': A_TEST, 'src/build.py': A_TEST },
  });

  assert.deepEqual(invocations, []);
});

// ── What reaches cargo ────────────────────────────────────────────────────────

void test('extra arguments reach cargo after the script flags', { skip: NEEDS_BASH }, () => {
  const { invocations } = runInCrate({
    files: { 'src/lib.rs': A_TEST },
    args: ['my_bench', '--', '--exact'],
  });

  assert.deepEqual(invocations[0].slice(1), [
    'flamegraph',
    '--root',
    '--test',
    'my_bench',
    '--',
    '--exact',
  ]);
});

void test('an argument holding a space stays one argument', { skip: NEEDS_BASH }, () => {
  const { invocations } = runInCrate({
    files: { 'src/lib.rs': A_TEST },
    args: ['a name with spaces'],
  });

  assert.ok(invocations[0].includes('a name with spaces'));
  assert.equal(invocations[0].length, 5, 'the argument was split');
});

void test(
  'an argument that looks like a shell expansion is not expanded',
  { skip: NEEDS_BASH },
  () => {
    const { invocations } = runInCrate({
      files: { 'src/lib.rs': A_TEST },
      args: ['$HOME', '*', 'a;b'],
    });

    assert.deepEqual(invocations[0].slice(4), ['$HOME', '*', 'a;b']);
  }
);

void test('a run with no argument still starts cargo, in every bash', { skip: NEEDS_BASH }, () => {
  // `set -u` and an empty array do not agree in bash 3.2, which macOS ships, so
  // the arguments stay as "$@" rather than becoming a copy. Only bash 3.2 shows
  // the fault, and it is rarely the bash the search path finds first, so the
  // case runs under each bash this machine has.
  for (const { interpreter, version } of bashInterpreters()) {
    const { result, invocations } = runInCrate({
      files: { 'src/lib.rs': A_TEST },
      args: [],
      interpreter,
    });

    assert.equal(result.stderr, '', `${version} reported an unbound variable`);
    assert.equal(result.status, 0, version);
    assert.equal(invocations.length, 1, version);
  }
});

// ── What the script reports ───────────────────────────────────────────────────

void test('a failing cargo fails the script', { skip: NEEDS_BASH }, () => {
  const { result } = runInCrate({ files: { 'src/lib.rs': A_TEST }, cargoBody: 'exit 101' });

  assert.notEqual(result.status, 0, 'a failing profile run reported a pass');
});

void test('the status of a failing cargo reaches the caller', { skip: NEEDS_BASH }, () => {
  const { result } = runInCrate({ files: { 'src/lib.rs': A_TEST }, cargoBody: 'exit 3' });

  assert.equal(result.status, 3);
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

void test(
  'a crate directory holding characters a path cannot carry runs',
  { skip: NEEDS_BASH },
  () => {
    // The script names no target directory, so the crate name reaches nothing but
    // the working directory. It must still run.
    const { result, invocations } = runInCrate({
      files: { 'src/lib.rs': A_TEST },
      name: 'odd name (v2)',
    });

    assert.equal(result.status, 0);
    assert.equal(invocations.length, 1);
  }
);
