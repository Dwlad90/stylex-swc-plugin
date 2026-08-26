const SPACE = 32;
const TAB = 9;
const LINE_FEED = 10;
const CARRIAGE_RETURN = 13;
const FORM_FEED = 12;

/**
 * CSS whitespace, by code point. Undefined for an index past the end, which is
 * not whitespace either.
 *
 * A regex test per character showed up on the dev-server hot path: the scan runs
 * over the whole collected rule set, twice per placeholder stylesheet load.
 * Comparing code points instead took a 1MB scan from 15.7ms to 4.5ms. No
 * whitespace character is a surrogate, so reading code points rather than code
 * units changes nothing here.
 */
function isWhitespace(code: number | undefined): boolean {
  return (
    code === SPACE ||
    code === TAB ||
    code === LINE_FEED ||
    code === CARRIAGE_RETURN ||
    code === FORM_FEED
  );
}

const VAR_PREFIX = 'var(--';

/**
 * Reports whether the CSS still contains an at-rule whose name is an unresolved
 * `var(--...)` reference, as in `var(--x) { ... }`.
 *
 * That happens when a `defineConsts` value was registered but not yet
 * transformed, so the metadata needed to name the at-rule is still missing. Such
 * CSS is not safe to serve, so the caller waits rather than inlining it.
 *
 * Comments and quoted strings are skipped so a `var(--x) {` written inside
 * either cannot be mistaken for the real thing.
 */
export default function hasUnresolvedDefineConstAtRule(css: string): boolean {
  // Only the start of a rule can begin an at-rule name, which is what keeps a
  // `var(--x)` used as an ordinary declaration value from matching.
  let atRuleStart = true;

  for (let index = 0; index < css.length; index += 1) {
    const character = css[index];

    if (character === '/' && css[index + 1] === '*') {
      const commentEnd = css.indexOf('*/', index + 2);
      if (commentEnd === -1) return false;
      index = commentEnd + 1;
      continue;
    }

    if (character === '"' || character === "'") {
      const quote = character;
      index += 1;

      for (; index < css.length; index += 1) {
        if (css[index] === '\\') {
          index += 1;
        } else if (css[index] === quote) {
          break;
        }
      }

      atRuleStart = false;
      continue;
    }

    if (character === '{' || character === '}') {
      atRuleStart = true;
      continue;
    }

    if (isWhitespace(css.codePointAt(index))) continue;

    if (atRuleStart && css.startsWith(VAR_PREFIX, index)) {
      const nameStart = index + VAR_PREFIX.length;
      const closingParenthesis = css.indexOf(')', nameStart);
      if (closingParenthesis === -1) return false;

      // An empty name is not a reference to anything, so only a non-empty one
      // followed by a block counts.
      if (closingParenthesis > nameStart) {
        let nextToken = closingParenthesis + 1;
        while (nextToken < css.length && isWhitespace(css.codePointAt(nextToken))) nextToken += 1;
        if (css[nextToken] === '{') return true;
      }

      index = closingParenthesis;
    }

    atRuleStart = false;
  }

  return false;
}
