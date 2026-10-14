/**
 * Where the fixtures of `stylex-stylesheet` are, named once.
 *
 * The stylesheet step's reference cases and its width-sort-key matrix are
 * recorded in that crate's `tests/fixtures/`; this package reads them from
 * `parity-sheets.ts` and from `__test__/processStylexRules.spec.ts`, so one
 * constant holds the path and `crates/stylex-stylesheet/CONTEXT.md` names the
 * two readers. A rename there moves this line and nothing else.
 */

import path from 'node:path';

/** `crates/stylex-stylesheet/tests/fixtures`, resolved from this package's directory. */
export function stylesheetFixturesDir(packageDir: string): string {
  return path.resolve(packageDir, '../stylex-stylesheet/tests/fixtures');
}

/** The reference's `processStylexRules` cases, recorded from `@stylexjs/babel-plugin`. */
export const PROCESS_STYLEX_RULES_CASES = 'process-stylex-rules-cases.json';

/** The at-rule chains the width sort key is tested against. */
export const WIDTH_SORT_KEY_MATRIX = 'width-sort-key-matrix.json';
