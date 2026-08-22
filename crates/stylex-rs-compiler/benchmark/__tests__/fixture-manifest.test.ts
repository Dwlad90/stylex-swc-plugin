/**
 * What `loadAllFixtures` refuses.
 *
 * The manifest is the one thing measurement and the trusted PR reporter
 * share, so a malformed entry must stop a run rather than be coerced into
 * something plausible: a fixture silently measured under the wrong shape, or
 * a name silently dropped from a trend series, is worse than a failed job.
 *
 * `fixtures.test.ts` covers the committed manifest. This file covers the
 * shapes that must never load, by pointing the loader at a manifest written
 * for the test.
 */

import fs from 'node:fs';
import os from 'node:os';
import path from 'node:path';

import { afterAll, describe, expect, test } from 'vitest';

import { loadAllFixtures } from '../lib/fixtures.js';

const roots: string[] = [];

afterAll(() => {
  for (const root of roots) {
    fs.rmSync(root, { recursive: true, force: true });
  }
});

const VALID_ENTRY = {
  name: 'only',
  file: 'styles.js',
  category: 'transform',
  weight: 'standard',
  batchSize: 1,
};

/**
 * A package directory whose `benchmark/fixtures.v1.json` holds `manifest`,
 * next to one readable fixture file the entries above can point at.
 *
 * `workspaceRoot` is the same directory, so a relative `file` in the manifest
 * resolves inside it.
 */
function withManifest(manifest: unknown): { packageDir: string; workspaceRoot: string } {
  const root = fs.mkdtempSync(path.join(os.tmpdir(), 'stylex-fixture-manifest-'));
  roots.push(root);

  fs.mkdirSync(path.join(root, 'benchmark'), { recursive: true });
  fs.writeFileSync(path.join(root, 'styles.js'), 'export const styles = 1;\n', 'utf8');
  fs.writeFileSync(
    path.join(root, 'benchmark', 'fixtures.v1.json'),
    typeof manifest === 'string' ? manifest : JSON.stringify(manifest),
    'utf8'
  );

  return { packageDir: root, workspaceRoot: root };
}

function load(manifest: unknown) {
  return loadAllFixtures(withManifest(manifest));
}

function withFixtures(fixtures: unknown) {
  return { schemaVersion: 1, fixtures };
}

function withEntry(overrides: Record<string, unknown>) {
  return withFixtures([{ ...VALID_ENTRY, ...overrides }]);
}

describe('the dev override', () => {
  test('loads as declared, both ways round', () => {
    expect(load(withEntry({ dev: true }))[0]?.dev).toBe(true);
    expect(load(withEntry({ dev: false }))[0]?.dev).toBe(false);
  });

  // Absent and `false` are different answers: absent means "whatever the
  // shared options say", which is what every fixture but one wants, and a
  // fixture that starts reporting `false` would pin the production shape
  // against a future change to that default.
  test('stays absent when the manifest does not mention it', () => {
    const [fixture] = load(withEntry({}));

    expect(fixture?.dev).toBeUndefined();
    expect(Object.hasOwn(fixture!, 'dev')).toBe(false);
  });

  test.each([
    ['a string', 'true'],
    ['the string false', 'false'],
    ['a number', 1],
    ['zero', 0],
    ['null', null],
    ['an object', {}],
    ['an array', []],
  ])('refuses %s', (_label, dev) => {
    expect(() => load(withEntry({ dev }))).toThrow(/dev must be a boolean/);
  });
});

describe('the option overrides', () => {
  test('loads every boolean feature key it accepts', () => {
    const options = {
      dev: true,
      debug: true,
      enableDebugDataProp: true,
      enableDebugClassNames: false,
      enableMinifiedKeys: false,
      useRealFileForSource: true,
      treeshakeCompensation: false,
    };

    expect(load(withEntry({ options }))[0]?.options).toEqual(options);
  });

  test('loads the two enum-valued keys', () => {
    expect(
      load(withEntry({ options: { styleResolution: 'legacy-expand-shorthands' } }))[0]?.options
    ).toEqual({ styleResolution: 'legacy-expand-shorthands' });
    expect(load(withEntry({ options: { propertyValidationMode: 'throw' } }))[0]?.options).toEqual({
      propertyValidationMode: 'throw',
    });
  });

  test('stays absent when the manifest does not mention it', () => {
    const [fixture] = load(withEntry({}));

    expect(fixture?.options).toBeUndefined();
    expect(Object.hasOwn(fixture!, 'options')).toBe(false);
  });

  // A misspelled key is the failure this exists for: measured under the
  // production shape while named for the debug one, and the number it reports
  // would look entirely reasonable.
  test('refuses a key it does not know', () => {
    expect(() => load(withEntry({ options: { enableDebugDataProps: true } }))).toThrow(
      /enableDebugDataProps is not a benchmarkable option/
    );
  });

  test.each([
    ['a string', 'true'],
    ['a number', 1],
    ['null', null],
    ['an object', {}],
  ])('refuses a boolean key given %s', (_label, value) => {
    expect(() => load(withEntry({ options: { dev: value } }))).toThrow(
      /options\.dev must be a boolean/
    );
  });

  test('refuses an unaccepted enum value', () => {
    expect(() => load(withEntry({ options: { styleResolution: 'application' } }))).toThrow(
      /must be one of application-order, property-specificity, legacy-expand-shorthands/
    );
    expect(() => load(withEntry({ options: { propertyValidationMode: 'quiet' } }))).toThrow(
      /must be one of throw, warn, silent/
    );
  });

  test.each([
    ['a string', 'dev'],
    ['an array', []],
    ['null', null],
  ])('refuses an options map that is %s', (_label, options) => {
    expect(() => load(withEntry({ options }))).toThrow(/options must be an object/);
  });
});

