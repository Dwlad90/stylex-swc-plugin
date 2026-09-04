/**
 * The per-crate Rust test runner, `scripts/packages/test/index.sh`.
 *
 * The script decides whether a crate holds a Rust test before it starts cargo,
 * and that decision is the whole of its behaviour: answer "no" for a crate that
 * does hold one and the crate's suite disappears from the run without a word,
 * which is the failure this suite exists to catch. It once did exactly that.
 * Every crate but one keeps its tests beside the code and has no `tests/`
 * directory, and the reader named `tests` anyway; a missing path is an error to
 * grep, and that error outranks the match found in `src/` beside it.
 *
 * Runs the real script against throwaway crates with a recording `cargo` on the
 * search path, so what is asserted is what cargo was asked to do.
 */

import assert from 'node:assert/strict';
import { spawnSync } from 'node:child_process';
import fs from 'node:fs';
import path from 'node:path';
import test from 'node:test';

import {
  createWorkspace,
  hermeticEnvironment,
  missing,
  readInvocations,
  repoRoot,
  stubPath,
  writeStubs,
  writeText,
} from './lib/test-harness.mjs';

const script = path.join(repoRoot, 'scripts/packages/test/index.sh');

const NEEDS_BASH = missing('bash', 'grep');

/** A source file holding the plainest of the four markers the script looks for. */
const A_TEST = '#[test]\nfn it_holds() {}\n';

/** A source file holding no marker at all. */
const NO_TEST = 'pub fn add(left: u64, right: u64) -> u64 { left + right }\n';

/**
 * Stands up one crate directory and runs the script inside it.
 *
 * `files` is a map of crate-relative path to contents. `directories` names
 * directories to create empty, for the cases where an empty `src` or `tests`
 * is the thing under test.
 */
function runInCrate({ files = {}, directories = [], args = [], name = 'a-crate' } = {}) {
  const workspace = createWorkspace('stylex-crate-test-runner-');
  const crate = path.join(workspace.directory, name);

  fs.mkdirSync(crate, { recursive: true });

  for (const directory of directories) {
    fs.mkdirSync(path.join(crate, directory), { recursive: true });
  }

  for (const [file, contents] of Object.entries(files)) {
    writeText(path.join(crate, file), contents);
  }

  // A cargo that records its argv and nothing else. The script runs two cargo
  // calls in a row and does not read their output, so a stub that only logs is
  // the whole of what it needs.
  writeStubs(workspace.bin, { cargo: { perArgument: true } });

  const result = spawnSync('bash', [script, ...args], {
    cwd: crate,
    encoding: 'utf8',
    env: hermeticEnvironment({
      [['PA', 'TH'].join('')]: stubPath(workspace.bin),
      FAKE_COMMAND_LOG: workspace.log,
    }),
  });

  return { result, invocations: readInvocations(workspace.log), crate };
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
    const target = invocations[0][invocations[0].indexOf('--target-dir') + 1];

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
    const target = invocations[0][invocations[0].indexOf('--target-dir') + 1];

    assert.ok(target.endsWith(path.join('target', 'test-odd_name__v2_')), target);
    assert.equal(invocations[0].filter(argument => argument.includes('test-odd')).length, 1);
  }
);

void test(
  'a crate far larger than any in this repository is still read',
  { skip: NEEDS_BASH },
  () => {
    // Thousands of files, one of them holding the marker on its last line, plus
    // one file large enough that a reader with a line budget would give up.
    const files = { 'src/huge.rs': `${NO_TEST.repeat(20_000)}${A_TEST}` };

    for (let index = 0; index < 2_000; index += 1) {
      files[`src/module_${index}/mod.rs`] = NO_TEST;
    }

    const { result, invocations } = runInCrate({ files });

    assert.equal(result.status, 0);
    assert.equal(invocations.length, 2);
  }
);
