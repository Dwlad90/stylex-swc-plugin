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
import * as path from 'node:path';

import { describe, expect, test } from 'vitest';

import { processStylexRules, transform } from '../dist/index.js';
import type { ProcessStylexRulesConfig, StyleXRule } from '../dist/index.js';
import { loadBabelPlugin } from '../parity/lib/compilers.js';
import { PROCESS_STYLEX_RULES_CASES, stylesheetFixturesDir } from '../parity/lib/fixtures.js';

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

/** What one implementation answers for an input: the sheet, or that it refused. */
type Answer = { sheet: string } | { refused: true };

function answerOf(run: () => string): Answer {
  try {
    return { sheet: run() };
  } catch {
    return { refused: true };
  }
}

/** The two implementations on one input, for `expect(...).toEqual(...)`. */
function answers(rules: readonly StyleXRule[], config?: unknown): [Answer, Answer] {
  const typedConfig = config as boolean | ProcessStylexRulesConfig | null | undefined;

  return [
    answerOf(() => processStylexRules(rules, typedConfig)),
    answerOf(() => babelProcessStylexRules(rules, typedConfig)),
  ];
}

const STYLE: StyleXRule[1] = { ltr: '.x1{color:red}', rtl: null };

/** A style with this `ltr` text and no RTL text. */
function styleOf(ltr: string): StyleXRule[1] {
  return { ltr, rtl: null };
}

/** The message a call refuses with; a call that answers fails the test. */
function refused(run: () => string): string {
  try {
    run();
  } catch (error) {
    return error instanceof Error ? error.message : String(error);
  }
  throw new Error('the call did not throw');
}

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

  test('refuses a circular constant with the reference message', () => {
    const rules: StyleXRule[] = [
      ['xA', { ltr: '', rtl: null, constKey: 'xA', constVal: 'var(--xB)' }, 0],
      ['xB', { ltr: '', rtl: null, constKey: 'xB', constVal: 'var(--xA)' }, 0],
      ['x1', { ltr: '.x1{color:var(--xA)}', rtl: null }, 3000],
    ];

    expect(() => processStylexRules(rules, true)).toThrow(
      'circular reference detected for constant --xB'
    );
    expect(() => babelProcessStylexRules(rules, true)).toThrow(
      'circular reference detected for constant --xB'
    );
  });

  test('refuses what the reference throws on, with the processStylexRules: prefix', () => {
    const rules: StyleXRule[] = [['x1', { ltr: '.x1{color:red}', rtl: null }, 3000]];

    expect(refused(() => processStylexRules([{ ltr: '.x{}' }] as unknown as StyleXRule[]))).toBe(
      'processStylexRules: rule 0 is an object, not a [className, style, priority] list'
    );
    expect(refused(() => processStylexRules('ab' as unknown as StyleXRule[]))).toBe(
      'processStylexRules: the rules are a string, not a list'
    );
    expect(
      refused(() =>
        processStylexRules(rules, {
          useLayers: { before: 5 },
        } as unknown as ProcessStylexRulesConfig)
      )
    ).toBe('processStylexRules: the layer option before is a number, not a list or a string');
    expect(refused(() => processStylexRules([['x1', { ltr: 5 }, 3000]] as never))).toBe(
      'processStylexRules: the ltr text of class x1 is a number, not a string'
    );
    expect(
      refused(() =>
        processStylexRules([
          ['xA', { ltr: '', rtl: null, constKey: 'xA', constVal: 'var(--xB)' }, 0],
          ['xB', { ltr: '', rtl: null, constKey: 'xB', constVal: 'var(--xA)' }, 0],
          ['x1', { ltr: '.x1{color:var(--xA)}', rtl: null }, 3000],
        ])
      )
    ).toBe('processStylexRules: circular reference detected for constant --xB');
  });

  test('refuses a value that has no JavaScript data form, wherever it sits', () => {
    const style = { ltr: '.x1{color:red}', rtl: null };
    const selfHolding: Record<string, unknown> = { ...style };
    selfHolding.self = selfHolding;
    let deep: unknown = 'x';
    for (let level = 0; level < 70; level++) {
      deep = [deep];
    }

    for (const [rules, message] of [
      [[['x1', style, 3000, () => 0]], 'the rules hold a function, which cannot be read'],
      [[['x1', style, Symbol('s')]], 'the rules hold a symbol, which cannot be read'],
      [
        [['x1', style, 10n]],
        'the rules hold a big integer or a value of an unknown kind, which cannot be read',
      ],
      [
        [['x1', selfHolding, 3000]],
        'the rules hold a value that contains itself, which cannot be read',
      ],
      [[['x1', style, deep]], 'the rules nest more than 64 levels deep, which cannot be read'],
    ] as const) {
      expect(refused(() => processStylexRules(rules as never))).toBe(
        `processStylexRules: ${message}`
      );
    }
    expect(refused(() => processStylexRules([['x1', style, 3000]], { a: () => 0 } as never))).toBe(
      'processStylexRules: the config hold a function, which cannot be read'
    );
    // One object in two places is not a cycle.
    const shared = { ltr: '.x1{color:red}', rtl: null };
    expect(
      processStylexRules([
        ['x1', shared, 3000],
        ['x1', shared, 3000],
      ])
    ).toBe(
      babelProcessStylexRules([
        ['x1', shared, 3000],
        ['x1', shared, 3000],
      ])
    );
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
        const [rust, babel] = answers(rules as unknown as StyleXRule[], config);
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
        const [rust, babel] = answers(rules as unknown as StyleXRule[], config);
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
      const [rust, babel] = answers(rules as unknown as StyleXRule[], config);
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
          const [rust, babel] = answers(rules as unknown as StyleXRule[], config);
          expect(rust).toEqual(babel);
        }
      }
    }
  });

  test('refuses a legacy sort receiver that is not a string, as V8 compares the pair', () => {
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
      const [rust, babel] = answers(rules as unknown as StyleXRule[], legacy);
      expect(rust).toEqual(babel);
    }
  });

  test('refuses a style rule with no ltr text, as the reference throws on it', () => {
    const cases: unknown[][] = [
      [['x1', { rtl: '.x1{color:red}' }, 3000]],
      [['x1', {}, 3000]],
      [['x1', { ltr: null }, 3000]],
      [['x1', { ltr: 5 }, 3000]],
      [['x1', { ltr: '.x1{a:0}', rtl: true }, 3000]], // a truthy rtl that is not a string
      [['x1', STYLE, 3000], undefined], // a sparse list: the reference cannot iterate the hole
      [['x1']], // no style object
    ];
    for (const rules of cases) {
      {
        const [rust, babel] = answers(rules as unknown as StyleXRule[], true);
        expect(rust).toEqual(babel);
      }
    }
    expect(() => processStylexRules([['x1', { rtl: '.x1{color:red}' }, 3000]] as never)).toThrow(
      'processStylexRules: the ltr text of class x1 is undefined, not a string'
    );
  });
});
