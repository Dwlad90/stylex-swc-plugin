import path from 'path';
import { sources } from '@rspack/core';
import { describe, expect, test, vi } from 'vitest';

import StyleXPlugin, { STYLEX_CHUNK_NAME } from '../src';
import { parseStylexRulesFromIdentifier } from '../src/utils';

import type { Compiler, RuleSetRule } from '@rspack/core';
import type { Rule as StyleXRule } from '@stylexjs/babel-plugin';

type Source = InstanceType<typeof sources.RawSource>;

function createMockCompiler(chunkModules: Array<{ identifier: () => string }> = []) {
  const rules: RuleSetRule[] = [];
  const assets: Record<string, Source> = {
    'stylex.css': new sources.RawSource('/* existing */') as Source,
  };

  let processAssetsCallback: ((assets: Record<string, Source>) => Promise<void>) | undefined;

  const compilation = {
    hooks: {
      processAssets: {
        tapPromise: vi.fn(
          (_options: unknown, callback: (assets: Record<string, Source>) => Promise<void>) => {
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
    updateAsset: vi.fn((assetName: string, update: (source: Source) => unknown) => {
      const source = assets[assetName];

      if (!source) {
        throw new Error(`Missing asset: ${assetName}`);
      }

      assets[assetName] = update(source) as Source;
    }),
  };

  const compiler = {
    options: {
      optimization: {
        splitChunks: {
          cacheGroups: {},
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
      NormalModule: {
        getCompilationHooks: () => ({
          loader: {
            tap: vi.fn(),
          },
        }),
      },
      sources,
    },
    hooks: {
      thisCompilation: {
        tap: vi.fn((_name: string, callback: (compilation: unknown) => void) =>
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
    runProcessAssets: () => processAssetsCallback?.(assets),
  };
}

describe('@stylexswc/rspack-plugin', () => {
  test('transforms generated StyleX CSS before appending to the CSS asset', async () => {
    const transformCss = vi.fn(async (css: string, filePath: string | undefined) => {
      return `${css}\n/* transformed:${filePath} */`;
    });
    const plugin = new StyleXPlugin({ transformCss });
    const stylexRules: StyleXRule[] = [['x1abcd', { ltr: 'color:red', rtl: null }, 3000]];
    const query = new URLSearchParams({
      from: '/button.tsx',
      stylex: JSON.stringify(stylexRules),
    });

    const { compiler, assets, runProcessAssets } = createMockCompiler([
      {
        identifier: () =>
          `css|/repo/node_modules/@stylexswc/rspack-plugin/dist/stylex.virtual.css?${query.toString()}|used-exports`,
      },
    ]);

    plugin.apply(compiler as unknown as Compiler);
    await runProcessAssets();

    expect(transformCss).toHaveBeenCalledTimes(1);
    expect(transformCss.mock.calls[0]?.[1]).toBe('stylex.css');
    expect(assets['stylex.css']?.source().toString()).toContain('/* existing */');
    expect(assets['stylex.css']?.source().toString()).toContain('color:red');
    expect(assets['stylex.css']?.source().toString()).toContain('/* transformed:stylex.css */');
  });

  test('clears stale StyleX rules when no current chunk modules contain virtual CSS', async () => {
    const transformCss = vi.fn((css: string) => css);
    const plugin = new StyleXPlugin({ transformCss });
    plugin.stylexRules.set('/deleted.tsx', [['xstale', { ltr: 'color:red', rtl: null }, 3000]]);

    const { compiler, runProcessAssets } = createMockCompiler();

    plugin.apply(compiler as unknown as Compiler);
    await runProcessAssets();

    expect(plugin.stylexRules.size).toBe(0);
    expect(transformCss).not.toHaveBeenCalled();
  });

  test('registers static module rules with enforce mapped from loaderOrder', () => {
    const plugin = new StyleXPlugin();
    const { compiler, rules } = createMockCompiler();

    plugin.apply(compiler as unknown as Compiler);

    expect(rules).toHaveLength(2);
    expect(rules[0]?.enforce).toBe('pre');
    expect(rules[1]?.sideEffects).toBe(true);

    const lastPlugin = new StyleXPlugin({ loaderOrder: 'last' });
    const { compiler: lastCompiler, rules: lastRules } = createMockCompiler();
    lastPlugin.apply(lastCompiler as unknown as Compiler);
    expect(lastRules[0]?.enforce).toBe('post');
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
    const query = new URLSearchParams({
      from: '/app/layout.tsx',
      stylex: JSON.stringify(rules),
    });
    const identifier = `css|/repo/node_modules/@stylexswc/rspack-plugin/dist/stylex.virtual.css?${query.toString()}|used-exports`;

    expect(parseStylexRulesFromIdentifier(identifier)).toEqual(rules);
    expect(parseStylexRulesFromIdentifier('css|/repo/app/global.css|used-exports')).toBeNull();
    expect(
      parseStylexRulesFromIdentifier('css|/repo/dist/stylex.virtual.css?from=/app/layout.tsx')
    ).toBeNull();
  });
});
