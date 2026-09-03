import { describe, expect, test } from 'vitest';

import { scanRustText } from '../lib/rust-literals.js';
import { maskNonCode } from '../lib/rust-source.js';

/** The decoded values, which is what the harvester actually consumes. */
function valuesOf(source: string): string[] {
  return scanRustText(source).literals.map(literal => literal.value);
}

/** The source with every non-code run blanked, which is what the scan is for. */
function maskOf(source: string): string {
  const { nonCode } = scanRustText(source);
  return maskNonCode(source, nonCode);
}

/** How many times `char` occurs in `text`. */
function countOf(text: string, char: string): number {
  return text.split(char).length - 1;
}

describe('cooked string literals', () => {
  test('decodes the escapes a CSS value can carry', () => {
    expect(valuesOf(String.raw`let a = "url(\"a.png\")";`)).toEqual(['url("a.png")']);
    expect(valuesOf(String.raw`let a = "a\\b";`)).toEqual(['a\\b']);
    expect(valuesOf(String.raw`let a = "a\tb\nc";`)).toEqual(['a\tb\nc']);
  });

  test('decodes unicode and hex escapes', () => {
    expect(valuesOf(String.raw`let a = "\u{1F600}";`)).toEqual(['\u{1F600}']);
    expect(valuesOf(String.raw`let a = "\x41";`)).toEqual(['A']);
  });

  test('an escaped quote does not end the literal', () => {
    // The whole point: `"a\"b"` is one value, not two.
    expect(valuesOf(String.raw`f("a\"b", "c");`)).toEqual(['a"b', 'c']);
  });

  test('a line-continuation escape eats the newline and its indentation', () => {
    expect(valuesOf('let a = "one\\\n      two";')).toEqual(['onetwo']);
  });
});

describe('raw string literals', () => {
  test('takes the body verbatim, backslashes and all', () => {
    expect(valuesOf('let a = r#"url("a\\b.png")"#;')).toEqual(['url("a\\b.png")']);
  });

  test('handles the hash-less and multi-hash forms', () => {
    expect(valuesOf('let a = r"plain";')).toEqual(['plain']);
    expect(valuesOf('let a = r##"has "# inside"##;')).toEqual(['has "# inside']);
  });

  test('an identifier ending in r does not open a raw string', () => {
    // `char` ends in `r`; the `"x"` after it is an ordinary literal.
    expect(valuesOf('let a = char"x";')).toEqual(['x']);
  });
});

describe('what the scanner must skip', () => {
  test('line comments', () => {
    expect(valuesOf('// "not a value"\nlet a = "real";')).toEqual(['real']);
  });

  test('block comments, including nested ones', () => {
    expect(valuesOf('/* "no" /* "deeper" */ "still no" */ let a = "real";')).toEqual(['real']);
  });

  test('a quote character literal does not open a phantom string', () => {
    // Without this, `'"'` would swallow everything up to the next quote.
    expect(valuesOf(`let q = '"'; let a = "real";`)).toEqual(['real']);
    expect(valuesOf(`let q = '\\''; let a = "real";`)).toEqual(['real']);
  });
});

describe('positions', () => {
  test('reports the 1-based line of each literal', () => {
    const { literals } = scanRustText('let a = "one";\nlet b = "two";\n\nlet c = "three";');
    expect(literals.map(literal => [literal.value, literal.line])).toEqual([
      ['one', 1],
      ['two', 2],
      ['three', 4],
    ]);
  });

  test('reports offsets that bracket the literal including its delimiters', () => {
    const source = 'x("ab")';
    const [literal] = scanRustText(source).literals;
    expect(source.slice(literal!.start, literal!.end)).toBe('"ab"');
  });

  test('flags which literals were written raw', () => {
    expect(scanRustText('f(r#"a"#, "b")').literals.map(literal => literal.raw)).toEqual([
      true,
      false,
    ]);
  });

  test('an unterminated literal does not report an offset past the end', () => {
    // A source that stops mid-literal is not valid Rust, but the harvester
    // reads whatever is on disk — a half-saved file, a truncated fixture. If
    // `end` ran past the source, the mask would come out longer than what it
    // masks and every offset compared against it would be off by one.
    for (const source of ['let a = "unterminated', 'let a = "trailing escape\\']) {
      const { literals, nonCode } = scanRustText(source);
      for (const literal of literals) expect(literal.end).toBeLessThanOrEqual(source.length);
      for (const span of nonCode) expect(span.end).toBeLessThanOrEqual(source.length);
      expect(maskNonCode(source, nonCode)).toHaveLength(source.length);
    }
  });
});

