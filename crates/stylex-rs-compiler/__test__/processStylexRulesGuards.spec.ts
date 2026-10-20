// The guards in front of the native `processStylexRules`: the collation that
// the native sort may stand in for, and the gate's answer when the ICU data of
// this Node is not the addon's. `processStylexRules.spec.ts` holds the whole
// path against the reference.
import { describe, expect, test } from 'vitest';

import {
  nativeCollationOf,
  ROOT_COLLATION_LANGUAGES,
} from '../dist/process-stylex-rules/collation.js';
import { nativeInputOf } from '../dist/process-stylex-rules/fast-path-gate.js';
import { collationDataVersion } from '../dist/transform.js';

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

    // This process runs under an `en` or root locale in CI.
    expect(nativeCollationOf(version, same)).toBe('all');
    for (const other of [{ icu: '1.0' }, { unicode: '1.0' }, { cldr: '1.0' }]) {
      expect(nativeCollationOf(version, { ...same, ...other })).toBe('printable-ascii');
    }
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
});
