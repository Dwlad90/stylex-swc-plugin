import { describe, expect, test } from 'vitest';

import hasUnresolvedDefineConstAtRule from '../src/utils/hasUnresolvedDefineConstAtRule';

describe('hasUnresolvedDefineConstAtRule', () => {
  test.each([
    ['an empty stylesheet', ''],
    ['only whitespace', ' \t\n\r\f'],
    ['a plain rule', '.a{color:red}'],
    ['a resolved at-rule', '@media (min-width:1px){.a{color:red}}'],
    ['a var() used as a declaration value', '.a{color:var(--brand)}'],
    ['a var() value followed by another rule', '.a{color:var(--brand)}.b{color:red}'],
    ['a var() with a fallback value', '.a{color:var(--brand, blue)}'],
    ['an empty var() name at a rule start', 'var(--){color:red}'],
    ['a var() at a rule start with no block', 'var(--x) .a{color:red}'],
  ])('reports no unresolved at-rule for %s', (_label, css) => {
    expect(hasUnresolvedDefineConstAtRule(css)).toBe(false);
  });

  test.each([
    ['at the very start', 'var(--x){.a{color:red}}'],
    ['after a complete rule', '.a{color:red}var(--x){.b{color:blue}}'],
    ['with whitespace before the block', 'var(--x)  \n\t{.a{color:red}}'],
    ['nested inside a block', '@media screen{var(--x){.a{color:red}}}'],
    ['with a long constant name', `var(--${'a'.repeat(500)}){.a{color:red}}`],
  ])('reports an unresolved at-rule %s', (_label, css) => {
    expect(hasUnresolvedDefineConstAtRule(css)).toBe(true);
  });

  describe('comments and strings', () => {
    test.each([
      ['inside a comment', '/* var(--x){ */ .a{color:red}'],
      ['inside a double-quoted string', '.a{content:"var(--x){"}'],
      ['inside a single-quoted string', ".a{content:'var(--x){'}"],
      ['inside a string with an escaped quote', '.a{content:"\\"var(--x){"}'],
      ['inside a string holding a backslash', '.a{content:"\\\\"}var(--y) .b{color:red}'],
    ])('does not mistake a marker %s for the real thing', (_label, css) => {
      expect(hasUnresolvedDefineConstAtRule(css)).toBe(false);
    });

    test('still finds a real at-rule after a comment', () => {
      expect(hasUnresolvedDefineConstAtRule('/* comment */var(--x){.a{color:red}}')).toBe(true);
    });

    test('still finds a real at-rule after a string', () => {
      expect(hasUnresolvedDefineConstAtRule('.a{content:"x"}var(--y){.b{color:red}}')).toBe(true);
    });
  });

  // Malformed CSS reaches this scanner as readily as valid CSS, and it has to
  // answer rather than hang or throw.
  describe('malformed input', () => {
    test.each([
      ['an unterminated comment', '/* var(--x){'],
      ['an unterminated string', '.a{content:"var(--x){'],
      ['an unterminated var()', 'var(--x'],
      ['an unterminated var() after a rule', '.a{color:red}var(--x'],
      ['unbalanced closing braces', '}}}.a{color:red}'],
      ['unbalanced opening braces', '.a{color:red'],
      ['a lone opening brace', '{'],
      ['a bare var prefix', 'var(--'],
      ['a null character', '\0var(--x){.a{color:red}}'],
      ['a lone surrogate', '\uD800var(--x)'],
    ])('answers for %s without throwing', (_label, css) => {
      expect(() => hasUnresolvedDefineConstAtRule(css)).not.toThrow();
      expect(typeof hasUnresolvedDefineConstAtRule(css)).toBe('boolean');
    });
  });

  describe('large and adversarial input', () => {
    test('scans a large stylesheet with no unresolved at-rule', () => {
      const css = Array.from({ length: 50_000 }, (_, index) => `.x${index}{color:red}`).join('');

      expect(css.length).toBeGreaterThan(500_000);
      expect(hasUnresolvedDefineConstAtRule(css)).toBe(false);
    });

    test('finds an unresolved at-rule at the end of a large stylesheet', () => {
      const filler = Array.from({ length: 50_000 }, (_, index) => `.x${index}{color:red}`).join('');

      expect(hasUnresolvedDefineConstAtRule(`${filler}var(--late){.a{color:red}}`)).toBe(true);
    });

    test('does not degrade on a stylesheet that is one enormous comment', () => {
      expect(hasUnresolvedDefineConstAtRule(`/*${'var(--x){'.repeat(50_000)}*/`)).toBe(false);
    });

    test('does not degrade on deeply nested blocks', () => {
      const depth = 10_000;
      const css = `${'@media screen{'.repeat(depth)}.a{color:red}${'}'.repeat(depth)}`;

      expect(hasUnresolvedDefineConstAtRule(css)).toBe(false);
    });

    test('does not degrade on many var() declaration values', () => {
      const css = Array.from(
        { length: 20_000 },
        (_, index) => `.x${index}{color:var(--brand-${index})}`
      ).join('');

      expect(hasUnresolvedDefineConstAtRule(css)).toBe(false);
    });

    // A backslash run is where a naive string scan loses track of the closing
    // quote and starts reading declarations as string contents.
    test('tracks the end of a string through a long escape run', () => {
      const css = `.a{content:"${'\\\\'.repeat(5_000)}"}var(--x){.b{color:red}}`;

      expect(hasUnresolvedDefineConstAtRule(css)).toBe(true);
    });
  });
});
