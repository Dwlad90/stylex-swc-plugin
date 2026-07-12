import path from 'path';
import { sources } from '@rspack/core';
import { describe, expect, test, vi } from 'vitest';

import { parseStylexRulesFromIdentifier } from '@stylexswc/plugin-shared';
import StyleXPlugin, { STYLEX_CHUNK_NAME } from '../src';

import type { Compiler, RuleSetRule } from '@rspack/core';
import type { Rule as StyleXRule } from '@stylexjs/babel-plugin';

type Source = InstanceType<typeof sources.RawSource>;

function createMockCompiler(chunkModules: Array<{ identifier: () => string }> = []) {
  const rules: RuleSetRule[] = [];
  const assets: Record<string, Source> = {
    'stylex.css': new sources.RawSource('/* carrier placeholder */') as Source,
  };

  let finishModulesCallback:
    ((_modules: Iterable<{ identifier: () => string }>) => void) | undefined;
  let processAssetsCallback: ((_assets: Record<string, Source>) => Promise<void>) | undefined;

  const warnings: Error[] = [];
  const compilation = {
    warnings,
    hooks: {
      chunkHash: {
        tap: vi.fn(),
      },
      finishModules: {
        tap: vi.fn(
          (_name: string, callback: (_modules: Iterable<{ identifier: () => string }>) => void) => {
            finishModulesCallback = callback;
          }
        ),
      },
      processAssets: {
        tapPromise: vi.fn(
          (_options: unknown, callback: (_assets: Record<string, Source>) => Promise<void>) => {
            processAssetsCallback = callback;
          }
        ),
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
    chunkGraph: {
      getChunkModules: vi.fn(() => chunkModules),
    },
    updateAsset: vi.fn(
      (assetName: string, update: (_source: Source) => Source, _info?: unknown) => {
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
      module: {
        rules,
      },
    },
    webpack: {
      Compilation: {
        PROCESS_ASSETS_STAGE_PRE_PROCESS: 0,
      },
      sources,
      WebpackError: Error,
    },
    hooks: {
      thisCompilation: {
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
    rules,
    warnings,
    runFinishModules: (modules: Iterable<{ identifier: () => string }>) =>
      finishModulesCallback?.(modules),
    runProcessAssets: () => processAssetsCallback?.(assets),
  };
}

function dummyImportIdentifier(rules: StyleXRule[], from = 'src/button.tsx') {
  const query = new URLSearchParams({
    from,
    stylex: JSON.stringify(rules),
  });

  return `css|/repo/node_modules/@stylexswc/plugin-shared/dist/stylex-virtual.css?${query.toString()}|used-exports`;
}

describe('@stylexswc/rspack-plugin', () => {
  test('re-collects rules from chunk module identifiers and appends to the carrier CSS asset', async () => {
    const transformCss = vi.fn(async (css: string, filePath: string | undefined) => {
      return `${css}\n/* transformed:${filePath} */`;
    });
    const plugin = new StyleXPlugin({ transformCss });
    const stylexRules: StyleXRule[] = [['x1abcd', { ltr: '.x1abcd{color:red}', rtl: null }, 3000]];

    const { compiler, assets, runProcessAssets } = createMockCompiler([
      { identifier: () => dummyImportIdentifier(stylexRules) },
    ]);

    plugin.apply(compiler as unknown as Compiler);
    await runProcessAssets();

    expect(transformCss).toHaveBeenCalledTimes(1);
    expect(transformCss.mock.calls[0]?.[1]).toBe('stylex.css');

    const finalCss = assets['stylex.css']?.source().toString();

    // existing asset content is PRESERVED (a widened cacheGroup funnels
    // foreign CSS into this asset), with the StyleX css appended after it
    expect(finalCss).toContain('/* carrier placeholder */');
    expect(finalCss).toContain('color:red');
    expect(finalCss).toContain('/* transformed:stylex.css */');
    expect(finalCss?.indexOf('color:red')).toBeGreaterThan(
      finalCss?.indexOf('/* carrier placeholder */') ?? -1
    );
  });

  test('clears stale StyleX rules when no current chunk modules contain dummy imports', async () => {
    const transformCss = vi.fn((css: string) => css);
    const plugin = new StyleXPlugin({ transformCss });
    plugin.stylexRules.set('/deleted.tsx', [['xstale', { ltr: 'color:red', rtl: null }, 3000]]);

    const { compiler, runProcessAssets } = createMockCompiler();

    plugin.apply(compiler as unknown as Compiler);
    await runProcessAssets();

    expect(plugin.stylexRules.size).toBe(0);
    expect(transformCss).not.toHaveBeenCalled();
  });

  test('collects rules from module identifiers in finishModules', () => {
    const plugin = new StyleXPlugin();
    const stylexRules: StyleXRule[] = [['x1abcd', { ltr: '.x1abcd{color:red}', rtl: null }, 3000]];
    const { compiler, runFinishModules } = createMockCompiler();

    plugin.apply(compiler as unknown as Compiler);
    runFinishModules([
      { identifier: () => dummyImportIdentifier(stylexRules) },
      { identifier: () => 'javascript/auto|/repo/src/button.tsx' },
    ]);

    expect(plugin.stylexRules.size).toBe(1);
  });

  test('warns when rules were extracted but no stylex chunk exists', async () => {
    const plugin = new StyleXPlugin();
    const stylexRules: StyleXRule[] = [['x1abcd', { ltr: '.x1abcd{color:red}', rtl: null }, 3000]];

    const { compiler, compilation, warnings, runFinishModules, runProcessAssets } =
      createMockCompiler();

    plugin.apply(compiler as unknown as Compiler);

    // finishModules collects the rules; the chunk then never materializes
    runFinishModules([{ identifier: () => dummyImportIdentifier(stylexRules) }]);
    compilation.namedChunks.clear();
    await runProcessAssets();

    expect(warnings).toHaveLength(1);
    expect(warnings[0]?.message).toContain('MISSING from the output');
    expect(warnings[0]?.message).toContain("import '@stylexswc/rspack-plugin/stylex.css'");
  });

  test('in Next.js mode only the client compiler warns about a missing carrier', async () => {
    const stylexRules: StyleXRule[] = [['x1abcd', { ltr: '.x1abcd{color:red}', rtl: null }, 3000]];

    // server compiler: collecting rules without a chunk is its normal state
    const server = createMockCompiler();
    server.compiler.name = 'server';
    const serverPlugin = new StyleXPlugin({ nextjsMode: true });

    serverPlugin.apply(server.compiler as unknown as Compiler);
    server.runFinishModules([{ identifier: () => dummyImportIdentifier(stylexRules) }]);
    server.compilation.namedChunks.clear();
    await server.runProcessAssets();
    expect(server.warnings).toHaveLength(0);

    // client compiler: it owns the emitted CSS, so the warning stands
    const client = createMockCompiler();
    client.compiler.name = 'client';
    const clientPlugin = new StyleXPlugin({ nextjsMode: true });

    clientPlugin.apply(client.compiler as unknown as Compiler);
    client.runFinishModules([{ identifier: () => dummyImportIdentifier(stylexRules) }]);
    client.compilation.namedChunks.clear();
    await client.runProcessAssets();
    expect(client.warnings).toHaveLength(1);
  });

  test('does not warn without extracted rules', async () => {
    const plugin = new StyleXPlugin();
    const { compiler, compilation, warnings, runProcessAssets } = createMockCompiler();

    plugin.apply(compiler as unknown as Compiler);
    compilation.namedChunks.clear();
    await runProcessAssets();

    expect(warnings).toHaveLength(0);
  });

  test('registers static module rules with enforce mapped from loaderOrder', () => {
    const plugin = new StyleXPlugin();
    const { compiler, compilation, rules } = createMockCompiler();

    plugin.apply(compiler as unknown as Compiler);

    expect(rules).toHaveLength(3);
    expect(rules[0]?.enforce).toBe('pre');
    expect(rules[1]?.sideEffects).toBe(true);
    // default carrier sideEffects rule
    expect(rules[2]?.sideEffects).toBe(true);
    expect(compilation.hooks.chunkHash.tap).toHaveBeenCalledTimes(1);

    const lastPlugin = new StyleXPlugin({ loaderOrder: 'last' });
    const { compiler: lastCompiler, rules: lastRules } = createMockCompiler();
    lastPlugin.apply(lastCompiler as unknown as Compiler);
    expect(lastRules[0]?.enforce).toBe('post');
  });

  test('carrierCss retargets the cacheGroup pattern and adds a sideEffects rule', () => {
    const plugin = new StyleXPlugin({ carrierCss: path.join('src', 'my-carrier.css') });
    const { compiler, rules } = createMockCompiler();

    plugin.apply(compiler as unknown as Compiler);

    const cacheGroup = compiler.options.optimization.splitChunks.cacheGroups[STYLEX_CHUNK_NAME];
    const test = cacheGroup?.test as RegExp;
    const resolved = path.join(path.sep, 'project', 'src', 'my-carrier.css');

    expect(test.test(resolved)).toBe(true);
    expect(test.test('/repo/plugin-shared/dist/stylex-virtual.css?from=App.js')).toBe(true);
    // the default packaged carrier is replaced by the custom one
    expect(test.test('/repo/rspack-plugin/dist/stylex.css')).toBe(false);

    // carrier sideEffects rule targets the custom path
    const carrierRule = rules[2];
    expect(carrierRule?.sideEffects).toBe(true);
    expect((carrierRule?.test as RegExp).test(resolved)).toBe(true);
  });

  test('default cache group matches only the packaged carrier stylesheet', () => {
    const plugin = new StyleXPlugin();
    const { compiler } = createMockCompiler();

    plugin.apply(compiler as unknown as Compiler);

    const test = compiler.options.optimization.splitChunks.cacheGroups[STYLEX_CHUNK_NAME]
      ?.test as RegExp;

    expect(test.test(require.resolve('../src/stylex.css'))).toBe(true);
    expect(test.test(path.join(path.sep, 'project', 'src', 'my-stylex.css'))).toBe(false);
    expect(test.test(path.join(path.sep, 'project', 'src', 'stylex.css'))).toBe(false);
  });

  test('a custom cache group replaces the default entirely (only name is defaulted)', () => {
    const plugin = new StyleXPlugin({
      cacheGroup: { name: 'custom-stylex', type: 'css/mini-extract', chunks: 'all', enforce: true },
    });
    const { compiler } = createMockCompiler();

    plugin.apply(compiler as unknown as Compiler);

    // Replacement semantics: no default `test` may leak in — omitting `test`
    // deliberately widens the group to every module, which is how consumers
    // funnel ALL extracted CSS into the stylex chunk.
    expect(compiler.options.optimization.splitChunks.cacheGroups['custom-stylex']).toEqual({
      name: 'custom-stylex',
      type: 'css/mini-extract',
      chunks: 'all',
      enforce: true,
    });
    expect(
      compiler.options.optimization.splitChunks.cacheGroups[STYLEX_CHUNK_NAME]
    ).toBeUndefined();
  });

  test('a custom cache group without a name falls back to the default chunk name', () => {
    const plugin = new StyleXPlugin({ cacheGroup: { priority: 20 } });
    const { compiler } = createMockCompiler();

    plugin.apply(compiler as unknown as Compiler);

    expect(compiler.options.optimization.splitChunks.cacheGroups[STYLEX_CHUNK_NAME]).toEqual({
      name: STYLEX_CHUNK_NAME,
      priority: 20,
    });
  });

  test('a RegExp cache group shorthand is normalized to a Rspack-compatible object', () => {
    const test = /\.css$/;
    const plugin = new StyleXPlugin({ cacheGroup: test });
    const { compiler } = createMockCompiler();

    plugin.apply(compiler as unknown as Compiler);

    expect(compiler.options.optimization.splitChunks.cacheGroups[STYLEX_CHUNK_NAME]).toEqual({
      name: STYLEX_CHUNK_NAME,
      test,
    });
  });

  test('scopes node_modules to the stylexPackages allowlist', () => {
    const plugin = new StyleXPlugin();
    const project = path.join(path.sep, 'project');
    const inNodeModules = (...segments: string[]) =>
      path.join(project, 'node_modules', ...segments);

    expect(plugin.shouldProcessFile(path.join(project, 'app', 'page.tsx'))).toBe(true);
    expect(plugin.shouldProcessFile(inNodeModules('react', 'index.js'))).toBe(false);
    expect(
      plugin.shouldProcessFile(inNodeModules('@stylexjs', 'open-props', 'lib', 'colors.js'))
    ).toBe(true);
    // pnpm layout: the final `node_modules/@stylexjs/...` segment still matches
    expect(
      plugin.shouldProcessFile(
        inNodeModules(
          '.pnpm',
          '@stylexjs+open-props@0.11.1',
          'node_modules',
          '@stylexjs',
          'open-props',
          'lib',
          'colors.js'
        )
      )
    ).toBe(true);
    expect(plugin.shouldProcessFile(path.join(project, 'app', 'styles.css'))).toBe(false);

    const custom = new StyleXPlugin({ stylexPackages: ['@stylexjs/', 'my-design-system'] });
    expect(custom.shouldProcessFile(inNodeModules('my-design-system', 'tokens.js'))).toBe(true);
    expect(custom.shouldProcessFile(inNodeModules('my-design-system-extra', 'tokens.js'))).toBe(
      false
    );
    expect(custom.shouldProcessFile(inNodeModules('other-lib', 'tokens.js'))).toBe(false);
  });

  test('parses StyleX rules from `|`-segmented css module identifiers', () => {
    const rules: StyleXRule[] = [['x1abcd', { ltr: '.x1abcd{color:red}', rtl: null }, 3000]];

    expect(parseStylexRulesFromIdentifier(dummyImportIdentifier(rules, 'app/layout.tsx'))).toEqual(
      rules
    );
    expect(parseStylexRulesFromIdentifier('css|/repo/app/global.css|used-exports')).toBeNull();
    expect(
      parseStylexRulesFromIdentifier('css|/repo/dist/stylex-virtual.css?from=app/layout.tsx')
    ).toBeNull();
  });
});
