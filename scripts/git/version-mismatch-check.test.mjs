/**
 * Drives the real `version-mismatch-check.sh` -- the script the lefthook
 * `version-mismatch` pre-commit job and the `pr-validation` matrix both invoke.
 * Neither call site passes arguments or reads anything but the exit status, so
 * this suite is what stands in for both of them: what it asserts here is
 * exactly what those two see.
 *
 * The fixture symlinks the real `scripts/` in and stubs only `syncpack`, so
 * the catalog half runs for real against a throwaway workspace while the
 * syncpack half is free to succeed or fail on demand. Stubbing both would
 * leave the suite asserting that the script calls two commands, which is the
 * wiring rather than the behaviour.
 */

import assert from 'node:assert/strict';
import { spawnSync } from 'node:child_process';
import fs from 'node:fs';
import path from 'node:path';
import test from 'node:test';

import {
  hermeticEnvironment,
  makeTemporaryDirectory,
  readLog,
  repoRoot,
  writeJson,
  writeStubs,
} from './lib/test-harness.mjs';

const WORKSPACE_YAML = `packages:
  - packages/*

catalogs:
  bundlers:
    webpack: '^5.109.2'
`;

/**
 * The lockfile's record of the catalog above. The script now also asserts that
 * a package catalogued twice resolves to one version, and that assertion reads
 * this file -- a fixture without one is not shaped like the repository.
 */
const LOCK_YAML = `lockfileVersion: '9.0'

catalogs:
  bundlers:
    webpack:
      specifier: '^5.109.2'
      version: 5.109.2

importers:
  .: {}
`;

const SYNCPACK = { source: ['**/package.json', '!**/node_modules/**'] };

/**
 * @param {{syncpackExit?: number, webpack?: string, lockYaml?: string}} [overrides]
 */
function createFixture(overrides = {}) {
  const root = makeTemporaryDirectory('stylex-version-mismatch-');
  const log = path.join(root, 'commands.log');

  fs.symlinkSync(path.join(repoRoot, 'scripts'), path.join(root, 'scripts'));
  fs.mkdirSync(path.join(root, 'node_modules/.bin'), { recursive: true });
  writeStubs(path.join(root, 'node_modules/.bin'), {
    syncpack: { body: `exit ${overrides.syncpackExit ?? 0}` },
  });

  fs.writeFileSync(path.join(root, 'pnpm-workspace.yaml'), WORKSPACE_YAML);
  fs.writeFileSync(path.join(root, 'pnpm-lock.yaml'), overrides.lockYaml ?? LOCK_YAML);
  writeJson(path.join(root, '.syncpackrc'), SYNCPACK);
  writeJson(path.join(root, 'package.json'), { name: 'root' });
  writeJson(path.join(root, 'packages/app/package.json'), {
    name: 'app',
    devDependencies: { webpack: overrides.webpack ?? 'catalog:bundlers' },
  });

  return { root, log };
}

function run(root, log) {
  return spawnSync('./scripts/git/version-mismatch-check.sh', {
    cwd: root,
    encoding: 'utf8',
    shell: false,
    env: hermeticEnvironment({ FAKE_COMMAND_LOG: log }),
  });
}

void test('a catalogued workspace with syncpack happy passes', () => {
  const { root, log } = createFixture();
  const result = run(root, log);

  assert.equal(result.status, 0, result.stderr);
  assert.match(readLog(log), /^syncpack lint$/m);
  assert.match(result.stdout, /catalog-integrity: manifests ok/);
  assert.match(result.stdout, /All dependencies are in sync/);
});

void test('a literal range fails the check both call sites run', () => {
  const { root, log } = createFixture({ webpack: '^5.109.2' });
  const result = run(root, log);

  assert.equal(result.status, 1);
  assert.match(result.stderr, /packages\/app\/package\.json.*devDependencies\.webpack/);
  assert.match(result.stderr, /catalog:bundlers/);
  assert.doesNotMatch(result.stdout, /All dependencies are in sync/);
});

void test('a syncpack failure still runs the catalog check, and both are reported', () => {
  const { root, log } = createFixture({ syncpackExit: 1, webpack: '^5.109.2' });
  const result = run(root, log);

  assert.equal(result.status, 1);
  assert.match(result.stdout, /pnpm syncpack fix/);
  assert.match(result.stderr, /devDependencies\.webpack/);
});

void test('a syncpack failure alone fails the check', () => {
  const { root, log } = createFixture({ syncpackExit: 1 });
  const result = run(root, log);

  assert.equal(result.status, 1);
  assert.match(result.stdout, /catalog-integrity: manifests ok/);
});

/**
 * The third assertion, reached through the script rather than directly: a
 * package catalogued twice that the lockfile pins to two versions. Both other
 * assertions pass here, so a failure can only come from this one -- which is
 * the point, since the split is invisible in the manifests they read.
 */
void test('a package pinned to two versions fails the check', () => {
  const { root, log } = createFixture({
    lockYaml: `lockfileVersion: '9.0'

catalogs:
  bundlers:
    webpack:
      specifier: '^5.109.2'
      version: 5.109.2

  peers:
    webpack:
      specifier: '>=5.0.0'
      version: 5.110.3

importers:
  .: {}
`,
  });
  const result = run(root, log);

  assert.equal(result.status, 1);
  assert.match(result.stderr, /`webpack` resolves to/);
  assert.match(result.stdout, /manifests ok/);
});
