// The guards in front of the native `processStylexRules`: the collation that
// the native sort may stand in for, the gate's answer when the ICU data of
// this Node is not the addon's, and the built-ins guard's answer after a
// program changes a prototype. `processStylexRules.spec.ts` holds the whole
// path against the reference.
import { createRequire } from 'node:module';
import * as path from 'node:path';

import { describe, expect, test } from 'vitest';

import {
  nativeCollationOf,
  ROOT_COLLATION_LANGUAGES,
} from '../dist/process-stylex-rules/collation.js';
import type { CollationRealm } from '../dist/process-stylex-rules/collation.js';
import { nativeInputOf } from '../dist/process-stylex-rules/fast-path-gate.js';
import { collationDataVersion } from '../dist/transform.js';
import { runNodeScriptOrThrow } from './nodeScript';

const require = createRequire(import.meta.url);
const distDir = path.resolve(__dirname, '..', 'dist');

/** A realm whose default collation is that of `locale`, whatever the host locale is. */
function realmWithLocale(locale: string): CollationRealm {
  class Collator extends Intl.Collator {
    constructor() {
      super(locale);
    }
  }
  return { Intl: { Collator, Locale: Intl.Locale } as unknown as typeof Intl };
}

const iteratorPrototype = 'Object.getPrototypeOf(Object.getPrototypeOf([][Symbol.iterator]()))';

/** Changes to the prototype of a built-in that the reference reads through. */
const PROTOTYPE_SWAPS = [
  `Object.setPrototypeOf(${iteratorPrototype}, {
    get return() { throw new Error('closed'); },
  });`,
  `Object.setPrototypeOf(${iteratorPrototype}, null);`,
  'Object.setPrototypeOf(Array.prototype, Object.create(Object.prototype));',
  'Object.setPrototypeOf(Map.prototype, Object.create(Object.prototype));',
  'Object.setPrototypeOf(String.prototype, null);',
  'Object.setPrototypeOf(Math, Object.create(Object.prototype));',
];

/**
 * The guard's answer and whether the export answers as the reference does,
 * in a child process that runs `change` before or after its first call.
 */
function afterAChange(change: string, when: 'before' | 'after' = 'after'): string {
  return runNodeScriptOrThrow(`
    const { processStylexRules } = require(${JSON.stringify(path.join(distDir, 'index.js'))});
    const { builtInsAreIntact } = require(
      ${JSON.stringify(path.join(distDir, 'process-stylex-rules/built-ins.js'))}
    );
    const babel = require(${JSON.stringify(require.resolve('@stylexjs/babel-plugin'))});
    const rules = [['a', { ltr: '.a{color:red}', rtl: null }, 3000]];
    ${when === 'after' ? 'processStylexRules(rules, false);' : ''}
    ${change}
    const answer = run => {
      try {
        return 'sheet ' + run(rules, false);
      } catch (error) {
        return 'error ' + error.name + ': ' + error.message;
      }
    };
    const intact = builtInsAreIntact();
    const same = answer(processStylexRules) === answer(babel.processStylexRules);
    process.stdout.write(intact + ' ' + same);
  `).stdout;
}

/** An `Intl.Locale` that refuses every tag, as it refuses an unknown one. */
function throwingLocale(): never {
  throw new RangeError('Incorrect locale information provided');
}

/** Strings that a tailoring would sort apart from root collation. */
const CORPUS = [
  'a',
  'A',
  'z',
  'Z',
  'å',
  'ä',
  'ö',
  'ø',
  'æ',
  'ß',
  'ss',
  'é',
  'e',
  'è',
  'ê',
  'ñ',
  'n',
  'ç',
  'c',
  'ch',
  'cz',
  'h',
  'i',
  'ı',
  'İ',
  'll',
  'l',
  'ł',
  'v',
  'w',
  'y',
  'ü',
  'u',
  'th',
  'þ',
  'ð',
  'd',
  'œ',
  'oe',
  'ij',
  'aa',
  'α',
  'ω',
  'а',
  'я',
  'ё',
  'е',
  '中',
  '日',
  '-',
  '_',
  ' ',
  '0',
  '9',
  '·',
  '½',
  '𝒜',
  '😀',
];

