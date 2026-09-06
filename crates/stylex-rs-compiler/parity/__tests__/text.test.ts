import { describe, expect, test } from 'vitest';

import { withLfEndings } from '../lib/text.js';

/**
 * The one reader every comparison against a checked-in file goes through.
 *
 * What it has to get right is narrow and easy to get wrong in both directions:
 * a CRLF checkout must read as the LF repository it came from, and a carriage
 * return a test value carries on purpose must survive — the corpus holds
 * escaped values, and dropping one would change the value it was called to
 * preserve. Offsets matter too: the scanner masks a source by index, so the
 * result must be exactly as long as the input minus the pairs removed.
 */

describe('collapsing CRLF to LF', () => {
  test('a CRLF checkout reads as the LF repository it came from', () => {
    expect(withLfEndings('a\r\nb\r\n')).toBe('a\nb\n');
  });

  test('text that is already LF is handed back unchanged', () => {
    expect(withLfEndings('a\nb\n')).toBe('a\nb\n');
  });

  test('nothing at all is nothing at all', () => {
    expect(withLfEndings('')).toBe('');
  });

  test('a lone carriage return is a value, not an ending', () => {
    // `\r` alone is what a Rust test spells to put a carriage return in a CSS
    // value. Reading it as an ending would silently rewrite the value.
    expect(withLfEndings('a\rb')).toBe('a\rb');
    expect(withLfEndings('\r')).toBe('\r');
  });

  test('a carriage return in front of a pair keeps its place', () => {
    // The pair goes and the deliberate `\r` stays, so `\r\r\n` is `\r\n`.
    expect(withLfEndings('a\r\r\nb')).toBe('a\r\nb');
  });

  test('an ending split across nothing is still one ending', () => {
    // A pair at the very start and at the very end, where an off-by-one in a
    // hand-written scan would miss it.
    expect(withLfEndings('\r\n')).toBe('\n');
    expect(withLfEndings('\r\na\r\n')).toBe('\na\n');
  });

  test('mixed endings all end up as LF', () => {
    expect(withLfEndings('a\r\nb\nc\r\nd')).toBe('a\nb\nc\nd');
  });

  test('a run of endings loses one character each', () => {
    const runs = '\r\n'.repeat(1000);

    expect(withLfEndings(runs)).toBe('\n'.repeat(1000));
    expect(withLfEndings(runs).length).toBe(runs.length / 2);
  });

  test('reading twice says what reading once said', () => {
    for (const text of ['a\r\nb', 'a\rb', 'a\nb', '', '\r\n\r\n']) {
      expect(withLfEndings(withLfEndings(text)), text).toBe(withLfEndings(text));
    }
  });

  test('a carriage return in front of a pair is why one read is one read', () => {
    // Collapsing `\r\r\n` leaves `\r\n`, so a second pass would take the
    // carriage return the first pass kept. Each file is read exactly once, and
    // this case is why that is a rule rather than an accident.
    expect(withLfEndings('\r\r\n')).toBe('\r\n');
    expect(withLfEndings(withLfEndings('\r\r\n'))).toBe('\n');
  });

  test('a character outside the basic plane is carried through by offset', () => {
    // The scanner's spans are UTF-16 indices, so a reader that walked code
    // points would shift every offset past an astral character. The corpus
    // covers exactly those values in `content`.
    const emoji = 'content: "🎉";\r\nnext';

    expect(withLfEndings(emoji)).toBe('content: "🎉";\nnext');
    expect(withLfEndings(emoji).indexOf('next')).toBe(emoji.indexOf('next') - 1);
  });

  test('a lone surrogate is neither repaired nor dropped', () => {
    const half = `a\r\n\u{D83C}b`;

    expect(withLfEndings(half)).toBe(`a\n\u{D83C}b`);
  });

  test('a very large source is collapsed without failing', () => {
    // Far larger than any Rust source in the workspace, so the reader is known
    // to be linear rather than quadratic and to hold a whole file at once.
    const large = 'a'.repeat(64).concat('\r\n').repeat(200_000);

    const read = withLfEndings(large);

    expect(read.length).toBe(large.length - 200_000);
    expect(read).not.toContain('\r');
  });

  test('a source of nothing but endings is still read', () => {
    const endings = '\r\n'.repeat(500_000);

    expect(withLfEndings(endings).length).toBe(500_000);
  });
});
