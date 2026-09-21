// A breakpoint declared with `defineConsts` is not readable as an `@media`
// string while a module is compiled: the consuming rule carries only a
// `var(--hash)` placeholder, and the constant that fills it lives in another
// module. The stylesheet assembler resolves the placeholder and then orders
// `min-width` ascending and `max-width` descending, so a wider breakpoint
// cannot win the cascade over a narrower one.
//
// That order is reachable only while this compiler emits the constant as a
// metadata tuple carrying `constKey` and `constVal`, and wraps the consuming
// rule in the matching placeholder. The tuples are pinned first, then the
// stylesheet they assemble into: a placeholder spelled differently would still
// compile, and the order would fall back to alphabetic with nothing to say so.
import * as path from 'path';

import stylexBabelPlugin from '@stylexjs/babel-plugin';
import pluginPkg from '@stylexjs/babel-plugin/package.json' with { type: 'json' };
import { describe, expect, test } from 'vitest';

import { transform } from '../dist/index.js';
import type { StyleXMetadata } from '../dist/index.js';

const BREAKPOINTS_SOURCE = `
  import * as stylex from '@stylexjs/stylex';

  export const breakpoints = stylex.defineConsts({
    tablet: '@media (min-width: 1000px)',
    desktop: '@media (min-width: 1500px)',
    small: '@media (max-width: 500px)',
    large: '@media (max-width: 1000px)',
  });
`;

// The wider breakpoint is authored first in both groups, so an assembler that
// kept the authored order -- or sorted by the placeholder hash -- would put it
// ahead of the narrower one and lose the cascade.
const COMPONENT_SOURCE = `
  import * as stylex from '@stylexjs/stylex';
  import { breakpoints } from 'breakpoints.stylex.js';

  export const styles = stylex.create({
    a: {
      width: {
        default: '100px',
        [breakpoints.desktop]: '300px',
        [breakpoints.tablet]: '200px',
      },
    },
    b: {
      color: {
        default: 'black',
        [breakpoints.large]: 'red',
        [breakpoints.small]: 'blue',
      },
    },
  });
`;

/**
 * Both modules through the compiler, with the metadata tuples they emit
 * concatenated in the order a bundler collects them.
 *
 * `haste` resolution names a module by its file name alone, so neither source
 * has to exist on disk for the import to resolve to the same constants.
 */
function collectMetadata(): StyleXMetadata['stylex'] {
  const rootDir = __dirname;
  const options = {
    dev: false,
    unstable_moduleResolution: { type: 'haste' as const, rootDir },
  };
  const compile = (name: string, source: string): StyleXMetadata['stylex'] =>
    transform(path.join(rootDir, name), source, options).metadata.stylex;

  return [
    ...compile('breakpoints.stylex.js', BREAKPOINTS_SOURCE),
    ...compile('component.js', COMPONENT_SOURCE),
  ];
}

describe('defineConsts breakpoints', () => {
  const metadata = collectMetadata();

  // The ordering below is the assembler's rather than this compiler's: no
  // production code here decides it. The catalog range is `^0.19.1`, so a
  // later release that reorders would fail the expectations under this one
  // with nothing to say why. Naming the version measured is cheaper than
  // pinning the catalog entry, which 61 projects share.
  test('measures the assembler this expectation was taken from', () => {
    expect(pluginPkg.version).toBe('0.19.1');
  });

  test('emits each constant beside the placeholder its consumer carries', () => {
    expect(metadata).toStrictEqual([
      [
        'x1flm7tz',
        { constKey: 'x1flm7tz', constVal: '@media (min-width: 1000px)', ltr: '', rtl: null },
        0,
      ],
      [
        'x21rhod',
        { constKey: 'x21rhod', constVal: '@media (min-width: 1500px)', ltr: '', rtl: null },
        0,
      ],
      [
        'x16yt4h9',
        { constKey: 'x16yt4h9', constVal: '@media (max-width: 500px)', ltr: '', rtl: null },
        0,
      ],
      [
        'xzg5jgv',
        { constKey: 'xzg5jgv', constVal: '@media (max-width: 1000px)', ltr: '', rtl: null },
        0,
      ],
      ['x1exxlbk', { ltr: '.x1exxlbk{width:100px}', rtl: null }, 4000],
      ['x193souu', { ltr: 'var(--x21rhod){.x193souu.x193souu{width:300px}}', rtl: null }, 7000],
      ['xxw7ul5', { ltr: 'var(--x1flm7tz){.xxw7ul5.xxw7ul5{width:200px}}', rtl: null }, 7000],
      ['x1mqxbix', { ltr: '.x1mqxbix{color:black}', rtl: null }, 3000],
      ['x1dypaho', { ltr: 'var(--xzg5jgv){.x1dypaho.x1dypaho{color:red}}', rtl: null }, 6000],
      ['x7bplha', { ltr: 'var(--x16yt4h9){.x7bplha.x7bplha{color:blue}}', rtl: null }, 6000],
    ]);
  });

  test('assembles min-width ascending and max-width descending', () => {
    const css = stylexBabelPlugin.processStylexRules(metadata, {
      useLayers: false,
      legacyDisableLayers: true,
    });

    expect(css).toBe(
      [
        '.x1mqxbix{color:black}',
        '.x1exxlbk{width:100px}',
        '@media (max-width: 1000px){.x1dypaho.x1dypaho{color:red}}',
        '@media (max-width: 500px){.x7bplha.x7bplha{color:blue}}',
        '@media (min-width: 1000px){.xxw7ul5.xxw7ul5{width:200px}}',
        '@media (min-width: 1500px){.x193souu.x193souu{width:300px}}',
      ].join('\n')
    );
  });
});