/**
 * What the mask removes, which is everything a bracket can hide behind.
 *
 * The harvester counts brackets to find where an argument starts and stops. A
 * bracket in a string, in prose or in a character literal is text, and one
 * counted as code puts every offset after it out of step — permanently, since
 * nothing ever closes it. So the scan reports all three and the mask blanks
 * all three.
 */
describe('the runs the mask blanks', () => {
  test('a line comment goes, and its newline stays', () => {
    // The newline is code: a masked comment must still end its own line, or
    // the code after it reads as part of the comment.
    expect(maskOf('let a = 1; // calc(\nlet b = 2;')).toBe('let a = 1;         \nlet b = 2;');
  });

  test('a block comment goes, however it is nested', () => {
    expect(maskOf('a /* one /* two */ three */ b')).toBe('a                           b');
  });

  test('a character literal goes, plain and escaped', () => {
    expect(maskOf("a.matches('(')")).toBe('a.matches(   )');
    expect(maskOf("a.matches('\\'')")).toBe('a.matches(    )');
  });

  test('a lifetime is not a character literal and stays', () => {
    // An apostrophe that opens nothing. Blanking it would take the type name
    // with it and leave the signature unreadable.
    expect(maskOf("fn f(v: &'static str) {}")).toBe("fn f(v: &'static str) {}");
  });

  test('a string goes, raw or cooked, with its delimiters', () => {
    expect(maskOf('f("a(b", r#"c)d"#)')).toBe('f(     ,         )');
  });

  test('an unterminated comment runs to the end', () => {
    expect(maskOf('a /* never closed')).toBe('a                ');
  });

  test('every span is ordered, non-overlapping and inside the source', () => {
    // The mask walks the spans once, in order, so an overlap would make the
    // result a different length from the source and invalidate every offset.
    const source = [
      'fn t() {',
      '  // a comment with calc( in it',
      "  let c = '(';",
      '  let s = "a;b{c}d";',
      '  let r = r#"raw ") text"#;',
      '  /* block ( comment */',
      '  let l: &\'static str = "x";',
      '}',
    ].join('\n');

    const { nonCode } = scanRustText(source);
    let previous = 0;
    for (const span of nonCode) {
      expect(span.start).toBeGreaterThanOrEqual(previous);
      expect(span.end).toBeGreaterThanOrEqual(span.start);
      expect(span.end).toBeLessThanOrEqual(source.length);
      previous = span.end;
    }
    expect(maskNonCode(source, nonCode)).toHaveLength(source.length);
  });

  test('holds up over a source of ten thousand runs of each kind', () => {
    const source = Array.from(
      { length: 10_000 },
      (_, index) => `  let a${index} = f("v(${index}", '(', /* ( */ r#"r(${index}"#); // ( \n`
    ).join('');

    const { literals, nonCode } = scanRustText(source);
    const masked = maskNonCode(source, nonCode);

    expect(literals).toHaveLength(20_000);
    expect(masked).toHaveLength(source.length);
    // Every parenthesis left is one the code really wrote, which is the `f(`
    // of each row and its close. The four in the value, the character
    // literal, the comment and the raw string are all gone.
    expect(countOf(masked, '(')).toBe(10_000);
    expect(countOf(masked, ')')).toBe(10_000);
  });
});
