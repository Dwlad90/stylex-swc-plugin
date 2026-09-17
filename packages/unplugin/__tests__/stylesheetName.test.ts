import path from 'node:path';

import { describe, expect, test } from 'vitest';

import {
  applyStylesheetRenames,
  assetNamesCarryHash,
  esbuildNamePattern,
  renameEsbuildStylesheet,
  resolveManifestFileName,
  shortContentHash,
  toPosixPath,
} from '../src/utils/stylesheetName';

describe('shortContentHash', () => {
  test('gives the same input the same digest', () => {
    expect(shortContentHash('.a{color:red}')).toBe(shortContentHash('.a{color:red}'));
  });

  test('gives different inputs different digests', () => {
    expect(shortContentHash('.a{color:red}')).not.toBe(shortContentHash('.a{color:blue}'));
  });

  test('is eight hex characters, whatever the input size', () => {
    expect(shortContentHash('')).toMatch(/^[0-9a-f]{8}$/);
    expect(shortContentHash('.x{color:red}'.repeat(100_000))).toMatch(/^[0-9a-f]{8}$/);
  });
});

describe('toPosixPath', () => {
  test('leaves a path that already uses forward slashes alone', () => {
    expect(toPosixPath('assets/main.css')).toBe('assets/main.css');
  });

  test('rewrites the platform separator', () => {
    expect(toPosixPath(path.join('assets', 'main.css'))).toBe('assets/main.css');
  });
});

describe('assetNamesCarryHash', () => {
  test.each([
    ['a pattern with a hash', 'assets/[name]-[hash][extname]', true],
    ['a pattern without one', 'assets/[name][extname]', false],
    ['an empty pattern', '', false],
  ])('reads %s', (_label, pattern, expected) => {
    expect(assetNamesCarryHash(pattern)).toBe(expected);
  });

  test.each([
    ['a function, which cannot be read', () => 'assets/[name]-[hash][extname]'],
    ['nothing at all, which leaves the host default', undefined],
  ])('assumes a hash for %s', (_label, pattern) => {
    expect(assetNamesCarryHash(pattern)).toBe(true);
  });
});

describe('resolveManifestFileName', () => {
  test.each([
    ['true takes the default', true, '.vite/manifest.json'],
    ['a string names the file', 'meta.json', 'meta.json'],
    ['false means no manifest', false, null],
    ['unset means no manifest', undefined, null],
  ])('%s', (_label, setting, expected) => {
    expect(resolveManifestFileName(setting, '.vite/manifest.json')).toBe(expected);
  });
});

describe('esbuildNamePattern', () => {
  test("returns null for esbuild's own default, which carries no hash", () => {
    expect(esbuildNamePattern('[dir]/[name]')).toBeNull();
  });

  test.each([
    ['[dir]/[name]-[hash]', 'main-AUEP42YD', 'AUEP42YD'],
    ['[dir]/[name]-[hash]', 'nested/main-AUEP42YD', 'AUEP42YD'],
    ['[dir]/[hash]-[name]', 'AUEP42YD-main', 'AUEP42YD'],
    ['css/[name].[hash]', 'css/main.AUEP42YD', 'AUEP42YD'],
    // A name of its own that holds dashes and capitals must not be mistaken for
    // the hash, which is the whole reason `[name]` is lazy.
    ['[dir]/[name]-[hash]', 'My-APP-AUEP42YD', 'AUEP42YD'],
  ])('reads the hash out of %s', (template, stem, expected) => {
    expect(esbuildNamePattern(template)?.exec(stem)?.[1]).toBe(expected);
  });

  test('does not match a name the template could not have produced', () => {
    expect(esbuildNamePattern('css/[name]-[hash]')?.exec('js/main-AUEP42YD')).toBeNull();
  });

  test('escapes a template that holds regular-expression characters', () => {
    const pattern = esbuildNamePattern('[name].bundle+[hash]');

    expect(pattern?.exec('main.bundle+AUEP42YD')?.[1]).toBe('AUEP42YD');
    expect(pattern?.exec('mainXbundle+AUEP42YD')).toBeNull();
  });

  // Wildcards next to each other used to make the engine try every way to
  // divide the text between them, which took minutes over a name this long.
  // Collapsing them is what holds this to microseconds.
  test('collapses a run of wildcards into one', () => {
    expect(esbuildNamePattern('[dir]/[name][name][name]-[hash]')?.source).toBe(
      esbuildNamePattern('[dir]/[name]-[hash]')?.source
    );
  });

  test('settles quickly on a long name that cannot match', () => {
    const pattern = esbuildNamePattern('[dir]/[name][name][name]-[hash]');
    const started = Date.now();

    expect(pattern?.exec('a'.repeat(20_000))).toBeNull();
    expect(Date.now() - started).toBeLessThan(1_000);
  });
});

