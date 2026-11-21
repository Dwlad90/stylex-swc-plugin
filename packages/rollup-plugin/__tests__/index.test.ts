'use strict';

import path from 'path';

import commonjs from '@rollup/plugin-commonjs';
import { nodeResolve } from '@rollup/plugin-node-resolve';
import * as rollup from 'rollup';
import { afterEach, beforeEach, describe, expect, it, vi } from 'vitest';

import packageJson from '../package.json';
import stylexPlugin from '../src/index';
import type { PluginOptions } from '../src/index';

describe('@stylexswc/rollup-plugin', () => {
  // The plugin gets its CSS targets from browserslist, and browserslist reads
  // the shell environment and config files above the working directory. Set a
  // fixed query, so that the snapshots are the same on each machine.
  beforeEach(() => {
    vi.stubEnv('BROWSERSLIST', 'defaults');
  });

  afterEach(() => {
    vi.unstubAllEnvs();
  });

  async function runStylex(options: PluginOptions, fixture = 'index.js') {
    // Configure a rollup bundle
    const bundle = await rollup.rollup({
      // Remove stylex runtime from bundle
      external: [
        'stylex',
        '@stylexjs/stylex',
        '@stylexjs/stylex/lib/stylex-inject',
        'custom-stylex',
      ],
      input: path.resolve(__dirname, '__fixtures__', fixture),
      plugins: [
        nodeResolve(),
        commonjs(),
        stylexPlugin({
          useCSSLayers: true,
          ...options,
          lightningcssOptions: { minify: false },
        }),
      ],
    });

    try {
      const { output } = await bundle.generate({
        file: path.resolve(__dirname, '/__builds__/bundle.js'),
      });

      let css, js;

      for (const chunkOrAsset of output) {
        const asset = chunkOrAsset as rollup.OutputAsset;
        const chunk = chunkOrAsset as rollup.OutputChunk;

        if (asset.fileName === 'stylex.css') {
          css = asset.source;
        } else if (chunk.fileName === 'bundle.js') {
          js = chunk.code;
        }
      }

      return { css, js, output };
    } finally {
      await bundle.close();
    }
  }

  it('still transforms built-in imports when a custom importSource is set', async () => {
    const { css } = await runStylex({
      fileName: 'stylex.css',
      rsOptions: { importSources: ['custom-stylex'] },
    });

    expect(css).toContain('display: flex;');
  });

  it('transforms a module that imports from a custom importSource', async () => {
    const { css } = await runStylex(
      { fileName: 'stylex.css', rsOptions: { importSources: ['custom-stylex'] } },
      'customSource.js'
    );

    expect(css).toContain('color: red;');
  });

  it('extracts CSS and removes stylex.inject calls', async () => {
    const { css, js } = await runStylex({ fileName: 'stylex.css' });

    expect(css).toMatchSnapshot();

    expect(js).toMatchSnapshot();
  });

  describe('runtimeInjection:true', () => {
    it('preserves stylex.inject calls and does not extract CSS', async () => {
      const { css, js } = await runStylex({
        rsOptions: {
          debug: true,
          runtimeInjection: true,
        },
      });

      expect(css).toBeUndefined();

      expect(js).toMatchSnapshot();
    });
  });
  it('output filename match pattern', async () => {
    const { output } = await runStylex({ fileName: 'stylex.[hash].css' });
    const css = output.find(
      chunkOrAsset =>
        chunkOrAsset.type === 'asset' && /^stylex.[0-9a-f]{8}\.css$/.test(chunkOrAsset.fileName)
    ) as rollup.OutputAsset | undefined;

    expect(css?.source).toMatchSnapshot();
  });

  it('gets the CSS targets from the browserslist config of the project', async () => {
    // Browserslist reads a config file only when the caller gives no query.
    // If the plugin sends a fixed query, this test fails. The fixture selects
    // Safari 8, and Safari 8 must have a prefix for `display: flex`.
    vi.stubEnv('BROWSERSLIST', undefined);
    vi.stubEnv(
      'BROWSERSLIST_CONFIG',
      path.resolve(__dirname, '__fixtures__/safari-8.browserslistrc')
    );

    const { css } = await runStylex({ fileName: 'stylex.css' });

    expect(css).toMatch(/display:\s*-webkit-flex/);
  });

  it('applies the include filter to every module, not only the first', async () => {
    // `index.js` is transformed first and imports `otherStyles.js`. The filter
    // excludes the second file, so its styles must not reach the sheet.
    const rsOptions = { include: [/index\.js$/] };
    const { css } = await runStylex({ fileName: 'stylex.css', rsOptions });

    expect(css).toContain('display: flex;');
    expect(css).not.toContain('width: 100%;');
    expect(rsOptions.include).toEqual([/index\.js$/]);
  });

  it('builds again from the cache when a module gave no rules', async () => {
    // The second build reads every module from the cache. `plain.js` has no
    // StyleX, so its cached meta holds no rules.
    const build = (cache?: rollup.RollupCache) =>
      rollup.rollup({
        cache,
        external: ['stylex'],
        input: path.resolve(__dirname, '__fixtures__', 'withPlainModule.js'),
        plugins: [stylexPlugin({ lightningcssOptions: { minify: false } })],
      });

    const first = await build();
    await first.close();

    const second = await build(first.cache);

    try {
      const { output } = await second.generate({ format: 'esm' });
      const sheet = output.find(file => file.fileName === 'stylex.css');

      expect(sheet?.type === 'asset' && String(sheet.source)).toContain('color: red;');
    } finally {
      await second.close();
    }
  });

  it('declares browserslist as a runtime dependency', () => {
    // `src/index.ts` imports `browserslist` at load time. A package manager does
    // not install a devDependency for a consumer, so
    // `require('@stylexswc/rollup-plugin')` fails.
    expect(packageJson.dependencies).toHaveProperty('browserslist');
  });
});
