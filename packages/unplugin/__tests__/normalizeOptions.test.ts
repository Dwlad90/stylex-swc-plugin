import { INCLUDE_EXTENSIONS } from '@stylexswc/plugin-shared/constants';
import { describe, expect, test } from 'vitest';

import normalizeOptions from '../src/utils/normalizeOptions';

describe('pageExtensions', () => {
  // The transform only reads a file whose extension is on this list. A list
  // that is shorter than the shared one lets StyleX reach the browser
  // uncompiled, which is the fault that .mjs modules showed first.
  test('defaults to every extension that the compiler transforms', () => {
    expect(normalizeOptions({}).pageExtensions).toEqual([...INCLUDE_EXTENSIONS]);
  });

  test.each(['mjs', 'cjs', 'mts', 'cts'])('covers %s by default', extension => {
    expect(normalizeOptions({}).pageExtensions).toContain(extension);
  });

  test('keeps a list that the user gives', () => {
    expect(normalizeOptions({ pageExtensions: ['ts'] }).pageExtensions).toEqual(['ts']);
  });

  test('keeps an empty list that the user gives', () => {
    expect(normalizeOptions({ pageExtensions: [] }).pageExtensions).toEqual([]);
  });

  test('gives each call its own list', () => {
    const first = normalizeOptions({}).pageExtensions;

    first.push('coffee');

    expect(normalizeOptions({}).pageExtensions).not.toContain('coffee');
  });
});
