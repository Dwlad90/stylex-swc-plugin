import path from 'path';
import webpack from 'webpack';
import { describe, expect, test, vi } from 'vitest';

import { BUILD_INFO_STYLEX_KEY } from '@stylexswc/plugin-shared';
import StyleXPlugin, { STYLEX_CHUNK_NAME } from '../src';

import type { Rule as StyleXRule } from '@stylexjs/babel-plugin';

type FinishModulesCallback = (_modules: Iterable<{ buildInfo?: Record<string, unknown> }>) => void;
type ProcessAssetsCallback = (_assets: Record<string, webpack.sources.Source>) => Promise<void>;

function createMockCompiler() {
  const { RawSource, ConcatSource } = webpack.sources;
  const assets: Record<string, webpack.sources.Source> = {
    'stylex.css': new RawSource('/* carrier placeholder */'),
  };

  let finishModulesCallback: FinishModulesCallback | undefined;
  let processAssetsCallback: ProcessAssetsCallback | undefined;

  const warnings: Error[] = [];
  const compilation = {
    warnings,
    hooks: {
      chunkHash: {
        tap: vi.fn(),
      },
      finishModules: {
        tap: vi.fn((_name: string, callback: FinishModulesCallback) => {
          finishModulesCallback = callback;
        }),
      },
      processAssets: {
        tapPromise: vi.fn((_options: unknown, callback: ProcessAssetsCallback) => {
          processAssetsCallback = callback;
        }),
      },
    },
    namedChunks: new Map([
      [
        STYLEX_CHUNK_NAME,
        {
          files: new Set(['stylex.css']),
        },
      ],
    ]),
    updateAsset: vi.fn(
      (
        assetName: string,
        update: (_source: webpack.sources.Source) => webpack.sources.Source,
        _info?: unknown
      ) => {
        const source = assets[assetName];

        if (!source) {
          throw new Error(`Missing asset: ${assetName}`);
        }

        assets[assetName] = update(source);
      }
    ),
  };

  const compiler = {
    name: undefined as string | undefined,
    context: path.join(path.sep, 'project'),
    options: {
      mode: 'production',
      optimization: {
        splitChunks: {
          cacheGroups: {} as Record<string, { test?: RegExp }>,
        },
      },
    },
    webpack: {
      Compilation: {
        PROCESS_ASSETS_STAGE_PRE_PROCESS: 0,
      },
      NormalModule: {
        getCompilationHooks: () => ({
          loader: {
            tap: vi.fn(),
          },
        }),
      },
      sources: {
        RawSource,
        ConcatSource,
      },
      WebpackError: Error,
    },
    hooks: {
      normalModuleFactory: {
        tap: vi.fn(),
      },
      make: {
        tap: vi.fn((_name: string, callback: (_compilation: unknown) => void) =>
          callback(compilation)
        ),
      },
    },
  };

  return {
    compiler,
    compilation,
    assets,
    warnings,
    runFinishModules: (modules: Iterable<{ buildInfo?: Record<string, unknown> }>) =>
      finishModulesCallback?.(modules),
    runProcessAssets: () => processAssetsCallback?.(assets),
  };
}

