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

  test('refuses a value outside the published types with the processStylexRules: prefix', () => {
    const rules: StyleXRule[] = [['x1', { ltr: '.x1{color:red}', rtl: null }, 3000]];

    // The rules are not an array of tuples.
    expect(refused(() => processStylexRules([{ ltr: '.x{}' }] as unknown as StyleXRule[]))).toMatch(
      /^processStylexRules: the rules could not be read/
    );
    expect(refused(() => processStylexRules({ length: 0 } as unknown as StyleXRule[]))).toMatch(
      /^processStylexRules: the rules could not be read/
    );
    // A layer option outside its declared type (the README lists these).
    for (const useLayers of [
      { prefix: 5 },
      { prefix: true },
      { before: 'ab' },
      { before: ['a', 5] },
    ]) {
      expect(
        refused(() =>
          processStylexRules(rules, { useLayers } as unknown as ProcessStylexRulesConfig)
        )
      ).toMatch(/^processStylexRules: the config could not be read/);
      // The reference writes a sheet from each of them.
      expect(typeof babelProcessStylexRules(rules, { useLayers } as never)).toBe('string');
    }
    // A class name that is not a string, a constant whose value is not a
    // string, a number or a boolean.
    expect(
      refused(() => processStylexRules([[12, { ltr: '.x1{color:red}' }, 3000]] as never))
    ).toMatch(/^processStylexRules: the rules could not be read/);
    expect(
      refused(() =>
        processStylexRules([
          ['xA', { constKey: 'xA', constVal: {} }, 0],
          ['x1', { ltr: '.x1{color:var(--xA)}' }, 3000],
        ] as never)
      )
    ).toMatch(/^processStylexRules: the rules could not be read/);
    // A refusal the reference shares, now under the same prefix.
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
    ];
    for (const rules of cases) {
      for (const config of [
        true,
        { useLayers: false },
        { useLayers: false, legacyDisableLayers: true },
      ]) {
        {
          const [rust, babel] = answers(rules as unknown as StyleXRule[], config);
          expect(rust).toEqual(babel);
        }
      }
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
      /^processStylexRules: the rules could not be read: .*a style rule has no ltr text/
    );
  });
});
