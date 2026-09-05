/**
 * Drives the real `dedupe-catalog-pins.mjs` against throwaway lockfiles shaped
 * like this repository's: a `catalogs:` block recording a range beside the
 * version it resolved, with one package declared in two catalogs.
 *
 * The script edits a lockfile, so what the assertions watch is what it leaves
 * behind: the pins for a split package gone, every other line of the file
 * untouched. A repair that rewrites more than it was asked to is worse than no
 * repair, because the lockfile of a repository that ships native bindings is
 * not something a reviewer can read for accidental damage.
 */

import assert from 'node:assert/strict';
import { spawnSync } from 'node:child_process';
import fs from 'node:fs';
import path from 'node:path';
import test from 'node:test';

import {
  hermeticEnvironment,
  makeTemporaryDirectory,
  repoRoot,
  writeText,
} from './lib/test-harness.mjs';

/**
 * `webpack` split across the two catalogs that declare it, `@swc/core` and
 * `vitest` agreeing, and a scoped name to prove the quoting is read. The
 * trailing `importers:` block is what the assertions use to show the writer
 * stops at the end of `catalogs:`.
 */
const SPLIT_LOCK = `lockfileVersion: '9.0'

settings:
  autoInstallPeers: true

catalogs:
  bundlers:
    '@swc/core':
      specifier: ^1.15.43
      version: 1.15.43
    webpack:
      specifier: ^5.110.3
      version: 5.110.3

  peers:
    '@swc/core':
      specifier: '>=1.0.0'
      version: 1.15.43
    webpack:
      specifier: '>=5.0.0'
      version: 5.109.2

  testing:
    vitest:
      specifier: ^4.1.10
      version: 4.1.10

importers:
  .: {}
`;

const SCRIPT = path.join(repoRoot, 'scripts/git/dedupe-catalog-pins.mjs');

function createFixture(lockYaml = SPLIT_LOCK) {
  const root = makeTemporaryDirectory('stylex-dedupe-catalog-pins-');

  writeText(path.join(root, 'pnpm-lock.yaml'), lockYaml);

  return { root, lockfile: path.join(root, 'pnpm-lock.yaml') };
}

function run(root, ...args) {
  return spawnSync('node', [SCRIPT, ...args, '--root', root], {
    encoding: 'utf8',
    env: hermeticEnvironment(),
  });
}

const read = lockfile => fs.readFileSync(lockfile, 'utf8');

void test('a lockfile with no split leaves the file alone', () => {
  const agreed = SPLIT_LOCK.replace('version: 5.109.2', 'version: 5.110.3');
  const { root, lockfile } = createFixture(agreed);
  const result = run(root);

  assert.equal(result.status, 0, result.stderr);
  assert.match(result.stdout, /every catalogued package resolves to one version/);
  assert.equal(read(lockfile), agreed);
});

void test('a split package loses its pin in every catalog that declares it', () => {
  const { root, lockfile } = createFixture();
  const result = run(root);

  assert.equal(result.status, 0, result.stderr);

  const after = read(lockfile);

  assert.doesNotMatch(after, /webpack:/);
  assert.doesNotMatch(after, /5\.110\.3/);
  assert.doesNotMatch(after, /5\.109\.2/);
});

/**
 * The pins that agree are the ones a reinstall would resolve identically, so
 * dropping them would be churn in a file reviewed by diff. `vitest` also
 * guards the walk past the end of the split catalog.
 */
void test('packages that agree keep their pins', () => {
  const { root, lockfile } = createFixture();

  assert.equal(run(root).status, 0);

  const after = read(lockfile);

  assert.match(after, /'@swc\/core':/);
  assert.match(after, /version: 1\.15\.43/);
  assert.match(after, /vitest:/);
  assert.match(after, /version: 4\.1\.10/);
});

void test('the catalogs stay, and so does everything after them', () => {
  const { root, lockfile } = createFixture();

  assert.equal(run(root).status, 0);

  const after = read(lockfile);

  assert.match(after, /^catalogs:$/m);
  assert.match(after, /^ {2}bundlers:$/m);
  assert.match(after, /^ {2}peers:$/m);
  assert.match(after, /^importers:$/m);
  assert.match(after, /^ {2}\.: \{\}$/m);
  assert.match(after, /^settings:$/m);
});

/**
 * A scoped package is the one whose entry is quoted, so a writer that matched
 * the bare name would walk past it and leave the split in place.
 */
void test('a scoped package is dropped despite the quoting', () => {
  // Only the `bundlers` copy moves, so the two `@swc/core` pins disagree.
  const split = SPLIT_LOCK.replace(
    'specifier: ^1.15.43\n      version: 1.15.43',
    'specifier: ^1.15.43\n      version: 1.16.0'
  );
  const { root, lockfile } = createFixture(split);

  assert.equal(run(root).status, 0);

  const after = read(lockfile);

  assert.doesNotMatch(after, /'@swc\/core':/);
  assert.match(after, /vitest:/);
});

void test('a lockfile with no catalogs block is not a conflict', () => {
  const lockYaml = "lockfileVersion: '9.0'\n\nimporters:\n  .: {}\n";
  const { root, lockfile } = createFixture(lockYaml);

  assert.equal(run(root).status, 0);
  assert.equal(read(lockfile), lockYaml);
});

void test('a missing lockfile fails loudly rather than writing one', () => {
  const root = makeTemporaryDirectory('stylex-dedupe-catalog-pins-');
  const result = run(root);

  assert.equal(result.status, 1);
  assert.match(result.stderr, /no pnpm-lock\.yaml/);
  assert.equal(fs.existsSync(path.join(root, 'pnpm-lock.yaml')), false);
});

void test('an unknown argument is rejected rather than ignored', () => {
  const { root } = createFixture();
  const result = run(root, '--force');

  assert.equal(result.status, 1);
  assert.match(result.stderr, /unknown argument `--force`/);
});

/** The repository's own lockfile, which is the state this script defends. */
void test('the real lockfile carries no split', () => {
  const result = spawnSync('node', [SCRIPT, '--root', repoRoot], {
    encoding: 'utf8',
    env: hermeticEnvironment(),
  });

  assert.equal(result.status, 0, result.stdout + result.stderr);
  assert.match(result.stdout, /every catalogued package resolves to one version/);
});

/**
 * pnpm writes a blank line between catalogs. Dropping the last entry of one
 * used to take that separator with it, running the next catalog straight onto
 * the previous -- invisible to every assertion above, because the keys all
 * survive, and visible in the diff a reviewer reads.
 */
void test('the blank line between catalogs survives dropping the entry above it', () => {
  const { root, lockfile } = createFixture();

  assert.equal(run(root).status, 0);

  const after = read(lockfile);

  assert.match(after, /version: 1\.15\.43\n\n {2}peers:/);
  assert.match(after, /version: 1\.15\.43\n\n {2}testing:/);
});