describe('@stylexswc/webpack-plugin', () => {
  test('collects rules from module buildInfo and replaces the carrier CSS asset', async () => {
    const transformCss = vi.fn(async (css: string, filePath: string | undefined) => {
      return `${css}\n/* transformed:${filePath} */`;
    });
    const plugin = new StyleXPlugin({ transformCss });
    const stylexRules: StyleXRule[] = [['x1abcd', { ltr: '.x1abcd{color:red}', rtl: null }, 3000]];

    const { compiler, assets, runFinishModules, runProcessAssets } = createMockCompiler();

    plugin.apply(compiler as unknown as webpack.Compiler);

    runFinishModules([
      { buildInfo: {} },
      {
        buildInfo: {
          [BUILD_INFO_STYLEX_KEY]: {
            resourcePath: 'src/button.tsx',
            stylexRules,
          },
        },
      },
    ]);
    await runProcessAssets();

    expect(transformCss).toHaveBeenCalledTimes(1);
    expect(transformCss.mock.calls[0]?.[1]).toBe('stylex.css');

    const finalCss = assets['stylex.css']?.source().toString();

    // the carrier content is REPLACED, not appended to
    expect(finalCss).not.toContain('/* carrier placeholder */');
    expect(finalCss).toContain('color:red');
    expect(finalCss).toContain('/* transformed:stylex.css */');
  });

  test('leaves the carrier asset untouched without collected rules', async () => {
    const transformCss = vi.fn((css: string) => css);
    const plugin = new StyleXPlugin({ transformCss });

    const { compiler, assets, runFinishModules, runProcessAssets } = createMockCompiler();

    plugin.apply(compiler as unknown as webpack.Compiler);

    runFinishModules([{ buildInfo: {} }]);
    await runProcessAssets();

    expect(transformCss).not.toHaveBeenCalled();
    expect(assets['stylex.css']?.source().toString()).toBe('/* carrier placeholder */');
  });

  test('warns when rules were extracted but no stylex chunk exists', async () => {
    const plugin = new StyleXPlugin();
    const stylexRules: StyleXRule[] = [['x1abcd', { ltr: '.x1abcd{color:red}', rtl: null }, 3000]];

    const { compiler, compilation, warnings, runFinishModules, runProcessAssets } =
      createMockCompiler();

    plugin.apply(compiler as unknown as webpack.Compiler);

    runFinishModules([
      {
        buildInfo: {
          [BUILD_INFO_STYLEX_KEY]: { resourcePath: 'src/button.tsx', stylexRules },
        },
      },
    ]);
    compilation.namedChunks.clear();
    await runProcessAssets();

    expect(warnings).toHaveLength(1);
    expect(warnings[0]?.message).toContain('MISSING from the output');
    expect(warnings[0]?.message).toContain("import '@stylexswc/webpack-plugin/stylex.css'");
  });

  test('in Next.js mode only the client compiler warns about a missing carrier', async () => {
    const stylexRules: StyleXRule[] = [['x1abcd', { ltr: '.x1abcd{color:red}', rtl: null }, 3000]];
    const buildInfoModules = [
      {
        buildInfo: {
          [BUILD_INFO_STYLEX_KEY]: { resourcePath: 'src/button.tsx', stylexRules },
        },
      },
    ];

    // server compiler: collecting rules without a chunk is its normal state
    const server = createMockCompiler();
    server.compiler.name = 'server';
    const serverPlugin = new StyleXPlugin({ nextjsMode: true });

    serverPlugin.apply(server.compiler as unknown as webpack.Compiler);
    server.runFinishModules(buildInfoModules);
    server.compilation.namedChunks.clear();
    await server.runProcessAssets();
    expect(server.warnings).toHaveLength(0);

    // client compiler: it owns the emitted CSS, so the warning stands
    const client = createMockCompiler();
    client.compiler.name = 'client';
    const clientPlugin = new StyleXPlugin({ nextjsMode: true });

    clientPlugin.apply(client.compiler as unknown as webpack.Compiler);
    client.runFinishModules(buildInfoModules);
    client.compilation.namedChunks.clear();
    await client.runProcessAssets();
    expect(client.warnings).toHaveLength(1);
  });

  test('does not warn without extracted rules or when extractCSS is disabled', async () => {
    const noRules = createMockCompiler();
    const plugin = new StyleXPlugin();

    plugin.apply(noRules.compiler as unknown as webpack.Compiler);
    noRules.compilation.namedChunks.clear();
    await noRules.runProcessAssets();
    expect(noRules.warnings).toHaveLength(0);

    const disabled = createMockCompiler();
    const disabledPlugin = new StyleXPlugin({ extractCSS: false });
    disabledPlugin.stylexRules.set('src/button.tsx', [
      ['x1abcd', { ltr: '.x1abcd{color:red}', rtl: null }, 3000],
    ]);

    disabledPlugin.apply(disabled.compiler as unknown as webpack.Compiler);
    disabled.compilation.namedChunks.clear();
    await disabled.runProcessAssets();
    expect(disabled.warnings).toHaveLength(0);
  });

  test('registers the chunkHash and finishModules hooks', () => {
    const plugin = new StyleXPlugin();
    const { compiler, compilation } = createMockCompiler();

    plugin.apply(compiler as unknown as webpack.Compiler);

    expect(compilation.hooks.chunkHash.tap).toHaveBeenCalledTimes(1);
    expect(compilation.hooks.finishModules.tap).toHaveBeenCalledTimes(1);
  });

  test('drops rules that are absent from a later compilation', () => {
    const plugin = new StyleXPlugin();
    const stylexRules: StyleXRule[] = [['x1abcd', { ltr: '.x1abcd{color:red}', rtl: null }, 3000]];
    const { compiler, runFinishModules } = createMockCompiler();

    plugin.apply(compiler as unknown as webpack.Compiler);
    runFinishModules([
      {
        buildInfo: {
          [BUILD_INFO_STYLEX_KEY]: { resourcePath: 'src/button.tsx', stylexRules },
        },
      },
    ]);
    expect(plugin.stylexRules.size).toBe(1);

    runFinishModules([{ buildInfo: {} }]);

    expect(plugin.stylexRules.size).toBe(0);
  });

  test('a custom cache group replaces the default entirely (only name is defaulted)', () => {
    const plugin = new StyleXPlugin({
      cacheGroup: { name: 'custom-stylex', priority: 20 },
    });
    const { compiler } = createMockCompiler();

    plugin.apply(compiler as unknown as webpack.Compiler);

    const cacheGroup = compiler.options.optimization.splitChunks.cacheGroups['custom-stylex'];

    // Replacement semantics: no default `test` (or other fields) may leak in —
    // omitting `test` deliberately widens the group to every module, which is
    // how consumers funnel ALL extracted CSS into the stylex chunk.
    expect(cacheGroup).toEqual({
      name: 'custom-stylex',
      priority: 20,
    });
  });

  test('a custom cache group without a name falls back to the default chunk name', () => {
    const plugin = new StyleXPlugin({
      cacheGroup: { priority: 20 },
    });
    const { compiler } = createMockCompiler();

    plugin.apply(compiler as unknown as webpack.Compiler);

    const cacheGroup = compiler.options.optimization.splitChunks.cacheGroups[STYLEX_CHUNK_NAME];

    expect(cacheGroup).toEqual({
      name: STYLEX_CHUNK_NAME,
      priority: 20,
    });
  });

  test('default cache group matches only the packaged carrier stylesheet', () => {
    const plugin = new StyleXPlugin();
    const { compiler } = createMockCompiler();

    plugin.apply(compiler as unknown as webpack.Compiler);

    const test = compiler.options.optimization.splitChunks.cacheGroups[STYLEX_CHUNK_NAME]
      ?.test as RegExp;

    expect(test.test(require.resolve('../src/stylex.css'))).toBe(true);
    expect(test.test(path.join(path.sep, 'project', 'src', 'my-stylex.css'))).toBe(false);
    expect(test.test(path.join(path.sep, 'project', 'src', 'stylex.css'))).toBe(false);
  });

  test('carrierCss retargets the cacheGroup pattern to the custom carrier', () => {
    const plugin = new StyleXPlugin({ carrierCss: path.join('src', 'my-carrier.css') });
    const { compiler } = createMockCompiler();

    plugin.apply(compiler as unknown as webpack.Compiler);

    const cacheGroup = compiler.options.optimization.splitChunks.cacheGroups[STYLEX_CHUNK_NAME];
    const test = cacheGroup?.test as RegExp;
    const resolved = path.join(path.sep, 'project', 'src', 'my-carrier.css');

    expect(test.test(resolved)).toBe(true);
    expect(test.test('/repo/plugin-shared/dist/stylex-virtual.css?from=App.js')).toBe(true);
    // the default packaged carrier is replaced by the custom one
    expect(test.test('/repo/webpack-plugin/dist/stylex.css')).toBe(false);
  });

  test('scopes node_modules to exact stylexPackages entries', () => {
    const project = path.join(path.sep, 'project');
    const inNodeModules = (...segments: string[]) =>
      path.join(project, 'node_modules', ...segments);
    const plugin = new StyleXPlugin({ stylexPackages: ['@stylexjs/', 'my-design-system'] });

    expect(plugin.shouldProcessFile(path.join(project, 'app', 'page.tsx'))).toBe(true);
    expect(plugin.shouldProcessFile(inNodeModules('@stylexjs', 'open-props', 'colors.js'))).toBe(
      true
    );
    expect(plugin.shouldProcessFile(inNodeModules('my-design-system', 'tokens.js'))).toBe(true);
    expect(plugin.shouldProcessFile(inNodeModules('my-design-system-extra', 'tokens.js'))).toBe(
      false
    );
    expect(plugin.shouldProcessFile(inNodeModules('react', 'index.js'))).toBe(false);
  });
});
