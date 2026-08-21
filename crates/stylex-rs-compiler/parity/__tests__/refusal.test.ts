import { describe, expect, test } from 'vitest';

import { refusalSentence } from '../lib/refusal.js';

/**
 * The path both compilers are handed by `lib/compare.ts`. Spelled out here
 * rather than imported because the point of passing it in is that the caller
 * chooses it: a test that read the same constant the implementation reads would
 * not notice a rule that stopped deriving anything from it.
 */
const FILENAME = '/abs/path/to/parity/__fixture__/value.js';

const sentence = (message: string): string => refusalSentence(message, FILENAME);

/**
 * What both compilers say about the same input, once the text saying *where*
 * is off. The verdict `both-reject` rests on this equality and
 * `both-reject-divergent` on its failure, so every shape either compiler
 * actually produces is pinned here — the harness cannot report a changed
 * refusal any more accurately than this function reduces one.
 */
describe('refusalSentence', () => {
  test('drops the marker and the key path this compiler prefixes', () => {
    expect(sentence('[StyleX] a > color > Invalid pseudo or at-rule.')).toBe(
      'Invalid pseudo or at-rule.'
    );
  });

  test('drops the marker where there is no key path at all', () => {
    expect(sentence('[StyleX] Invalid pseudo or at-rule.')).toBe('Invalid pseudo or at-rule.');
  });

  test('drops a key path of one segment as readily as of three', () => {
    expect(sentence('[StyleX] w > zIndex > A style value can only contain a string.')).toBe(
      'A style value can only contain a string.'
    );
    expect(sentence('[StyleX] w > A style value can only contain a string.')).toBe(
      'A style value can only contain a string.'
    );
  });

  test('drops the filename the reference implementation prefixes', () => {
    expect(sentence(`${FILENAME}: Invalid pseudo or at-rule.`)).toBe('Invalid pseudo or at-rule.');
  });

  test('leaves a refusal carrying no prefix of either kind alone', () => {
    // Not every throw on either side goes through a diagnostic builder: a
    // harness bug, a NAPI-level failure, or a `TypeError` out of the plugin
    // arrives bare. Reducing one to nothing would make two unrelated failures
    // compare equal.
    expect(sentence('Cannot read properties of undefined')).toBe(
      'Cannot read properties of undefined'
    );
  });

  test('keeps a colon inside the complaint', () => {
    // `Unsupported expression: SpreadElement` is the most common refusal in the
    // corpus, and the reference implementation's prefix ends in a colon too.
    // Splitting on the first colon would leave the node kind alone and throw
    // the complaint away.
    expect(sentence('[StyleX] x > color > Unsupported expression: SpreadElement')).toBe(
      'Unsupported expression: SpreadElement'
    );
    expect(sentence(`${FILENAME}: Unsupported expression: SpreadElement`)).toBe(
      'Unsupported expression: SpreadElement'
    );
  });

  test('keeps a key path segment that is itself a selector carrying a colon', () => {
    expect(sentence('[StyleX] a > :hover > color > Invalid pseudo or at-rule.')).toBe(
      'Invalid pseudo or at-rule.'
    );
    expect(
      sentence('[StyleX] a > @media (min-width: 100px) > color > Invalid pseudo or at-rule.')
    ).toBe('Invalid pseudo or at-rule.');
  });

  test('keeps a complaint that runs to a second line', () => {
    // Both compilers write this one as two lines, and the second carries the
    // advice. Cutting at the first newline would compare two refusals on their
    // shared opening and call unrelated advice agreement.
    const complaint =
      'There was an error when attempting to evaluate the imported file.\n' +
      'Please ensure that the imported file is self-contained and does not rely on dynamic behavior.';
    expect(sentence(`[StyleX] wrapper > color > ${complaint}\n`)).toBe(complaint);
    expect(sentence(`${FILENAME}: ${complaint}\n`)).toBe(complaint);
  });

  test('drops the location line this compiler writes under the complaint', () => {
    expect(
      sentence(
        `[StyleX] A style array value can only contain strings or numbers.\n  --> ${FILENAME}:1`
      )
    ).toBe('A style array value can only contain strings or numbers.');
    expect(
      sentence(`[StyleX] A style value can only contain a string.\n  --> ${FILENAME}:12:34`)
    ).toBe('A style value can only contain a string.');
  });

  test('drops the stack-trace line info logging adds', () => {
    expect(
      sentence(
        `[StyleX] A style value can only contain a string.\n  --> ${FILENAME}:1\n[Stack trace]: crates/stylex-css/src/css/common.rs:701`
      )
    ).toBe('A style value can only contain a string.');
  });

  test('drops the repaired rule this compiler appends to a CSS refusal', () => {
    // The rule text is this compiler's answer to *where*, which is the job the
    // reference implementation's code frame does. Both are decoration around
    // one shared sentence.
    expect(
      sentence('[StyleX] Rule contains an unclosed function, css rule: * { width: calc(1px }')
    ).toBe('Rule contains an unclosed function');
    expect(sentence(`${FILENAME}: Rule contains an unclosed function`)).toBe(
      'Rule contains an unclosed function'
    );
  });

  test('keeps the detail a CSS refusal carries ahead of the rule', () => {
    expect(
      sentence(
        '[StyleX] Rule contains a value nested more deeply than the compiler supports ' +
          '(limit 50, found 51), css rule: * { width: x }'
      )
    ).toBe(
      'Rule contains a value nested more deeply than the compiler supports (limit 50, found 51)'
    );
  });

  test('reads a child selector in a rejected rule as rule text, not as a key path', () => {
    // The rule is arbitrary author CSS and can spell the breadcrumb separator.
    // Removing the rule before looking for breadcrumbs is what keeps the
    // complaint from being eaten by the value it is complaining about.
    expect(
      sentence('[StyleX] Rule contains an unclosed function, css rule: * { width: a > b ( }')
    ).toBe('Rule contains an unclosed function');
  });

  test('drops the code frame the reference implementation appends', () => {
    const framed =
      `${FILENAME}: Unsupported expression: SpreadElement\n\n\n` +
      '  2 |\n' +
      '  3 | export const styles = stylex.create({\n' +
      '> 4 |   spread: { content: [...[1, 2]].length },\n' +
      '    |                       ^^^^^^^^^\n' +
      '  5 | });\n' +
      '  6 |';
    expect(sentence(framed)).toBe('Unsupported expression: SpreadElement');
  });

  test('drops a code frame whose complaint ran to a second line', () => {
    const framed =
      `${FILENAME}: Unexpected error:\nCould not resolve the code being evaluated.\n\n` +
      "  1 | import * as stylex from '@stylexjs/stylex';\n" +
      '    | ^\n';
    expect(sentence(framed)).toBe('Unexpected error:\nCould not resolve the code being evaluated.');
  });

  test('drops SGR escapes, so a coloured refusal is not divergent for its colours', () => {
    // This compiler's `colored` output decides for itself whether a terminal is
    // attached, and the reference implementation never colours anything.
    expect(
      sentence('\u001B[94m\u001B[1m[StyleX]\u001B[0m \u001B[31mInvalid pseudo or at-rule.\u001B[0m')
    ).toBe('Invalid pseudo or at-rule.');
  });

  test('trims the trailing blank lines either compiler leaves behind', () => {
    expect(sentence('[StyleX] a > color > Referenced value is used before declaration.\n\n')).toBe(
      'Referenced value is used before declaration.'
    );
  });

  test('answers the empty string for a refusal that carried no text', () => {
    expect(sentence('')).toBe('');
    expect(sentence('[StyleX] ')).toBe('');
    expect(sentence(`${FILENAME}: `)).toBe('');
  });

  test('leaves a message alone when the filename prefix is a different file', () => {
    // The prefix is derived from the path the harness handed both compilers, so
    // a message naming some other file is not one this rule may cut into — an
    // imported theme file, for instance, whose own path a refusal can name.
    const other = '/abs/path/to/other.stylex.js: Referenced constant is not defined.';
    expect(sentence(other)).toBe(other);
  });

  test('reduces the two spellings of one complaint to the same sentence', () => {
    // The equality the `both-reject` verdict is decided by, on the pair the
    // harness reads most: an unclosed construct, where each compiler attaches
    // its own kind of location.
    expect(
      sentence('[StyleX] Rule contains an unclosed string, css rule: * { fontFamily: "solid }')
    ).toBe(sentence(`${FILENAME}: Rule contains an unclosed string`));
  });

  test("applies this compiler's rules only to what this compiler branded", () => {
    // The breadcrumb and rule-text rules are written for one side's decoration,
    // and a message carrying neither -- upstream's, or a bare throw out of
    // either side -- must not lose text to them. Upstream quotes author CSS in
    // some of its complaints, and author CSS spells `a > b`.
    const quoted = `${FILENAME}: Unknown property: "a > b"`;
    expect(sentence(quoted)).toBe('Unknown property: "a > b"');

    const bare = 'Unexpected token, css rule: it has one';
    expect(sentence(bare)).toBe(bare);
  });

  test("keeps a code frame from cutting into this compiler's own complaint", () => {
    // The mirror of the rule above: a caret row is upstream's decoration, and a
    // branded complaint whose second line opens with a pipe -- CSS grammar
    // quoted as advice -- keeps it.
    const advice = 'Invalid value.\n  | a | b';
    expect(sentence(`[StyleX] x > color > ${advice}`)).toBe(advice);
  });

  test('tolerates a change of indent in the location line', () => {
    // The indent is presentation. Pinning it would turn every row carrying a
    // location divergent the day `StyleXError`'s `Display` changed it, which
    // reports on this file rather than on either compiler.
    expect(sentence(`[StyleX] Invalid pseudo or at-rule.\n    --> ${FILENAME}:1`)).toBe(
      'Invalid pseudo or at-rule.'
    );
    expect(sentence(`[StyleX] Invalid pseudo or at-rule.\n--> ${FILENAME}:1`)).toBe(
      'Invalid pseudo or at-rule.'
    );
  });

  test('loses the head of a branded complaint that quotes the separator itself', () => {
    // The one known limit, pinned so it is a decision rather than a surprise:
    // `Invalid media query: {query}` echoes the author's query, and a media
    // range condition is spelled `(width > 600px)`. Harmless in the direction
    // that matters -- a verdict only ever compares one subject's two messages,
    // so a mangled sentence reads unequal against upstream's intact one, which
    // is `both-reject-divergent`, and two compilers wording a media-query
    // refusal differently is what that says.
    expect(sentence('[StyleX] Invalid media query: @media (width > 600px)')).toBe('600px)');
  });

  test('keeps two genuinely different complaints apart', () => {
    expect(sentence('[StyleX] a > A style value can only contain a string.')).not.toBe(
      sentence(`${FILENAME}: Invalid pseudo or at-rule.`)
    );
  });
});
