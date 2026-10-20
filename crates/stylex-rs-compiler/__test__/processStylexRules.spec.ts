// `processStylexRules` is the stylesheet step: every rule a build's `transform`
// calls emitted, assembled into the one sheet the bundler plugins write. The
// bundler plugins of this repository imported it from `@stylexjs/babel-plugin`
// until now; this is the native one, exported beside `transform`, and it has to
// write the same bytes as the reference for the same rules, or a consumer that
// switches assembles a different sheet for the same markup.
//
// The cases are the reference's own test file, recorded as `stylex-stylesheet`
// carries it (each `processStylexRules` call a test makes, with the rules handed
// in and the string answered), held three ways: this export against the recorded
// answer, the reference against it, and the two against each other on rules this
// compiler emits here and now.
import * as fs from 'node:fs';
import { createRequire } from 'node:module';
import * as path from 'node:path';

import { afterEach, describe, expect, test } from 'vitest';

import { processStylexRules, transform } from '../dist/index.js';
import type { ProcessStylexRulesConfig, StyleXRule } from '../dist/index.js';
import { nativeInputOf } from '../dist/process-stylex-rules/fast-path-gate.js';
import { loadBabelPlugin } from '../parity/lib/compilers.js';
import { PROCESS_STYLEX_RULES_CASES, stylesheetFixturesDir } from '../parity/lib/fixtures.js';
import { runNodeScriptOrThrow } from './nodeScript';

const require = createRequire(import.meta.url);

interface RecordedCall {
  rules: StyleXRule[];
  config: boolean | ProcessStylexRulesConfig | null;
  output: string;
}

interface RecordedCase {
  name: string;
  calls: RecordedCall[];
}

const packageDir = path.resolve(__dirname, '..');
const fixture = JSON.parse(
  fs.readFileSync(path.join(stylesheetFixturesDir(packageDir), PROCESS_STYLEX_RULES_CASES), 'utf8')
) as { cases: RecordedCase[] };

const { processStylexRules: babelProcessStylexRules } = loadBabelPlugin();

type ProcessStylexRulesFn = typeof processStylexRules;

/** An implementation, called with values outside its published types. */
function untyped(fn: ProcessStylexRulesFn): (rules: unknown, config?: unknown) => string {
  return fn as (rules: unknown, config?: unknown) => string;
}

/**
 * What one implementation answers for an input: the sheet, or the class and
 * message of the error it throws.
 */
type Answer = { sheet: string } | { error: { name: string; message: string } };

function answerOf(run: () => string): Answer {
  try {
    return { sheet: run() };
  } catch (error) {
    return error instanceof Error
      ? { error: { name: error.name, message: error.message } }
      : { error: { name: typeof error, message: String(error) } };
  }
}

/** The two implementations on one input, for `expect(...).toEqual(...)`. */
function answers(rules: unknown, config?: unknown): [Answer, Answer] {
  return [
    answerOf(() => untyped(processStylexRules)(rules, config)),
    answerOf(() => untyped(babelProcessStylexRules)(rules, config)),
  ];
}

/** One input, by its index in a case list, on which the two answer differently. */
interface Divergence {
  index: number;
  config: unknown;
  native: Answer;
  reference: Answer;
}

/**
 * Every input, under each config, on which this export and the reference
 * answer differently. A test asserts that there is none.
 */
function divergences(
  cases: readonly unknown[],
  configs: readonly unknown[] = [true, false, { useLegacyClassnamesSort: true }]
): Divergence[] {
  const found: Divergence[] = [];
  for (const [index, rules] of cases.entries()) {
    for (const config of configs) {
      const [native, reference] = answers(rules, config);
      if (JSON.stringify(native) !== JSON.stringify(reference)) {
        found.push({ index, config, native, reference });
      }
    }
  }
  return found;
}

const STYLE: StyleXRule[1] = { ltr: '.x1{color:red}', rtl: null };

/** A style with this `ltr` text and no RTL text. */
function styleOf(ltr: string): StyleXRule[1] {
  return { ltr, rtl: null };
}

/** The rules with getters that log their reads, and the log. */
function withGetters(): [unknown[], string[]] {
  const reads: string[] = [];
  const read = <T>(name: string, value: T) => {
    reads.push(name);
    return value;
  };
  const style = {
    get ltr() {
      return read('ltr', '.x1{color:red}');
    },
    get constKey() {
      return read('constKey', undefined);
    },
  };
  const tuple = ['x1', style, 3000];
  Object.defineProperty(tuple, 0, { get: () => read('[0]', 'x1'), enumerable: true });
  const config = {
    get useLayers() {
      return read('useLayers', true);
    },
  };
  return [[tuple, config], reads];
}

/** A chain of constants, each reading the next, in declaration order. */
const chain = (names: readonly string[]) =>
  names.map((name, index) => [
    name,
    {
      constKey: name,
      constVal: index + 1 < names.length ? `var(--${names[index + 1]}) 1px` : '2px',
    },
    0,
  ]);

