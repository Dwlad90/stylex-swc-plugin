import { describe, expect, test } from 'vitest';

import { scanRustLiterals } from '../lib/rust-literals.js';

/** The decoded values, which is what the harvester actually consumes. */
function valuesOf(source: string): string[] {
  return scanRustLiterals(source).map(literal => literal.value);
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
    const literals = scanRustLiterals('let a = "one";\nlet b = "two";\n\nlet c = "three";');
    expect(literals.map(literal => [literal.value, literal.line])).toEqual([
      ['one', 1],
      ['two', 2],
      ['three', 4],
    ]);
  });

  test('reports offsets that bracket the literal including its delimiters', () => {
    const source = 'x("ab")';
    const [literal] = scanRustLiterals(source);
    expect(source.slice(literal!.start, literal!.end)).toBe('"ab"');
  });

  test('flags which literals were written raw', () => {
    expect(scanRustLiterals('f(r#"a"#, "b")').map(literal => literal.raw)).toEqual([true, false]);
  });
});
