import stylexBabelPlugin from '@stylexjs/babel-plugin';
import path from 'path';
import {
  DEFAULT_STYLEX_PACKAGES,
  INCLUDE_REGEXP,
  IS_DEV_ENV,
  PLUGIN_NAME,
  STYLEX_CHUNK_NAME,
  VIRTUAL_CSS_PATH,
  VIRTUAL_CSS_PATTERN,
} from './constants';
import { shouldTransformFile } from '@stylexswc/rs-compiler';

import type { TransformedOptions } from '@stylexswc/rs-compiler';

import type webpack from 'webpack';
import type { Rule as StyleXRule } from '@stylexjs/babel-plugin';
import type {
  CSSTransformer,
  StyleXPluginOption,
  StyleXWebpackLoaderOptions,
  SupplementedLoaderContext,
  CacheGroupOptions,
} from './types';
import type { CssModule } from 'mini-css-extract-plugin';

function resolveLoaderPath(loaderName: string) {
  try {
    return require.resolve(`./${loaderName}`);
  } catch (error) {
    const isModuleNotFound =
      error instanceof Error && 'code' in error && error.code === 'MODULE_NOT_FOUND';

    if (!isModuleNotFound) {
      throw error;
    }

    // Loaders resolve as `.ts` only when the plugin runs from source (e.g. vitest);
    // published dist builds always resolve above
    return require.resolve(`./${loaderName}.ts`);
  }
}

const stylexLoaderPath = resolveLoaderPath('stylex-loader');
const stylexVirtualLoaderPath = resolveLoaderPath('stylex-virtual-css-loader');

const getStyleXRules = (
  stylexRules: Map<string, readonly StyleXRule[]>,
  transformedOptions: TransformedOptions
) => {
  if (stylexRules.size === 0) {
    return null;
  }
  // Take styles for the modules that were included in the last compilation.
  const allRules: StyleXRule[] = Array.from(stylexRules.values()).flat();

  return stylexBabelPlugin.processStylexRules(allRules, transformedOptions);
};

const identityTransform: CSSTransformer = css => css;

function isAllowlistedPackage(resourcePath: string, stylexPackages: string[]) {
  const nodeModulesSegment = `${path.sep}node_modules${path.sep}`;
  const nodeModulesEntries = path.normalize(resourcePath).split(nodeModulesSegment).slice(1);

  return stylexPackages.some(packageName => {
    const normalizedPackageName = path.normalize(packageName).replace(/[\\/]$/, '');

    return nodeModulesEntries.some(
      entry =>
        entry === normalizedPackageName || entry.startsWith(`${normalizedPackageName}${path.sep}`)
    );
  });
}

export type RegisterStyleXRules = (_resourcePath: string, _stylexRules: StyleXRule[]) => void;

export default class StyleXPlugin {
  stylexRules = new Map<string, readonly StyleXRule[]>();
  transformedOptions: TransformedOptions;

  loaderOption: StyleXWebpackLoaderOptions;
  cacheGroup?: CacheGroupOptions;
  transformCss: CSSTransformer;
  loaderOrder: StyleXPluginOption['loaderOrder'];
  stylexPackages: string[];
  constructor({
    stylexImports = ['stylex', '@stylexjs/stylex'],
    useCSSLayers = false,
    rsOptions = {},
    nextjsMode = false,
    transformCss = identityTransform,
    extractCSS = true,
    loaderOrder = 'first',
    cacheGroup,
    stylexPackages = DEFAULT_STYLEX_PACKAGES,
  }: StyleXPluginOption = {}) {
    this.transformedOptions = {
      useLayers: useCSSLayers,
      legacyDisableLayers: rsOptions.legacyDisableLayers,
      enableLTRRTLComments: rsOptions.enableLTRRTLComments,
    };
    this.loaderOption = {
      stylexImports,
      rsOptions: {
        dev: IS_DEV_ENV,
        enableFontSizePxToRem: true,
        runtimeInjection: false,
        treeshakeCompensation: true,
        importSources: stylexImports,
        injectStylexSideEffects: loaderOrder !== 'last',
        ...rsOptions,
      },
      nextjsMode,
      extractCSS,
    };
    this.transformCss = transformCss;
    this.loaderOrder = loaderOrder;
    this.cacheGroup = cacheGroup;
    this.stylexPackages = stylexPackages;
  }

