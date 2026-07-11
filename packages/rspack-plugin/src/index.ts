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

import type { CacheGroupOptions } from '@stylexswc/plugin-shared';
import type { Compiler, sources } from '@rspack/core';

export const STYLEX_CHUNK_NAME = '_stylex-rspack-generated';

const PACKAGE_NAME = '@stylexswc/rspack-plugin';

export default class StyleXPlugin extends StyleXPluginCore {
  apply(compiler: Compiler) {
    this.resolveCarrier(compiler.context);
    this.assertAndInstallCacheGroup(
      compiler.options.optimization as {
        splitChunks?: false | { cacheGroups?: Record<string, CacheGroupOptions> };
      },
      PACKAGE_NAME,
      STYLEX_CHUNK_NAME
    );
    this.resolveDevOption(compiler.options.mode);

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
        test: VIRTUAL_STYLEX_CSS_DUMMY_IMPORT_PATTERN,
        use: [
          {
            loader: stylexVirtualCssLoaderPath,
            options: {},
          },
        ],
        // Prevents the dummy stylex-virtual.css imports from being tree
        // shaken (replaces the webpack plugin's nmf createModule hook)
        sideEffects: true,
      },
      {
        // The carrier stylesheet has no imports of its own — keep it from
        // being tree shaken even under a `sideEffects: false` package
        test: this.getCarrierPattern(),
        sideEffects: true,
      }
    );

    const { Compilation, sources: rspackSources } = compiler.webpack;
    const { RawSource } = rspackSources;

    compiler.hooks.thisCompilation.tap(PLUGIN_NAME, compilation => {
      // Plugin options change the generated CSS without any module changing;
      // mix them into chunk hashes so long-term caching notices.
      compilation.hooks.chunkHash.tap(PLUGIN_NAME, (_chunk, hash) => {
        hash.update(Buffer.from(this.buildChunkHashMeta(PACKAGE_NAME)));
      });

      // rspack rule transport: rebuild the rules map from the dummy-import
      // queries embedded in module identifiers, then publish for the Next.js
      // App Router client merge. Identifiers survive caching, and rebuilding
      // drops rules of files that were deleted or stopped importing stylex.
      compilation.hooks.finishModules.tap(PLUGIN_NAME, modules => {
        this.replaceFromChunkModuleIdentifiers(modules);
        this.publishNextjsRegistry(compiler.name);
      });

      compilation.hooks.processAssets.tapPromise(
        {
          name: PLUGIN_NAME,
          stage: Compilation.PROCESS_ASSETS_STAGE_PRE_PROCESS,
        },
        async assets => {
          const stylexChunk = compilation.namedChunks.get(STYLEX_CHUNK_NAME);

          if (stylexChunk != null) {
            // Refresh from the stylex chunk's own modules: the chunk graph is
            // final here, so this is the source of truth for the emitted
            // asset. When the chunk is missing, the finishModules collection
            // is kept so finalizeStylexAsset can warn about orphaned rules.
            this.replaceFromChunkModuleIdentifiers(
              compilation.chunkGraph.getChunkModules(stylexChunk)
            );
          }

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
              compilation.updateAsset(name, () => source as sources.Source, {
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
export { isAllowlistedPackage } from '@stylexswc/plugin-shared';
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
