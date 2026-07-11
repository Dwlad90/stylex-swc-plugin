import path from 'path';

import {
  DEFAULT_STYLEX_PACKAGES,
  INCLUDE_REGEXP,
  PLUGIN_NAME,
  StyleXPluginCore,
  VIRTUAL_CSS_PATTERN,
  VIRTUAL_STYLEX_CSS_DUMMY_IMPORT_PATTERN,
  buildVirtualCssPattern,
  stylexLoaderPath,
  stylexVirtualCssLoaderPath,
} from '@stylexswc/plugin-shared';

import type webpack from 'webpack';

export const STYLEX_CHUNK_NAME = '_stylex-webpack-generated';

const PACKAGE_NAME = '@stylexswc/webpack-plugin';

export default class StyleXPlugin extends StyleXPluginCore {
  apply(compiler: webpack.Compiler) {
    this.resolveCarrier(compiler.context);
    this.assertAndInstallCacheGroup(
      compiler.options.optimization,
      PACKAGE_NAME,
      STYLEX_CHUNK_NAME
    );
    this.resolveDevOption(compiler.options.mode);

    const carrierPattern = this.getCarrierPattern();

    // The carrier stylesheet has no imports of its own, so force
    // "sideEffects" to keep it from being tree shaken out of the graph.
    compiler.hooks.normalModuleFactory.tap(PLUGIN_NAME, nmf => {
      nmf.hooks.createModule.tap(PLUGIN_NAME, createData => {
        const modPath = createData.matchResource ?? createData.resourceResolveData?.path;

        if (typeof modPath === 'string' && carrierPattern.test(modPath)) {
          createData.settings ??= {};
          createData.settings.sideEffects = true;
        }
      });
    });

    const { Compilation, NormalModule, sources } = compiler.webpack;
    const { RawSource } = sources;

    compiler.hooks.make.tap(PLUGIN_NAME, compilation => {
      // Plugin options change the generated CSS without any module changing;
      // mix them into chunk hashes so long-term caching notices.
      compilation.hooks.chunkHash.tap(PLUGIN_NAME, (_chunk, hash) => {
        hash.update(this.buildChunkHashMeta(PACKAGE_NAME));
      });

      // Apply loader to JS modules.
      NormalModule.getCompilationHooks(compilation).loader.tap(PLUGIN_NAME, (_loaderContext, mod) => {
        const modResource = mod.matchResource || mod.resource;
        const extname = path.extname(modResource);

        if (INCLUDE_REGEXP.test(extname)) {
          // Add path filtering check, including the node_modules allowlist
          if (!this.shouldProcessFile(mod.resource)) {
            return;
          }

          // Webpack runs loaders in reverse order, so `push` makes the
          // stylex-loader run before anything else ('first').
          const insertMethod = this.loaderOrder === 'last' ? 'unshift' : 'push';

          mod.loaders[insertMethod]({
            loader: stylexLoaderPath,
            options: this.loaderOption,
            ident: null,
            type: null,
          });
        } else if (VIRTUAL_STYLEX_CSS_DUMMY_IMPORT_PATTERN.test(modResource)) {
          mod.loaders.push({
            loader: stylexVirtualCssLoaderPath,
            options: {},
            ident: null,
            type: null,
          });
        }
      });

      // webpack rule transport: buildInfo written by the stylex-loader is
      // restored from the filesystem cache together with the module, so this
      // also sees rules of modules whose loaders didn't re-run.
      compilation.hooks.finishModules.tap(PLUGIN_NAME, modules => {
        this.collectFromBuildInfo(modules);
        this.publishNextjsRegistry(compiler.name);
      });

      compilation.hooks.processAssets.tapPromise(
        {
          name: PLUGIN_NAME,
          stage: Compilation.PROCESS_ASSETS_STAGE_PRE_PROCESS,
        },
        async assets => {
          this.mergeNextjsRegistry(compiler.name);

          await this.finalizeStylexAsset({
            assets,
            chunkName: STYLEX_CHUNK_NAME,
            compilerName: compiler.name,
            carrierHint: this.carrierCss
              ? `import '${this.carrierCss}'`
              : `import '${PACKAGE_NAME}/stylex.css'`,
            getNamedChunk: name => compilation.namedChunks.get(name),
            createSource: css => new RawSource(css),
            updateAsset: (name, source) =>
              compilation.updateAsset(name, () => source as webpack.sources.Source, {
                minimized: false,
              }),
            // compilation warnings surface in stats and the dev overlay,
            // unlike the infrastructure logger
            warn: message =>
              compilation.warnings.push(
                new compiler.webpack.WebpackError(`${PACKAGE_NAME}: ${message}`)
              ),
          });
        }
      );
    });
  }
}

export { StyleXPlugin, VIRTUAL_CSS_PATTERN, DEFAULT_STYLEX_PACKAGES, buildVirtualCssPattern };

// ESM exports keep the loader paths reachable in environments where the guarded
// CJS block below is skipped (e.g. tooling that evaluates this file as ESM)
export { stylexLoaderPath as loader, stylexVirtualCssLoaderPath as virtualLoader };

export type {
  CacheGroupOptions,
  RegisterStyleXRules,
  StyleXPluginOption,
} from '@stylexswc/plugin-shared';

// Skipped when `module.exports` is an ES module namespace (frozen, cannot be
// reassigned) — the ESM exports above provide the same surface there
if (
  typeof module !== 'undefined' &&
  Object.prototype.toString.call(module.exports) !== '[object Module]'
) {
  module.exports = StyleXPlugin;
  module.exports.default = StyleXPlugin;
  module.exports.StyleXPlugin = StyleXPlugin;
  module.exports.loader = stylexLoaderPath;
  module.exports.virtualLoader = stylexVirtualCssLoaderPath;
  module.exports.VIRTUAL_CSS_PATTERN = VIRTUAL_CSS_PATTERN;
  module.exports.STYLEX_CHUNK_NAME = STYLEX_CHUNK_NAME;
  module.exports.DEFAULT_STYLEX_PACKAGES = DEFAULT_STYLEX_PACKAGES;
  module.exports.buildVirtualCssPattern = buildVirtualCssPattern;
}
