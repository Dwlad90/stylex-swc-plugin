/**
 * Shared fixture for the three per-crate Rust scripts under
 * `scripts/packages/test`: `index.sh`, `coverage.sh` and `flamegraph.sh`.
 *
 * The three scripts read the same library, `scripts/packages/test/lib/crate.sh`,
 * so they answer "does this crate hold a test?" the same way and they fail the
 * same way. Their suites therefore stand up the same fixture: a throwaway crate
 * directory, a recording `cargo` on the search path, and a run of the real
 * script inside that directory.
 *
 * Each suite held its own copy of that fixture. The copies then diverged, which
 * is the fault `crate.sh` was written to stop in the scripts themselves. Keep
 * the fixture here, so that one correction reaches all three suites.
 *
 * Not a `*.test.mjs` file, so the test runners do not try to run it as a suite.
 */

import { spawnSync } from 'node:child_process';
import fs from 'node:fs';
import path from 'node:path';

import {
  createWorkspace,
  hermeticEnvironment,
  missing,
  pathVariable,
  readInvocations,
  stubPath,
  writeStubs,
  writeText,
} from './test-harness.mjs';

/** A source file holding the plainest of the markers the scripts look for. */
export const A_TEST = '#[test]\nfn it_holds() {}\n';

/** A source file holding no marker at all. */
export const NO_TEST = 'pub fn add(left: u64, right: u64) -> u64 { left + right }\n';

/**
 * A `skip` reason for the whole family, or `false` when both commands are
 * present. The scripts are bash and the decision they make is a grep.
 */
export const NEEDS_BASH = missing('bash', 'grep');

/** The version string of `interpreter`, or an empty string if it does not run. */
function versionOf(interpreter) {
  const result = spawnSync(interpreter, ['--version'], { encoding: 'utf8' });

  return result.status === 0 ? (result.stdout.split('\n')[0] ?? '') : '';
}

/**
 * Every distinct bash on this machine that the scripts can be started with.
 *
 * macOS ships bash 3.2 at `/bin/bash` and most developers also have a much
 * newer bash ahead of it on the search path. The two do not agree about an
 * empty array under `set -u`: bash 3.2 stops with "unbound variable" where
 * bash 4 and later print nothing. A suite that runs only the bash it finds
 * first therefore cannot see the fault that this difference causes, which is
 * why the scripts keep their arguments as "$@" rather than copying them.
 *
 * Duplicates are dropped by version, so a machine whose `/bin/bash` is the
 * bash on the search path runs each case once.
 */
export function bashInterpreters() {
  const found = new Map();

  for (const interpreter of ['bash', '/bin/bash']) {
    const version = versionOf(interpreter);

    if (version !== '' && !found.has(version)) {
      found.set(version, interpreter);
    }
  }

  return [...found].map(([version, interpreter]) => ({ interpreter, version }));
}

/**
 * Stands up one crate directory and runs `script` inside it.
 *
 * `files` is a map of crate-relative path to contents. `directories` names
 * directories to create empty, for the cases where an empty `src` or `tests` is
 * the thing under test. `cargoBody` is shell that the recording cargo runs
 * after it logs, which is how a failing cargo is put under test. `interpreter`
 * names the bash to start the script with; see `bashInterpreters`.
 *
 * The stub always writes `CARGO_TARGET_DIR` where the caller can read it back.
 * Only the coverage script gives cargo its target directory that way; for the
 * other two the value is empty, which costs nothing and keeps one code path.
 */
export function runCrateScript({
  script,
  prefix,
  files = {},
  directories = [],
  args = [],
  name = 'a-crate',
  cargoBody = '',
  interpreter = 'bash',
}) {
  const workspace = createWorkspace(prefix);
  const crate = path.join(workspace.directory, name);

  fs.mkdirSync(crate, { recursive: true });

  for (const directory of directories) {
    fs.mkdirSync(path.join(crate, directory), { recursive: true });
  }

  for (const [file, contents] of Object.entries(files)) {
    writeText(path.join(crate, file), contents);
  }

  const targetDirLog = path.join(workspace.directory, 'target-dir');
  const recordTargetDir = `printf '%s' "$CARGO_TARGET_DIR" > "$CARGO_TARGET_DIR_LOG"`;

  writeStubs(workspace.bin, {
    cargo: { perArgument: true, body: `${recordTargetDir}\n${cargoBody}` },
  });

  const result = spawnSync(interpreter, [script, ...args], {
    cwd: crate,
    encoding: 'utf8',
    env: hermeticEnvironment({
      [pathVariable]: stubPath(workspace.bin),
      FAKE_COMMAND_LOG: workspace.log,
      CARGO_TARGET_DIR_LOG: targetDirLog,
    }),
  });

  const targetDir = fs.existsSync(targetDirLog) ? fs.readFileSync(targetDirLog, 'utf8') : '';

  return { result, invocations: readInvocations(workspace.log), crate, targetDir };
}

/** The value that follows `flag` in a recorded invocation. */
export function valueAfter(invocation, flag) {
  return invocation[invocation.indexOf(flag) + 1];
}

/**
 * A crate holding thousands of files, with the one marker on the last line of a
 * file far larger than any in this repository.
 *
 * A reader with a line budget gives the wrong answer here, and a wrong answer
 * turns a whole suite off in silence. The large file is `src/lib.rs`, because
 * the coverage script measures a library only.
 */
export function hugeCrateFiles() {
  const files = { 'src/lib.rs': `${NO_TEST.repeat(20_000)}${A_TEST}` };

  for (let index = 0; index < 2_000; index += 1) {
    files[`src/module_${index}/mod.rs`] = NO_TEST;
  }

  return files;
}
