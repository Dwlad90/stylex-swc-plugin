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

  async function runStylex(options: PluginOptions) {
    // Configure a rollup bundle
    const bundle = await rollup.rollup({
      // Remove stylex runtime from bundle
      external: ['stylex', '@stylexjs/stylex', '@stylexjs/stylex/lib/stylex-inject'],
      input: path.resolve(__dirname, '__fixtures__/index.js'),
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

  it('declares browserslist as a runtime dependency', () => {
    // `src/index.ts` imports `browserslist` at load time. A package manager does
    // not install a devDependency for a consumer, so
    // `require('@stylexswc/rollup-plugin')` fails.
    expect(packageJson.dependencies).toHaveProperty('browserslist');
  });
});
