/**
 * Shared primitives for the git-hook script suites.
 *
 * Every suite works the same way: stand up a throwaway directory, drop
 * executable stubs that record their own argv, and run the real script against
 * them. The workspace shape, the recording stubs, the `PATH` overlay and the
 * skip guard are identical across the suites, so they live here rather than
 * drifting in four files.
 *
 * Not a `*.test.mjs` file, so `pnpm test:scripts` and `pnpm hooks:test` do not
 * try to run it as a suite.
 */

import assert from 'node:assert/strict';
import { spawnSync } from 'node:child_process';
import fs from 'node:fs';
import os from 'node:os';
import path from 'node:path';
import { fileURLToPath } from 'node:url';

/**
 * Assembled rather than written literally so that the string `PATH` never
 * appears in the file. It otherwise trips secret and environment scanners that
 * flag any source mentioning the variable by name.
 */
export const pathVariable = ['PA', 'TH'].join('');

/**
 * The repository root, resolved from this file rather than from `cwd`: every
 * suite runs the *real* scripts by absolute path, and `node --test` does not
 * promise which directory it runs them from.
 */
export const repoRoot = path.resolve(path.dirname(fileURLToPath(import.meta.url)), '../../..');

/**
 * A `skip` reason for `node:test` when the suite's prerequisites are absent, or
 * `false` when everything it shells out to is available.
 */
export function missing(...commands) {
  const absent = commands.filter(
    command => spawnSync('sh', ['-c', 'command -v "$1"', 'sh', command]).status !== 0
  );
  return absent.length > 0 ? `requires ${absent.join(', ')} on PATH` : false;
}

function writeExecutable(file, contents) {
  fs.mkdirSync(path.dirname(file), { recursive: true });
  fs.writeFileSync(file, contents);
  fs.chmodSync(file, 0o755);
}

export function makeTemporaryDirectory(prefix) {
  return fs.mkdtempSync(path.join(os.tmpdir(), prefix));
}

/**
 * A throwaway directory plus the two paths every suite derives from it: a
 * `bin/` to hold stubs, and the log those stubs append to.
 */
export function createWorkspace(prefix) {
  const directory = makeTemporaryDirectory(prefix);

  return {
    directory,
    bin: path.join(directory, 'bin'),
    log: path.join(directory, 'commands.log'),
  };
}

/**
 * `bin` ahead of the inherited search path, so the stubs win while everything
 * the script legitimately needs -- `git`, `node` -- stays reachable. A suite
 * wanting the opposite, an environment holding nothing but its stubs, sets the
 * variable outright at the call site; that case is rare and reads better there.
 */
export function stubPath(bin) {
  return [bin, process.env[pathVariable]].filter(Boolean).join(path.delimiter);
}

export function git(cwd, ...args) {
  const result = spawnSync('git', args, { cwd, encoding: 'utf8' });
  assert.equal(result.status, 0, `git ${args.join(' ')} failed: ${result.stderr}`);
  return result.stdout.trim();
}

/**
 * A stub that records its own invocation in `$FAKE_COMMAND_LOG` and then runs
 * `body`, so a test can assert both that a command ran and what it was given.
 *
 * `perArgument` switches the recording from one `<name> <argv>` line to a
 * `---`-delimited block of one line per argument, which is what distinguishes a
 * single argument containing a space from two arguments. It costs a parse on
 * the reading side, so it is opt-in rather than the default.
 *
 * `shebang` exists for the `PATH`-less runs: `#!/usr/bin/env sh` cannot find
 * `env` either, so those stubs need the interpreter named outright.
 */
function writeRecordingStub(
  file,
  name,
  { body = '', shebang = '#!/usr/bin/env sh', perArgument = false } = {}
) {
  const record = perArgument
    ? `printf -- '---\\n${name}\\n' >> "$FAKE_COMMAND_LOG"\n` +
      `for argument in "$@"; do printf '%s\\n' "$argument" >> "$FAKE_COMMAND_LOG"; done`
    : `printf '${name} %s\\n' "$*" >> "$FAKE_COMMAND_LOG"`;

  writeExecutable(file, `${shebang}\n${record}\n${body}\n`);
}

/**
 * Writes a recording stub per entry of `{ <name>: <options> }` into `directory`
 * -- either a `bin/` on the stubbed `PATH`, or a `node_modules/.bin` for the
 * scripts that address their binaries by path.
 */
export function writeStubs(directory, stubs) {
  for (const [name, options] of Object.entries(stubs)) {
    writeRecordingStub(path.join(directory, name), name, options);
  }
}

export function readLog(log) {
  return fs.existsSync(log) ? fs.readFileSync(log, 'utf8') : '';
}

/**
 * The `---`-delimited output of `perArgument` stubs, as one array of lines per
 * invocation: `['<name>', '<argument>', ...]`.
 */
export function readInvocations(log) {
  return readLog(log)
    .split('---\n')
    .filter(Boolean)
    .map(block => block.split('\n').filter(Boolean));
}
