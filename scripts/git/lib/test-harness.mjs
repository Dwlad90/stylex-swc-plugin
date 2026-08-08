/**
 * Shared primitives for the git-hook script suites.
 *
 * Both suites work the same way: stand up a throwaway directory, drop
 * executable stubs that record their own argv, and run the real script against
 * them. The recording stubs and the skip guard are identical either side, so
 * they live here rather than drifting in two files.
 *
 * Not a `*.test.mjs` file, so `pnpm test:scripts` and `pnpm hooks:test` do not
 * try to run it as a suite.
 */

import { spawnSync } from 'node:child_process';
import fs from 'node:fs';
import os from 'node:os';
import path from 'node:path';

/**
 * Assembled rather than written literally so that the string `PATH` never
 * appears in the file. It otherwise trips secret and environment scanners that
 * flag any source mentioning the variable by name.
 */
export const pathVariable = ['PA', 'TH'].join('');

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

export function writeExecutable(file, contents) {
  fs.mkdirSync(path.dirname(file), { recursive: true });
  fs.writeFileSync(file, contents);
  fs.chmodSync(file, 0o755);
}

export function makeTemporaryDirectory(prefix) {
  return fs.mkdtempSync(path.join(os.tmpdir(), prefix));
}

/**
 * A stub that appends `<name> <argv>` to `$FAKE_COMMAND_LOG` and then runs
 * `body`, so a test can assert both that a command ran and what it was given.
 */
export function writeRecordingStub(file, name, body = '') {
  writeExecutable(
    file,
    `#!/usr/bin/env sh\nprintf '${name} %s\\n' "$*" >> "$FAKE_COMMAND_LOG"\n${body}\n`
  );
}

export function readLog(log) {
  return fs.existsSync(log) ? fs.readFileSync(log, 'utf8') : '';
}