describe('processStylexRules', () => {
  test('the fixture is the whole reference file', () => {
    expect(fixture.cases).toHaveLength(36);
  });

  for (const recorded of fixture.cases) {
    test(`answers as the reference does: ${recorded.name}`, () => {
      for (const call of recorded.calls) {
        expect(processStylexRules(call.rules, call.config)).toBe(call.output);
        expect(babelProcessStylexRules(call.rules, call.config)).toBe(call.output);
      }
    });
  }

  test('assembles the rules this compiler emits as the reference assembles them', () => {
    const source = `
      import * as stylex from '@stylexjs/stylex';
      export const breakpoints = stylex.defineConsts({
        tablet: '@media (min-width: 1000px)',
        small: '@media (max-width: 500px)',
      });
      export const styles = stylex.create({
        root: {
          color: { default: 'red', [breakpoints.tablet]: 'blue', [breakpoints.small]: 'green' },
          float: 'inline-start',
          '::before': { content: '"·"' },
          '::after': { content: '""' },
        },
      });
    `;
    const { metadata } = transform(path.join(__dirname, 'sheet.stylex.js'), source, {
      dev: false,
      unstable_moduleResolution: { type: 'haste' },
    });
    const rules = metadata.stylex;

    expect(rules.length).toBeGreaterThan(5);
    for (const config of [
      true,
      false,
      { useLayers: { prefix: 'stylex', before: ['reset'], after: ['theme'] } },
      { useLayers: false, legacyDisableLayers: true },
      { useLayers: false, enableLTRRTLComments: true },
      { useLegacyClassnamesSort: true },
    ] as const) {
      expect(processStylexRules(rules, config)).toBe(babelProcessStylexRules(rules, config));
    }
  });

  test('reads a bare boolean, a null and an absent config as the reference does', () => {
    const rules: StyleXRule[] = [['x1', { ltr: '.x1{color:red}', rtl: null }, 3000]];

    expect(processStylexRules(rules, true)).toBe(
      '\n@layer priority1;\n@layer priority1{\n.x1{color:red}\n}'
    );
    // The polyfill adds one `:not(#\#)` per group index, and a lone group is
    // index 0: the rule is written as it is.
    expect(processStylexRules(rules, false)).toBe('.x1{color:red}');
    expect(processStylexRules(rules, null)).toBe(processStylexRules(rules));
    for (const config of [true, false, null, undefined]) {
      expect(processStylexRules(rules, config)).toBe(babelProcessStylexRules(rules, config));
    }
  });

  test('throws the reference error for a circular constant', () => {
    const rules: StyleXRule[] = [
      ['xA', { ltr: '', rtl: null, constKey: 'xA', constVal: 'var(--xB)' }, 0],
      ['xB', { ltr: '', rtl: null, constKey: 'xB', constVal: 'var(--xA)' }, 0],
      ['x1', { ltr: '.x1{color:var(--xA)}', rtl: null }, 3000],
    ];

    expect(() => processStylexRules(rules, true)).toThrow(
      new Error('circular reference detected for constant --xB')
    );
    expect(divergences([rules])).toEqual([]);
  });

  test('throws the reference error, of the same class, wherever the reference throws', () => {
    const revoked = Proxy.revocable<StyleXRule[]>([], {});
    revoked.revoke();

    expect(
      divergences([
        [['x1', null, 3000]], // a null style cannot be spread
        [['x1', undefined, 3000]],
        [['x1', { rtl: '.x1{color:red}' }, 3000]], // no ltr text
        [['x1', {}, 3000]],
        [['x1', { ltr: null }, 3000]],
        [['x1', { ltr: undefined }, 3000]],
        [['x1', { ltr: 5 }, 3000]],
        [['x1', { ltr: '.x1{a:0}', rtl: true }, 3000]], // a truthy rtl that is not a string
        [['x1', STYLE, 3000], undefined], // the reference cannot iterate the second rule
        [['x1']], // no style object
        [['x1', STYLE, -1]], // a first rule at priority level -1 has no group
        [{ ltr: '.x{}' }], // a rule that is not iterable
        revoked.proxy,
      ])
    ).toEqual([]);

    // A config read and a constant both fail: the reference reads the config
    // first.
    expect(
      divergences(
        [
          [
            ['xA', { constKey: 'xA', constVal: 'var(--xB)' }, 0],
            ['xB', { constKey: 'xB', constVal: 'var(--xA)' }, 0],
            ['x1', { ltr: '.x1{color:var(--xA)}' }, 3000],
          ],
        ],
        [{ useLayers: { before: 5 } }, { useLayers: { after: {} } }]
      )
    ).toEqual([]);
  });

  test('writes the reference sheet for a string rule and a string list', () => {
    expect(divergences(['ab', ['ab'], [['ab1', ['a', { ltr: '.a{x:0}' }, 1]]]])).toEqual([]);
  });

  test('reads undefined, null, -0 and a number past 2^63 as JavaScript holds them', () => {
    const cases: unknown[][] = [
      // An undefined priority is NaN and a null one is 0.
      [
        ['x1', styleOf('.x1{a:0}'), 1000],
        ['x2', styleOf('.x2{b:0}'), undefined],
      ],
      [
        ['x1', styleOf('.x1{a:0}'), 1000],
        ['x2', styleOf('.x2{b:0}'), null],
      ],
      // `String(undefined)` and `String(null)` name two constants.
      [
        [undefined, { constKey: 'k', constVal: 'red' }, 0],
        [null, { constKey: 'k', constVal: 'blue' }, 0],
        ['x1', styleOf('.x1{a:var(--undefined);b:var(--null)}'), 3000],
      ],
      // A number past 2^63 is spelled with an exponent.
      [
        [1e21, { constKey: 'k', constVal: 1e21 }, 0],
        ['x1', styleOf('.x1{a:var(--1e+21)}'), 3000],
      ],
      [[-0, styleOf('.a{a:0}'), -0]],
      [
        [0, styleOf('.a{a:0}'), 3000],
        [-0, styleOf('.b{a:0}'), 3000],
      ],
      // A key with a NUL character is a key like any other.
      [['x1', { ltr: '.x1{a:0}', 'a\u0000b': 1 }, 3000]],
    ];
    for (const rules of cases) {
      for (const config of [true, false]) {
        const [rust, babel] = answers(rules, config);
        expect(rust).toEqual(babel);
      }
    }
  });

  test('skips a hole in the rules as filter does, and writes the header for no rules', () => {
    const rule: StyleXRule = ['x1', { ltr: '.x1{a:0}', rtl: null }, 3000];
    // oxlint-disable-next-line no-sparse-arrays -- a hole is the input under test
    const holes: unknown[] = [, ,];
    const withHole: unknown[] = [];
    withHole[1] = rule;
    for (const rules of [holes, withHole, [undefined, rule], '', { length: 0 }, { length: 1 }]) {
      for (const config of [true, false, { useLayers: { before: ['a'], after: ['b'] } }]) {
        const [rust, babel] = answers(rules, config);
        expect(rust).toEqual(babel);
      }
    }
  });

  test('sorts as V8 sorts when NaN priorities make the order inconsistent', () => {
    const properties = ['color', 'margin', 'padding', 'top', 'left'];
    const rules: unknown[] = [];
    for (let i = 0; i < 80; i++) {
      const priority =
        i % 7 === 3 ? Number.NaN : i % 11 === 5 ? undefined : (i % 3) * 1000 + (i % 4) * 100;
      rules.push([
        `x${i}`,
        { ltr: `.x${i}{${properties[(i * 7) % 5]}:${(i * 13) % 17}}`, rtl: null },
        priority,
      ]);
    }
    for (const config of [true, false, { useLegacyClassnamesSort: true }]) {
      const [rust, babel] = answers(rules, config);
      expect(rust).toEqual(babel);
    }
  });

  test('reads a config as loosely as the reference does', () => {
    const rules: StyleXRule[] = [['x1', { ltr: '.x1{color:red}', rtl: null }, 3000]];
    const loose: unknown[] = [
      'layers', // `config ?? {}` then the property reads: a string has none of the properties
      { useLayers: null }, // `rawConfig.useLayers ?? false`
      { useLayers: 'yes' }, // `rawUseLayers !== false`: on, with the defaults
      { useLayers: 0 },
      { useLayers: NaN },
      { useLayers: { before: null, prefix: null } }, // `?? []`, `?? ''`
      { useLayers: false, enableLTRRTLComments: 1, legacyDisableLayers: null }, // truthiness
      { useLayers: false, enableLTRRTLComments: NaN }, // NaN is falsy
      { useLayers: [1] }, // an array is an object with none of the properties
      { useLayers: { before: 'a😀', after: [true, {}, null, [1, 2]] } }, // spread, then join
      { useLayers: { before: ['a', 5, null, undefined] } },
      { useLayers: { before: 5 } }, // not iterable: both throw
      { useLayers: { after: {} } },
      { useLayers: { prefix: 5 } }, // truthiness, then `String(...)`
      { useLayers: { prefix: 0 } },
      { useLayers: { prefix: [] } },
      { useLayers: { prefix: {} } },
      { useLayers: { prefix: NaN } },
      { useLayers: { prefix: true } },
    ];
    const withRtl: StyleXRule[] = [['x1', { ltr: '.x1{a:0}', rtl: '.x1{b:0}' }, 3000]];
    for (const config of loose) {
      {
        const [rust, babel] = answers(rules, config);
        expect(rust).toEqual(babel);
      }
      {
        const [rust, babel] = answers(withRtl, config);
        expect(rust).toEqual(babel);
      }
    }
  });

  test('reads a rule tuple as loosely as the reference does', () => {
    // Each row of the review's table, and the undefined and NaN values that
    // cross the boundary as JavaScript values rather than as JSON.
    const style = { ltr: '.x1{color:red}', rtl: null };
    const cases: unknown[][] = [
      [['x1', style, 3000, 'extra']], // a fourth element is not read
      [['x1', style]], // no priority: NaN, a group of its own outside the layers
      [['x1', style, undefined]], // an undefined priority slot: NaN too
      [['x1', style, NaN]],
      [
        ['x1', style, NaN],
        ['x2', { ltr: '.x2{margin:0}', rtl: null }, 3000],
      ],
      [
        ['x1', style, '3000'],
        ['x2', { ltr: '.x2{margin:0}', rtl: null }, 1000],
      ], // a numeric string
      [
        ['x1', style, null],
        ['x2', { ltr: '.x2{margin:0}', rtl: null }, 1000],
      ], // null is 0
      [
        ['x1', style, true],
        ['x2', { ltr: '.x2{margin:0}', rtl: null }, 1000],
      ], // true is 1
      [
        ['x1', style, Infinity],
        ['x2', { ltr: '.x2{margin:0}', rtl: null }, Infinity],
      ],
      [['x1', { ltr: '.x1{a:0}', rtl: false }, 3000]], // a falsy rtl is no RTL text
      [
        ['xA', { constKey: 'xA', constVal: 'red', ltr: 5, rtl: 7 }, 0],
        ['x1', { ltr: '.x1{color:var(--xA)}' }, 3000],
      ],
      // NaN and Infinity priorities: a comparator that is not a consistent order.
      [
        ['x1', { ltr: '.x1{b:0}' }],
        ['x2', { ltr: '.x2{a:0}' }],
      ],
      [
        ['x1', { ltr: '.x1{b:0}' }, Infinity],
        ['x2', { ltr: '.x2{a:0}' }, Infinity],
      ],
      // A priority, a class name or a constant of any kind, coerced.
      [
        ['x1', style, [1000]],
        ['x2', { ltr: '.x2{b:0}' }, 3000],
      ],
      [
        ['x1', style, {}],
        ['x2', { ltr: '.x2{b:0}' }, 3000],
      ],
      [
        ['x1', style, [1, 2]],
        ['x2', { ltr: '.x2{b:0}' }, 3000],
      ],
      [[12, { ltr: '.x12{a:0}' }, 3000]],
      [
        [1, { ltr: '.a{a:0}' }, 3000],
        ['1', { ltr: '.b{a:0}' }, 3000],
      ],
      [
        [NaN, { ltr: '.a{a:0}' }, 3000],
        [NaN, { ltr: '.b{a:0}' }, 3000],
      ],
      [
        [{}, { ltr: '.a{a:0}' }, 3000],
        [{}, { ltr: '.b{a:0}' }, 3000],
      ],
      [
        ['xA', { constKey: 5, constVal: 'red' }, 0],
        ['x1', { ltr: '.x1{color:var(--xA)}' }, 3000],
      ],
      [
        ['xA', { constKey: 'xA', constVal: {} }, 0],
        ['x1', { ltr: '.x1{color:var(--xA)}' }, 3000],
      ],
      [
        ['xA', { constKey: 'xA', constVal: ['a', ['b', null]] }, 0],
        ['x1', { ltr: '.x1{color:var(--xA)}' }, 3000],
      ],
      // A string rule is iterable, and its style is a character.
      [['ab']],
      [['ab1', ['a', { ltr: '.a{x:0}' }, 1]]],
      // A style that is not an object has no ltr.
      [['x1', 5, 3000]],
      [['x1', ['a'], 3000]],
      // A text that is never read is never refused.
      [
        ['x1', { ltr: 5 }, 1000],
        ['x1', { ltr: '.x1{a:0}' }, 1500],
      ],
      [
        ['x1', { ltr: 5 }, 1000],
        ['x2', { ltr: '.x2{a:0}' }, 1500],
      ],
      [
        ['x1', { ltr: '.x1{a:0}', rtl: 5 }, 1000],
        ['x1', { ltr: '.x1{b:0}' }, 1500],
      ],
      [
        ['x1', { ltr: ['var(--stylex-logical-start)'] }, 1000],
        ['x1', { ltr: '.x1{a:0}' }, 1500],
      ],
      // A first rule at priority level -1 has no group in the reference.
      [['x1', style, -1]],
      [['x1', style, -1000]],
      [['x1', style, -1001]],
      [
        ['x0', style],
        ['x1', style, -1],
      ],
      // A prelude is trimmed as `trimEnd` trims.
      [
        ['x1', { ltr: '@media (min-width: 10px)\u0085{.x1{a:0}}' }, 3000],
        ['x2', { ltr: '@media (min-width: 5px)\u0085{.x2{a:0}}' }, 3000],
      ],
      [
        ['x1', { ltr: '@media (min-width: 10px)\uFEFF{.x1{a:0}}' }, 3000],
        ['x2', { ltr: '@media (min-width: 5px)\uFEFF{.x2{a:0}}' }, 3000],
      ],
    ];
    for (const rules of cases) {
      for (const config of [
        true,
        { useLayers: false },
        { useLayers: false, legacyDisableLayers: true },
        { useLegacyClassnamesSort: true },
      ]) {
        {
          const [rust, babel] = answers(rules, config);
          expect(rust).toEqual(babel);
        }
      }
    }
  });

  test('answers as the reference for a legacy sort receiver that is not a string', () => {
    const legacy = { useLegacyClassnamesSort: true };
    for (const rules of [
      [
        [1, { ltr: '.a{a:0}' }, 3000],
        ['b', { ltr: '.b{a:0}' }, 3000],
      ],
      [
        ['b', { ltr: '.b{a:0}' }, 3000],
        [1, { ltr: '.a{a:0}' }, 3000],
      ],
      [
        ['b', { ltr: '.b{a:0}' }, 3000],
        [{}, { ltr: '.a{a:0}' }, 3000],
      ],
      [
        [1, { ltr: '.a{a:0}' }, 3000],
        ['b', { ltr: '.b{a:0}' }, 1000],
      ],
    ]) {
      const [rust, babel] = answers(rules, legacy);
      expect(rust).toEqual(babel);
    }
  });

  // ==================== inputs the gate leaves to the reference ====================

  test('runs every getter as often as the reference does, in the same order', () => {
    const [[nativeRules, nativeConfig], nativeReads] = withGetters();
    const [[babelRules, babelConfig], babelReads] = withGetters();
    expect(untyped(processStylexRules)([nativeRules], nativeConfig)).toBe(
      untyped(babelProcessStylexRules)([babelRules], babelConfig)
    );
    expect(nativeReads).toEqual(babelReads);
    // Two filters and the spread each read it.
    expect(nativeReads.filter(name => name === 'constKey')).toHaveLength(3);
  });

  test('throws the error a getter throws, at the read that runs it', () => {
    const throwing = {
      get ltr(): string {
        throw new RangeError('the ltr getter refuses');
      },
    };
    expect(
      divergences([
        [['x1', throwing, 3000]],
        [
          ['x1', STYLE, 3000],
          ['x2', throwing, 3000],
        ],
      ])
    ).toEqual([]);
  });

  test('reads a Proxy of the rules, of a rule and of a style as the reference does', () => {
    const traps: string[] = [];
    const logging = <T extends object>(target: T): T =>
      new Proxy(target, {
        get(object, key, receiver) {
          traps.push(String(key));
          return Reflect.get(object, key, receiver);
        },
      });

    expect(
      divergences([
        logging([['x1', STYLE, 3000]]),
        [logging(['x1', STYLE, 3000])],
        [['x1', logging({ ...STYLE }), 3000]],
        [['x1', STYLE, 3000]].map(rule => logging(rule)),
      ])
    ).toEqual([]);
    expect(traps.length).toBeGreaterThan(0);
  });

  test('coerces an object constant or class name as the reference does', () => {
    const named = { toString: () => 'named' };
    const once = {};
    expect(
      divergences([
        [
          ['xA', { constKey: 'xA', constVal: named }, 0],
          ['x1', { ltr: '.x1{color:var(--xA)}' }, 3000],
        ],
        [
          ['xA', { constKey: 'xA', constVal: new Date(0) }, 0],
          ['x1', { ltr: '.x1{color:var(--xA)}' }, 3000],
        ],
        [
          ['xA', { constKey: 'xA', constVal: new Map([[1, 2]]) }, 0],
          ['x1', { ltr: '.x1{color:var(--xA)}' }, 3000],
        ],
        // One object as two class names is one Map key; two objects are two.
        [
          [once, { ltr: '.a{a:0}' }, 3000],
          [once, { ltr: '.b{a:0}' }, 3000],
        ],
        [
          [named, { ltr: '.a{a:0}' }, 3000],
          ['named', { ltr: '.b{a:0}' }, 3000],
        ],
      ])
    ).toEqual([]);
  });

  test('compares an array ltr text as the reference does, on either side', () => {
    expect(
      divergences([
        [
          ['x1', { ltr: ['.x1{a:0}'] }, 3000],
          ['x2', { ltr: '.x2{a:0}' }, 3000],
        ],
        [
          ['x2', { ltr: '.x2{a:0}' }, 3000],
          ['x1', { ltr: ['.x1{a:0}'] }, 3000],
        ],
      ])
    ).toEqual([]);
  });

  test('ignores a function, a symbol or a big integer where the reference never reads it', () => {
    const configs = [true, { useLayers: true, fn: () => 0, sym: Symbol('s'), big: 10n }];
    expect(
      divergences(
        [
          [['x1', STYLE, 3000, () => 0]],
          [['x1', STYLE, 3000, Symbol('s')]],
          [['x1', STYLE, 3000, 10n]],
          [['x1', { ...STYLE, fn: () => 0, sym: Symbol('s'), big: 10n }, 3000]],
          [['x1', { ...STYLE, [Symbol('key')]: 'value' }, 3000]],
        ],
        configs
      )
    ).toEqual([]);
    // Where the reference does read them, both throw.
    expect(
      divergences([
        [['x1', STYLE, Symbol('s')]],
        [['x1', STYLE, 10n]],
        [[Symbol('s'), STYLE, 3000]],
      ])
    ).toEqual([]);
  });

  test('keeps a lone surrogate wherever it sits, as JavaScript holds it', () => {
    const lone = '\uD800';
    expect(
      divergences(
        [
          [['x1', styleOf(`.x1{content:"${lone}"}`), 3000]],
          [['x1', { ltr: '.x1{a:0}', rtl: `.x1{content:"${lone}"}` }, 3000]],
          [[`x${lone}`, styleOf('.x1{a:0}'), 3000]],
          [
            ['xA', { constKey: 'xA', constVal: `a${lone}` }, 0],
            ['x1', styleOf('.x1{content:var(--xA)}'), 3000],
          ],
          [
            ['xA', { constKey: `k${lone}`, constVal: 'red' }, 0],
            ['x1', styleOf('.x1{color:var(--xA)}'), 3000],
          ],
          [['x1', { ...STYLE, [`k${lone}`]: 'v' }, 3000]],
        ],
        [true, false, { useLayers: { before: [`a${lone}`], prefix: `p${lone}` } }]
      )
    ).toEqual([]);
  });

  test('sorts a lone surrogate, U+FFFD and U+10FFFF in the order JavaScript gives', () => {
    const texts = ['\uD800', '�', '\u{10FFFF}'];
    const orders = [
      [0, 1, 2],
      [0, 2, 1],
      [1, 0, 2],
      [1, 2, 0],
      [2, 0, 1],
      [2, 1, 0],
    ];
    expect(
      divergences(
        orders.map(order =>
          order.map(index => [`x${index}`, styleOf(`.x{content:"${texts[index]}"}`), 3000])
        )
      )
    ).toEqual([]);
  });

  test('cuts an astral last character of a text with no block as JavaScript does', () => {
    expect(
      divergences([
        [['x1', styleOf('.a😀'), 3000]],
        [
          ['x1', styleOf('ab:😁'), 3000],
          ['x2', styleOf('ab:😀'), 3000],
        ],
        [
          ['x0', styleOf('.k{a:0}'), 0],
          ['x1', { ltr: '.a{b:c}', rtl: '.a😀' }, 3000],
        ],
      ])
    ).toEqual([]);
  });

  test('reads the rules in odd shapes as the reference does', () => {
    const frozen = Object.freeze([Object.freeze(['x1', Object.freeze({ ...STYLE }), 3000])]);
    class Rules extends Array<unknown> {}
    const subclassed = Rules.from([['x1', STYLE, 3000]]);
    const named = Object.assign([['x1', STYLE, 3000]], { foo: 'bar' });
    const withConstructor = Object.assign([['x1', STYLE, 3000]], { constructor: Array });
    const sparse: unknown[] = [];
    sparse.length = 1e7;
    sparse[5] = ['x1', STYLE, 3000];
    const inherited = ['x1', Object.create({ ltr: '.x1{a:0}' }), 3000];
    const numericKeys = ['x1', { 2: 'b', ltr: '.x1{a:0}', 1: 'a' }, 3000];
    const nullPrototype = ['x1', Object.assign(Object.create(null), STYLE), 3000];

    expect(
      divergences([
        frozen,
        subclassed,
        named,
        withConstructor,
        [inherited],
        [numericKeys],
        [nullPrototype],
        // A boxed number: `valueOf` runs at each comparison.
        [['x1', STYLE, Reflect.construct(Number, [3000])]],
        [['x1', STYLE, new Float64Array([3000])]],
      ])
    ).toEqual([]);

    // The gate stops at the first hole, so a sparse list costs only the
    // reference's own walk. A clock bound here failed under a parallel run.
    expect(nativeInputOf(sparse, true, 'all')).toBeNull();
    expect(divergences([sparse], [true])).toEqual([]);
  });

  test('reads a deep or shared value only where the reference reads it', () => {
    let shared: unknown = 'leaf';
    for (let level = 0; level < 40; level++) {
      shared = { a: shared, b: shared };
    }
    let deep: unknown = 'x';
    for (let level = 0; level < 70; level++) {
      deep = [deep];
    }
    let deepConfig: unknown = true;
    for (let level = 0; level < 70; level++) {
      deepConfig = { useLayers: deepConfig };
    }

    // The gate takes the rule natively without a walk of the unused key: a
    // walk of the 40 shared levels would not end.
    expect(nativeInputOf([['x1', { ...STYLE, unused: shared }, 3000]], true, 'all')).not.toBeNull();
    expect(
      divergences(
        [
          [['x1', { ...STYLE, unused: shared }, 3000]],
          [['x1', STYLE, 3000, shared]],
          // Babel writes `""` for a constant nested 70 arrays deep.
          [
            ['xA', { constKey: 'xA', constVal: deep }, 0],
            ['x1', styleOf('.x1{color:var(--xA)}'), 3000],
          ],
        ],
        [true, deepConfig]
      )
    ).toEqual([]);
  });

  test('reads a constant rule with no ltr text, or a text of any kind', () => {
    expect(
      divergences([
        [
          ['xA', { constKey: 'xA', constVal: 'red' }, 0],
          ['x1', styleOf('.x1{color:var(--xA)}'), 3000],
        ],
        [
          ['xA', { constKey: 'xA', constVal: 'red', ltr: 5, rtl: 7 }, 0],
          ['x1', styleOf('.x1{color:var(--xA)}'), 3000],
        ],
        // A half-declared constant is a style rule.
        [['xA', { constKey: 'xA' }, 0]],
        [['xA', { constVal: 'red', ltr: '.xA{a:0}' }, 0]],
        // An empty tuple has no style.
        [[]],
      ])
    ).toEqual([]);
  });

  test('resolves constants in the order the reference resolves them', () => {
    const reader = ['x1', styleOf('.x1{margin:var(--a);padding:var(--c);--a:0}'), 3000];

    expect(
      divergences([
        [...chain(['a', 'b', 'c']), reader],
        [...chain(['a', 'b', 'c']).toReversed(), reader],
        // A name declared twice keeps its first position and its last value.
        [
          ['a', { constKey: 'a', constVal: 'var(--b)' }, 0],
          ['b', { constKey: 'b', constVal: '1px' }, 0],
          ['a', { constKey: 'a', constVal: 'var(--c)' }, 0],
          ['c', { constKey: 'c', constVal: '3px' }, 0],
          reader,
        ],
        // Ten constants: the addon searches them with one automaton.
        [
          ...[
            ['c0', '0px'],
            ['c1', 'var(--t1)'],
            ['c2', '2px'],
            ['c3', 'var(--c7)'],
            ['c4', '4px'],
            ['c5', 'var(--u5, 1px)'],
            ['c6', '6px'],
            ['c7', '7px'],
            ['c8', 'x --c5: y'],
            ['c9', 'z --c1: w'],
          ].map(([name, value]) => [name, { constKey: name, constVal: value }, 0]),
          ['x1', styleOf('.x1{margin:var(--c0) var(--c3) var(--c9)}'), 3000],
          ['x2', styleOf('.x2{--c1:1;--c5:5;padding:var(--c8)}'), 3000],
          ['x3', styleOf('.x3{--c3:var(--c4)}'), 3000],
          ['x4', { ltr: '.x4{a:var(--c2)}', rtl: '.x4{b:var(--c6)}' }, 3000],
        ],
        // A replacement template with `$` patterns, and an unknown reference.
        // Only a rule reads `a`: a constant that reads it loops without end
        // upstream, because `$&` writes the reference back.
        [
          ['a', { constKey: 'a', constVal: "$$ $& $` $' $1 var(--zz)" }, 0],
          ['b', { constKey: 'b', constVal: 'x var(--d) y' }, 0],
          ['c', { constKey: 'c', constVal: 'var(--b)' }, 0],
          ['d', { constKey: 'd', constVal: "$$ $` $' $1 var(--zz)" }, 0],
          reader,
        ],
      ])
    ).toEqual([]);
  });

  // ==================== built-ins and environment ====================

  describe('with a changed built-in', () => {
    const restore: Array<() => void> = [];

    afterEach(() => {
      for (const undo of restore.splice(0).toReversed()) {
        undo();
      }
    });

    /** Defines a property for one test, and puts back what was there. */
    function patch(target: object, key: PropertyKey, value: unknown): void {
      const previous = Object.getOwnPropertyDescriptor(target, key);
      Object.defineProperty(target, key, { value, configurable: true, writable: true });
      restore.push(() => {
        if (previous === undefined) {
          Reflect.deleteProperty(target, key);
        } else {
          Object.defineProperty(target, key, previous);
        }
      });
    }

    const rules = [
      ['x1', styleOf('.x1{color:red}'), 3000],
      ['x2', styleOf('.x2{color:blue}'), 3000],
    ];

    test('sorts with a replaced localeCompare', () => {
      // A call before the change records the built-ins as they are.
      expect(divergences([rules])).toEqual([]);
      patch(String.prototype, 'localeCompare', function (this: string, other: string) {
        return this < other ? 1 : this > other ? -1 : 0;
      });
      expect(divergences([rules])).toEqual([]);
    });

    test('reads an index or a property that a prototype holds', () => {
      expect(divergences([rules])).toEqual([]);
      patch(Array.prototype, 1, ['x9', styleOf('.x9{a:0}'), 3000]);
      // oxlint-disable-next-line no-sparse-arrays -- a hole reads the prototype
      expect(divergences([[rules[0], , rules[1]], [['x1', {}, 3000]]])).toEqual([]);
    });

    test('reads an ltr text that Object.prototype holds', () => {
      expect(divergences([rules])).toEqual([]);
      patch(Object.prototype, 'ltr', '.inherited{a:0}');
      expect(divergences([[['x1', {}, 3000]], rules])).toEqual([]);
    });

    test('creates arrays through a replaced Array[Symbol.species]', () => {
      expect(divergences([rules])).toEqual([]);
      class Species extends Array<unknown> {}
      const own = Object.assign([...rules], { constructor: { [Symbol.species]: Species } });
      expect(divergences([own])).toEqual([]);
      patch(Array, Symbol.species, Species);
      expect(divergences([rules])).toEqual([]);
    });
  });
});

