import { describe, expect, it } from 'vitest';

import { INCLUDE_EXTENSIONS, INCLUDE_REGEXP, buildIncludeGlob } from '../src/constants';

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
  // The two forms agree when the pattern accepts an extension exactly when the
  // list names it. Read against a wide alphabet, this states the agreement as
  // behaviour rather than repeating how the pattern is built.
  it('accepts an extension exactly when the list names it', () => {
    const alphabet = [
      ...INCLUDE_EXTENSIONS,
      'js ',
      ' js',
      'jss',
      'sjs',
      'j',
      't',
      'm',
      'c',
      'x',
      'ts2',
      'd',
      'node',
      'json',
      'mjsx',
      'JS',
      'Ts',
      '',
    ];

    for (const extension of alphabet) {
      const named = (INCLUDE_EXTENSIONS as readonly string[]).includes(extension);

      expect(INCLUDE_REGEXP.test(`/src/module.${extension}`)).toBe(named);
    }
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

  // A pattern that can backtrack takes exponential time on input of this shape,
  // so it would run out of the test timeout instead of answering. The timeout,
  // not a stopwatch reading, is what reports the fault here.
  it('answers on a long path that almost matches', () => {
    const nearMiss = `/src/${'module.mjsx/'.repeat(20_000)}name.txt`;

    expect(INCLUDE_REGEXP.test(nearMiss)).toBe(false);
  });

  it('holds no group that can repeat', () => {
    // A repeat inside a group is the shape that makes a pattern backtrack.
    expect(INCLUDE_REGEXP.source).not.toMatch(/[*+?]/);
  });
});

describe('buildIncludeGlob', () => {
  const extensions = `{${INCLUDE_EXTENSIONS.join(',')}}`;

  it('names every extension the list holds', () => {
    expect(buildIncludeGlob('src')).toBe(`src/**/*.${extensions}`);
  });

  it('scans from the place it is applied when no directory is given', () => {
    expect(buildIncludeGlob()).toBe(`**/*.${extensions}`);
  });

  it('treats an empty directory as no directory', () => {
    expect(buildIncludeGlob('')).toBe(`**/*.${extensions}`);
  });

  it('accepts a relative directory', () => {
    expect(buildIncludeGlob('./src')).toBe(`./src/**/*.${extensions}`);
  });

  it('accepts a nested directory', () => {
    expect(buildIncludeGlob('app/components')).toBe(`app/components/**/*.${extensions}`);
  });

  it.each(['src/', 'src//', 'src///'])('cuts the separators at the end of %s', dir => {
    expect(buildIncludeGlob(dir)).toBe(`src/**/*.${extensions}`);
  });

  it('keeps a separator that stands alone', () => {
    expect(buildIncludeGlob('/')).toBe(`/**/*.${extensions}`);
  });

  it('keeps a separator inside the directory', () => {
    expect(buildIncludeGlob('a//b')).toBe(`a//b/**/*.${extensions}`);
  });

  // A pattern such as `/\/+$/` reads this path in square time. The loop that
  // replaced it reads the path once, so this test answers rather than running
  // out of the test timeout.
  it('cuts the separators of a path that holds very many of them', () => {
    const dir = `a${'/'.repeat(200_000)}`;

    expect(buildIncludeGlob(dir)).toBe(`a/**/*.${extensions}`);
  });

  it('reads a path whose separators do not sit at the end', () => {
    // This is the shape that makes a backtracking pattern slow: many
    // separators, and a character after them that stops the match.
    const dir = `a${'/'.repeat(200_000)}b`;

    expect(buildIncludeGlob(dir)).toBe(`${dir}/**/*.${extensions}`);
  });
});
