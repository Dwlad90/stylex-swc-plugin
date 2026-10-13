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

import stylexBabelPlugin from '@stylexjs/babel-plugin';
import { describe, expect, test } from 'vitest';

import { processStylexRules, transform } from '../dist/index.js';
import type { ProcessStylexRulesConfig, StyleXRule } from '../dist/index.js';

interface RecordedCall {
  rules: StyleXRule[];
  config: boolean | ProcessStylexRulesConfig | null;
  output: string;
}

interface RecordedCase {
  name: string;
  calls: RecordedCall[];
}

const fixture = JSON.parse(
  fs.readFileSync(
    path.join(__dirname, '../../stylex-stylesheet/tests/fixtures/process-stylex-rules-cases.json'),
    'utf8'
  )
) as { cases: RecordedCase[] };

const babelProcessStylexRules = (
  stylexBabelPlugin as unknown as { processStylexRules: typeof processStylexRules }
).processStylexRules;

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

  test('refuses rules that are not the metadata tuples, and a layer option outside its type', () => {
    expect(() => processStylexRules([{ ltr: '.x{}' }] as unknown as StyleXRule[], true)).toThrow(
      /processStylexRules: the rules are not an array of \[className, \{ ltr, rtl, constKey, constVal \}, priority\] tuples/
    );
    const rules: StyleXRule[] = [['x1', { ltr: '.x1{color:red}', rtl: null }, 3000]];
    expect(() =>
      processStylexRules(rules, { useLayers: { prefix: 5 } } as unknown as ProcessStylexRulesConfig)
    ).toThrow(/processStylexRules: the config could not be read/);
  });

  test('reads a config as loosely as the reference does', () => {
    const rules: StyleXRule[] = [['x1', { ltr: '.x1{color:red}', rtl: null }, 3000]];
    const loose = [
      'layers', // `config ?? {}` then the property reads: a string has none of the properties
      { useLayers: null }, // `rawConfig.useLayers ?? false`
      { useLayers: 'yes' }, // `rawUseLayers !== false`: on, with the defaults
      { useLayers: 0 },
      { useLayers: { before: null, prefix: null } }, // `?? []`, `?? ''`
      { useLayers: false, enableLTRRTLComments: 1, legacyDisableLayers: null }, // truthiness
    ] as unknown as ProcessStylexRulesConfig[];
    for (const config of loose) {
      expect(processStylexRules(rules, config)).toBe(babelProcessStylexRules(rules, config));
    }
  });
});
