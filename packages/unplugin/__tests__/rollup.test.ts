'use strict';

import path from 'path';

import commonjs from '@rollup/plugin-commonjs';
import { nodeResolve } from '@rollup/plugin-node-resolve';
import * as rollup from 'rollup';
import { describe, expect, test } from 'vitest';

import type { UnpluginStylexRSOptions } from '../src/index.js';
import stylexPlugin from '../src/rollup.js';

describe('@stylexswc/unplugin/rollup', () => {
  async function runStylex(
    options: UnpluginStylexRSOptions,
    extraPlugins: rollup.Plugin[] = [],
    warnings: rollup.RollupLog[] = []
  ) {
    // Configure a rollup bundle
    const bundle = await rollup.rollup({
      // Remove stylex runtime from bundle
      external: ['stylex', '@stylexjs/stylex', '@stylexjs/stylex/lib/stylex-inject'],
      onwarn: warning => warnings.push(warning),
      input: path.resolve(__dirname, '__fixtures__/index.js'),
      plugins: [
        nodeResolve(),
        commonjs(),
        stylexPlugin({
          useCSSLayers: true,
          ...options,
        }),
        ...extraPlugins,
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

  // Rollup has no CSS pipeline of its own, so the stylesheet carrying the
  // marker is emitted the way a CSS plugin would emit it.
  const placeholder = '/* @stylex-placeholder */';

  function emitPlaceholderStylesheet(): rollup.Plugin {
    return {
      name: 'emit-placeholder-stylesheet',
      buildEnd() {
        this.emitFile({
          type: 'asset',
          fileName: 'styles.css',
          source: `body{margin:0}\n${placeholder}\n`,
        });
      },
    };
  }

  test('replaces the placeholder marker in an emitted stylesheet', async () => {
    const { output } = await runStylex({ useCssPlaceholder: placeholder }, [
      emitPlaceholderStylesheet(),
    ]);

    const stylesheet = output.find(
      chunkOrAsset => chunkOrAsset.type === 'asset' && chunkOrAsset.fileName === 'styles.css'
    ) as rollup.OutputAsset | undefined;
    const cssFileNames = output
      .filter(chunkOrAsset => chunkOrAsset.fileName.endsWith('.css'))
      .map(chunkOrAsset => chunkOrAsset.fileName);

    expect(stylesheet?.source).toContain('body{margin:0}');
    expect(stylesheet?.source).toContain('color');
    expect(stylesheet?.source).not.toContain(placeholder);
    // Placeholder mode never links a standalone stylesheet, so there must not
    // be a second one.
    expect(cssFileNames).toEqual(['styles.css']);
  });

  test('warns instead of emitting a stylesheet nothing links', async () => {
    const warnings: rollup.RollupLog[] = [];
    // No CSS plugin at all, so nothing in the bundle can carry the marker.
    const { output } = await runStylex({ useCssPlaceholder: placeholder }, [], warnings);

    expect(output.filter(chunkOrAsset => chunkOrAsset.fileName.endsWith('.css'))).toEqual([]);
    expect(warnings.map(warning => warning.message ?? '')).toContainEqual(
      expect.stringContaining('no CSS asset contained the placeholder')
    );
  });

  test('extracts CSS and removes stylex.inject calls', async () => {
    const { css, js } = await runStylex({ fileName: 'stylex.css' });

    expect(css).toMatchSnapshot();

    expect(js).toMatchSnapshot();
  });

  describe('runtimeInjection:true', () => {
    test('preserves stylex.inject calls and does not extract CSS', async () => {
      const { css, js } = await runStylex({
        rsOptions: {
          debug: true,
          runtimeInjection: true,
          enableDebugClassNames: true,
        },
      });

      expect(css).toMatchSnapshot();

      expect(js).toMatchSnapshot();
    });
  });
  test('output filename match pattern', async () => {
    const { output } = await runStylex({ fileName: 'stylex.[hash].css' });
    const css = output.find(
      chunkOrAsset =>
        chunkOrAsset.type === 'asset' && /^stylex.[0-9a-f]{8}\.css$/.test(chunkOrAsset.fileName)
    ) as rollup.OutputAsset | undefined;

    expect(css?.source).toMatchSnapshot();
  });

  test('transforms extracted CSS before emit', async () => {
    const seenFilePaths: Array<string | undefined> = [];
    const { css } = await runStylex({
      fileName: 'stylex.css',
      async transformCss(css, filePath) {
        seenFilePaths.push(filePath);

        return `${css}\n/* transformed:${filePath} */`;
      },
    });

    expect(seenFilePaths).toEqual(['stylex.css']);
    expect(css).toContain('color');
    expect(css).toContain('/* transformed:stylex.css */');
  });
});
