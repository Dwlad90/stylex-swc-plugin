/**
 * Reading the position a refusal points at, out of what each compiler writes.
 *
 * A refused build hands an author two things: a sentence, and a place to look.
 * The value harness compares the sentence and strips the place — deliberately,
 * because the two compilers decorate a message differently and the decoration is
 * not the complaint. That leaves the place unmeasured, and it is a contract of
 * its own: a diagnostic pointing at a line that is correct as written sends the
 * reader to the wrong file position, however right the sentence is.
 *
 * The two compilers publish it in different channels, which is why this is
 * parsed rather than read off an object:
 *
 * - `@stylexjs/babel-plugin` throws the position *inside* the message, as a
 *   `@babel/code-frame` excerpt. Its error carries no `loc`, so the excerpt is
 *   the only copy.
 * - `@stylexswc/rs-compiler` writes a code frame to **stderr** and throws the
 *   sentence alone, so the position has to be captured from the process's error
 *   output rather than from the thrown value.
 */

/** One position in the fixture, 1-based on both axes, as a frame prints it. */
export interface ReportedPosition {
  line: number;
  column: number;
}

/** `line:column`, the form a corpus file pins and a report prints. */
export function formatPosition(position: ReportedPosition | undefined): string {
  return position === undefined ? '—' : `${position.line}:${position.column}`;
}

/**
 * A `@babel/code-frame` excerpt's gutter line and the caret row under it:
 *
 * ```text
 *   1 | import * as stylex from '@stylexjs/stylex';
 * > 2 | let c = 'red';
 *     |     ^^^^^^^^^
 * ```
 *
 * The marked line carries the line number and the caret row carries the column.
 * Both are found by the `|` that separates gutter from source, which is what
 * makes the column a count of characters in the source rather than in the
 * excerpt: source text can itself contain a `|`, so the *first* one on the caret
 * row is the separator and everything after it is padding and carets.
 */
const BABEL_MARKED_LINE = /^>\s*(\d+)\s*\|/;

export function babelPosition(message: string): ReportedPosition | undefined {
  const lines = message.split('\n');

  for (let index = 0; index < lines.length; index += 1) {
    const marked = BABEL_MARKED_LINE.exec(lines[index] ?? '');
    if (marked === null) continue;

    const line = Number(marked[1]);
    const column = babelColumn(lines[index + 1]);
    if (!Number.isFinite(line) || column === undefined) continue;

    return { line, column };
  }

  return undefined;
}

/**
 * The 1-based column the caret row points at, or `undefined` when the row after
 * the marked line is not one.
 *
 * A marked line without a caret row is a frame that underlines nothing, and the
 * shapes measured here always carry one — so the absence is reported rather than
 * guessed at as column 1.
 */
function babelColumn(caretRow: string | undefined): number | undefined {
  if (caretRow === undefined) return undefined;

  const separator = caretRow.indexOf('|');
  const caret = caretRow.indexOf('^');
  if (separator === -1 || caret === -1 || caret < separator) return undefined;

  // One space always follows the separator before the source begins, so the
  // first source character sits at `separator + 2` and is column 1.
  return caret - separator - 1;
}

/**
 * The location line this compiler's code frame writes above the excerpt:
 *
 * ```text
 *  --> /abs/path/to/value.js:2:5
 * ```
 *
 * The path is not matched, only skipped to the last two colon-separated numbers,
 * because an absolute path on Windows carries a colon of its own.
 */
const RUST_LOCATION_LINE = /-->\s+.*?:(\d+):(\d+)\s*$/;

export function rustPosition(stderr: string): ReportedPosition | undefined {
  for (const written of stderr.split('\n')) {
    const located = RUST_LOCATION_LINE.exec(written.trimEnd());
    if (located === null) continue;

    const line = Number(located[1]);
    const column = Number(located[2]);
    if (!Number.isFinite(line) || !Number.isFinite(column)) continue;

    return { line, column };
  }

  return undefined;
}

/**
 * What one subject's two positions add up to.
 *
 * `no-position` is kept apart from `divergent` because the two are different
 * failures: one compiler stopped without saying where, which is a hole in a
 * diagnostic rather than a disagreement about a line.
 */
export type PositionVerdict = 'identical' | 'divergent' | 'no-position' | 'not-refused';

export function positionVerdict(
  rust: ReportedPosition | undefined,
  babel: ReportedPosition | undefined,
  bothRefused: boolean
): PositionVerdict {
  if (!bothRefused) return 'not-refused';
  if (rust === undefined || babel === undefined) return 'no-position';

  return rust.line === babel.line && rust.column === babel.column ? 'identical' : 'divergent';
}
