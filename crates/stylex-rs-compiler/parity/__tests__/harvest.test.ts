import fs from 'node:fs';
import os from 'node:os';
import path from 'node:path';

import { afterAll, describe, expect, test } from 'vitest';

import { declarationKey, entryId, harvestCorpus } from '../lib/harvest.js';
import type { CorpusEntry } from '../lib/types.js';

/**
 * The harvester walks `<root>/crates/<crate>` for `.rs` files, so a case is a
 * throwaway tree holding one source. Scanning the real suites is what
 * `parity:harvest` does; these pin the extractors instead.
 */
const roots: string[] = [];

function harvestOf(source: string, filename = 'tests/case.rs'): CorpusEntry[] {
  const root = fs.mkdtempSync(path.join(os.tmpdir(), 'parity-harvest-'));
  roots.push(root);
  const file = path.join(root, 'crates/stylex-css', filename);
  fs.mkdirSync(path.dirname(file), { recursive: true });
  fs.writeFileSync(file, source, 'utf8');
  return harvestCorpus({ workspaceRoot: root });
}

/** Just the declarations, which is all a corpus entry means. */
function declarationsOf(source: string, filename?: string): [string, string][] {
  return harvestOf(source, filename).map(entry => [entry.property, entry.value]);
}

afterAll(() => {
  for (const root of roots) fs.rmSync(root, { recursive: true, force: true });
});

describe('shape 1 — direct normalize_css_property_value calls', () => {
  test('takes the property and value literals as a pair', () => {
    expect(
      declarationsOf(`
        #[test]
        fn t() {
          let result = normalize_css_property_value("color", "#ff0000", &opts);
        }
      `)
    ).toEqual([['color', '#ff0000']]);
  });

  test('takes raw-string values verbatim', () => {
    expect(
      declarationsOf(`
        #[test]
        fn t() {
          normalize_css_property_value("content", r#""a\\"b""#, &opts);
        }
      `)
    ).toEqual([['content', String.raw`"a\"b"`]]);
  });

  test('skips the definition of the function itself', () => {
    expect(declarationsOf('pub fn normalize_css_property_value(a: &str, b: &str) {}')).toEqual([]);
  });
});

describe('shape 2 — case tables looped through one property', () => {
  test('takes tuple inputs and ignores the expected outputs', () => {
    expect(
      declarationsOf(`
        #[test]
        fn t() {
          let cases = [
            ("calc-size(any, 300px)", "calc-size(any,300px)"),
            ("foo * bar", "foo * bar"),
          ];
          for (value, expected) in cases {
            assert_eq!(normalize_css_property_value("height", value, &opts), expected);
          }
        }
      `)
    ).toEqual([
      ['height', 'calc-size(any, 300px)'],
      ['height', 'foo * bar'],
    ]);
  });

  test('takes every element of a flat array of inputs', () => {
    expect(
      declarationsOf(`
        #[test]
        fn t() {
          let values = ["1px", "2px"];
          for value in values {
            normalize_css_property_value("width", value, &opts);
          }
        }
      `)
    ).toEqual([
      ['width', '1px'],
      ['width', '2px'],
    ]);
  });

  test('leaves a block alone when the call already passed a literal value', () => {
    // Shape 1 covers it; re-reading the block would attribute the expected
    // output to the property as though it were an input.
    expect(
      declarationsOf(`
        #[test]
        fn t() {
          assert_eq!(normalize_css_property_value("color", "red", &opts), "red");
        }
      `)
    ).toEqual([['color', 'red']]);
  });
});

describe('shapes 3 and 4 — whole rules in one literal', () => {
  test('reads the doubled-brace form the visitor tests use', () => {
    expect(declarationsOf('let x = "* {{ transitionProperty: opacity, margin-top; }}";')).toEqual([
      ['transitionProperty', 'opacity, margin-top'],
    ]);
  });

  test('reads the minified form', () => {
    expect(declarationsOf('let x = "*{color:red}";')).toEqual([['color', 'red']]);
  });

  test('reads a custom property name', () => {
    expect(declarationsOf('let x = "*{--myVar:blue}";')).toEqual([['--myVar', 'blue']]);
  });

  test('ignores a literal that is not a whole rule', () => {
    expect(declarationsOf('let message = "some prose";')).toEqual([]);
    expect(declarationsOf('let x = "1px#000";')).toEqual([]);
  });
});

