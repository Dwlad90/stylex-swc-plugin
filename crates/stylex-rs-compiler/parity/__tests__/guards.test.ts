import { describe, expect, test } from 'vitest';

import { arrayAt, configurationOptionAt, isRecord, stringAt, verdictAt } from '../lib/guards.js';
import { CONFIGURATION_OPTIONS, VERDICTS } from '../lib/types.js';

/**
 * The guards were reachable only through `loadCorpus`, which reads them in one
 * order and only ever over parsed JSON. Tested here directly because their whole
 * job is the input the loader never sees: a value of the wrong type, a key that
 * is absent, and a key that every object has.
 */

/** The message a throwing call wrote, or a failure saying it did not throw. */
function messageOf(run: () => unknown): string {
  try {
    run();
  } catch (thrown) {
    return thrown instanceof Error ? thrown.message : String(thrown);
  }

  throw new Error('the call answered rather than throwing');
}

describe('isRecord', () => {
  test.each([
    ['a plain object', {}],
    ['an object carrying keys', { a: 1 }],
    ['an object with no prototype', Object.create(null)],
  ])('%s is a record', (_label, value) => {
    expect(isRecord(value)).toBe(true);
  });

  test.each([
    ['null', null],
    ['undefined', undefined],
    ['an array', []],
    ['a populated array', [1, 2]],
    ['a string', 'a'],
    ['a number', 1],
    ['a boolean', true],
    ['a function', () => undefined],
  ])('%s is not a record', (_label, value) => {
    expect(isRecord(value)).toBe(false);
  });
});

describe('stringAt', () => {
  test('answers the string at the key', () => {
    expect(stringAt({ id: 'a' }, 'id')).toBe('a');
  });

  /** An empty string is a value the loader must be able to tell from absence. */
  test('an empty string is answered rather than read as absent', () => {
    expect(stringAt({ note: '' }, 'note')).toBe('');
  });

  test.each([
    ['the key is absent', {}, 'id'],
    ['the value is a number', { id: 7 }, 'id'],
    ['the value is null', { id: null }, 'id'],
    ['the value is an array of strings', { id: ['a'] }, 'id'],
    ['the value is an object', { id: { a: 'b' } }, 'id'],
    ['the input is an array', ['a'], '0'],
    ['the input is null', null, 'id'],
    ['the input is a string', 'id', 'id'],
  ])('answers nothing when %s', (_label, value, key) => {
    expect(stringAt(value, key)).toBeUndefined();
  });

  /**
   * `toString` and friends are functions rather than strings, so the type check
   * already refuses them — this pins that, since the fields the loader reads are
   * named by corpus authors.
   */
  test.each(['toString', 'constructor', 'valueOf', 'hasOwnProperty'])(
    'a prototype key such as %s answers nothing',
    key => {
      expect(stringAt({}, key)).toBeUndefined();
    }
  );

  /**
   * The hazard the corpus actually carries is a prototype name arriving as a
   * *value*. This guard is not where it is caught — it answers the string as
   * written, and the closed tables below refuse it.
   */
  test('a prototype name as the value is answered, for the table to refuse', () => {
    expect(stringAt({ expected: 'toString' }, 'expected')).toBe('toString');
  });
});

describe('arrayAt', () => {
  test('answers the array at the key', () => {
    expect(arrayAt({ entries: [1, 2] }, 'entries')).toStrictEqual([1, 2]);
  });

  test('an empty array is answered rather than read as absent', () => {
    expect(arrayAt({ entries: [] }, 'entries')).toStrictEqual([]);
  });

  test.each([
    ['the key is absent', {}, 'entries'],
    ['the value is an object', { entries: {} }, 'entries'],
    ['the value is a string', { entries: 'a' }, 'entries'],
    ['the value is null', { entries: null }, 'entries'],
    ['the input is an array', [[1]], '0'],
    ['the input is undefined', undefined, 'entries'],
  ])('answers nothing when %s', (_label, value, key) => {
    expect(arrayAt(value, key)).toBeUndefined();
  });

  /** A corpus file is 800-odd entries; the guard must not walk what it answers. */
  test('a very large array is answered whole and not copied', () => {
    const entries = Array.from({ length: 200_000 }, (_, index) => index);

    expect(arrayAt({ entries }, 'entries')).toBe(entries);
  });
});

describe('verdictAt', () => {
  test('answers nothing when the key is absent', () => {
    expect(verdictAt({}, 'expected', 'edge.json')).toBeUndefined();
  });

  test.each(Object.keys(VERDICTS))('%s is a verdict', verdict => {
    expect(verdictAt({ expected: verdict }, 'expected', 'edge.json')).toBe(verdict);
  });

  test('an unknown verdict names the file, the field and every verdict', () => {
    const complaint = messageOf(() => verdictAt({ expected: 'nearly' }, 'expected', 'edge.json'));

    expect(complaint).toContain('edge.json names an unknown expected verdict: nearly');
    // Every verdict, asserted one at a time: which order `Object.keys` answers
    // them in is not what this is about.
    for (const verdict of Object.keys(VERDICTS)) expect(complaint).toContain(verdict);
  });

  test.each(['toString', 'constructor', '__proto__'])(
    'the prototype key %s is not a verdict',
    key => {
      expect(() => verdictAt({ expected: key }, 'expected', 'edge.json')).toThrow(
        /unknown expected verdict/
      );
    }
  );
});

describe('configurationOptionAt', () => {
  test('answers nothing when the key is absent', () => {
    expect(configurationOptionAt({}, 'configuration', 'edge.json')).toBeUndefined();
  });

  test.each(Object.keys(CONFIGURATION_OPTIONS))('%s is an option', option => {
    expect(configurationOptionAt({ configuration: option }, 'configuration', 'edge.json')).toBe(
      option
    );
  });

  test('an unknown option names the file, the field and every option', () => {
    const complaint = messageOf(() =>
      configurationOptionAt({ configuration: 'maxDepth' }, 'configuration', 'edge.json')
    );

    expect(complaint).toContain('edge.json names an unknown configuration option: maxDepth');
    for (const option of Object.keys(CONFIGURATION_OPTIONS)) expect(complaint).toContain(option);
  });

  test.each(['toString', 'constructor', '__proto__'])(
    'the prototype key %s is not an option',
    key => {
      expect(() =>
        configurationOptionAt({ configuration: key }, 'configuration', 'edge.json')
      ).toThrow(/unknown configuration option/);
    }
  );
});
