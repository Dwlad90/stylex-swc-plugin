import path from 'path';
import stylexBabelPlugin from '@stylexjs/babel-plugin';
import { transform } from '@stylexswc/rs-compiler';
import webpack from 'webpack';

import type { StyleXOptions } from '@stylexswc/rs-compiler';
import type { Rule } from '@stylexjs/babel-plugin';
import type { Compiler, NormalModule as NormalModuleType, WebpackError } from 'webpack';

const { NormalModule, Compilation } = webpack;

const PLUGIN_NAME = 'stylex';

const IS_DEV_ENV = process.env.NODE_ENV === 'development';

const { RawSource, ConcatSource } = webpack.sources;

class StylexPlugin {
  filesInLastRun: any[] | null = null;
  filePath?: string | null = null;
  dev: boolean;
  appendTo: any;
  filename?: string;
  stylexImports: any[];
  useCSSLayers: any;
  rsOptions: StyleXOptions;

  stylexRules: Record<string, NonNullable<Rule>[]> = {};

  constructor({
    dev = IS_DEV_ENV,
    appendTo,
    filename = appendTo == null ? 'stylex.css' : undefined,
    stylexImports = ['stylex', '@stylexjs/stylex'],
    useCSSLayers = false,
    rsOptions = {},
  }: any = {}) {
    this.dev = dev;
    this.appendTo = appendTo;
    this.filename = filename;
    this.stylexImports = stylexImports;

    this.useCSSLayers = useCSSLayers;
    this.rsOptions = rsOptions;
  }

  apply(compiler: Compiler) {
    compiler.hooks.make.tap(PLUGIN_NAME, compilation => {
      // Apply loader to JS modules.
      NormalModule.getCompilationHooks(compilation).loader.tap(
        PLUGIN_NAME,
        (loaderContext, module) => {
          if (
            // .js, .jsx, .mjs, .cjs, .ts, .tsx, .mts, .cts
            /\.[mc]?[jt]sx?$/.test(path.extname(module.resource))
          ) {
            // It might make sense to use .push() here instead of .unshift()
            // Webpack usually runs loaders in reverse order and we want to ideally run
            // our loader before anything else.
            module.loaders.unshift({
              loader: path.resolve(__dirname, 'loader.js'),
              options: { stylexPlugin: this },
              ident: null,
              type: null,
            });
          }
        }
      );

      // Make a list of all modules that were included in the last compilation.
      // This might need to be tweaked if not all files are included after a change
      compilation.hooks.finishModules.tap(PLUGIN_NAME, modules => {
        this.filesInLastRun = [...(modules as Set<NormalModuleType>).values()].map(m => m.resource);
      });

      const getStyleXRules = () => {
        const { stylexRules } = this;
        if (Object.keys(stylexRules).length === 0) {
          return null;
        }
        // Take styles for the modules that were included in the last compilation.
        const allRules = Object.keys(stylexRules)
          .filter(filename =>
            this.filesInLastRun == null ? true : this.filesInLastRun.includes(filename)
          )
          .map(filename => stylexRules[filename])
          .flat()
          .filter(Boolean) as Rule[];

        return stylexBabelPlugin.processStylexRules(allRules, this.useCSSLayers);
      };

      if (this.appendTo) {
        compilation.hooks.processAssets.tap(
          {
            name: PLUGIN_NAME,
            stage: Compilation.PROCESS_ASSETS_STAGE_PRE_PROCESS, // see below for more stages
          },
          assets => {
            const cssFileName = Object.keys(assets).find(
              typeof this.appendTo === 'function'
                ? this.appendTo
                : filename => filename.endsWith(this.appendTo)
            );
            if (cssFileName) {
              const cssAsset = assets[cssFileName];
              const stylexCSS = getStyleXRules();
              if (stylexCSS != null && cssAsset) {
                assets[cssFileName] = new ConcatSource(cssAsset, new RawSource(stylexCSS));
              }
            }
          }
        );
      } else {
        // We'll emit an asset ourselves. This comes with some complications in from Webpack.
        // If the filename contains replacement tokens, like [contenthash], we need to
        // process those tokens ourselves. Webpack does provide a way to reuse the configured
        // hashing functions. We'll take advantage of that to process tokens.
        const getContentHash = (source: string) => {
          const { outputOptions } = compilation;
          const { hashDigest, hashDigestLength, hashFunction, hashSalt } = outputOptions;
          const hash = compiler.webpack.util.createHash(hashFunction);

          if (hashSalt) {
            hash.update(hashSalt);
          }

          hash.update(source);

          const fullContentHash = hash.digest(hashDigest);

          return fullContentHash.toString().slice(0, hashDigestLength);
        };
        // Consume collected rules and emit the stylex CSS asset
        compilation.hooks.processAssets.tap(
          {
            name: PLUGIN_NAME,
            stage: Compilation.PROCESS_ASSETS_STAGE_ADDITIONAL,
          },
          () => {
            try {
              const collectedCSS = getStyleXRules();

              if (collectedCSS && this.filename) {
                // build up a content hash for the rules using webpack's configured hashing functions
                const contentHash = getContentHash(collectedCSS);

                // pretend to be a chunk so we can reuse the webpack routine to process the filename and do token replacement
                // see https://github.com/webpack/webpack/blob/main/lib/Compilation.js#L4733
                // see https://github.com/webpack/webpack/blob/main/lib/TemplatedPathPlugin.js#L102
                const data = {
                  filename: this.filename,
                  contentHash: contentHash,
                  chunk: {
                    id: this.filename,
                    name: path.parse(this.filename).name,
                    hash: contentHash,
                  },
                };

                const { path: hashedPath, info: assetsInfo } = compilation.getPathWithInfo(
                  data.filename,
                  data
                );
                compilation.emitAsset(hashedPath, new RawSource(collectedCSS), assetsInfo);
              }
            } catch (e) {
              compilation.errors.push(e as WebpackError);
            }
          }
        );
      }
    });
  }

  // This function is not called by Webpack directly.
  // Instead, `NormalModule.getCompilationHooks` is used to inject a loader
  // for JS modules. The loader than calls this function.
  async transformCode(inputCode: string, filename: string, logger: any) {
    if (this.stylexImports.some(importName => inputCode.includes(importName))) {
      const originalSource = inputCode;

      let result = transform(filename, originalSource, this.rsOptions);

      const metadata = result?.metadata;
      const code = result.code;
      const map = result.map;

      if (metadata.stylex != null && metadata.stylex.length > 0) {
        this.stylexRules[filename] = metadata.stylex;
        logger.debug(`Read stylex styles from ${filename}:`, metadata.stylex);
      }

      return { code, map };
    }

    return { code: inputCode };
  }
}

module.exports = StylexPlugin;