describe('shape 5 — stylex.create objects in transform tests', () => {
  test('reads every declaration in the object', () => {
    expect(
      declarationsOf(`
        stylex_test!(t, |tr| x, r#"
          const styles = stylex.create({ x: { color: 'red', marginTop: '1px' } });
        "#);
      `)
    ).toEqual([
      ['color', 'red'],
      ['marginTop', '1px'],
    ]);
  });

  test('skips interpolated values, which are not literal CSS', () => {
    expect(
      declarationsOf(`
        stylex_test!(t, |tr| x, r#"
          const styles = stylex.create({ x: { width: '\${size}px', color: 'red' } });
        "#);
      `)
    ).toEqual([['color', 'red']]);
  });

  test('skips keys that are not CSS properties', () => {
    const harvested = declarationsOf(`
      stylex_test!(t, |tr| x, r#"
        const styles = stylex.create({ x: { default: 'a', ':hover': 'b', color: 'red' } });
      "#);
    `);
    expect(harvested).toEqual([['color', 'red']]);
  });

  test('ignores a raw string with no stylex.create in it', () => {
    expect(declarationsOf('let x = r#"const a = { color: \'red\' };"#;')).toEqual([]);
  });
});

describe('collection', () => {
  test('records the path and line each declaration came from', () => {
    const [entry] = harvestOf('\n\nlet x = "*{color:red}";', 'tests/where.rs');
    expect(entry?.origin).toBe('crates/stylex-css/tests/where.rs:3');
  });

  test('collapses duplicate declarations onto the first origin seen', () => {
    const harvested = harvestOf('let a = "*{color:red}";\nlet b = "*{color:red}";');
    expect(harvested).toHaveLength(1);
    expect(harvested[0]?.origin).toMatch(/:1$/);
  });

  test('keeps declarations that differ only in property or only in value', () => {
    expect(declarationsOf('let x = "*{color:red}"; let y = "*{color:blue}";')).toHaveLength(2);
    expect(declarationsOf('let x = "*{color:red}"; let y = "*{background:red}";')).toHaveLength(2);
  });

  test('skips generated snapshot directories', () => {
    expect(declarationsOf('let x = "*{color:red}";', '__swc_snapshots__/gen.rs')).toEqual([]);
  });
});

describe('shape 6 — verdict case tables', () => {
  test('takes the property and value, and never the expected output', () => {
    expect(
      declarationsOf(`
        #[test]
        fn t() {
          check(
            &[
              same("transitionDuration", "500ms", ".5s"),
              diverges("transform", "rotate(0rad)", "rotate(0rad)", "rotate(0deg)"),
            ],
            &default_options(),
          );
        }
      `)
    ).toEqual([
      ['transform', 'rotate(0rad)'],
      ['transitionDuration', '500ms'],
    ]);
  });

  test('reads the constructor whose expectation is the input itself', () => {
    expect(
      declarationsOf(`
        #[test]
        fn t() {
          check(&[unchanged("width", "var(--x)px")], &default_options());
        }
      `)
    ).toEqual([['width', 'var(--x)px']]);
  });

  test('skips the definitions of the constructors themselves', () => {
    expect(
      declarationsOf(`
        const fn unchanged(property: &'static str, value: &'static str) -> Case {}
        const fn same(property: &'static str, value: &'static str) -> Case {}
        const fn diverges(property: &'static str, value: &'static str) -> Case {}
      `)
    ).toEqual([]);
  });

  // A short name would otherwise match the tail of a longer identifier.
  test('does not match a call whose name merely ends in one of the constructors', () => {
    expect(
      declarationsOf(`
        #[test]
        fn t() {
          assert!(is_same("color", "red"));
        }
      `)
    ).toEqual([]);
  });
});

describe('shape 7 — rejection tables', () => {
  test('takes every value in the slice, against the property before it', () => {
    expect(
      declarationsOf(`
        #[test]
        fn t() {
          rejects(
            "width",
            &["*(", "/.5 /-1 *( *3"],
            UNCLOSED_FUNCTION,
            &default_options(),
          );
        }
      `)
    ).toEqual([
      ['width', '*('],
      ['width', '/.5 /-1 *( *3'],
    ]);
  });

  // The diagnostic is a message, not a CSS value. Harvesting it would put a
  // sentence in the corpus and report it as an acceptance divergence.
  test('stops at the end of the slice, so a later literal argument is not a value', () => {
    expect(
      declarationsOf(`
        #[test]
        fn t() {
          rejects("color", &[")("], "unclosed function", &default_options());
        }
      `)
    ).toEqual([['color', ')(']]);
  });

  test('skips the definition of the runner itself', () => {
    expect(
      declarationsOf(`
        fn rejects(property: &str, values: &[&str], expected: &str) {}
      `)
    ).toEqual([]);
  });
});

describe('identity', () => {
  test('an id follows the declaration, not its position', () => {
    expect(entryId('color', 'red')).toBe(entryId('color', 'red'));
    expect(entryId('color', 'red')).not.toBe(entryId('color', 'blue'));
    expect(entryId('color', 'red')).not.toBe(entryId('background', 'red'));
  });

  test('the key separator cannot be forged out of a property and a value', () => {
    // With a printable separator, `("a b", "c")` and `("a", "b c")` collide.
    expect(declarationKey('a b', 'c')).not.toBe(declarationKey('a', 'b c'));
  });
});