describe('renameEsbuildStylesheet', () => {
  const outDir = path.join('/out');

  test('replaces the hash and keeps everything else', () => {
    const pattern = esbuildNamePattern('[dir]/[name]-[hash]');
    const renamed = renameEsbuildStylesheet(
      pattern!,
      outDir,
      path.join(outDir, 'main-AUEP42YD.css'),
      '.a{color:red}'
    );

    expect(path.basename(renamed ?? '')).toMatch(/^main-[0-9A-F]{8}\.css$/);
    expect(renamed).not.toContain('AUEP42YD');
  });

  test('keeps the sub-directory a nested template asks for', () => {
    const pattern = esbuildNamePattern('[dir]/[name]-[hash]');
    const renamed = renameEsbuildStylesheet(
      pattern!,
      outDir,
      path.join(outDir, 'nested', 'main-AUEP42YD.css'),
      '.a{color:red}'
    );

    expect(renamed).toBe(
      path.join(outDir, 'nested', `main-${shortContentHash('.a{color:red}').toUpperCase()}.css`)
    );
  });

  test('gives the same contents the same name', () => {
    const pattern = esbuildNamePattern('[dir]/[name]-[hash]');
    const cssFile = path.join(outDir, 'main-AUEP42YD.css');
    const rename = (source: string) => renameEsbuildStylesheet(pattern!, outDir, cssFile, source);

    expect(rename('.a{color:red}')).toBe(rename('.a{color:red}'));
    expect(rename('.a{color:red}')).not.toBe(rename('.a{color:blue}'));
  });

  test('leaves a name the template could not have produced alone', () => {
    const pattern = esbuildNamePattern('css/[name]-[hash]');

    expect(
      renameEsbuildStylesheet(pattern!, outDir, path.join(outDir, 'main.css'), '.a{color:red}')
    ).toBeNull();
  });
});

describe('applyStylesheetRenames', () => {
  test('returns the text untouched when there is nothing to rename', () => {
    expect(applyStylesheetRenames('<link href="a.css">', [])).toBe('<link href="a.css">');
  });

  test('replaces every occurrence of every old name', () => {
    const text = '<link href="/a.css"><link href="/b.css"><link href="/a.css">';

    expect(
      applyStylesheetRenames(text, [
        { from: 'a.css', to: 'a-1.css' },
        { from: 'b.css', to: 'b-2.css' },
      ])
    ).toBe('<link href="/a-1.css"><link href="/b-2.css"><link href="/a-1.css">');
  });

  // One pass, so a new name that happens to be another rename's old name is not
  // rewritten a second time.
  test('does not rewrite a name it has just written', () => {
    expect(
      applyStylesheetRenames('a.css b.css', [
        { from: 'a.css', to: 'b.css' },
        { from: 'b.css', to: 'c.css' },
      ])
    ).toBe('b.css c.css');
  });

  // StyleX hashes have no fixed length, so a shorter name can sit inside a
  // longer one. Longest first is what keeps the longer one whole.
  test('prefers the longest old name when one contains another', () => {
    expect(
      applyStylesheetRenames('assets/main-ab.css', [
        { from: 'main-ab.css', to: 'main-zz.css' },
        { from: 'ab.css', to: 'cd.css' },
      ])
    ).toBe('assets/main-zz.css');
  });

  test('leaves regular-expression characters in a name alone', () => {
    expect(applyStylesheetRenames('a+b.css', [{ from: 'a+b.css', to: 'a+c.css' }])).toBe('a+c.css');
  });

  test('handles a very large document', () => {
    const text = `${'x'.repeat(2_000_000)}assets/a.css${'y'.repeat(2_000_000)}`;
    const result = applyStylesheetRenames(text, [{ from: 'assets/a.css', to: 'assets/b.css' }]);

    expect(result).toContain('assets/b.css');
    expect(result).not.toContain('assets/a.css');
  });
});
