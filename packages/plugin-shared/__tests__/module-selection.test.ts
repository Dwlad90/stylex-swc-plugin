// Shared by all five bundler plugins, so it is tested here directly rather
// than through any one of them.
import { describe, expect, test } from 'vitest';

import { shouldProcessSource } from '../src/module-selection';

const IMPORTING_MODULE = "import * as stylex from '@stylexjs/stylex';";

// A leaf component that only forwards the prop has nothing to import, so the
// import scan alone would drop it and leave the element unstyled.
const NO_IMPORT = { importSources: ['@stylexjs/stylex'] };

// Every plugin runs the scan on every module, so a slow answer is felt on each
// build. The large cases below measure near one millisecond, so this budget
// leaves about fifty times the room. That is enough for a cold or loaded
// machine, and still fails long before a pattern that backtracks: the shape
// these cases catch took sixty seconds.
const BUDGET_MS = 50;

function timedScan(sourceCode: string): [boolean, number] {
  const startedAt = performance.now();
  const answer = shouldProcessSource(sourceCode, NO_IMPORT);

  return [answer, performance.now() - startedAt];
}

describe('shouldProcessSource', () => {
  describe('no import source to look for', () => {
    test('skips the module when the option is missing', () => {
      expect(shouldProcessSource(IMPORTING_MODULE, {})).toBe(false);
    });

    test('skips the module when the list is empty', () => {
      expect(shouldProcessSource(IMPORTING_MODULE, { importSources: [] })).toBe(false);
    });
  });

  describe('a bare specifier', () => {
    test('processes a module that mentions it', () => {
      expect(shouldProcessSource(IMPORTING_MODULE, { importSources: ['@stylexjs/stylex'] })).toBe(
        true
      );
    });

    test('skips a module that does not', () => {
      expect(
        shouldProcessSource('export const a = 1;', { importSources: ['@stylexjs/stylex'] })
      ).toBe(false);
    });

    test('processes the module when any one entry matches', () => {
      expect(
        shouldProcessSource(IMPORTING_MODULE, { importSources: ['react-strict-dom', 'stylex'] })
      ).toBe(true);
    });
  });

  describe('a from/as pair', () => {
    test('processes a module that mentions the source', () => {
      expect(
        shouldProcessSource("import { css } from 'react-strict-dom';", {
          importSources: [{ as: 'css', from: 'react-strict-dom' }],
        })
      ).toBe(true);
    });

    test('processes a module that mentions only the local name', () => {
      // The source can be re-exported under another path, so the local name is
      // a match on its own.
      expect(
        shouldProcessSource('export const styles = css.create({});', {
          importSources: [{ as: 'css', from: 'react-strict-dom' }],
        })
      ).toBe(true);
    });

    test('skips a module that mentions neither half', () => {
      expect(
        shouldProcessSource('export const a = 1;', {
          importSources: [{ as: 'css', from: 'react-strict-dom' }],
        })
      ).toBe(false);
    });

    test('a missing local name decides nothing', () => {
      // The webpack and Turbopack loaders used to search the module for the
      // text "undefined" when `as` was left out. That made an unrelated module
      // compile for no reason.
      expect(
        shouldProcessSource('const a = undefined;', {
          importSources: [{ from: 'react-strict-dom' }],
        })
      ).toBe(false);
    });
  });

  describe('a blank entry', () => {
    // A blank entry matches every module. One misconfigured option would turn
    // into a whole-project compile.
    test.each([[''], ['   ']])('skips the module for the bare specifier %j', specifier => {
      expect(shouldProcessSource(IMPORTING_MODULE, { importSources: [specifier] })).toBe(false);
    });

    test('skips the module for a blank source', () => {
      expect(shouldProcessSource(IMPORTING_MODULE, { importSources: [{ from: '  ' }] })).toBe(
        false
      );
    });

    test('skips the module for a blank local name', () => {
      expect(
        shouldProcessSource(IMPORTING_MODULE, { importSources: [{ as: '', from: 'nowhere' }] })
      ).toBe(false);
    });
  });

  describe('a source that arrived as JSON text', () => {
    test('reads the specifier out of it', () => {
      expect(
        shouldProcessSource(IMPORTING_MODULE, {
          importSources: ['{"from":"@stylexjs/stylex","as":"stylex"}'],
        })
      ).toBe(true);
    });

    test('skips a module that does not mention the specifier', () => {
      expect(
        shouldProcessSource('export const a = 1;', {
          importSources: ['{"from":"@stylexjs/stylex"}'],
        })
      ).toBe(false);
    });

    test('falls back to a plain scan when the text is not JSON', () => {
      expect(shouldProcessSource('import x from "{broken";', { importSources: ['{broken'] })).toBe(
        true
      );
    });

    test('falls back to a plain scan when the JSON carries no string source', () => {
      expect(
        shouldProcessSource('const a = \'{"from":1}\';', { importSources: ['{"from":1}'] })
      ).toBe(true);
    });
  });

  describe('the scan is text, not a parse', () => {
    test('a mention inside a comment is enough', () => {
      // Deliberate. A false positive costs one compile that changes nothing,
      // while a false negative costs a silently unstyled element.
      expect(
        shouldProcessSource('// see @stylexjs/stylex', { importSources: ['@stylexjs/stylex'] })
      ).toBe(true);
    });
  });

  describe('the sx prop', () => {
    describe('each prop-like position', () => {
      test.each([
        ['a JSX attribute', 'export const Box = props => <div sx={props.sx} />;'],
        ['an explicit property', 'export const box = { sx: props.sx };'],
        ['the object shorthand', 'export const box = ({ sx }) => sx;'],
        ['the shorthand before more properties', 'export const box = ({ sx, id }) => id;'],
        ['the name spaced from its separator', 'export const box = { sx : props.sx };'],
      ])('processes a module using %s', (_position, sourceCode) => {
        expect(shouldProcessSource(sourceCode, NO_IMPORT)).toBe(true);
      });
    });

    describe('each quoted position the compiler transforms', () => {
      // The compiler reads the prop from a quoted or computed key as well. A
      // module that reaches the plugin already compiled carries these forms,
      // and it is the one most likely to have no import to find.
      test.each([
        ['a string key', '_jsx("div", { "sx": styles.a });'],
        ['a single-quoted key', "_jsx('div', { 'sx': styles.a });"],
        ['a computed string key', '_jsx("div", { ["sx"]: styles.a });'],
        ['a computed template key', '_jsx("div", { [`sx`]: styles.a });'],
        ['a spaced computed key', '_jsx("div", { [ "sx" ] : styles.a });'],
        ['the Solid.js attribute call', '_$setAttribute(_el$, "sx", styles.main);'],
      ])('processes a module using %s', (_position, sourceCode) => {
        expect(shouldProcessSource(sourceCode, NO_IMPORT)).toBe(true);
      });

      test.each([
        ['a quoted name with no separator after it', 'el.getAttribute("sx");'],
        ['a quoted name inside a longer string', 'const help = "pass sx to style it";'],
        ['quotes that do not match', 'const a = String.raw`"sx`;'],
      ])('skips a module whose only mention is %s', (_form, sourceCode) => {
        expect(shouldProcessSource(sourceCode, NO_IMPORT)).toBe(false);
      });
    });

    test('skips a module that uses neither the prop nor an import', () => {
      expect(shouldProcessSource('export const noop = 1;', NO_IMPORT)).toBe(false);
    });

    test('a mention with no prop-like separator is not enough', () => {
      expect(shouldProcessSource('export function sx() {}', NO_IMPORT)).toBe(false);
    });

    test('an assignment to the name is a match', () => {
      // Text cannot tell `sx = 1` from a formatted attribute. The cost is one
      // compile that changes nothing, against a silently unstyled element.
      expect(shouldProcessSource('export const sx = 1;', NO_IMPORT)).toBe(true);
    });

    describe('the name must start a word', () => {
      // Without this, the default name matches every module a build step has
      // already compiled, through the `jsx` import of the JSX runtime.
      test.each([
        ['the JSX runtime import', 'import { jsx } from "react/jsx-runtime";'],
        ['its renamed form', 'import { jsx as _jsx, jsxs as _jsxs } from "react/jsx-runtime";'],
        ['a longer identifier', 'const boxsx = 1;'],
        ['a minified property', 'function a(e){return{esx:1}}'],
        ['an underscore prefix', 'const _sx = 1;'],
        ['a dollar prefix', 'const $sx = 1;'],
        ['a digit prefix', 'const a1sx = 1;'],
      ])('skips a module whose only mention is %s', (_form, sourceCode) => {
        expect(shouldProcessSource(sourceCode, NO_IMPORT)).toBe(false);
      });

      test('still matches the name at the very start of the module', () => {
        expect(shouldProcessSource('sx: 1', NO_IMPORT)).toBe(true);
      });

      test('still matches a member named after the prop', () => {
        // A dot is not part of an identifier, so `props.sx` still reads as the
        // prop. The scan is text, so this cannot be told from the real thing.
        expect(shouldProcessSource('const a = { ...props.sx };', NO_IMPORT)).toBe(true);
      });
    });

    test('uses a renamed prop', () => {
      expect(shouldProcessSource('<div css={styles} />', { ...NO_IMPORT, sxPropName: 'css' })).toBe(
        true
      );
    });

    test('skips the default name once the prop is renamed', () => {
      expect(shouldProcessSource('<div sx={styles} />', { ...NO_IMPORT, sxPropName: 'css' })).toBe(
        false
      );
    });

    test('escapes a name carrying pattern metacharacters', () => {
      // `s.` must match the two characters, not "s" and any character.
      expect(shouldProcessSource('<div s.={styles} />', { ...NO_IMPORT, sxPropName: 's.' })).toBe(
        true
      );
      expect(shouldProcessSource('<div sx={styles} />', { ...NO_IMPORT, sxPropName: 's.' })).toBe(
        false
      );
    });

    test('skips the prop scan when the prop is disabled', () => {
      expect(shouldProcessSource('<div sx={styles} />', { ...NO_IMPORT, sxPropName: false })).toBe(
        false
      );
    });

    test('still reads the import scan when the prop is disabled', () => {
      expect(shouldProcessSource(IMPORTING_MODULE, { ...NO_IMPORT, sxPropName: false })).toBe(true);
    });

    test('skips the prop scan when the name is blank', () => {
      expect(shouldProcessSource('<div sx={styles} />', { ...NO_IMPORT, sxPropName: '  ' })).toBe(
        false
      );
    });

    test('processes the module with no import source to look for', () => {
      // The prop transform does not read the import sources, so an empty list
      // does not decide this module.
      expect(shouldProcessSource('<div sx={styles} />', { importSources: [] })).toBe(true);
    });
  });

  describe('large and hostile input', () => {
    test('finds the prop at the end of a very large module', () => {
      const padding = 'const value = compute(argument, other);\n'.repeat(40_000);
      const [answer, elapsedMs] = timedScan(`${padding}export const Box = ({ sx }) => sx;`);

      expect(answer).toBe(true);
      expect(elapsedMs).toBeLessThan(BUDGET_MS);
    });

    test('answers quickly for a very large module that never mentions either', () => {
      const [answer, elapsedMs] = timedScan('const value = compute(a, b);\n'.repeat(40_000));

      expect(answer).toBe(false);
      expect(elapsedMs).toBeLessThan(BUDGET_MS);
    });

    test('does not backtrack on a long run of quotes and separators', () => {
      // The quoted half reads an opening quote, the name and a closing quote.
      // A run that offers many openings and never completes one is the shape
      // that would expose a pattern able to backtrack.
      const [answer, elapsedMs] = timedScan(`const a = '${'"sx'.repeat(50_000)}';`);

      expect(answer).toBe(false);
      expect(elapsedMs).toBeLessThan(BUDGET_MS);
    });

    test('does not backtrack on a long run of whitespace before no separator', () => {
      const [answer, elapsedMs] = timedScan(`"sx"${' '.repeat(200_000)}end`);

      expect(answer).toBe(false);
      expect(elapsedMs).toBeLessThan(BUDGET_MS);
    });

    test('handles a module that is a single very long line', () => {
      const [answer, elapsedMs] = timedScan(`${'a'.repeat(500_000)};_jsx("div",{"sx":s});`);

      expect(answer).toBe(true);
      expect(elapsedMs).toBeLessThan(BUDGET_MS);
    });

    test('handles empty source', () => {
      expect(shouldProcessSource('', NO_IMPORT)).toBe(false);
    });

    test('handles a name as long as the module', () => {
      const name = 'a'.repeat(10_000);

      expect(shouldProcessSource(`{ ${name}: 1 }`, { ...NO_IMPORT, sxPropName: name })).toBe(true);
    });

    test('reads a name whose characters are outside the Latin alphabet', () => {
      // The name is compared as text, so any character works. The word-start
      // guard must not treat a non-Latin letter as the end of an identifier.
      expect(shouldProcessSource('<div стиль={s} />', { ...NO_IMPORT, sxPropName: 'стиль' })).toBe(
        true
      );
      expect(
        shouldProcessSource('const мойстиль = 1;', { ...NO_IMPORT, sxPropName: 'стиль' })
      ).toBe(false);
    });

    test('reads a name that is an emoji', () => {
      expect(
        shouldProcessSource('{ "\u{1F600}": s }', { ...NO_IMPORT, sxPropName: '\u{1F600}' })
      ).toBe(true);
    });
  });
});
