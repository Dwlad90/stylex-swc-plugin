// `Math.hypot` folds in the JavaScript engine, which calls the `hypot` of the
// C library. On linux-gnu the addon routes that call through a jump in
// `src/glibc_compat.rs`, which keeps the addon loadable on glibc 2.34. These
// cases run the built addon, so the binding tests of the gnu targets run that
// jump in the artifact that gets published.
//
// The cases run in a child process, because a test in this process cannot
// report a crash of this process. Each expected rule is the output of the
// reference compiler for the same source.
import * as path from 'path';

import { describe, expect, test } from 'vitest';

import { runNodeScriptOrThrow } from './nodeScript';

const compilerEntry = path.resolve(__dirname, '../dist/index.js');

/** Each expression goes into a `width` template, which prints every digit. */
const cases: [expression: string, rule: string][] = [
  ['Math.hypot()', '.xnalus7{width:0}'],
  ['Math.hypot(-12)', '.xsmyaan{width:12px}'],
  ['Math.hypot(3, 4)', '.x1ftt334{width:5px}'],
  ['Math.hypot(0.1, 0.2)', '.x17p1j7s{width:.223606797749979px}'],
  ['Math.hypot(-0, -0)', '.xnalus7{width:0}'],
  ['Math.hypot(NaN, Infinity)', '.x1fssspx{width:Infinitypx}'],
  ['Math.hypot(-Infinity)', '.x1fssspx{width:Infinitypx}'],
  ['Math.hypot(1, NaN)', '.x1c9rq88{width:NaNpx}'],
  ['Math.hypot(1e308, 1e308)', '.x15wz0cg{width:1.4142135623730951e+308px}'],
  ['Math.hypot(1.7976931348623157e308, 1.7976931348623157e308)', '.x1fssspx{width:Infinitypx}'],
  ['Math.hypot(3e-160, 4e-160)', '.xm3uyjt{width:5e-160px}'],
  ['Math.hypot(5e-324, 5e-324)', '.x6pb8ui{width:5e-324px}'],
  ["Math.hypot('3', [4], true)", '.xbiu1ca{width:5.0990195135927845px}'],
  ['Math.hypot(1e300, 1e300, 1e300, 1e300)', '.x1sjc2a5{width:2e+300px}'],
  ['Math.hypot(Math.hypot(3, 4), 12)', '.x1fxhmyf{width:13px}'],
];

/** A module whose one `stylex.create` call holds `styles`. */
const moduleWith = (styles: string) =>
  `import * as stylex from '@stylexjs/stylex';\n` +
  `export const styles = stylex.create({ ${styles} });\n`;

/** A style that prints the value of `expression` as a width. */
const widthOf = (expression: string) => `{ width: \`\${${expression}}px\` }`;

/** Compiles each source in one child process, and returns the rules of each. */
const compileInChild = (sources: string[]): string[][] => {
  const script = `
    const { transform } = require(${JSON.stringify(compilerEntry)});
    const rules = ${JSON.stringify(sources)}.map(source => {
      const result = transform('page.tsx', source, {
        dev: false,
        unstable_moduleResolution: { type: 'commonJS' },
      });
      return result.metadata.stylex.map(([, rule]) => rule.ltr);
    });
    process.stdout.write(JSON.stringify(rules));
  `;

  return JSON.parse(runNodeScriptOrThrow(script).stdout) as string[][];
};

describe('Math.hypot', () => {
  test('folds every case to the rule of the reference compiler', () => {
    const rules = compileInChild(
      cases.map(([expression]) => moduleWith(`root: ${widthOf(expression)}`))
    );

    expect(
      Object.fromEntries(cases.map(([expression], i) => [expression, rules[i]?.join('')]))
    ).toEqual(Object.fromEntries(cases));
  });

  test('folds a module with many calls in one compile', () => {
    // One call per key, each with its own value, in a single module.
    const count = 2_000;
    const keys = Array.from(
      { length: count },
      (_, n) => `k${n}: ${widthOf(`Math.hypot(${3 * (n + 1)}, ${4 * (n + 1)})`)}`
    );

    const [rules = []] = compileInChild([moduleWith(keys.join(', '))]);

    // The order of the rules is not part of this check.
    const widths = rules.map(rule => Number(/width:(\d+)px/.exec(rule)?.[1]));
    expect(widths).toHaveLength(count);
    expect(new Set(widths)).toEqual(new Set(Array.from({ length: count }, (_, n) => 5 * (n + 1))));
  });
});
