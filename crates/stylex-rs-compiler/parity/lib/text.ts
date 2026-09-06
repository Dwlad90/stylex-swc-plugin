/**
 * Reading checked-in text the same way on every platform.
 *
 * Two places in this directory compare text against a file in the repository:
 * the scanner reads Rust sources whose literals become corpus values, and the
 * harvest check compares the corpus it would write against the copy that is
 * committed. Git hands a Windows working tree CRLF when `core.autocrlf` is on,
 * which is the default of Git for Windows, so both comparisons would answer
 * differently on a Windows checkout than on a Linux one while the repository
 * holds exactly the same bytes.
 *
 * `.gitattributes` asks for LF in every working tree, which is the fix for the
 * checkout. This is the fix for the *comparison*: what a checked-in file says
 * is its content, and its line endings are the checkout's business.
 */

/**
 * `text` with every CRLF pair collapsed to a single LF.
 *
 * Only the pair is collapsed. A lone carriage return is left where it is,
 * because a CSS value in a Rust test may carry one on purpose — an escape a
 * declaration is meant to keep — and a reader that dropped it would change the
 * value it was called to preserve. So `\r\r\n` becomes `\r\n`: the pair goes,
 * the deliberate carriage return in front of it stays.
 *
 * That is also why text is read through this once and not again: a second pass
 * over `\r\n` would take a carriage return the first pass was right to keep.
 */
export function withLfEndings(text: string): string {
  return text.replaceAll('\r\n', '\n');
}
