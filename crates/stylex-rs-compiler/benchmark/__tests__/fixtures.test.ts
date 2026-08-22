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
    expect(fixtures).toHaveLength(55);
    expect(new Set(fixtures.map(fixture => fixture.name)).size).toBe(55);
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

  // The shared options must stay production-shaped -- see `createStylexOptions`
  // -- so a fixture that measures anything else says so itself. The legacy
  // `dev` field is one fixture's, and every later one asks through `options`;
  // a fixture silently losing either would leave a development feature measured
  // by nothing at all in this harness.
  test('carries the one legacy dev override and leaves every other fixture alone', () => {
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

  // Every feature fixture is registered twice, production and development, and
  // the pair is the measurement: one number alone says nothing about what the
  // feature costs. A `(dev)` entry that lost its override would quietly become a
  // second production run reported under a development name.
  test('every fixture named for the dev shape is measured under it', () => {
    const named = fixtures.filter(fixture => fixture.name.endsWith('(dev)'));
    const base = createStylexOptions(packageDir);

    expect(named.length).toBeGreaterThan(0);

    for (const fixture of named) {
      expect(fixtureStylexOptions(fixture, base).dev, fixture.name).toBe(true);

      const production = fixtures.find(
        candidate => candidate.name === fixture.name.replace(' (dev)', '')
      );
      expect(production, `${fixture.name} has no production twin`).toBeDefined();
      expect(production?.filePath).toBe(fixture.filePath);
      expect(fixtureStylexOptions(production!, base).dev).toBe(false);
    }
  });

  // Each override reaches the options the fixture is timed under. A key the
  // loader accepted but the merge dropped would leave the fixture measuring the
  // shared shape under a name that claims otherwise.
  test('every declared option override reaches the options a fixture runs with', () => {
    const base = createStylexOptions(packageDir);
    const overridden = fixtures.filter(fixture => fixture.options !== undefined);

    expect(overridden.length).toBeGreaterThan(0);

    for (const fixture of overridden) {
      const resolved = fixtureStylexOptions(fixture, base);

      for (const [key, value] of Object.entries(fixture.options!)) {
        expect(resolved[key as keyof typeof resolved], `${fixture.name}.${key}`).toBe(value);
      }
    }
  });

  // The features are what these fixtures exist for, so each one is registered
  // once. A second fixture measuring the same option shape on the same file
  // would report two numbers for one thing and hide a third nobody covered.
  test('no two fixtures measure the same file under the same options', () => {
    const seen = new Map<string, string>();

    for (const fixture of fixtures) {
      const shape = `${fixture.filePath}::${JSON.stringify({
        dev: fixture.dev,
        options: fixture.options,
      })}`;
      const previous = seen.get(shape);

      expect(previous, `${fixture.name} duplicates ${String(previous)}`).toBeUndefined();
      seen.set(shape, fixture.name);
    }
  });

  // The `dev` fixture is a slice of `apps/rollup-large-example/lotsOfStyles.js`
  // shared with `crates/stylex-transform/benches/transform_debug_bench.rs`, and
  // its name is part of a trend series. Re-cutting it at a different size would
  // silently reshape that series under an unchanged name, so the size the name
  // claims is pinned here.
  test('the dev fixture still holds the number of creates its name claims', () => {
    const fixture = fixtures.find(candidate => candidate.dev === true);
    const creates = fixture?.code.match(/stylex\.create\(/g)?.length;

    expect(fixture?.name).toContain('100 creates');
    expect(creates).toBe(100);
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
    expect(fixtures.some(fixture => fixture.name.startsWith('Feature -'))).toBe(true);
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