describe('processStylexRules in a child process', () => {
  const distDir = path.join(packageDir, 'dist');

  /** The rules of the fixture that a build emits, as JSON. */
  const buildRules = JSON.stringify(
    fixture.cases.flatMap(recorded => recorded.calls.map(call => call.rules))
  );

  test('writes the sheet natively for the rules a build emits', () => {
    const outcome = runNodeScriptOrThrow(`
      const reference = require(${JSON.stringify(path.join(distDir, 'process-stylex-rules/reference.js'))});
      const create = reference.createReferenceProcessStylexRules;
      let calls = 0;
      reference.createReferenceProcessStylexRules = hook => {
        const run = create(hook);
        return (...args) => { calls++; return run(...args); };
      };
      const { processStylexRules } = require(${JSON.stringify(path.join(distDir, 'index.js'))});
      for (const rules of ${buildRules}) {
        for (const config of [true, false, { enableLTRRTLComments: true }]) {
          processStylexRules(rules, config);
        }
      }
      process.stdout.write(String(calls));
    `);

    expect(outcome.stdout).toBe('0');
  });

  /**
   * Whether the export and the reference answer alike, the sheet or the
   * error, in a child process that runs `setup` before its first call. Both
   * modules load first: some changes stop a module from loading at all.
   */
  function sameSheetAfter(setup: string, rules: string): string {
    return runNodeScriptOrThrow(`
      const { processStylexRules } = require(${JSON.stringify(path.join(distDir, 'index.js'))});
      const babel = require(${JSON.stringify(require.resolve('@stylexjs/babel-plugin'))});
      ${setup}
      const rules = ${rules};
      const answer = run => {
        try {
          return 'sheet ' + run(rules, false);
        } catch (error) {
          return 'error ' + error.name + ': ' + error.message;
        }
      };
      process.stdout.write(String(answer(processStylexRules) === answer(babel.processStylexRules)));
    `).stdout;
  }

  test('sees a native function that a program moved before the first call', () => {
    const rules = JSON.stringify([['a', styleOf('.a{float:var(--stylex-logical-start)}'), 3000]]);
    for (const setup of [
      'String.prototype.includes = Array.prototype.includes;',
      'String.prototype.includes = Object.getPrototypeOf(Uint8Array.prototype).includes;',
      'String.prototype.localeCompare = function localeCompare() { return 0; };',
      'Object.defineProperty(Object.prototype, "ltr", { value: ".x{a:0}", configurable: true });',
    ]) {
      expect(sameSheetAfter(setup, rules)).toBe('true');
    }
  });

  test('reads no global that a program can replace', () => {
    const rules = `(() => {
      const rule = ['a', { ltr: '.a{color:red}' }, 3000];
      rule[Symbol.iterator] = function* () {
        yield 'zz'; yield { ltr: '.zz{color:green}' }; yield 3000;
      };
      return [rule, ['b', { ltr: '.b{color:blue}' }, 3000]];
    })()`;
    expect(sameSheetAfter('', rules)).toBe('true');
    expect(
      sameSheetAfter(
        'const S = Symbol; globalThis.Symbol = { iterator: "nope" }; globalThis.SymbolS = S;',
        rules.replaceAll('Symbol.iterator', 'SymbolS.iterator')
      )
    ).toBe('true');
    expect(
      sameSheetAfter(
        'globalThis.Boolean = () => true;',
        '[["a", { ltr: ".a{a:0}", rtl: ".a{b:0}" }, 3000]]'
      )
    ).toBe('true');
  });

  test('sorts as the reference does under a locale with its own collation', () => {
    const rules = [
      ['x1', styleOf('.x1{content:"z"}'), 3000],
      ['x2', styleOf('.x2{content:"ö"}'), 3000],
      ['x3', styleOf('.x3{content:"o"}'), 3000],
    ];
    const outcome = runNodeScriptOrThrow(
      `
      const { processStylexRules } = require(${JSON.stringify(path.join(distDir, 'index.js'))});
      const babel = require(${JSON.stringify(require.resolve('@stylexjs/babel-plugin'))});
      const rules = ${JSON.stringify(rules)};
      const locale = new Intl.Collator().resolvedOptions().locale;
      const same = processStylexRules(rules, false) === babel.processStylexRules(rules, false);
      process.stdout.write(locale + ' ' + same);
    `,
      { env: { ...process.env, LANG: 'sv_SE.UTF-8', LC_ALL: 'sv_SE.UTF-8' } }
    );

    expect(outcome.stdout).toBe('sv-SE true');
  });
});
