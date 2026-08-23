import path from 'node:path';
import { fileURLToPath } from 'node:url';

import { describe, expect, test } from 'vitest';

import { createStylexOptions, fixtureStylexOptions } from '../lib/config.js';
import { loadAllFixtures } from '../lib/fixtures.js';
import { loadSubject } from '../lib/subjects.js';
import { BOOLEAN_OPTION_KEYS } from '../lib/types.js';

const benchmarkDir = path.dirname(fileURLToPath(import.meta.url));
const packageDir = path.resolve(benchmarkDir, '../..');
const workspaceRoot = path.resolve(packageDir, '../..');

describe('loadAllFixtures', () => {
  const fixtures = loadAllFixtures({ packageDir, workspaceRoot });

  // The other half of the allowlist rule. `fixture-manifest.test.ts` proves an
  // unknown key is refused; nothing proved that an accepted key is used, which
  // is how four options nobody had shown this compiler reacting to survived in
  // the list. A key whose fixture is later deleted now fails here instead of
  // sitting there looking measured.
  test('every allowlisted option key is measured by a fixture', () => {
    const used = new Set(fixtures.flatMap(fixture => Object.keys(fixture.options ?? {})));
    const unused = [
      ...BOOLEAN_OPTION_KEYS,
      'styleResolution',
      'sourceMap',
      'classNamePrefix',
    ].filter(key => !used.has(key));

    expect(unused, 'allowlisted but no fixture measures them').toEqual([]);
  });

  test('loads the complete versioned registry', () => {
    expect(fixtures).toHaveLength(61);
    expect(new Set(fixtures.map(fixture => fixture.name)).size).toBe(61);
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
      // The whole declared map at once, rather than key by key: the claim is
      // that the resolved options contain every override the fixture asked for,
      // and asserting it this way needs no narrowing of a key read off the map.
      expect(fixtureStylexOptions(fixture, base), fixture.name).toMatchObject(fixture.options!);
    }
  });

  // The assertion that makes a feature fixture worth having: the option shape it
  // declares has to change what the compiler emits. Seven entries did not when
  // this was written -- `enableMediaQueryOrder`, `legacyDisableLayers`,
  // `propertyValidationMode: throw` and `treeshakeCompensation: false` changed
  // not one byte on any fixture in the corpus, and a `(dev)` twin of a token file
  // emitted the same module as its production run. Each was reported as a
  // measurement of a development feature and was a second measurement of the
  // production shape.
  //
  // Asserted against the real binding rather than against the option object,
  // because the question is what the compiler does with the option, not whether
  // the harness passed it along.
  test('every option override changes what the compiler emits', async () => {
    const { transform } = await import('../../dist/index.js');
    const base = createStylexOptions(packageDir);

    for (const fixture of fixtures) {
      if (fixture.dev === undefined && fixture.options === undefined) continue;

      const production = transform(fixture.filePath, fixture.code, base);
      const shaped = transform(fixture.filePath, fixture.code, fixtureStylexOptions(fixture, base));

      // The map is compared too, because `sourceMap` changes nothing else: the
      // whole of its work lands in a field the emitted module does not carry.
      expect(
        shaped.code === production.code &&
          JSON.stringify(shaped.metadata) === JSON.stringify(production.metadata) &&
          JSON.stringify(shaped.map) === JSON.stringify(production.map),
        `${fixture.name} emits exactly what its production run emits, so its ` +
          `options price nothing`
      ).toBe(false);
    }
  });

  // Per *key*, not per entry. The test above passes as soon as one key in a map
  // moves the output, which let an entry named for a chained input source map
  // price `dev: true` while carrying a map that made no difference at all --
  // nor did a garbage one.
  //
  // A boolean key is *flipped* rather than dropped, and that distinction is the
  // whole of what this asks. Dropping it would reject a key that restates a
  // default -- `enableDebugDataProp` is already on under `debug`, and
  // `useRealFileForSource` under `dev` -- and an entry is entitled to name the
  // option it exists to measure rather than leave a reader to know the defaults.
  // What must not happen is an option the compiler does not react to: flipping
  // it says exactly that, and it is what keeps the four dead keys and the inert
  // input source map out. Keys that only matter together still pass, since
  // flipping one leaves the rest in place.
  //
  // A key whose value is not a boolean has no flip, so it is dropped instead.
  test('every option key in a map changes what the compiler emits', async () => {
    const { transform } = await import('../../dist/index.js');
    const base = createStylexOptions(packageDir);

    for (const fixture of fixtures) {
      const declared = fixture.options;
      if (declared === undefined) continue;

      const asDeclared = fixtureStylexOptions(fixture, base);
      const full = transform(fixture.filePath, fixture.code, asDeclared);

      for (const [key, value] of Object.entries(declared)) {
        const varied =
          typeof value === 'boolean'
            ? { ...asDeclared, [key]: !value }
            : Object.fromEntries(
                Object.entries(asDeclared).filter(([candidate]) => candidate !== key)
              );
        const other = transform(fixture.filePath, fixture.code, varied);

        expect(
          full.code === other.code &&
            JSON.stringify(full.metadata) === JSON.stringify(other.metadata) &&
            JSON.stringify(full.map) === JSON.stringify(other.map),
          `${fixture.name} emits the same thing whichever way ${key} is set`
        ).toBe(false);
      }
    }
  });

  // The data prop is attached where styles are *read*, not where they are
  // defined, so a fixture that only calls `create` cannot measure it however
  // many debug options it names. That is how the first version of this corpus
  // came to have two entries named for the data prop and no `stylex.props` call
  // anywhere in it.
  test('the data prop fixtures emit the data prop', async () => {
    const { transform } = await import('../../dist/index.js');
    const base = createStylexOptions(packageDir);
    const named = fixtures.filter(
      fixture =>
        fixture.name.includes('data prop') && fixture.options?.enableDebugDataProp !== false
    );

    expect(named.length).toBeGreaterThan(0);

    for (const fixture of named) {
      const shaped = transform(fixture.filePath, fixture.code, fixtureStylexOptions(fixture, base));
      const emitted = shaped.code
        .split('\n')
        .filter(line => line.includes('data-style-src') && !line.trimStart().startsWith('*'));

      expect(emitted.length, fixture.name).toBeGreaterThan(0);
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
