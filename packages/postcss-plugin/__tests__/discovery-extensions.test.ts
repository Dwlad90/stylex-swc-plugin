import { INCLUDE_EXTENSIONS } from '@stylexswc/plugin-shared/constants';
import { describe, expect, test } from 'vitest';

import { DEFAULT_INCLUDE_GLOB } from '../src/discovery';

// CSS discovery scans with a glob. The bundler plugins compile from a list.
// The two must name the same extensions. When discovery scans less than the
// plugins compile, StyleX compiles but the page gets no CSS.
describe('DEFAULT_INCLUDE_GLOB', () => {
  test('names every extension that the bundler plugins compile', () => {
    expect(DEFAULT_INCLUDE_GLOB).toBe(`**/*.{${INCLUDE_EXTENSIONS.join(',')}}`);
  });

  test('scans every directory below the root', () => {
    expect(DEFAULT_INCLUDE_GLOB.startsWith('**/*.')).toBe(true);
  });

  test.each(INCLUDE_EXTENSIONS)('holds the %s extension', extension => {
    expect(DEFAULT_INCLUDE_GLOB).toContain(extension);
  });
});
