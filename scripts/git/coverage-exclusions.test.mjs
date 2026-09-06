/**
 * Asserts that the four lists of crates excluded from coverage agree, and that
 * every name on them is a crate this workspace holds.
 *
 * The lists are copies of one decision kept by hand, in two spellings, and
 * nothing compared them. Taking `stylex_state` off three of the four left
 * `pnpm test`, `pnpm lint:check` and the coverage gate itself green, and failed
 * in the pre-push hook against a list nobody had thought to edit. The rule is
 * stated here so the fault is named where it is made.
 *
 * Most cases run against a synthetic tree, so the suite states a rule rather
 * than a snapshot of today's lists. The last case runs against the real
 * repository, which is what makes the rule load-bearing.
 */

import assert from 'node:assert/strict';
import path from 'node:path';
import test from 'node:test';

import {
  findExclusionFaults,
  readCratePackageNames,
  readExclusionLists,
} from './lib/coverage-exclusions.mjs';
import { makeTemporaryDirectory, repoRoot, writeJson, writeText } from './lib/test-harness.mjs';

/**
 * The crates a synthetic tree holds: a directory name paired with the Cargo
 * package name it declares. The two differ by more than the hyphens for one of
 * them, which is the whole reason the lists cannot be compared as text.
 */
const CRATES = [
  ['stylex-logs', 'stylex_logs'],
  ['stylex-rs-compiler', 'stylex_compiler_rs'],
  ['stylex-state', 'stylex_state'],
  ['stylex-state-index', 'stylex_state_index'],
];

/**
 * A tree holding the four lists, each given the names it should carry.
 *
 * @param {{workspace?: string[], missing?: string[], runner?: string[], suite?: string[]}} lists
 *   package names for the first two, directory names for the last two
 * @returns {string} the tree's root
 */
function createTree(lists = {}) {
  const {
    workspace = ['stylex_logs'],
    missing = ['stylex_logs'],
    runner = ['stylex-logs'],
    suite = ['stylex-logs'],
  } = lists;

  const root = makeTemporaryDirectory('stylex-coverage-exclusions-');

  for (const [directory, name] of CRATES) {
    writeText(path.join(root, 'crates', directory, 'Cargo.toml'), `[package]\nname = "${name}"\n`);
  }

  writeJson(path.join(root, 'package.json'), {
    scripts: {
      'test:coverage:workspace': [
        'cargo +nightly llvm-cov nextest --workspace --all-features',
        ...workspace.map(name => `--exclude ${name}`),
        '--fail-uncovered-lines 0',
      ].join(' '),
    },
  });

  writeText(
    path.join(root, 'scripts/coverage-missing.sh'),
    `EXCLUDED_CRATES=(\n${missing.map(name => `  ${name} # permanent\n`).join('')})\n`
  );

  writeText(
    path.join(root, 'scripts/packages/test/coverage.sh'),
    `case "$crate_name" in\n  ${runner.join('|')})\n    exit 0\n    ;;\nesac\n`
  );

  writeText(
    path.join(root, 'scripts/git/crate-coverage-runner.test.mjs'),
    `const EXCLUDED = [\n${suite.map(name => `  '${name}',\n`).join('')}];\n`
  );

  return root;
}

// ── The rule ─────────────────────────────────────────────────────────────────

void test('four lists naming one crate agree', () => {
  assert.deepEqual(findExclusionFaults(createTree()), []);
});

void test('the two spellings of one crate are read as one crate', () => {
  // `stylex-rs-compiler` is the crate `stylex_compiler_rs`. Comparing the lists
  // as text would call this a disagreement, which is why the directory names
  // are resolved through each crate's own manifest.
  const root = createTree({
    workspace: ['stylex_compiler_rs'],
    missing: ['stylex_compiler_rs'],
    runner: ['stylex-rs-compiler'],
    suite: ['stylex-rs-compiler'],
  });

  assert.deepEqual(findExclusionFaults(root), []);
});

