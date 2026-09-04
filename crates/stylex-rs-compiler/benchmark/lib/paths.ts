/**
 * One spelling of a path on disk.
 *
 * Several readers name the same file: the file system walk, the diagnostic
 * report the runtime writes, an environment variable a caller sets, and a test
 * that built the file itself. They agree only if they settle a path the same
 * way, and a comparison of two spellings of one file is the mistake this module
 * exists to prevent.
 */

import fs from 'node:fs';

/**
 * The real path of `file`, resolved by the operating system rather than by
 * walking the path in JavaScript, so the answer carries the case the file
 * system holds instead of the case a caller typed.
 *
 * `native` is the whole point, and `fs.realpathSync` is not a substitute. Only
 * the operating system expands a Windows 8.3 short name: `TEMP` on a CI runner
 * holds `C:\Users\RUNNER~1\AppData\Local\Temp` for the account `runneradmin`,
 * and the JavaScript walk hands that back unchanged. Five binding cases failed
 * on the Windows runner, and nowhere else, because a test settled a path with
 * the other function.
 *
 * Throws when the path names nothing. A caller that treats an absent file as
 * "nothing to compare" catches it.
 */
export function realPathOf(file: string): string {
  return fs.realpathSync.native(file);
}

/**
 * `realPathOf`, or the path as given when it names nothing.
 *
 * For the readers that must key a path they cannot resolve -- a caller's
 * environment variable naming a file that is not there. Keying the raw string
 * is a worse answer than the settled one and a better answer than dropping the
 * path, which would silently empty the set a guard reads.
 */
export function settledPathOf(file: string): string {
  try {
    return realPathOf(file);
  } catch {
    return file;
  }
}
