import { describe, expect, it } from 'vitest';

import { INCLUDE_EXTENSIONS, INCLUDE_REGEXP } from '../src/constants';

// `INCLUDE_EXTENSIONS` and `INCLUDE_REGEXP` say the same thing in two
// different forms. The bundler plugins read one or the other, so they must
// agree. These tests keep the two forms in step.
describe('INCLUDE_EXTENSIONS', () => {
  it('lists the eight JavaScript and TypeScript extensions', () => {
    expect([...INCLUDE_EXTENSIONS]).toEqual(['js', 'jsx', 'mjs', 'cjs', 'ts', 'tsx', 'mts', 'cts']);
  });

  it('has no duplicates', () => {
    expect(new Set(INCLUDE_EXTENSIONS).size).toBe(INCLUDE_EXTENSIONS.length);
  });

  it.each(INCLUDE_EXTENSIONS)('matches INCLUDE_REGEXP for .%s', extension => {
    expect(INCLUDE_REGEXP.test(`/src/module.${extension}`)).toBe(true);
  });
});

describe('INCLUDE_REGEXP', () => {
  it('is built from the list, so the two always agree', () => {
    expect(INCLUDE_REGEXP.source).toBe(`\\.(${INCLUDE_EXTENSIONS.join('|')})$`);
  });

  // These four read like the list entries but Node and TypeScript do not use
  // them. An earlier pattern accepted them.
  it.each(['mjsx', 'cjsx', 'mtsx', 'ctsx'])('rejects .%s, which the list does not name', ext => {
    expect(INCLUDE_REGEXP.test(`/src/module.${ext}`)).toBe(false);
  });

  it.each(['json', 'css', 'mjson', 'coffee', 'jsx2', 'tsxx', 'js.map', 'mts~', 'JS', 'TSX'])(
    'rejects .%s',
    ext => {
      expect(INCLUDE_REGEXP.test(`/src/module.${ext}`)).toBe(false);
    }
  );

  it('rejects a bare name that has no extension', () => {
    expect(INCLUDE_REGEXP.test('/src/Makefile')).toBe(false);
  });

  it('rejects a name that is only the extension without a dot', () => {
    expect(INCLUDE_REGEXP.test('/src/ts')).toBe(false);
  });

  it('rejects a path where the extension is only part of a directory name', () => {
    expect(INCLUDE_REGEXP.test('/src/app.ts/README')).toBe(false);
  });

  it('rejects an empty path', () => {
    expect(INCLUDE_REGEXP.test('')).toBe(false);
  });

  it('accepts a declaration file, because .d.ts ends in .ts', () => {
    expect(INCLUDE_REGEXP.test('/src/types.d.ts')).toBe(true);
  });

  it('accepts a name that is only a dot and an extension', () => {
    expect(INCLUDE_REGEXP.test('.ts')).toBe(true);
  });

  it('accepts a path that holds a newline', () => {
    expect(INCLUDE_REGEXP.test('/src/odd\nname.ts')).toBe(true);
  });

  it('rejects a path whose last line has no extension', () => {
    expect(INCLUDE_REGEXP.test('/src/module.ts\nREADME')).toBe(false);
  });

  it('stays anchored on a very long path', () => {
    const deepPath = `/${'nested/'.repeat(5_000)}module.mts`;

    expect(INCLUDE_REGEXP.test(deepPath)).toBe(true);
    expect(INCLUDE_REGEXP.test(`${deepPath}.map`)).toBe(false);
  });

  it('reads a long path that almost matches without slowing down', () => {
    // A pattern that can backtrack takes exponential time on input like this.
    const nearMiss = `/src/${'module.mjsx/'.repeat(20_000)}name.txt`;
    const start = Date.now();

    expect(INCLUDE_REGEXP.test(nearMiss)).toBe(false);
    expect(Date.now() - start).toBeLessThan(1_000);
  });

  it('holds no group that can repeat', () => {
    // A repeat inside a group is the shape that makes a pattern backtrack.
    expect(INCLUDE_REGEXP.source).not.toMatch(/[*+?]/);
  });
});