describe('the collation guard', () => {
  test('lists only languages that collate as root collation does', () => {
    const root = new Intl.Collator('und');
    const differences: string[] = [];
    for (const language of ROOT_COLLATION_LANGUAGES) {
      const collator = new Intl.Collator(language);
      for (const a of CORPUS) {
        for (const b of CORPUS) {
          if (Math.sign(collator.compare(a, b)) !== Math.sign(root.compare(a, b))) {
            differences.push(`${language}: ${a} ${b}`);
          }
        }
      }
    }
    expect(differences).toEqual([]);
  });

  test('allows every text only on a Node with the same ICU data', () => {
    const version = collationDataVersion();
    const same = {
      ...process.versions,
      icu: `${version.icu}.1`,
      unicode: version.unicode,
      cldr: `${version.cldr}.0`,
    };

    const english = realmWithLocale('en-US');
    expect(nativeCollationOf(version, same, english)).toBe('all');
    for (const other of [{ icu: '1.0' }, { unicode: '1.0' }, { cldr: '1.0' }]) {
      expect(nativeCollationOf(version, { ...same, ...other }, english)).toBe('printable-ascii');
    }
  });

  test('allows no text under a locale with a tailoring of its own', () => {
    const version = collationDataVersion();
    expect(nativeCollationOf(version, process.versions, realmWithLocale('sv-SE'))).toBe('none');
    expect(nativeCollationOf(version, process.versions, realmWithLocale('de-u-co-phonebk'))).toBe(
      'none'
    );
  });

  test('allows no text on a runtime whose Intl cannot tell the default collation', () => {
    const version = collationDataVersion();
    const brokenLocale = {
      Intl: { Collator: Intl.Collator, Locale: throwingLocale } as unknown as typeof Intl,
    };

    expect(nativeCollationOf(version, process.versions, {})).toBe('none');
    expect(nativeCollationOf(version, process.versions, { Intl: undefined })).toBe('none');
    expect(nativeCollationOf(version, process.versions, brokenLocale)).toBe('none');
    expect(
      nativeCollationOf(version, process.versions, { Intl: {} as unknown as typeof Intl })
    ).toBe('none');
  });
});

describe('the gate', () => {
  const ascii = [['x1', { ltr: '.x1{content:"a"}', rtl: null }, 3000]];
  const accented = [['x1', { ltr: '.x1{content:"é"}', rtl: null }, 3000]];

  test('packs a build’s rules', () => {
    const input = nativeInputOf(ascii, true, 'all');
    expect(input?.texts).toBe('x1.x1{content:"a"}');
    expect([...(input?.lengths ?? [])]).toEqual([2, 16]);
    expect([...(input?.priorities ?? [])]).toEqual([3000]);
    expect(input?.config.useLayers).toBe(true);
  });

  test('leaves text beyond printable ASCII to the reference on other ICU data', () => {
    expect(nativeInputOf(accented, true, 'all')).not.toBeNull();
    expect(nativeInputOf(accented, true, 'printable-ascii')).toBeNull();
    expect(nativeInputOf(ascii, true, 'printable-ascii')).not.toBeNull();
    expect(nativeInputOf(ascii, true, 'none')).toBeNull();

    // Under the legacy sort, the class names are compared too.
    const accentedClass = [['xé', { ltr: '.x1{a:0}', rtl: null }, 3000]];
    const legacy = { useLegacyClassnamesSort: true };
    expect(nativeInputOf(accentedClass, false, 'printable-ascii')).not.toBeNull();
    expect(nativeInputOf(accentedClass, legacy, 'printable-ascii')).toBeNull();

    // A constant's value becomes part of the texts that are compared.
    const accentedConstant = [
      ['xA', { constKey: 'xA', constVal: 'é' }, 0],
      ['x1', { ltr: '.x1{content:var(--xA)}', rtl: null }, 3000],
    ];
    expect(nativeInputOf(accentedConstant, true, 'printable-ascii')).toBeNull();
  });

  test('leaves a sparse list of any length to the reference', () => {
    const rule = ['x1', { ltr: '.x1{a:0}', rtl: null }, 3000];
    for (const length of [2, 2 ** 20, 2 ** 32 - 1]) {
      const rules: unknown[] = [rule];
      rules.length = length;
      expect(nativeInputOf(rules, false, 'all')).toBeNull();
    }
  });
});

describe('the built-ins guard', () => {
  test('holds while no built-in changes', () => {
    expect(afterAChange('')).toBe('true true');
  });

  test.each(['before', 'after'] as const)(
    'sees a prototype that a program swapped %s the first call',
    when => {
      for (const change of PROTOTYPE_SWAPS) {
        expect(afterAChange(change, when)).toBe('false true');
      }
    }
  );
});
