import path from 'path';
import { describe, expect, test } from 'vitest';

import { StyleXPluginCore, VIRTUAL_ENTRYPOINT_CSS_PATTERN } from '../src';

import type { CacheGroupOptions, StyleXPluginOption } from '../src';

const DEFAULT_CHUNK_NAME = '_stylex-test-generated';
const PACKAGE_NAME = '@stylexswc/plugin-shared';
const CONTEXT = path.join(path.sep, 'project');
const CARRIER_PATH = path.join(path.sep, 'repo', 'plugin-shared', 'dist', 'stylex.css');

type SplitChunksOptimization = {
  splitChunks?: false | { cacheGroups?: Record<string, CacheGroupOptions> };
};

function createCore(options?: StyleXPluginOption) {
  const core = new StyleXPluginCore(options);
  core.resolveCarrier(CONTEXT, CARRIER_PATH);
  return core;
}

function installCacheGroup(core: StyleXPluginCore, optimization: SplitChunksOptimization) {
  const chunkName = core.getChunkName(DEFAULT_CHUNK_NAME);
  core.assertAndInstallCacheGroup(optimization, PACKAGE_NAME, chunkName);
  return optimization.splitChunks !== false ? optimization.splitChunks?.cacheGroups : undefined;
}

describe('StyleXPluginCore.assertAndInstallCacheGroup', () => {
  test('throws when optimization.splitChunks is false', () => {
    const core = createCore();

    expect(() => installCacheGroup(core, { splitChunks: false })).toThrow(
      /optimization\.splitChunks.*should be enabled/
    );
  });

  test('throws when optimization.splitChunks is missing', () => {
    const core = createCore();

    expect(() => installCacheGroup(core, {})).toThrow(/optimization\.splitChunks/);
  });

  test('creates the cacheGroups record when splitChunks has none', () => {
    const core = createCore();
    const optimization: SplitChunksOptimization = { splitChunks: {} };

    const cacheGroups = installCacheGroup(core, optimization);

    expect(cacheGroups?.[DEFAULT_CHUNK_NAME]).toBeDefined();
  });

  test('installs the full default cache group when no cacheGroup option is given', () => {
    const core = createCore();

    const cacheGroups = installCacheGroup(core, { splitChunks: { cacheGroups: {} } });
    const installed = cacheGroups?.[DEFAULT_CHUNK_NAME];

    expect(installed).toMatchObject({
      name: DEFAULT_CHUNK_NAME,
      type: 'css/mini-extract',
      chunks: 'all',
      enforce: true,
    });

    const pattern = (installed as { test: RegExp }).test;
    expect(pattern).toBeInstanceOf(RegExp);
    expect(pattern.test(CARRIER_PATH)).toBe(true);
    expect(pattern.test('/repo/plugin-shared/dist/stylex-virtual.css?from=App.js')).toBe(true);
    expect(pattern.test(path.join(CONTEXT, 'src', 'other.css'))).toBe(false);
  });

  test('preserves sibling cache groups', () => {
    const core = createCore();
    const siblings: Record<string, CacheGroupOptions> = {
      framework: { name: 'framework', priority: 40 },
      htz: { priority: 50 },
    };

    const cacheGroups = installCacheGroup(core, { splitChunks: { cacheGroups: siblings } });

    expect(cacheGroups?.framework).toEqual({ name: 'framework', priority: 40 });
    expect(cacheGroups?.htz).toEqual({ priority: 50 });
    expect(cacheGroups?.[DEFAULT_CHUNK_NAME]).toBeDefined();
  });

  test('a custom cache group replaces the default entirely — no default test may leak in', () => {
    // Omitting `test` deliberately widens the group to every module of the
    // given type: this is how consumers funnel ALL extracted CSS into a
    // single stylex chunk. Merging the default `test` back in silently
    // narrowed the group to the carrier again (0.18.0-rc.1 regression).
    const core = createCore({
      cacheGroup: {
        name: 'stylex-all-css',
        type: 'css/mini-extract',
        chunks: 'all',
        enforce: true,
      },
    });

    const cacheGroups = installCacheGroup(core, { splitChunks: { cacheGroups: {} } });

    expect(cacheGroups?.['stylex-all-css']).toEqual({
      name: 'stylex-all-css',
      type: 'css/mini-extract',
      chunks: 'all',
      enforce: true,
    });
    expect(cacheGroups?.[DEFAULT_CHUNK_NAME]).toBeUndefined();
  });

  test('an explicit custom test is preserved verbatim', () => {
    const customTest = /\.special\.css$/;
    const core = createCore({
      cacheGroup: { name: 'special', test: customTest },
    });

    const cacheGroups = installCacheGroup(core, { splitChunks: { cacheGroups: {} } });

    expect((cacheGroups?.special as { test: RegExp }).test).toBe(customTest);
  });

  test('a custom cache group without a name falls back to the default chunk name', () => {
    const core = createCore({ cacheGroup: { priority: 20 } });

    const cacheGroups = installCacheGroup(core, { splitChunks: { cacheGroups: {} } });

    expect(cacheGroups?.[DEFAULT_CHUNK_NAME]).toEqual({
      name: DEFAULT_CHUNK_NAME,
      priority: 20,
    });
  });

  test('an explicit `name: undefined` still gets the default chunk name pinned', () => {
    // `{ name: chunkName, ...cacheGroup }` would let the user's explicit
    // undefined win the spread, un-naming the chunk and orphaning the
    // finalizeStylexAsset lookup — every extracted style would be dropped.
    const core = createCore({
      cacheGroup: { name: undefined, priority: 20 } as unknown as CacheGroupOptions,
    });

    const cacheGroups = installCacheGroup(core, { splitChunks: { cacheGroups: {} } });

    expect(cacheGroups?.[DEFAULT_CHUNK_NAME]).toEqual({
      name: DEFAULT_CHUNK_NAME,
      priority: 20,
    });
  });

  test('rejects a dynamic name because the emitted chunk cannot be located', () => {
    const nameFn = () => 'computed';
    const core = createCore({
      cacheGroup: { name: nameFn, priority: 20 } as unknown as CacheGroupOptions,
    });

    expect(() => installCacheGroup(core, { splitChunks: { cacheGroups: {} } })).toThrow(
      /cacheGroup\.name.*static string/
    );
  });

  test('a RegExp shorthand is normalized as test with a static chunk name', () => {
    const shorthand = /\.css$/;
    const core = createCore({ cacheGroup: shorthand });

    const cacheGroups = installCacheGroup(core, { splitChunks: { cacheGroups: {} } });

    expect(cacheGroups?.[DEFAULT_CHUNK_NAME]).toEqual({
      name: DEFAULT_CHUNK_NAME,
      test: shorthand,
    });
  });

  test('a string shorthand is normalized as test with a static chunk name', () => {
    const core = createCore({ cacheGroup: 'some-module-path' });

    const cacheGroups = installCacheGroup(core, { splitChunks: { cacheGroups: {} } });

    expect(cacheGroups?.[DEFAULT_CHUNK_NAME]).toEqual({
      name: DEFAULT_CHUNK_NAME,
      test: 'some-module-path',
    });
  });

  test('`false` passes through verbatim and disables the group', () => {
    const core = createCore({ cacheGroup: false });

    const cacheGroups = installCacheGroup(core, { splitChunks: { cacheGroups: {} } });

    expect(cacheGroups?.[DEFAULT_CHUNK_NAME]).toBe(false);
  });

  test('overwrites a pre-existing group under the same key', () => {
    const core = createCore();
    const optimization: SplitChunksOptimization = {
      splitChunks: {
        cacheGroups: { [DEFAULT_CHUNK_NAME]: { priority: 1 } },
      },
    };

    const cacheGroups = installCacheGroup(core, optimization);

    expect(cacheGroups?.[DEFAULT_CHUNK_NAME]).toMatchObject({
      name: DEFAULT_CHUNK_NAME,
      type: 'css/mini-extract',
    });
  });

  test('rejects a dynamic cache group function', () => {
    const core = createCore({
      cacheGroup: (() => ({ priority: 20 })) as unknown as CacheGroupOptions,
    });

    expect(() => installCacheGroup(core, { splitChunks: { cacheGroups: {} } })).toThrow(
      /cacheGroup.*must be an object/
    );
  });
});

