/**
 * Installs lefthook git hooks. Run from the `prepare` npm script.
 *
 * Node rather than shell because `prepare` is the one script every install runs
 * on every platform, unconditionally. npm and pnpm hand a script to `cmd.exe`
 * on Windows, which cannot execute a `*.sh` by path: the release workflow's
 * Windows build legs failed their `pnpm install` with `'.' is not recognized as
 * an internal or external command` long before the CI guard below could skip
 * anything. Node is the one interpreter guaranteed present -- the package
 * manager is already running on it.
 *
 * The defensive unset is the whole reason this wrapper exists. husky set
 * `core.hooksPath` to `.husky/_`, and lefthook refuses to install while that is
 * set: it prints advice and exits 0. On an existing clone that means
 * `pnpm install` succeeds, reports nothing unusual, and leaves the repository
 * with no hooks at all. The unset is narrowly scoped to husky's exact value so a
 * developer's intentional custom hooks path is never clobbered.
 *
 * Note this only needs to happen once per clone, not per worktree: lefthook
 * writes into the common git dir, so a single install covers every worktree.
 */

import { spawnSync } from 'node:child_process';
import path from 'node:path';
import process from 'node:process';

/**
 * npm and pnpm run `prepare` from the package root, so `cwd` is the repository
 * root. Resolving from `import.meta.url` instead would follow the symlink a
 * test fixture uses to borrow the real `scripts/` and aim the install back at
 * this checkout.
 */
const root = process.cwd();

/**
 * Addressed by path like every other npm binary in the hooks -- see the
 * `Gotchas` section of guidelines/git/HOOKS.md. It pins the install to the
 * workspace's own lefthook rather than to whatever a stale global provides.
 *
 * On Windows the installed shim is the `.cmd`, and Node refuses to spawn one
 * without a shell.
 */
const windows = process.platform === 'win32';
const lefthook = path.join(root, 'node_modules/.bin', windows ? 'lefthook.cmd' : 'lefthook');

/** `stdio: 'inherit'` so lefthook's own output reaches the installing terminal. */
function run(command, args, { capture = false } = {}) {
  return spawnSync(command, args, {
    cwd: root,
    encoding: 'utf8',
    shell: windows,
    stdio: capture ? ['ignore', 'pipe', 'inherit'] : 'inherit',
  });
}

/**
 * CI checks out fresh, never commits, and runs its own named jobs for
 * everything the hooks would do. Installing there would only mutate the
 * runner's git config for no benefit.
 *
 * Truthiness rather than `=== 'true'`: CI providers agree on setting CI, not on
 * its value.
 */
if (process.env.CI || process.env.LEFTHOOK === '0') {
  console.log('Skipping lefthook install.');
  process.exit(0);
}

// A repository with no `core.hooksPath` exits non-zero here; that is the common
// case, not an error, so only the output of a successful read is read.
const configured = run('git', ['config', '--get', 'core.hooksPath'], { capture: true });
const hooksPath = configured.status === 0 ? configured.stdout.trim() : '';

if (hooksPath === '.husky/_') {
  console.log("Removing husky's core.hooksPath so lefthook can install...");

  const unset = run('git', ['config', '--unset', 'core.hooksPath']);

  if (unset.status !== 0) {
    console.error('Failed to unset core.hooksPath; lefthook would refuse to install.');
    process.exit(unset.status ?? 1);
  }
}

// Never `--force`: that writes lefthook's hooks into husky's `.husky/_`,
// resurrecting the directory this migration deletes.
const install = run(lefthook, ['install']);

if (install.error) {
  console.error(`Failed to run ${lefthook}: ${install.error.message}`);
  process.exit(1);
}

process.exit(install.status ?? 1);
