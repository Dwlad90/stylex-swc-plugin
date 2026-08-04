// Comments must survive the transform. They are not cosmetic: bundlers and
// minifiers read some of them, and dropping those silently changes chunk names
// and defeats tree shaking.
import { expect, test } from 'vitest';

import { transform } from '../dist/index.js';

const FILENAME = '/abs/path/page.tsx';

function compile(code: string): string {
  return transform(FILENAME, code, {
    unstable_moduleResolution: { type: 'commonJS' },
  }).code;
}

test('webpack magic comments survive on dynamic imports', () => {
  // Losing this silently renames the emitted chunk.
  const code = compile('export const lazy = () => import(/* webpackChunkName: "lazy" */ "./m");');

  expect(code).toContain('/* webpackChunkName: "lazy" */');
});

test('#__PURE__ annotations survive', () => {
  // Losing this defeats minifier tree shaking for the annotated call.
  const code = compile('export const value = /* #__PURE__ */ compute();');

  expect(code).toContain('/* #__PURE__ */');
});

test('license banners survive', () => {
  const code = compile('/*! @license MIT */\nexport const x = 1;');

  expect(code).toContain('/*! @license MIT */');
});

test('line, block and jsdoc comments all survive', () => {
  const code = compile(
    ['// line', '/** jsdoc */', 'export const a = 1;', 'export const b = 2; // trailing'].join('\n')
  );

  expect(code).toContain('// line');
  expect(code).toContain('/** jsdoc */');
  expect(code).toContain('// trailing');
});

test('comments around a stylex.create call survive its transformation', () => {
  const code = compile(
    [
      'import stylex from "@stylexjs/stylex";',
      '',
      '// styles below',
      'export const styles = stylex.create({',
      '  default: { color: "red" },',
      '});',
      '',
      '// after',
      'export const done = true;',
    ].join('\n')
  );

  expect(code).toContain('// styles below');
  expect(code).toContain('// after');
  // The call itself is still compiled away.
  expect(code).not.toContain('stylex.create');
});

test('a comment attached to a removed node does not resurrect the node', () => {
  const code = compile(
    [
      'import stylex from "@stylexjs/stylex";',
      'export const styles = stylex.create({',
      '  // per-namespace note',
      '  default: { color: "red" },',
      '});',
    ].join('\n')
  );

  expect(code).not.toContain('stylex.create');
  expect(code).toContain('$$css');
});

test('a file that is only a comment compiles to just that comment', () => {
  const code = compile('// nothing else here\n');

  expect(code.trim()).toBe('// nothing else here');
});