describe('StyleXPluginCore.getChunkName', () => {
  test('returns the default without a cacheGroup option', () => {
    const core = createCore();

    expect(core.getChunkName(DEFAULT_CHUNK_NAME)).toBe(DEFAULT_CHUNK_NAME);
  });

  test('returns a custom string name', () => {
    const core = createCore({ cacheGroup: { name: 'custom-stylex' } });

    expect(core.getChunkName(DEFAULT_CHUNK_NAME)).toBe('custom-stylex');
  });

  test('returns the default when the cacheGroup has no name', () => {
    const core = createCore({ cacheGroup: { priority: 20 } });

    expect(core.getChunkName(DEFAULT_CHUNK_NAME)).toBe(DEFAULT_CHUNK_NAME);
  });

  test('returns the default for a non-string name', () => {
    const core = createCore({
      cacheGroup: { name: () => 'computed' } as unknown as CacheGroupOptions,
    });

    expect(core.getChunkName(DEFAULT_CHUNK_NAME)).toBe(DEFAULT_CHUNK_NAME);
  });

  test('returns the default for shorthand cacheGroups', () => {
    expect(createCore({ cacheGroup: /\.css$/ }).getChunkName(DEFAULT_CHUNK_NAME)).toBe(
      DEFAULT_CHUNK_NAME
    );
    expect(createCore({ cacheGroup: false }).getChunkName(DEFAULT_CHUNK_NAME)).toBe(
      DEFAULT_CHUNK_NAME
    );
  });
});

describe('StyleXPluginCore carrier patterns', () => {
  test('getCarrierPattern matches only the resolved carrier', () => {
    const core = createCore();
    const pattern = core.getCarrierPattern();

    expect(pattern.test(CARRIER_PATH)).toBe(true);
    expect(pattern.test(path.join(CONTEXT, 'src', 'stylex.css'))).toBe(false);
  });

  test('a relative carrierCss resolves against the compiler context', () => {
    const core = new StyleXPluginCore({ carrierCss: path.join('src', 'my-carrier.css') });
    core.resolveCarrier(CONTEXT, CARRIER_PATH);

    const pattern = core.getCarrierPattern();

    expect(pattern.test(path.join(CONTEXT, 'src', 'my-carrier.css'))).toBe(true);
    expect(pattern.test(CARRIER_PATH)).toBe(false);
  });

  test('getCarrierPattern falls back to the packaged pattern before resolveCarrier', () => {
    const core = new StyleXPluginCore();

    expect(core.getCarrierPattern()).toBe(VIRTUAL_ENTRYPOINT_CSS_PATTERN);
  });
});
