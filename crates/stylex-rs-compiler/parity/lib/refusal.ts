/**
 * Reduces a refusal to the sentence two compilers can be compared on.
 *
 * A verdict that only asks *whether* both compilers rejected reads two
 * refusals for opposite reasons as agreement, which hides a whole class of
 * divergence: the corpus cannot report a refusal whose wording changed. The
 * blocker was never the comparison but the decoration around it — the same
 * complaint arrives wrapped differently on each side:
 *
 * ```text
 * [StyleX] a > color > Invalid pseudo or at-rule.
 * /abs/path/to/value.js: Invalid pseudo or at-rule.
 * ```
 *
 * Neither wrapper can be hard-coded away: this compiler's carries the
 * evaluator's key path, which is the authored object's own keys, and the
 * reference implementation's carries an absolute file path. So the wrapper is
 * *derived* — from the marker this compiler brands every diagnostic with, and
 * from the filename the harness itself handed both compilers — and every step
 * below is pinned by a test in `__tests__/refusal.test.ts`.
 *
 * What is stripped is decoration in the strict sense: text that says *where*
 * the refusal happened. Both compilers attach it, in different shapes — a code
 * frame there, a repaired rule and a `-->` location line here — and none of it
 * is the complaint. What survives is the complaint, newlines included: several
 * diagnostics are two sentences on two lines in both compilers, and the second
 * line carries as much of the disagreement as the first.
 */

/**
 * The marker this compiler brands every user-facing diagnostic with, from
 * `stylex_constants::logger::STYLEX_LOG_PREFIX`. It is what makes this side's
 * wrapper recognizable without knowing the key path that follows it.
 */
const STYLEX_MARKER = '[StyleX] ';

/** Separator between key-path breadcrumbs, from `StyleXError`'s `Display`. */
const BREADCRUMB = ' > ';

/**
 * How this compiler attaches the rule text a CSS lint rejected — the same job
 * the reference implementation's code frame does, spelled as a suffix. Written
 * by `stylex_css::css::normalizers::reject_value` and by
 * `normalize_css_property_value`.
 */
const RULE_SUFFIX = ', css rule: ';

/**
 * The location line `StyleXError`'s `Display` writes under the message, and the
 * stack-trace line it writes there when info logging is on.
 *
 * Both tolerate leading whitespace rather than matching the two spaces
 * `Display` currently writes: an indent is presentation, and pinning it would
 * turn every row carrying a location divergent the day it changed, which is a
 * report about this file rather than about either compiler.
 */
const LOCATION_LINE = /^\s*-->\s/;
const STACK_TRACE_LINE = /^\s*\[Stack trace\]:/;

/**
 * A line of a `@babel/code-frame` excerpt: a gutter carrying a line number, or
 * the caret row under it, which has no number but is indented to line up with
 * the ones that have. The indent is required, so a complaint whose own second
 * line opens with a pipe — CSS grammar quoted in advice text — is not read as a
 * caret row.
 *
 * Matched rather than counted from a blank line, because a refusal's own text
 * can be several lines and a blank line between them would end the sentence
 * early.
 */
const CODE_FRAME_LINE = /^\s*(?:>\s*)?\d+\s*\||^\s+\|/;

/**
 * SGR escape sequences, in case this compiler's `colored` output decides a
 * terminal is attached. The harness never wants them and the reference
 * implementation never writes them, so a coloured diagnostic would otherwise
 * read as divergent for its colours.
 */