/**
 * The assembled stylesheet for `constants`, each used once by a rule.
 *
 * Returns the `@media` preludes in the order they were assembled, which is the
 * whole of what a breakpoint order can be asserted on: the class names are
 * hashes of the values, so naming them would pin the hash and not the order.
 */
function assembleQueryOrder(constants: Record<string, string>): string[] {
  const rootDir = __dirname;
  const names = Object.keys(constants);

  const constantsSource = `
    import * as stylex from '@stylexjs/stylex';

    export const bp = stylex.defineConsts(${JSON.stringify(constants)});
  `;

  // One declaration per constant, each with its own value, so every rule gets
  // a class name of its own and no two can merge to hide an ordering mistake.
  const componentSource = `
    import * as stylex from '@stylexjs/stylex';
    import { bp } from 'bp.stylex.js';

    export const styles = stylex.create({
      a: {
        zIndex: {
          default: 0,
          ${names.map((name, i) => `[bp.${name}]: ${i + 1},`).join('\n          ')}
        },
      },
    });
  `;

  const options = {
    dev: false,
    unstable_moduleResolution: { type: 'haste' as const, rootDir },
  };
  const compile = (file: string, source: string): StyleXMetadata['stylex'] =>
    transform(path.join(rootDir, file), source, options).metadata.stylex;

  const css = stylexBabelPlugin.processStylexRules(
    [...compile('bp.stylex.js', constantsSource), ...compile('component.js', componentSource)],
    { useLayers: false, legacyDisableLayers: true }
  );

  return css
    .split('\n')
    .map(line => line.slice(0, line.indexOf('{')))
    .filter(prelude => prelude.startsWith('@media'));
}

describe('defineConsts breakpoint shapes', () => {
  test('sorts min-width with screen and media type', () => {
    expect(
      assembleQueryOrder({
        wide: '@media screen and (min-width: 900px)',
        narrow: '@media screen and (min-width: 400px)',
      })
    ).toStrictEqual([
      '@media screen and (min-width: 400px)',
      '@media screen and (min-width: 900px)',
    ]);
  });

  test('orders the min-width group before the max-width group', () => {
    expect(
      assembleQueryOrder({
        maxNarrow: '@media (max-width: 300px)',
        minWide: '@media (min-width: 900px)',
      })
    ).toStrictEqual(['@media (min-width: 900px)', '@media (max-width: 300px)']);
  });

  test('sorts unitless zero, any-unit zero, and mixed-case px', () => {
    expect(
      assembleQueryOrder({
        mixedCase: '@media (min-width: 700PX)',
        zeroUnitless: '@media (min-width: 0)',
        zeroRem: '@media (min-width: 0rem)',
      })
    ).toStrictEqual([
      '@media (min-width: 0)',
      '@media (min-width: 0rem)',
      '@media (min-width: 700PX)',
    ]);
  });

  test('does not sort rem breakpoints', () => {
    // A rem length has no fixed pixel value, so the rules keep the order they
    // arrived in rather than being compared against one another.
    expect(
      assembleQueryOrder({
        wide: '@media (min-width: 60rem)',
        narrow: '@media (min-width: 20rem)',
      })
    ).toStrictEqual(['@media (min-width: 60rem)', '@media (min-width: 20rem)']);
  });

  test('sort is a total order across px min- and max-width rules', () => {
    // Every arrangement of the same three breakpoints must assemble to one
    // answer, which is what makes the comparison a total order.
    const shapes = {
      a: '@media (min-width: 500px)',
      b: '@media (max-width: 300px)',
      c: '@media (min-width: 900px)',
    };
    const permutations = [
      ['a', 'b', 'c'],
      ['a', 'c', 'b'],
      ['b', 'a', 'c'],
      ['b', 'c', 'a'],
      ['c', 'a', 'b'],
      ['c', 'b', 'a'],
    ];

    const results = permutations.map(order =>
      assembleQueryOrder(
        Object.fromEntries(order.map(name => [name, shapes[name as keyof typeof shapes]]))
      ).join('\n')
    );

    expect(new Set(results).size).toBe(1);
    expect(results[0]).toBe(
      ['@media (min-width: 500px)', '@media (min-width: 900px)', '@media (max-width: 300px)'].join(
        '\n'
      )
    );
  });
});
