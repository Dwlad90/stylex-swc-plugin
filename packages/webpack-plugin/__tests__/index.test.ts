import webpack from 'webpack';
import { describe, expect, test, vi } from 'vitest';

import StyleXPlugin, { STYLEX_CHUNK_NAME } from '../src';

describe('@stylexswc/webpack-plugin', () => {
  test('transforms generated StyleX CSS before appending to the CSS asset', async () => {
    const { RawSource, ConcatSource } = webpack.sources;
    const transformCss = vi.fn(async (css: string, filePath: string | undefined) => {
      return `${css}\n/* transformed:${filePath} */`;
    });
    const plugin = new StyleXPlugin({ transformCss });
    plugin.stylexRules.set('/button.tsx', [['x1abcd', { ltr: 'color:red' }, 3000]] as never);

    let processAssetsCallback:
      | ((assets: Record<string, webpack.sources.Source>) => Promise<void>)
      | undefined;
    const assets: Record<string, webpack.sources.Source> = {
      'stylex.css': new RawSource('/* existing */'),
    };
    const compilation = {
      hooks: {
        processAssets: {
          tapPromise: vi.fn(
            (
              _options: unknown,
              callback: (assets: Record<string, webpack.sources.Source>) => Promise<void>
            ) => {
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
      updateAsset: vi.fn(
        (assetName: string, update: (source: webpack.sources.Source) => unknown) => {
          const source = assets[assetName];

          if (!source) {
            throw new Error(`Missing asset: ${assetName}`);
          }

          assets[assetName] = update(source) as webpack.sources.Source;
        }
      ),
    };
    const compiler = {
      options: {
        optimization: {
          splitChunks: {
            cacheGroups: {},
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
      },
      hooks: {
        normalModuleFactory: {
          tap: vi.fn(),
        },
        make: {
          tap: vi.fn((_name: string, callback: (compilation: unknown) => void) =>
            callback(compilation)
          ),
        },
      },
    };

    plugin.apply(compiler as unknown as webpack.Compiler);
    await processAssetsCallback?.(assets);

    expect(transformCss).toHaveBeenCalledTimes(1);
    expect(transformCss.mock.calls[0]?.[1]).toBe('stylex.css');
    expect(assets['stylex.css']?.source().toString()).toContain('/* existing */');
    expect(assets['stylex.css']?.source().toString()).toContain('color:red');
    expect(assets['stylex.css']?.source().toString()).toContain('/* transformed:stylex.css */');
  });
});
