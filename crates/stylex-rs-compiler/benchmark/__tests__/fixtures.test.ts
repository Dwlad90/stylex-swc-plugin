import path from 'node:path';
import { fileURLToPath } from 'node:url';

import { describe, expect, test } from 'vitest';

import { createStylexOptions, fixtureStylexOptions } from '../lib/config.js';
import { loadAllFixtures } from '../lib/fixtures.js';
import { loadSubject } from '../lib/subjects.js';

const benchmarkDir = path.dirname(fileURLToPath(import.meta.url));
const packageDir = path.resolve(benchmarkDir, '../..');
const workspaceRoot = path.resolve(packageDir, '../..');

describe('loadAllFixtures', () => {
  const fixtures = loadAllFixtures({ packageDir, workspaceRoot });

  test('loads the complete versioned registry', () => {
    expect(fixtures).toHaveLength(25);
    expect(new Set(fixtures.map(fixture => fixture.name)).size).toBe(25);
  });

  // The runner refuses to time a subject that produces no rules, so a
  // registered zero-rule fixture fails every benchmark job in CI rather
  // than here. Keep this assertion against the real binding, and under the
  // options each fixture is actually measured with -- a `dev` fixture that
  // only produces rules in production would pass a check run the other way.
  test('every registered fixture produces at least one StyleX rule', async () => {
    const subject = await loadSubject({ label: 'current', packageDir });
    const stylexOptions = createStylexOptions(packageDir);

    for (const fixture of fixtures) {
      expect(
        subject.run(fixture, fixtureStylexOptions(fixture, stylexOptions)),
        fixture.name
      ).toBeGreaterThan(0);
    }
  });

  // `dev` is the only per-fixture option override, and the reason it exists
  // is that the shared options must stay production-shaped -- see
  // `createStylexOptions`. A fixture silently losing its `dev: true` would
  // leave the debug path measured by nothing at all in this harness.
  test('carries a declared dev override and leaves every other fixture alone', () => {
    const dev = fixtures.filter(fixture => fixture.dev === true);

    expect(dev.map(fixture => fixture.name)).toEqual([
      'Debug data - lotsOfStyles.js (100 creates, dev)',
    ]);

    for (const fixture of fixtures) {
      if (!dev.includes(fixture)) {
        expect(fixture.dev, fixture.name).toBeUndefined();
      }
    }
  });

  test('resolves the options a fixture is measured under from its override', () => {
    const base = createStylexOptions(packageDir);
    const devFixture = fixtures.find(fixture => fixture.dev === true);
    const prodFixture = fixtures.find(fixture => fixture.dev === undefined);

    expect(fixtureStylexOptions(devFixture!, base).dev).toBe(true);
    expect(fixtureStylexOptions(prodFixture!, base).dev).toBe(false);

    // The base object is shared across every fixture in a run, so a merge
    // that mutated it would put one fixture's shape on all the later ones.
    expect(base.dev).toBe(false);
  });

  // The point of the `dev` fixture is the `file:line` annotation on `$$css`,
  // and a rule count cannot see it: both configurations produce the same
  // rules. Asserted against the real binding, because a default flipping
  // under the transform would leave a fixture named `dev` measuring a
  // production build.
  test('a dev fixture emits the file:line annotations it exists to measure', async () => {
    const { transform } = await import('../../dist/index.js');
    const devFixture = fixtures.find(fixture => fixture.dev === true);
    const stylexOptions = createStylexOptions(packageDir);
    const annotation = /\$\$css: *"[^"]+:\d+"/;

    const dev = transform(devFixture!.filePath, devFixture!.code, {
      ...stylexOptions,
      dev: true,
    });
    const prod = transform(devFixture!.filePath, devFixture!.code, {
      ...stylexOptions,
      dev: false,
    });

    expect(dev.code).toMatch(annotation);
    expect(prod.code).not.toMatch(annotation);
  });

  test('produces at least one fixture of each expected group', () => {
    expect(fixtures.some(fixture => fixture.name.startsWith('Performance -'))).toBe(true);
    expect(fixtures.some(fixture => fixture.name.startsWith('Rollup plugin -'))).toBe(true);
    expect(fixtures.some(fixture => !fixture.name.includes(' - '))).toBe(true);
  });

  test('assigns heavy weight only to rollup fixtures', () => {
    for (const fixture of fixtures) {
      if (fixture.weight === 'heavy') {
        expect(fixture.name.startsWith('Rollup plugin -')).toBe(true);
      }
    }
  });

  test('every fixture has non-empty code and batchSize >= 1', () => {
    for (const fixture of fixtures) {
      expect(fixture.code.length).toBeGreaterThan(0);
      expect(fixture.batchSize).toBeGreaterThanOrEqual(1);
    }
  });

  test('filter narrows the returned set', () => {
    const filtered = loadAllFixtures({
      packageDir,
      workspaceRoot,
      filter: ['Rollup plugin'],
    });
    expect(filtered.length).toBeGreaterThan(0);
    expect(filtered.every(fixture => fixture.name.startsWith('Rollup plugin'))).toBe(true);
  });
});
