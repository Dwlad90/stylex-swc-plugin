/**
 * Temporary directories for one test file.
 *
 * Several suites make directories on disk and must remove them after each
 * test. The bookkeeping is the same in each, so it lives here once.
 */

import fs from 'node:fs';
import os from 'node:os';
import path from 'node:path';

export interface TempDirs {
  /** Makes a directory and keeps it for removal. */
  make(prefix: string): string;
  /** Keeps a directory that the caller made, and returns it. */
  keep(dir: string): string;
  /** Removes every directory of this test. Call it from `afterEach`. */
  removeAll(): void;
}

export function createTempDirs(): TempDirs {
  const created: string[] = [];

  return {
    make(prefix) {
      const dir = fs.mkdtempSync(path.join(os.tmpdir(), prefix));
      created.push(dir);
      return dir;
    },
    keep(dir) {
      created.push(dir);
      return dir;
    },
    removeAll() {
      for (const dir of created.splice(0)) fs.rmSync(dir, { force: true, recursive: true });
    },
  };
}
