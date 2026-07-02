import stylexBabelPlugin from '@stylexjs/babel-plugin';
import path from 'path';
import {
  DEFAULT_STYLEX_PACKAGES,
  INCLUDE_REGEXP,
  PLUGIN_NAME,
  STYLEX_CHUNK_NAME,
  VIRTUAL_CSS_PATTERN,
} from './constants';
import { shouldTransformFile } from '@stylexswc/rs-compiler';
import { parseStylexRulesFromIdentifier } from './utils';

import type { StyleXOptions, TransformedOptions } from '@stylexswc/rs-compiler';

import type { Compiler } from '@rspack/core';
import type { Rule as StyleXRule } from '@stylexjs/babel-plugin';
import type {
  CSSTransformer,
  StyleXPluginOption,
  StyleXRspackLoaderOptions,
  CacheGroupOptions,
} from './types';

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

  loaderOption: StyleXRspackLoaderOptions;
  cacheGroup?: CacheGroupOptions;
  transformCss: CSSTransformer;
  loaderOrder: StyleXPluginOption['loaderOrder'];
  stylexPackages: string[];
  include: StyleXOptions['include'];
  exclude: StyleXOptions['exclude'];

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
    // include/exclude filtering happens in the module rule condition; the loader
    // receives them stripped so the transform doesn't re-filter
    this.include = rsOptions.include;
    this.exclude = rsOptions.exclude;
    this.loaderOption = {
      stylexImports,
      rsOptions: {
        enableFontSizePxToRem: true,
        runtimeInjection: false,
        treeshakeCompensation: true,
        importSources: stylexImports,
        injectStylexSideEffects: loaderOrder !== 'last',
        ...rsOptions,
        include: undefined,
        exclude: undefined,
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
   * Rspack invokes JS loaders across a native boundary, so a no-op loader touch
   * is not free like it is in webpack: node_modules is excluded unless the
   * package is allowlisted via `stylexPackages`.
   */
  shouldProcessFile(resourcePath: string): boolean {
    if (!resourcePath || !INCLUDE_REGEXP.test(resourcePath)) {
      return false;
    }

    const nodeModulesSegment = `${path.sep}node_modules${path.sep}`;

    if (resourcePath.includes(nodeModulesSegment)) {
      if (!isAllowlistedPackage(resourcePath, this.stylexPackages)) {
        return false;
      }
    }

    return shouldTransformFile(resourcePath, this.include, this.exclude);
  }

  apply(compiler: Compiler) {
    // If splitChunk is enabled, we create a dedicated chunk for stylex css
    if (!compiler.options.optimization.splitChunks) {
      throw new Error(
        [
          'You don\'t have "optimization.splitChunks" enabled.',
          '"optimization.splitChunks" should be enabled for "@stylexswc/rspack-plugin" to function properly.',
        ].join(' ')
      );
    }

    // Resolved lazily against the actual build mode instead of a module-load-time
    // NODE_ENV snapshot, so `dev` reflects this compilation, not whatever env var
    // was set when the plugin module was first `require`'d.
    this.loaderOption.rsOptions.dev ??= compiler.options.mode !== 'production';

    compiler.options.optimization.splitChunks.cacheGroups ??= {};
    compiler.options.optimization.splitChunks.cacheGroups[STYLEX_CHUNK_NAME] = this.cacheGroup ?? {
      name: STYLEX_CHUNK_NAME,
      test: VIRTUAL_CSS_PATTERN,
      type: 'css/mini-extract',
      chunks: 'all',
      enforce: true,
    };

    // Rspack computes loader lists natively, so loaders can't be injected
    // per-module like the webpack plugin does. Static module rules replace it:
    // `enforce` maps loaderOrder ('first' -> 'pre' runs before normal loaders).
    compiler.options.module.rules.push(
      {
        test: INCLUDE_REGEXP,
        enforce: this.loaderOrder === 'last' ? 'post' : 'pre',
        include: (resourcePath: string) => this.shouldProcessFile(resourcePath),
        use: [
          {
            loader: stylexLoaderPath,
            options: this.loaderOption,
          },
        ],
      },
      {
        test: VIRTUAL_CSS_PATTERN,
        use: [
          {
            loader: stylexVirtualLoaderPath,
            options: {},
          },
        ],
        // Prevents "stylex.virtual.css" imports from being tree shaken
        // (replaces the webpack plugin's nmf createModule hook)
        sideEffects: true,
      }
    );

    const { NormalModule, Compilation, sources } = compiler.webpack;
    const { RawSource, ConcatSource } = sources;

    compiler.hooks.thisCompilation.tap(PLUGIN_NAME, compilation => {
      // Supplement the loader context so stylex-loader can register rules
      // (rspack supports context augmentation, not loader-list mutation)
      NormalModule.getCompilationHooks(compilation).loader.tap(PLUGIN_NAME, loaderContext => {
        (
          loaderContext as unknown as {
            StyleXRspackContextKey: { registerStyleXRules: RegisterStyleXRules };
          }
        ).StyleXRspackContextKey = {
          registerStyleXRules: (resourcePath, stylexRules) => {
            this.stylexRules.set(resourcePath, stylexRules);
          },
        };
      });

      compilation.hooks.processAssets.tapPromise(
        {
          name: PLUGIN_NAME,
          stage: Compilation.PROCESS_ASSETS_STAGE_PRE_PROCESS,
        },
        async assets => {
          // on previous step, we create a "stylex" chunk to hold all virtual stylex css
          // the chunk contains all css chunks generated by the css extract plugin
          const stylexChunk = compilation.namedChunks.get(STYLEX_CHUNK_NAME);

          if (stylexChunk == null) {
            return;
          }

          // Collect stylex rules from module identifiers instead of the
          // self-maintained map: identifiers survive caching and carry the
          // rules across compilations, so this is the source of truth for
          // every mode -- not just Next.js. The self-maintained map only
          // grows (loader registrations are never unset), so relying on it
          // alone would keep serving CSS for files that were since deleted
          // or stopped importing stylex.
          const chunkModules = compilation.chunkGraph.getChunkModules(stylexChunk);
          const recollected = new Map<string, readonly StyleXRule[]>();

          for (const mod of chunkModules) {
            const identifier = mod.identifier();
            const rules = parseStylexRulesFromIdentifier(identifier);

            if (rules != null) {
              recollected.set(identifier, rules);
            }
          }

          this.stylexRules = recollected;

          // Let's find the css file that belongs to the stylex chunk
          const stylexChunkFiles = new Set(stylexChunk.files);
          const cssAssetDetails = Object.entries(assets).filter(
            ([assetName]) => stylexChunkFiles.has(assetName) && assetName.endsWith('.css')
          );

          if (cssAssetDetails.length === 0) {
            return;
          }
          if (cssAssetDetails.length > 1) {
            compiler
              .getInfrastructureLogger?.(PLUGIN_NAME)
              ?.warn(
                'Multiple CSS assets found for the stylex chunk. This should not happen. Please report this issue.'
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