  /**
   * Excludes node_modules by default, matching the rspack plugin, so
   * unrelated dependencies whose source happens to mention a StyleX import
   * string aren't parsed and transformed unless explicitly allowlisted.
   */
  shouldProcessFile(resourcePath: string): boolean {
    if (!resourcePath) {
      return false;
    }

    const nodeModulesSegment = `${path.sep}node_modules${path.sep}`;

    if (resourcePath.includes(nodeModulesSegment)) {
      if (!isAllowlistedPackage(resourcePath, this.stylexPackages)) {
        return false;
      }
    }

    return shouldTransformFile(
      resourcePath,
      this.loaderOption.rsOptions?.include,
      this.loaderOption.rsOptions?.exclude
    );
  }

  apply(compiler: webpack.Compiler) {
    // If splitChunk is enabled, we create a dedicated chunk for stylex css
    if (!compiler.options.optimization.splitChunks) {
      throw new Error(
        [
          'You don\'t have "optimization.splitChunks" enabled.',
          '"optimization.splitChunks" should be enabled for "@stylexswc/webpack-plugin" to function properly.',
        ].join(' ')
      );
    }

    compiler.options.optimization.splitChunks.cacheGroups ??= {};
    compiler.options.optimization.splitChunks.cacheGroups[STYLEX_CHUNK_NAME] = this.cacheGroup ?? {
      name: STYLEX_CHUNK_NAME,
      test: VIRTUAL_CSS_PATTERN,
      type: 'css/mini-extract',
      chunks: 'all',
      enforce: true,
    };

    // stylex-loader adds virtual css import (which triggers virtual-loader)
    // This prevents "stylex.virtual.css" files from being tree shaken by forcing
    // "sideEffects" setting.
    compiler.hooks.normalModuleFactory.tap(PLUGIN_NAME, nmf => {
      nmf.hooks.createModule.tap(PLUGIN_NAME, createData => {
        const modPath = createData.matchResource ?? createData.resourceResolveData?.path;

        if (modPath === VIRTUAL_CSS_PATH) {
          createData.settings ??= {};
          createData.settings.sideEffects = true;
        }
      });
    });

    const { Compilation, NormalModule, sources } = compiler.webpack;
    const { RawSource, ConcatSource } = sources;

    compiler.hooks.make.tap(PLUGIN_NAME, compilation => {
      // Apply loader to JS modules.
      NormalModule.getCompilationHooks(compilation).loader.tap(
        PLUGIN_NAME,
        (loaderContext, mod) => {
          const extname = path.extname(mod.matchResource || mod.resource);

          if (INCLUDE_REGEXP.test(extname)) {
            // Add path filtering check, including the node_modules allowlist
            const shouldTransform = this.shouldProcessFile(mod.resource);

            if (!shouldTransform) {
              return; // Skip adding loader if filtered out
            } else {
              this.loaderOption.rsOptions.include = undefined;
              this.loaderOption.rsOptions.exclude = undefined;
            }

            (loaderContext as SupplementedLoaderContext).StyleXWebpackContextKey = {
              registerStyleXRules: (resourcePath, stylexRules) => {
                this.stylexRules.set(resourcePath, stylexRules);
              },
            };

            const insertMethod = this.loaderOrder === 'last' ? 'unshift' : 'push';

            mod.loaders[insertMethod]({
              loader: stylexLoaderPath,
              options: this.loaderOption,
              ident: null,
              type: null,
            });
          }

          if (VIRTUAL_CSS_PATTERN.test(mod.matchResource || mod.resource)) {
            mod.loaders.push({
              loader: stylexVirtualLoaderPath,
              options: {},
              ident: null,
              type: null,
            });
          }
        }
      );

      compilation.hooks.processAssets.tapPromise(
        {
          name: PLUGIN_NAME,
          stage: Compilation.PROCESS_ASSETS_STAGE_PRE_PROCESS,
        },
        async assets => {
          // on previous step, we create a "stylex" chunk to hold all virtual stylex css
          // the chunk contains all css chunks generated by mini-css-extract-plugin
          const stylexChunk = compilation.namedChunks.get(STYLEX_CHUNK_NAME);

          if (stylexChunk == null) {
            return;
          }

          // Collect stylex rules from module instead of self maintained map
          if (this.loaderOption.nextjsMode) {
            const cssModulesInStylexChunk =
              compilation.chunkGraph.getChunkModulesIterableBySourceType(
                stylexChunk,
                'css/mini-extract'
              );

            // we only re-collect stylex rules if we can found css in the stylex chunk
            if (cssModulesInStylexChunk) {
              this.stylexRules.clear();

              for (const cssModule of cssModulesInStylexChunk as Iterable<CssModule>) {
                // eslint-disable-next-line @typescript-eslint/no-explicit-any
                const stringifiedStylexRule = ((cssModule as any)._identifier as string)
                  .split('!')
                  .pop()
                  ?.split('?')
                  .pop();

                if (!stringifiedStylexRule) {
                  continue;
                }

                const params = new URLSearchParams(stringifiedStylexRule);
                const stylex = params.get('stylex');
                if (stylex != null) {
                  this.stylexRules.set(cssModule.identifier(), JSON.parse(stylex));
                }
              }
            }
          }

          // Let's find the css file that belongs to the stylex chunk
          const cssAssetDetails = Object.entries(assets).filter(
            ([assetName]) => stylexChunk.files.has(assetName) && assetName.endsWith('.css')
          );

          if (cssAssetDetails.length === 0) {
            return;
          }
          if (cssAssetDetails.length > 1) {
            console.warn(
              '[stylex-webpack] Multiple CSS assets found for the stylex chunk. This should not happen. Please report this issue.'
            );
          }
          const stylexAsset = cssAssetDetails[0];

          const stylexCSS = getStyleXRules(this.stylexRules, this.transformedOptions);

          if (stylexCSS == null) {
            return;
          }

          const cssAsset = stylexAsset?.[0];
          const finalCss = await this.transformCss(stylexCSS, cssAsset);

          if (cssAsset) {
            compilation.updateAsset(
              cssAsset,
              source => new ConcatSource(source, new RawSource(finalCss))
            );
          }
        }
      );
    });
  }
}

export { VIRTUAL_CSS_PATTERN, STYLEX_CHUNK_NAME, DEFAULT_STYLEX_PACKAGES };
// ESM exports keep the loader paths reachable in environments where the guarded
// CJS block below is skipped (e.g. tooling that evaluates this file as ESM)
export { stylexLoaderPath as loader, stylexVirtualLoaderPath as virtualLoader };

export type { StyleXPluginOption, CacheGroupOptions } from './types';

// Skipped when `module.exports` is an ES module namespace (frozen, cannot be
// reassigned) — the ESM exports above provide the same surface there
if (
  typeof module !== 'undefined' &&
  Object.prototype.toString.call(module.exports) !== '[object Module]'
) {
  module.exports = StyleXPlugin;
  module.exports.default = StyleXPlugin;
  module.exports.loader = stylexLoaderPath;
  module.exports.virtualLoader = stylexVirtualLoaderPath;
  module.exports.VIRTUAL_CSS_PATTERN = VIRTUAL_CSS_PATTERN;
  module.exports.STYLEX_CHUNK_NAME = STYLEX_CHUNK_NAME;
  module.exports.DEFAULT_STYLEX_PACKAGES = DEFAULT_STYLEX_PACKAGES;
}
