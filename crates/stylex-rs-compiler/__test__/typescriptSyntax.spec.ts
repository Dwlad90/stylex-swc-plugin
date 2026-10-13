// Which syntax a file name earns, read across the boundary. The native
// transform picks the parser's syntax from the extension before anything the
// Rust suites can see -- `cargo test` names the syntax itself -- so a module
// that parses there can still be refused here, which is how every `.ts` file
// holding a generic arrow was refused while the suites stayed green.
//
// In a `.ts`, `.mts` or `.cts` module an angle bracket opens a type parameter
// list or a type assertion and never an element; in every other module it may
// open an element. That is `tsc`'s rule, and the one `@babel/preset-typescript`
// and Next.js's SWC loader apply, so the reference implementation compiles
// these modules on the host's parser. The rules asserted below are its own,
// read off `@stylexjs/babel-plugin` run over the same sources.
import { describe, expect, test } from 'vitest';

import { transform } from '../dist/index.js';

const TYPESCRIPT_WITHOUT_JSX = ['ts', 'mts', 'cts'] as const;
const ADMITTING_JSX = ['tsx', 'js', 'jsx', 'mjs', 'cjs'] as const;

/** The one rule every module below declares, as the reference emits it. */
const PADDING_RULE = ['x1tamke2', { ltr: '.x1tamke2{padding:16px}', rtl: null }, 1000];

function compile(code: string, extension: string) {
  return transform(`/abs/path/styles.${extension}`, code, {
    dev: false,
    unstable_moduleResolution: { type: 'commonJS' },
  });
}

/**
 * A generic arrow spelled `<T>(…)`. TSX forces the `<T,>` spelling, which is
 * how a compiler reading every file as TSX kept passing on code written for
 * `.tsx` files.
 */
const GENERIC_ARROW = [
  "import * as stylex from '@stylexjs/stylex';",
  '',
  'const px = <T>(value: T) => `${value}px`;',
  '',
  'export const styles = stylex.create({',
  '  root: { padding: px(16) },',
  '});',
  '',
].join('\n');

/** An angle-bracket type assertion, beside a style it takes no part in. */
const TYPE_ASSERTION = [
  "import * as stylex from '@stylexjs/stylex';",
  '',
  'export const root = <HTMLElement>document.body;',
  '',
  'export const styles = stylex.create({',
  '  root: { padding: 16 },',
  '});',
  '',
].join('\n');

/** An element, which only a module admitting JSX reads. */
const ELEMENT = [
  "import * as stylex from '@stylexjs/stylex';",
  '',
  'const styles = stylex.create({',
  '  root: { padding: 16 },',
  '});',
  '',
  'export const Box = () => <div {...stylex.props(styles.root)} />;',
  '',
].join('\n');

describe('a TypeScript module without JSX', () => {
  test.each(TYPESCRIPT_WITHOUT_JSX)(
    'reads `<T>(value: T) =>` as a generic arrow in a .%s file',
    extension => {
      const out = compile(GENERIC_ARROW, extension);

      expect(out.metadata.stylex).toEqual([PADDING_RULE]);
      expect(out.code).toContain('kmVPX3: "x1tamke2"');
    }
  );

  test.each(TYPESCRIPT_WITHOUT_JSX)(
    'reads `<HTMLElement>node` as a type assertion in a .%s file',
    extension => {
      const out = compile(TYPE_ASSERTION, extension);

      expect(out.metadata.stylex).toEqual([PADDING_RULE]);
      expect(out.code).toContain('export const root = document.body;');
    }
  );

  // The one thing the extension takes away, and the thing every TypeScript
  // toolchain refuses in such a file.
  test.each(TYPESCRIPT_WITHOUT_JSX)('refuses an element in a .%s file', extension => {
    expect(() => compile(ELEMENT, extension)).toThrow('Failed to parse file');
  });
});

describe('a module admitting JSX', () => {
  test.each(ADMITTING_JSX)('still reads an element in a .%s file', extension => {
    const out = compile(ELEMENT, extension);

    expect(out.metadata.stylex).toEqual([PADDING_RULE]);
    expect(out.code).toContain('className="x1tamke2"');
  });

  // `<T>` opens an element in TSX, so a generic arrow spelled that way is a
  // parse error there in every toolchain, and stays one.
  test('refuses a `<T>(value: T) =>` arrow in a .tsx file', () => {
    expect(() => compile(GENERIC_ARROW, 'tsx')).toThrow('Failed to parse file');
  });
});
