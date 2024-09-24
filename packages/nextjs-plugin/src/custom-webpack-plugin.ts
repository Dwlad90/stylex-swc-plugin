import path from 'path';
import stylexBabelPlugin from '@stylexjs/babel-plugin';
import { transform } from '@stylexswc/rs-compiler';
import webpack from 'webpack';
import fs from 'fs/promises';

import type { Rule } from '@stylexjs/babel-plugin';
import type { Compiler, WebpackError } from 'webpack';

import type { StyleXOptions } from '@stylexswc/rs-compiler';

const { NormalModule, Compilation } = webpack;

const PLUGIN_NAME = 'stylex';

const IS_DEV_ENV = process.env.NODE_ENV === 'development';

const { RawSource, ConcatSource } = webpack.sources;

const stylexRules: Record<string, Rule[]> = {};
const cssFiles = new Set<any>();
const compilers = new Set<any>();

class StylexPlugin {
  filesInLastRun: any = null;
  filePath?: string | null = null;
  dev: boolean;
  appendTo: any;
  filename?: string;
  stylexImports: any[];
  useCSSLayers: any;
  rsOptions: StyleXOptions;

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
      NormalModule.getCompilationHooks(compilation).loader.tap(PLUGIN_NAME, (_, module) => {
        if (
          // JavaScript (and Flow) modules
          /\.jsx?/.test(path.extname(module.resource)) ||
          // TypeScript modules
          /\.tsx?/.test(path.extname(module.resource))
        ) {
          // We use .unshift() and not .push() like original babel plugin
          // because we want to run other transformations first, e.g. custom SWC plugins.
          module.loaders.unshift({
            loader: path.resolve(__dirname, 'custom-webpack-loader.js'),
            options: { stylexPlugin: this },
            ident: null,
            type: null,
          });
        }

        if (
          // JavaScript (and Flow) modules
          /\.css/.test(path.extname(module.resource))
        ) {
          cssFiles.add(module.resource);
        }
      });

      const getStyleXRules = () => {
        if (Object.keys(stylexRules).length === 0) {
          return null;
        }
        // Take styles for the modules that were included in the last compilation.
        const allRules = Object.keys(stylexRules)
          .map(filename => stylexRules[filename])
          .flat()
          .filter((rule): rule is Rule => !!rule);

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
            const stylexCSS = getStyleXRules();

            if (cssFileName && stylexCSS != null) {
              this.filePath = path.join(process.cwd(), '.next', cssFileName);

              const updatedSource = new ConcatSource(
                new RawSource(assets[cssFileName]?.source() || ''),
                new RawSource(stylexCSS)
              );

              compilation.updateAsset(cssFileName, updatedSource);
              compilers.add(compiler);
            }
          }
        );
      } else {
        // Consume collected rules and emit the stylex CSS asset
        compilation.hooks.additionalAssets.tap(PLUGIN_NAME, () => {
          try {
            const collectedCSS = getStyleXRules();
            if (collectedCSS && this.filename) {
              console.log('emitting asset', this.filename, collectedCSS);
              compilation.emitAsset(this.filename, new RawSource(collectedCSS));
              fs.writeFile(this.filename, collectedCSS).then(() =>
                console.log('wrote file', this.filename)
              );
            }
          } catch (e) {
            compilation.errors.push(e as WebpackError);
          }
        });
      }
    });
  }

  // This function is not called by Webpack directly.
  // Instead, `NormalModule.getCompilationHooks` is used to inject a loader
  // for JS modules. The loader than calls this function.
  async transformCode(inputCode: string, filename: string, logger: any) {
    const originalSource = inputCode;

    if (inputCode.includes('Welcome to my MDX page'))
      console.log('originalSource: ', originalSource);

    if (this.stylexImports.some(importName => originalSource.includes(importName))) {
      let result = transform(filename, inputCode, this.rsOptions);

      const metadata = result?.metadata;
      const code = result.code;
      const map = result.map;

      if (metadata.stylex != null && metadata.stylex.length > 0) {
        const oldRules = stylexRules[filename] || [];
        stylexRules[filename] = metadata.stylex;
        logger.debug(`Read stylex styles from ${filename}:`, metadata.stylex);

        const oldClassNames = new Set(oldRules.map(rule => rule[0]));
        const newClassNames = new Set(metadata.stylex.map(rule => rule[0]));

        // If there are any new classNames in the output we need to recompile
        // the CSS bundle.
        if (
          oldClassNames.size !== newClassNames.size ||
          [...newClassNames].some(className => !oldClassNames.has(className)) ||
          filename.endsWith('.stylex.ts') ||
          filename.endsWith('.stylex.tsx') ||
          filename.endsWith('.stylex.js')
        ) {
          compilers.forEach(compiler => {
            cssFiles.forEach(cssFile => {
              compiler.watchFileSystem.watcher.fileWatchers.get(cssFile).watcher.emit('change');
            });
          });
        }
      }

      return { code, map };
    }
    return { code: inputCode };
  }
}
export default StylexPlugin;

module.exports = StylexPlugin;
