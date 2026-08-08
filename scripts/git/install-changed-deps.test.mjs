import assert from 'node:assert/strict';
import { spawnSync } from 'node:child_process';
import fs from 'node:fs';
import path from 'node:path';
import test from 'node:test';

import {
  createWorkspace,
  git,
  missing,
  pathVariable,
  readLog,
  repoRoot,
  stubPath,
  writeStubs,
} from './lib/test-harness.mjs';

const script = path.join(repoRoot, 'scripts/git/install-changed-deps.mjs');

/**
 * These tests drive the real script against a real, throwaway git repository --
 * the branch it takes depends entirely on what `git diff` says about two reflog
 * entries, so stubbing git would only assert that the stub was written to match
 * the script. `pnpm` and `cargo` are stubbed, because running them for real
 * would install packages and hit the network.
 *
 * The failure this guards against is a post-checkout hook that either reinstalls
 * on every branch switch or never reinstalls at all; both are invisible until
 * someone debugs a stale `node_modules` for an afternoon.
 */
const NEEDS_GIT = missing('git', 'sh');

/**
 * A repository with both lockfiles on `main` and a `feature` branch that the
 * caller mutates, so that checking `feature` out produces exactly the reflog
 * move the script inspects.
 *
 * `stubs` defaults to recording no-ops for both package managers, since that is
 * what all but three cases want; pass `{}` to make a tool genuinely absent, or
 * a `body` to make one fail.
 */
function createRepository({ pnpmLock, cargoLock, stubs = { pnpm: {}, cargo: {} } }) {
  const { directory, bin, log } = createWorkspace('install-changed-deps-');

  writeStubs(bin, stubs);

  git(directory, 'init', '--initial-branch=main', '--quiet');
  git(directory, 'config', 'user.email', 'test@example.com');
  git(directory, 'config', 'user.name', 'Test');

  fs.writeFileSync(path.join(directory, 'pnpm-lock.yaml'), 'lockfileVersion: 9\n');
  fs.writeFileSync(path.join(directory, 'Cargo.lock'), 'version = 4\n');
  fs.writeFileSync(path.join(directory, 'README.md'), 'before\n');
  git(directory, 'add', '.');
  git(directory, 'commit', '--quiet', '-m', 'initial');

  git(directory, 'checkout', '--quiet', '-b', 'feature');
  if (pnpmLock) {
    fs.writeFileSync(path.join(directory, 'pnpm-lock.yaml'), 'lockfileVersion: 9\n# changed\n');
  }
  if (cargoLock) {
    fs.writeFileSync(path.join(directory, 'Cargo.lock'), 'version = 4\n# changed\n');
  }
  fs.writeFileSync(path.join(directory, 'README.md'), 'after\n');
  git(directory, 'add', '.');
  git(directory, 'commit', '--quiet', '-m', 'feature');

  // The move the hook reacts to: HEAD@{1} is `main`, HEAD@{0} is `feature`.
  git(directory, 'checkout', '--quiet', 'main');
  git(directory, 'checkout', '--quiet', 'feature');

  return { bin, directory, log };
}

function run({ directory, bin, log }, environment = {}) {
  const result = spawnSync(process.execPath, [script], {
    cwd: directory,
    encoding: 'utf8',
    env: {
      ...process.env,
      [pathVariable]: stubPath(bin),
      FAKE_COMMAND_LOG: log,
      STYLEX_SKIP_INSTALL: '',
      ...environment,
    },
  });

  return { ...result, log: readLog(log) };
}

void test('install-changed-deps', { skip: NEEDS_GIT }, async t => {
  await t.test('runs neither installer when no lockfile moved', () => {
    const harness = createRepository({});
    const result = run(harness);

    assert.equal(result.status, 0);
    assert.equal(result.log, '');
  });

  await t.test('installs node dependencies when pnpm-lock.yaml moved', () => {
    const harness = createRepository({ pnpmLock: true });
    const result = run(harness);

    assert.equal(result.status, 0);
    assert.match(result.log, /^pnpm install --prefer-offline --prefer-frozen-lockfile$/m);
    assert.doesNotMatch(result.log, /^cargo /m);
  });

  await t.test('fetches crates when Cargo.lock moved', () => {
    const harness = createRepository({ cargoLock: true });
    const result = run(harness);

    assert.equal(result.status, 0);
    assert.match(result.log, /^cargo fetch$/m);
    assert.doesNotMatch(result.log, /^pnpm /m);
  });

  await t.test('handles both lockfiles in one move', () => {
    const harness = createRepository({ pnpmLock: true, cargoLock: true });
    const result = run(harness);

    assert.equal(result.status, 0);
    assert.match(result.log, /^pnpm install /m);
    assert.match(result.log, /^cargo fetch$/m);
  });

  // `cargo build` after a branch switch is a 40s-class operation. Firing that
  // automatically is the hook behaviour that gets hooks disabled team-wide, so
  // the choice of `fetch` is pinned by a test rather than only by a comment.
  await t.test('never builds crates', () => {
    const harness = createRepository({ cargoLock: true });
    const result = run(harness);

    assert.doesNotMatch(result.log, /cargo (build|check|test)/);
  });

  await t.test('does nothing when STYLEX_SKIP_INSTALL is set', () => {
    const harness = createRepository({ pnpmLock: true, cargoLock: true });
    const result = run(harness, { STYLEX_SKIP_INSTALL: '1' });

    assert.equal(result.status, 0);
    assert.equal(result.log, '');
  });

  await t.test('propagates a failing install', () => {
    const harness = createRepository({
      pnpmLock: true,
      stubs: { pnpm: { body: 'exit 1' }, cargo: {} },
    });

    const result = run(harness);

    assert.notEqual(result.status, 0);
  });

  await t.test('propagates a failing cargo fetch', () => {
    const harness = createRepository({
      cargoLock: true,
      stubs: { pnpm: {}, cargo: { body: 'exit 1' } },
    });

    assert.notEqual(run(harness).status, 0);
  });

  // The two dependency graphs are independent, so a broken `pnpm install` is no
  // reason to leave the crate cache stale.
  await t.test('still fetches crates when the node install fails', () => {
    const harness = createRepository({
      pnpmLock: true,
      cargoLock: true,
      stubs: { pnpm: { body: 'exit 1' }, cargo: {} },
    });

    const result = run(harness);

    assert.notEqual(result.status, 0, 'the failure still has to surface');
    assert.match(result.log, /^cargo fetch$/m);
  });

  // A missing package manager is the state a fresh clone is in before its first
  // install. Reporting it is useful; failing on it is not, because the hook has
  // no bearing on whether the checkout succeeded.
  await t.test('warns rather than fails when a tool is absent', () => {
    const harness = createRepository({ pnpmLock: true, cargoLock: true, stubs: {} });

    // git has to stay reachable -- it is absent `pnpm` and `cargo` under test.
    const result = run(harness, {
      [pathVariable]: [harness.bin, '/usr/bin', '/bin'].join(path.delimiter),
    });

    assert.equal(result.status, 0);
    assert.match(result.stderr, /`pnpm` is not on PATH/);
    assert.match(result.stderr, /`cargo` is not on PATH/);
  });

  // A repository whose reflog has a single entry -- a fresh clone -- has no
  // previous ref to diff against, and must not be read as "everything changed".
  await t.test('exits quietly when there is no previous ref', () => {
    const harness = createRepository({ pnpmLock: true });
    fs.rmSync(path.join(harness.directory, '.git/logs'), { recursive: true, force: true });

    const result = run(harness);

    assert.equal(result.status, 0);
    assert.equal(result.log, '');
  });
});