// The escape is the whole point of the pattern: an SGR sequence is what a
// coloured diagnostic is made of, and there is no spelling of it that does not
// match a control character.
// oxlint-disable-next-line no-control-regex
const SGR = /\u001B\[[0-9;]*m/g;

/**
 * The complaint inside one compiler's refusal, with the decoration that says
 * where it happened removed.
 *
 * `filename` is the path the harness handed *both* compilers — the reference
 * implementation prefixes its message with it, and this compiler names it in a
 * location line. Passing it in is what keeps the rule derived: the one absolute
 * path involved is the one the caller chose.
 */
export function refusalSentence(message: string, filename: string): string {
  const text = message.replace(SGR, '');

  // Which compiler wrote it decides which decoration may come off, and the
  // marker is the only thing that says so. Reducing every message by both sets
  // of rules is how a refusal carrying neither -- upstream's, or a bare
  // `TypeError` out of either side -- loses text to a rule written for the
  // other: an upstream complaint quoting author CSS that spells `a > b` would
  // be cut at the quote.
  return text.startsWith(STYLEX_MARKER)
    ? branded(text.slice(STYLEX_MARKER.length))
    : unbranded(text, filename);
}

/** Reduces this compiler's own diagnostic, marker already off. */
function branded(text: string): string {
  // Ahead of the breadcrumbs, because a rejected rule is arbitrary CSS and can
  // spell `a > b`. Stripping it first is what keeps a child selector in the
  // author's value from being read as a key path.
  const ruleSuffix = text.indexOf(RULE_SUFFIX);
  const complaint = ruleSuffix === -1 ? text : text.slice(0, ruleSuffix);

  return upTo(withoutBreadcrumbs(complaint), isLocation);
}

/**
 * Reduces a refusal this compiler did not brand: upstream's, which carries the
 * filename it was handed and a code frame, or a bare throw from either side,
 * which carries neither and comes back as it went in.
 */
function unbranded(text: string, filename: string): string {
  const filenamePrefix = `${filename}: `;
  const complaint = text.startsWith(filenamePrefix) ? text.slice(filenamePrefix.length) : text;

  return upTo(complaint, isCodeFrame);
}

/** `text` up to the first line `ends` accepts, trimmed. */
function upTo(text: string, ends: (line: string) => boolean): string {
  const lines: string[] = [];
  for (const line of text.split('\n')) {
    if (ends(line)) break;
    lines.push(line);
  }

  return lines.join('\n').trim();
}

/**
 * `text` with the `key > path > ` breadcrumbs of a `StyleXError` removed.
 *
 * Taken as everything up to the **last** separator on the first line: a key
 * path is many segments deep and dropping only the first would leave the rest
 * in front of the sentence. Bounded to the first line because a two-line
 * complaint's second line is not a place a breadcrumb can appear, and searching
 * it would let a sentence containing the separator eat its own first line.
 *
 * A message whose own first line contains the separator loses its head, and one
 * diagnostic can: `Invalid media query: {query}` echoes the author's query, and
 * a media range condition is spelled `(width > 600px)`. The CSS-lint refusals
 * are the other interpolating family and their rule text is already off the end
 * by the time this runs.
 *
 * Left as it is, because the direction it can fail in is the harmless one. A
 * verdict only ever compares the two messages of *one* subject, so a mangled
 * sentence cannot collide with another subject's; within a subject, a mangling
 * of this side's text against upstream's intact text reads unequal, which is
 * `both-reject-divergent` — and two compilers that word a media-query refusal
 * differently is what that says. Producing a false *agreement* would need the
 * mangled text to land exactly on upstream's sentence, and that failure is the
 * one the harness already had before any of this compared wording at all.
 */
function withoutBreadcrumbs(text: string): string {
  const firstNewline = text.indexOf('\n');
  const firstLine = firstNewline === -1 ? text : text.slice(0, firstNewline);
  const lastBreadcrumb = firstLine.lastIndexOf(BREADCRUMB);
  return lastBreadcrumb === -1 ? text : text.slice(lastBreadcrumb + BREADCRUMB.length);
}

/** Whether a line is where this compiler's complaint stops and its location starts. */
function isLocation(line: string): boolean {
  return LOCATION_LINE.test(line) || STACK_TRACE_LINE.test(line);
}

/** Whether a line is where upstream's complaint stops and its code frame starts. */
function isCodeFrame(line: string): boolean {
  return CODE_FRAME_LINE.test(line);
}
