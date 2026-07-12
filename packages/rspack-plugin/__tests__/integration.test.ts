import path from 'path';
import { CssExtractRspackPlugin, rspack } from '@rspack/core';
import { createFsFromVolume, Volume } from 'memfs';
import { describe, expect, test } from 'vitest';

import {
  BASIC_FIXTURE_ENTRY,
  BASIC_FIXTURE_EXPECTED_CSS,
  BASIC_FIXTURE_ROOT,
  CARRIER_ALIAS,
  findStylexCss,
} from '../../plugin-shared/test-utils/fixtures';
import StyleXPlugin, { STYLEX_CHUNK_NAME } from '../src';

import type { Configuration, Stats } from '@rspack/core';

const CARRIER_PATH = path.join(__dirname, '..', 'src', 'stylex.css');

function createConfig(mode: 'development' | 'production'): Configuration {
  return {
    context: BASIC_FIXTURE_ROOT,
    entry: BASIC_FIXTURE_ENTRY,
    mode,
    devtool: false,
    output: {
      path: path.join(BASIC_FIXTURE_ROOT, 'dist'),
      filename: '[name].js',
    },
    optimization: {
      // keep assertions minifier-independent; minification runs after the
      // plugin's PRE_PROCESS asset replacement either way
      minimize: false,
      splitChunks: {
        chunks: 'all',
      },
    },
    resolve: {
      alias: {
        [CARRIER_ALIAS]: CARRIER_PATH,
        // the fixture lives in plugin-shared; resolve its imports from this
        // package's node_modules (pnpm's strict layout hides them otherwise)
        '@stylexjs/stylex': require.resolve('@stylexjs/stylex'),
      },
    },
    module: {
      rules: [
        {
          test: /\.css$/i,
          use: [CssExtractRspackPlugin.loader, require.resolve('css-loader')],
        },
      ],
    },
    plugins: [new StyleXPlugin({ rsOptions: { dev: false } }), new CssExtractRspackPlugin()],
  };
}

type CompileResult = {
  stats: Stats;
  /** output file name -> content, snapshotted from the in-memory output fs */
  files: Record<string, string>;
};

async function compile(config: Configuration): Promise<CompileResult> {
  const volume = new Volume();
  const compiler = rspack(config);

  compiler.outputFileSystem = createFsFromVolume(volume) as never;

  const stats = await new Promise<Stats>((resolve, reject) => {
    compiler.run((error, result) => {
      if (error || !result) {
        return reject(error ?? new Error('No stats returned'));
      }

      resolve(result);
    });
  });

  await new Promise<void>((resolve, reject) => {
    compiler.close(closeError => (closeError ? reject(closeError) : resolve()));
  });

  if (stats.hasErrors()) {
    throw new Error(stats.toString({ errors: true }));
  }

  const files: Record<string, string> = {};

  for (const [filePath, content] of Object.entries(volume.toJSON())) {
    files[path.basename(filePath)] = content ?? '';
  }

  return { stats, files };
}

function getStylexCss({ files }: CompileResult): string | null {
  return findStylexCss(files, STYLEX_CHUNK_NAME);
}

describe('@stylexswc/rspack-plugin integration', () => {
  test('production build replaces the carrier asset with aggregated StyleX CSS', async () => {
    const result = await compile(createConfig('production'));
    const css = getStylexCss(result);

    expect(css).not.toBeNull();

    for (const declaration of BASIC_FIXTURE_EXPECTED_CSS) {
      expect(css).toContain(declaration);
    }

    // carrier placeholder content is replaced, not appended to
    expect(css).not.toContain('StyleX carrier stylesheet');
    // dev-only HMR dummy rules never reach production output
    expect(css).not.toContain('.stylex-hashed-');
  });

  test('development build emits the CSS and content-hashed HMR dummy rules', async () => {
    const result = await compile(createConfig('development'));
    const css = getStylexCss(result);

    expect(css).not.toBeNull();

    for (const declaration of BASIC_FIXTURE_EXPECTED_CSS) {
      expect(css).toContain(declaration);
    }
  });

  test('module identifiers contain no absolute fixture paths', async () => {
    const { stats } = await compile(createConfig('production'));
    const { modules = [] } = stats.toJson({ modules: true, ids: true });

    const dummyModules = modules.filter(mod => mod.identifier?.includes('stylex-virtual.css'));

    expect(dummyModules.length).toBeGreaterThan(0);

    for (const mod of dummyModules) {
      const query = mod.identifier!.split('stylex-virtual.css?')[1] ?? '';
      const from = new URLSearchParams(query.split('|')[0]).get('from');

      expect(from).not.toBeNull();
      expect(path.isAbsolute(from!)).toBe(false);
    }
  });

  test('a custom carrierCss stylesheet receives the extracted CSS', async () => {
    const customCarrier = path.join(BASIC_FIXTURE_ROOT, 'custom-carrier.css');
    const config = createConfig('production');

    (config.resolve!.alias as Record<string, string>)[CARRIER_ALIAS] = customCarrier;
    config.plugins = [
      // relative path: exercises resolution against compiler.context
      new StyleXPlugin({ rsOptions: { dev: false }, carrierCss: 'custom-carrier.css' }),
      new CssExtractRspackPlugin(),
    ];

    const result = await compile(config);
    const css = getStylexCss(result);

    expect(css).not.toBeNull();

    for (const declaration of BASIC_FIXTURE_EXPECTED_CSS) {
      expect(css).toContain(declaration);
    }

    expect(result.stats.toJson({ warnings: true }).warnings ?? []).toHaveLength(0);
  });

  test('a replacement cache group without test emits the named StyleX asset', async () => {
    const config = createConfig('production');

    config.plugins = [
      new StyleXPlugin({
        rsOptions: { dev: false },
        cacheGroup: { type: 'css/mini-extract', chunks: 'all', enforce: true },
      }),
      new CssExtractRspackPlugin(),
    ];

    const result = await compile(config);
    const css = getStylexCss(result);

    expect(css).not.toBeNull();
    expect(result.stats.toJson({ warnings: true }).warnings ?? []).toHaveLength(0);
  });

  test('a second fresh build emits identical CSS', async () => {
    const firstCss = getStylexCss(await compile(createConfig('production')));
    const secondCss = getStylexCss(await compile(createConfig('production')));

    expect(firstCss).not.toBeNull();
    expect(secondCss).toBe(firstCss);
  });
});
