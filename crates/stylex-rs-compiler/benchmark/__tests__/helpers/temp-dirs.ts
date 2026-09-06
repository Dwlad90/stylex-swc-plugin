/**
 * Temporary directories for one test file.
 *
 * Several suites make directories on disk and must remove them after each
 * test. The bookkeeping is the same in each, so it lives here once.
 *
 * Every directory is handed back as its real path. A test compares what it
 * built against what the code under test answers, and the code resolves a path
 * through the operating system. On Windows the two are not the same string:
 * `os.tmpdir()` reads `TEMP`, which holds the 8.3 short name of a long
 * directory -- `C:\Users\RUNNER~1\AppData\Local\Temp` for the CI account
 * `runneradmin` -- and anything that resolves that path answers with the long
 * name. Five `findNativeBindings` cases failed on the Windows runner for that
 * reason and nowhere else.
 */

import fs from 'node:fs';
import os from 'node:os';
import path from 'node:path';

import { realPathOf } from '../../lib/paths.js';

export interface TempDirs {
  /** Makes a directory and keeps it for removal. Answers its real path. */
  make(prefix: string): string;
  /** Removes every directory of this test. Call it from `afterEach`. */
  removeAll(): void;
}

export function createTempDirs(): TempDirs {
  const created: string[] = [];

  return {
    make(prefix) {
      const dir = realPathOf(fs.mkdtempSync(path.join(os.tmpdir(), prefix)));
      created.push(dir);
      return dir;
    },
    removeAll() {
      for (const dir of created.splice(0)) fs.rmSync(dir, { force: true, recursive: true });
    },
  };
}
