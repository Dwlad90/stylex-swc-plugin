// Shared by all five bundler plugins, so it is tested here directly rather
// than through any one of them.
import { describe, expect, test } from 'vitest';

import { shouldProcessSource } from '../src/module-selection';

const IMPORTING_MODULE = "import * as stylex from '@stylexjs/stylex';";

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
});
