import { execFileSync } from 'node:child_process';
import fs from 'node:fs';
import path from 'node:path';

import { afterEach, beforeEach, describe, expect, test, vi } from 'vitest';

import { captureEnvironment } from '../lib/env.js';
import { createTempDirs } from './helpers/temp-dirs.js';

const packageDir = path.resolve(import.meta.dirname, '..', '..');
const workspaceRoot = path.resolve(packageDir, '..', '..');

// A merge SHA that GitHub recomputed away. No checkout resolves to it.
const STALE_SHA = '6d747c85ab48865d9cbd787ea2a7ea0ddd2986dc';
const OTHER_SHA = '73c212524fb4eed2787f64d50b8066f715431617';

function git(repo: string, ...args: string[]): string {
  return execFileSync('git', ['-C', repo, ...args], {
    encoding: 'utf-8',
    stdio: ['ignore', 'pipe', 'ignore'],
  }).trim();
}

/** Starts a git repository in a directory and returns the directory. */
function initRepo(repo: string): string {
  git(repo, 'init', '--initial-branch', 'main');
  git(repo, 'config', 'user.email', 'bench@example.com');
  git(repo, 'config', 'user.name', 'bench');
  return repo;
}

function commit(repo: string, message: string, contents: string): string {
  fs.writeFileSync(path.join(repo, 'file.txt'), contents);
  git(repo, 'add', 'file.txt');
  git(repo, 'commit', '-m', message);
  return git(repo, 'rev-parse', 'HEAD');
}

/** Reads the commit that the recorder gives for a directory. */
function commitOf(cwd: string): string | undefined {
  return captureEnvironment({ packageDir, workspaceRoot, cwd }).commit;
}

describe('captureEnvironment commit provenance', () => {
  const temp = createTempDirs();
  const tempDir = (prefix: string) => temp.make(prefix);

  // CI runs these tests inside a job that exports a real `GITHUB_SHA`. A case
  // that names no environment SHA would read the runner's own, so the fallback
  // order and the no-commit case would pass locally and fail in CI, on a SHA
  // that has nothing to do with the fixture. Each test declares the variables
  // it wants; the rest start out absent.
  beforeEach(() => {
    vi.stubEnv('GITHUB_SHA', undefined);
    vi.stubEnv('CI_COMMIT_SHA', undefined);
  });

  afterEach(() => {
    vi.unstubAllEnvs();
    temp.removeAll();
  });

  // On `pull_request`, GITHUB_SHA holds the merge SHA from the event payload.
  // GitHub makes the test-merge again in the background. The payload can name a
  // merge commit that the checkout replaced. If the recorder keeps that SHA, it
  // gives the numbers to a tree that no job measured. Two runs of one commit
  // then look like two commits, and runner noise looks like a regression.
  test('records the checked-out HEAD, not a stale GITHUB_SHA', () => {
    const repo = initRepo(tempDir('bench-env-'));
    const head = commit(repo, 'the tree that gets benchmarked', 'measured\n');
    vi.stubEnv('GITHUB_SHA', STALE_SHA);

    expect(commitOf(repo)).toBe(head);
  });

  test('prefers HEAD over both environment variables at once', () => {
    const repo = initRepo(tempDir('bench-env-both-'));
    const head = commit(repo, 'measured', 'measured\n');
    vi.stubEnv('GITHUB_SHA', STALE_SHA);
    vi.stubEnv('CI_COMMIT_SHA', OTHER_SHA);

    expect(commitOf(repo)).toBe(head);
  });

  test('follows HEAD when a later commit moves it', () => {
    const repo = initRepo(tempDir('bench-env-moved-'));
    const first = commit(repo, 'first', 'one\n');
    const second = commit(repo, 'second', 'two\n');

    expect(second).not.toBe(first);
    expect(commitOf(repo)).toBe(second);
  });

  // A CI checkout of `refs/pull/N/merge` has no branch. The SHA must still come
  // from the tree that the job holds.
  test('reads a detached HEAD', () => {
    const repo = initRepo(tempDir('bench-env-detached-'));
    const head = commit(repo, 'measured', 'measured\n');
    git(repo, 'checkout', '--detach', head);

    expect(commitOf(repo)).toBe(head);
  });

  test('falls back to GITHUB_SHA outside a git checkout', () => {
    const bare = tempDir('bench-env-nogit-');
    vi.stubEnv('GITHUB_SHA', STALE_SHA);

    expect(commitOf(bare)).toBe(STALE_SHA);
  });

  test('falls back to CI_COMMIT_SHA when GITHUB_SHA is absent', () => {
    const bare = tempDir('bench-env-ci-');
    vi.stubEnv('CI_COMMIT_SHA', OTHER_SHA);

    expect(commitOf(bare)).toBe(OTHER_SHA);
  });

  // An empty variable is not a SHA. Keeping it would put `"commit": ""` in the
  // artifact, which reads as a measured value.
  test('skips an empty GITHUB_SHA and uses the next source', () => {
    const bare = tempDir('bench-env-empty-');
    vi.stubEnv('GITHUB_SHA', '');
    vi.stubEnv('CI_COMMIT_SHA', OTHER_SHA);

    expect(commitOf(bare)).toBe(OTHER_SHA);
  });

  test('reports no commit when there is no checkout and no environment SHA', () => {
    const bare = tempDir('bench-env-none-');

    expect(commitOf(bare)).toBeUndefined();
  });

  // `git init` with no commit makes `rev-parse HEAD` fail. The recorder must
  // fall back and must not stop the benchmark.
  test('falls back when the repository has no commit yet', () => {
    const repo = initRepo(tempDir('bench-env-unborn-'));
    vi.stubEnv('GITHUB_SHA', STALE_SHA);

    expect(commitOf(repo)).toBe(STALE_SHA);
  });

  test('reads HEAD through a deep history and a very large commit message', () => {
    const repo = initRepo(tempDir('bench-env-deep-'));
    let head = '';
    for (let index = 0; index < 40; index += 1) {
      head = commit(repo, `step ${index} ${'x'.repeat(20_000)}`, `body ${index}\n`);
    }

    expect(commitOf(repo)).toBe(head);
  });

  test('reads HEAD from a path that holds spaces and unicode', () => {
    const parent = tempDir('bench-env-odd-');
    const repo = path.join(parent, 'rüna path — ok');
    fs.mkdirSync(repo);
    initRepo(repo);
    const head = commit(repo, 'measured', 'measured\n');

    expect(commitOf(repo)).toBe(head);
  });

  test('always returns a trimmed 40-character SHA from a checkout', () => {
    const repo = initRepo(tempDir('bench-env-shape-'));
    commit(repo, 'measured', 'measured\n');

    expect(commitOf(repo)).toMatch(/^[0-9a-f]{40}$/);
  });
});
