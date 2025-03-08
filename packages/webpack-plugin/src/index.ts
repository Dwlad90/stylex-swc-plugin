import stylexBabelPlugin from '@stylexjs/babel-plugin';
import path from 'path';
import {
  INCLUDE_REGEXP,
  IS_DEV_ENV,
  PLUGIN_NAME,
  STYLEX_CHUNK_NAME,
  VIRTUAL_CSS_PATH,
  VIRTUAL_CSS_PATTERN,
} from './constants';

import type webpack from 'webpack';
import type { Rule as StyleXRule } from '@stylexjs/babel-plugin';
import type {
  CSSTransformer,
  StyleXPluginOption,
  StyleXWebpackLoaderOptions,
  SupplementedLoaderContext,
} from './types';
import type { CssModule } from 'mini-css-extract-plugin';

const stylexLoaderPath = require.resolve('./stylex-loader');
const stylexVirtualLoaderPath = require.resolve('./stylex-virtual-css-loader');

const getStyleXRules = (stylexRules: Map<string, readonly StyleXRule[]>, useCSSLayers: boolean) => {
  if (stylexRules.size === 0) {
    return null;
  }
  // Take styles for the modules that were included in the last compilation.
  const allRules: StyleXRule[] = Array.from(stylexRules.values()).flat();

  return stylexBabelPlugin.processStylexRules(allRules, useCSSLayers);
};

const identityTransfrom: CSSTransformer = css => css;

export type RegisterStyleXRules = (_resourcePath: string, _stylexRules: StyleXRule[]) => void;

export default class StyleXPlugin {
  stylexRules = new Map<string, readonly StyleXRule[]>();
  useCSSLayers: boolean;

  loaderOption: StyleXWebpackLoaderOptions;

  transformCss: CSSTransformer;

  constructor({
    stylexImports = ['stylex', '@stylexjs/stylex'],
    useCSSLayers = false,
    rsOptions = {},
    nextjsMode = false,
    transformCss = identityTransfrom,
    transformer = 'rs-compiler',
    extractCSS = true,
  }: StyleXPluginOption = {}) {
    this.useCSSLayers = useCSSLayers;
    this.loaderOption = {
      stylexImports,
      rsOptions: {
        dev: IS_DEV_ENV,
        useRemForFontSize: true,
        runtimeInjection: false,
        genConditionalClasses: true,
        treeshakeCompensation: true,
        importSources: stylexImports,
        ...rsOptions,
      },
      nextjsMode,
      transformer,
      extractCSS,
    };
    this.transformCss = transformCss;
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
    compiler.options.optimization.splitChunks.cacheGroups[STYLEX_CHUNK_NAME] = {
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
        const modPath: string | undefined =
          createData.matchResource ?? createData.resourceResolveData?.path;

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
            (loaderContext as SupplementedLoaderContext).StyleXWebpackContextKey = {
              registerStyleXRules: (resourcePath, stylexRules) => {
                this.stylexRules.set(resourcePath, stylexRules);
              },
            };

            // We use .unshift() and not .push() like original webpack plugin
            // because we want to transpile theme imports first,
            // else it will be unused imports, that will be removed by tree shaking,
            // and to run other transformations first, e.g. custom SWC plugins.
            mod.loaders.unshift({
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

          const stylexCSS = getStyleXRules(this.stylexRules, this.useCSSLayers);

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

export { VIRTUAL_CSS_PATTERN };

export type { StyleXPluginOption } from './types';

module.exports = StyleXPlugin;
module.exports.default = StyleXPlugin;
module.exports.VIRTUAL_CSS_PATTERN = VIRTUAL_CSS_PATTERN;