describe('the manifest as a whole', () => {
  test.each([
    ['a missing schema version', { fixtures: [VALID_ENTRY] }],
    ['the wrong schema version', { schemaVersion: 2, fixtures: [VALID_ENTRY] }],
    ['a schema version as a string', { schemaVersion: '1', fixtures: [VALID_ENTRY] }],
    ['fixtures that are not an array', { schemaVersion: 1, fixtures: {} }],
    ['a top-level array', [VALID_ENTRY]],
    ['a top-level null', null],
  ])('refuses %s', (_label, manifest) => {
    expect(() => load(manifest)).toThrow(/schema version 1/);
  });

  test('refuses an empty registry', () => {
    expect(() => load(withFixtures([]))).toThrow(/non-empty and unique/);
  });

  test('refuses a duplicated name', () => {
    expect(() => load(withFixtures([VALID_ENTRY, { ...VALID_ENTRY }]))).toThrow(
      /non-empty and unique/
    );
  });

  test('refuses a file that escapes the workspace', () => {
    expect(() => load(withEntry({ file: '../../etc/passwd' }))).toThrow(/must be a relative path/);
    expect(() => load(withEntry({ file: path.resolve('/etc/passwd') }))).toThrow(
      /must be a relative path/
    );
  });

  test.each([
    ['an unsupported category', { category: 'debug' }, /category is unsupported/],
    ['an unsupported weight', { weight: 'light' }, /weight is unsupported/],
    ['a fractional batch size', { batchSize: 1.5 }, /batchSize must be a positive integer/],
    ['a zero batch size', { batchSize: 0 }, /batchSize must be a positive integer/],
    ['a negative batch size', { batchSize: -1 }, /batchSize must be a positive integer/],
    ['an empty name', { name: '' }, /name must be a non-empty string/],
  ])('refuses %s', (_label, overrides, message) => {
    expect(() => load(withEntry(overrides))).toThrow(message);
  });

  test('names the offending entry by index', () => {
    expect(() =>
      load(withFixtures([VALID_ENTRY, { ...VALID_ENTRY, name: 'second', dev: 'yes' }]))
    ).toThrow(/entry 1\.dev/);
  });

  test('refuses invalid JSON rather than loading nothing', () => {
    expect(() => load('{ "schemaVersion": 1, ')).toThrow();
  });
});

describe('filtering', () => {
  const manifest = withFixtures([
    { ...VALID_ENTRY, name: 'transform-one' },
    { ...VALID_ENTRY, name: 'perf-one', category: 'perf' },
    { ...VALID_ENTRY, name: 'rollup-one', category: 'rollup', weight: 'heavy' },
  ]);

  test('an empty category list means every category', () => {
    const paths = withManifest(manifest);

    expect(loadAllFixtures({ ...paths, categories: [] })).toHaveLength(3);
  });

  test('a category list narrows to it, and carries the dev override through', () => {
    const paths = withManifest(
      withFixtures([
        { ...VALID_ENTRY, name: 'prod-one', category: 'perf' },
        { ...VALID_ENTRY, name: 'dev-one', category: 'perf', dev: true },
        { ...VALID_ENTRY, name: 'other', category: 'rollup', weight: 'heavy' },
      ])
    );
    const loaded = loadAllFixtures({ ...paths, categories: ['perf'], filter: ['dev-'] });

    expect(loaded.map(fixture => fixture.name)).toEqual(['dev-one']);
    expect(loaded[0]?.dev).toBe(true);
  });

  test('a filter matching nothing returns nothing rather than everything', () => {
    const paths = withManifest(manifest);

    expect(loadAllFixtures({ ...paths, filter: ['no-such-fixture'] })).toEqual([]);
  });
});