void test('a row taken off one list is a fault naming that list', () => {
  // The regression this suite exists for, in the direction it happened: three
  // lists lost the row and the fourth kept it.
  const root = createTree({
    workspace: ['stylex_logs'],
    missing: ['stylex_logs'],
    runner: ['stylex-logs'],
    suite: ['stylex-logs', 'stylex-state'],
  });

  assert.deepEqual(findExclusionFaults(root), [
    'scripts/git/crate-coverage-runner.test.mjs excludes `stylex_state`, which package.json does not',
  ]);
});

void test('a row added to one list is a fault naming that list', () => {
  const root = createTree({ missing: ['stylex_logs', 'stylex_state'] });

  assert.deepEqual(findExclusionFaults(root), [
    'scripts/coverage-missing.sh excludes `stylex_state`, which package.json does not',
  ]);
});

void test('a row missing from one list is a fault naming that list', () => {
  const root = createTree({
    workspace: ['stylex_logs', 'stylex_state'],
    missing: ['stylex_logs', 'stylex_state'],
    runner: ['stylex-logs', 'stylex-state'],
    suite: ['stylex-logs'],
  });

  assert.deepEqual(findExclusionFaults(root), [
    'scripts/git/crate-coverage-runner.test.mjs does not exclude `stylex_state`, which package.json does',
  ]);
});

void test('a row for a crate the workspace does not hold is a fault', () => {
  // A crate that was renamed or deleted leaves a row behind that excludes
  // nothing, and every list agreeing on it hides that.
  const root = createTree({
    workspace: ['stylex_departed'],
    missing: ['stylex_departed'],
    runner: ['stylex-departed'],
    suite: ['stylex-departed'],
  });

  assert.deepEqual(findExclusionFaults(root), [
    'package.json excludes `stylex_departed`, which is no crate in this workspace',
    'scripts/coverage-missing.sh excludes `stylex_departed`, which is no crate in this workspace',
    'scripts/packages/test/coverage.sh excludes `stylex-departed`, which is no crate directory',
    'scripts/git/crate-coverage-runner.test.mjs excludes `stylex-departed`, which is no crate directory',
  ]);
});

void test('a list written in the wrong spelling is a fault', () => {
  // A directory-name list given a package name, which is the mistake the two
  // spellings invite.
  const root = createTree({ runner: ['stylex_logs'] });

  assert.deepEqual(findExclusionFaults(root), [
    'scripts/packages/test/coverage.sh excludes `stylex_logs`, which is no crate directory',
    'scripts/packages/test/coverage.sh does not exclude `stylex_logs`, which package.json does',
  ]);
});

// ── What happens when a list moves ────────────────────────────────────────────

void test('a list that cannot be read is a fault, never an empty list', () => {
  // An empty list agrees with nothing and disagrees with nothing, so a parser
  // that answered one would turn a reshuffled file into a green suite.
  const root = createTree();

  writeText(path.join(root, 'scripts/coverage-missing.sh'), '# the array moved elsewhere\n');

  assert.deepEqual(findExclusionFaults(root), [
    'scripts/coverage-missing.sh has no `EXCLUDED_CRATES` array to read',
  ]);
});

void test('a list whose file is gone is a fault', () => {
  const root = makeTemporaryDirectory('stylex-coverage-exclusions-empty-');
  const faults = findExclusionFaults(root);

  assert.equal(faults.length, 4);
  assert.ok(faults.every(fault => fault.endsWith('is missing')));
});

// ── Against the real repository ───────────────────────────────────────────────

void test('every list in this repository is readable', () => {
  // Read apart from the comparison below, so a file that moved reads as a
  // parser that needs updating rather than as a list that disagrees.
  for (const list of readExclusionLists(repoRoot)) {
    assert.equal(list.fault, null, `${list.name}: ${list.fault}`);
    assert.ok(list.names.length > 0, `${list.name} names no crate`);
  }
});

void test('every crate in this repository declares a package name', () => {
  const names = readCratePackageNames(repoRoot);

  assert.ok(names.size > 0, 'no crate manifest was read');
  assert.ok(names.has('stylex-state'), '`stylex-state` was not read');
});

void test("this repository's four lists agree", () => {
  assert.deepEqual(findExclusionFaults(repoRoot), []);
});
