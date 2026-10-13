// The `swcPlugins` pre-pass parses the module with `@swc/core` before the
// native transform does, so it has to pick the same syntax, or a `.ts` module
// holding a generic arrow is refused by the pre-pass alone. The rule lives in
// `admitsJsx` and is asserted here over the table `stylex_utils::swc::
// syntax_for_file` is asserted over on the Rust side (`swc_test.rs`,
// `file_syntax`), the bare `.ts` name included: `Path::extension` reads it as a
// hidden file with no extension, so both sides keep JSX for it.
//
// No wasm plugin is installed to run the pre-pass for real, and one would pin
// this suite to an ABI; the parser options handed to `@swc/core` are what the
// rule decides, so the module is stood in for through Node's require cache,
// which `dist/index.js` reads it from.
import { createRequire } from 'node:module';

import { describe, expect, test } from 'vitest';

import { admitsJsx, transform } from '../dist/index.js';

const TYPESCRIPT_WITHOUT_JSX = [
  'styles.ts',
  'styles.mts',
  'styles.cts',
  '/abs/path/styles.ts',
  'types.d.ts',
  'Styles.TS',
  'styles.Mts',
  'STYLES.CTS',
  'C:\\dir\\styles.ts',
  '..ts',
];
const ADMITTING_JSX = [
  'App.tsx',
  '/abs/path/App.tsx',
  'App.js',
  'App.jsx',
  'App.mjs',
  'App.cjs',
  'App.TSX',
  'page',
  'page.',
  'page.vue',
  'page.ts.txt',
  '.ts',
  '/dir/.mts',
  'C:\\dir\\.ts',
];

describe('admitsJsx', () => {
  test('a `.ts`, `.mts` or `.cts` module reads an angle bracket as a type, whatever its case', () => {
    const admitting = TYPESCRIPT_WITHOUT_JSX.filter(filename => admitsJsx(filename));
    expect(admitting).toEqual([]);
  });

  test('every other name keeps admitting JSX, a name that is nothing but the extension included', () => {
    const refusing = ADMITTING_JSX.filter(filename => !admitsJsx(filename));
    expect(refusing).toEqual([]);
  });
});

describe('the swcPlugins pre-pass', () => {
  const require = createRequire(import.meta.url);
  const distEntry = require.resolve('../dist/index.js');
  const swcCore = require.resolve('@swc/core', { paths: [distEntry] });

  /** Runs `transform` with a plugin list against a stand-in `@swc/core`, and answers the parser options it was handed. */
  function parserOptionsHandedToSwc(filename: string): unknown {
    const handed: unknown[] = [];
    const standIn = {
      transformSync(code: string, options: { jsc: { parser: unknown } }) {
        handed.push(options.jsc.parser);
        return { code, map: undefined };
      },
    };
    const previous = require.cache[swcCore];
    require.cache[swcCore] = {
      id: swcCore,
      filename: swcCore,
      loaded: true,
      exports: standIn,
    } as NodeJS.Module;
    try {
      transform(filename, 'export const padding = 16;\n', {
        dev: false,
        unstable_moduleResolution: { type: 'commonJS' },
        swcPlugins: [['a-plugin', {}]],
      });
    } finally {
      if (previous) {
        require.cache[swcCore] = previous;
      } else {
        Reflect.deleteProperty(require.cache, swcCore);
      }
    }
    expect(handed).toHaveLength(1);
    return handed[0];
  }

  test('parses a `.ts` module as TypeScript without JSX, as the native transform then does', () => {
    expect(parserOptionsHandedToSwc('/abs/path/styles.ts')).toEqual({
      syntax: 'typescript',
      tsx: false,
    });
    expect(parserOptionsHandedToSwc('/abs/path/styles.mts')).toEqual({
      syntax: 'typescript',
      tsx: false,
    });
  });

  test('parses every other module with JSX admitted', () => {
    const handed = ['/abs/path/App.tsx', '/abs/path/App.js', '/abs/path/.ts'].map(filename => [
      filename,
      parserOptionsHandedToSwc(filename),
    ]);
    expect(handed).toEqual(
      handed.map(([filename]) => [filename, { syntax: 'typescript', tsx: true }])
    );
  });
});
