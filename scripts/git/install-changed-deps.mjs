#!/usr/bin/env node

/**
 * Keeps the two dependency graphs in step with the ref you just moved to.
 *
 * Invoked from `post-checkout`, `post-merge` and `post-rewrite`. The repository
 * has two lockfiles -- `pnpm-lock.yaml` and `Cargo.lock` -- and switching
 * branches across a dependency bump otherwise leaves `node_modules` describing
 * the branch you left.
 *
 * These are `post-*` hooks: ergonomics, not gates. git ignores their exit
 * status, so nothing here can fail the operation that triggered it. A non-zero
 * exit is still propagated so lefthook reports the failure rather than hiding
 * it.
 *
 * Opt out with `STYLEX_SKIP_INSTALL=1`.
 */

import { spawnSync } from 'node:child_process';

// Not a turbo input: this script only ever runs from a git hook, so declaring
// the variable in `turbo.json` would claim a relationship that does not exist.
// oxlint-disable-next-line turbo/no-undeclared-env-vars
if (process.env.STYLEX_SKIP_INSTALL) {
  process.exit(0);
}

/**
 * Every command runs from the repository root so that the lockfile pathspecs
 * below are unambiguous no matter where git invoked the hook from.
 */
const toplevel = spawnSync('git', ['rev-parse', '--show-toplevel'], { encoding: 'utf8' });

if (toplevel.status !== 0) {
  process.exit(0);
}

const cwd = toplevel.stdout.trim();

function git(...args) {
  return spawnSync('git', args, { cwd, encoding: 'utf8' });
}

/**
 * The reflog, not the hook's own argv, because one script serves three hooks
 * whose argument shapes have nothing in common. `HEAD@{1}` is the ref before the
 * move; a repository that has none -- a fresh clone, or a rewritten reflog --
 * has nothing to compare against, which is not the same as "everything changed".
 */
const previous = git('rev-parse', '--verify', '--quiet', 'HEAD@{1}');
const current = git('rev-parse', '--verify', '--quiet', 'HEAD@{0}');

if (previous.status !== 0 || current.status !== 0) {
  process.exit(0);
}

const before = previous.stdout.trim();
const after = current.stdout.trim();

/** `git diff --quiet` exits 1 for "differs" and 0 for "identical". */
function changed(lockfile) {
  return git('diff', '--quiet', before, after, '--', lockfile).status === 1;
}

/**
 * A missing tool is the state a fresh clone is in before its first install.
 * Reporting it is useful; failing on it is not, since the checkout already
 * succeeded.
 */
function install(reason, command, args) {
  process.stderr.write(`${reason} changed -- running \`${command} ${args.join(' ')}\`\n`);

  const result = spawnSync(command, args, { cwd, stdio: 'inherit' });

  if (result.error?.code === 'ENOENT') {
    process.stderr.write(`\`${command}\` is not on PATH -- skipping.\n`);
    return 0;
  }

  return result.status ?? 1;
}

const statuses = [];

if (changed('pnpm-lock.yaml')) {
  statuses.push(
    install('pnpm-lock.yaml', 'pnpm', ['install', '--prefer-offline', '--prefer-frozen-lockfile'])
  );
}

// `fetch`, never `build`. Fetch is network-only, cheap, and leaves you able to
// build offline; a `cargo build` here is the 40s-class operation that gets hooks
// disabled team-wide.
//
// Attempted even when the pnpm install above failed: the two graphs are
// independent, and short-circuiting would leave the crate cache stale for a
// reason that has nothing to do with it.
if (changed('Cargo.lock')) {
  statuses.push(install('Cargo.lock', 'cargo', ['fetch']));
}

process.exit(statuses.find(Boolean) ?? 0);
