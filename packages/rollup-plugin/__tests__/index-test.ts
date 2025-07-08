'use strict';

import path from 'path';
import rollup from 'rollup';
import commonjs from '@rollup/plugin-commonjs';
import { nodeResolve } from '@rollup/plugin-node-resolve';
import stylexPlugin from '../src/index';
import type { PluginOptions } from '../src/index';

describe('@stylexswc/rollup-plugin', () => {
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

    // Generate output specific code in-memory
    // You can call this function multiple times on the same bundle object
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
});
