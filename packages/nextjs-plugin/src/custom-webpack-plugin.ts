/**
 * Copyright (c) Meta Platforms, Inc. and affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 *
 *
 */

import * as babel from "@babel/core";
import path from "path";
import stylexBabelPlugin from "@stylexjs/babel-plugin";
import webpack, { Compiler } from "webpack";
//@ts-expect-error - no types
import flowSyntaxPlugin from "@babel/plugin-syntax-flow";
//@ts-expect-error - no types
import jsxSyntaxPlugin from "@babel/plugin-syntax-jsx";
//@ts-expect-error - no types
import typescriptSyntaxPlugin from "@babel/plugin-syntax-typescript";
import fs from "fs/promises";
const { NormalModule, Compilation } = webpack;

const PLUGIN_NAME = "stylex";

const IS_DEV_ENV =
  process.env.NODE_ENV === "development" ||
  process.env.BABEL_ENV === "development";

const { RawSource, ConcatSource } = webpack.sources;

/*::
type PluginOptions = $ReadOnly<{
  dev?: boolean,
  useRemForFontSize?: boolean,
  stylexImports?: $ReadOnlyArray<string>,
  babelConfig?: $ReadOnly<{
    plugins?: $ReadOnlyArray<mixed>,
    presets?: $ReadOnlyArray<mixed>,
    babelrc?: boolean,
  }>,
  filename?: string,
  appendTo?: string | (string) => boolean,
  useCSSLayers?: boolean,
}>
*/

const stylexRules: any = {};
const cssFiles = new Set<any>();
const compilers = new Set<any>();

class StylexPlugin {
  filesInLastRun: any = null;
  filePath: any = null;
  dev: any;
  appendTo: any;
  filename: any;
  babelConfig: any;
  stylexImports: any[];
  babelPlugin: any;
  useCSSLayers: any;

  constructor({
    dev = IS_DEV_ENV,
    useRemForFontSize,
    appendTo,
    filename = appendTo == null ? "stylex.css" : undefined,
    stylexImports = ["stylex", "@stylexjs/stylex"],
    rootDir,
    babelConfig = {},
    aliases,
    useCSSLayers = false,
  }: any = {}) {
    this.dev = dev;
    this.appendTo = appendTo;
    this.filename = filename;
    // this.babelConfig = {
    //   plugins: [],
    //   presets: [],
    //   babelrc: [],
    //   ...babelConfig,
    // };
    this.stylexImports = stylexImports;
    // this.babelPlugin = [
    //   stylexBabelPlugin,
    //   {
    //     dev,
    //     useRemForFontSize,
    //     aliases: aliases,
    //     runtimeInjection: false,
    //     genConditionalClasses: true,
    //     treeshakeCompensation: true,
    //     unstable_moduleResolution: {
    //       type: "commonJS",
    //       rootDir,
    //     },
    //     importSources: stylexImports,
    //   },
    // ];
    this.useCSSLayers = useCSSLayers;
  }

  apply(compiler: Compiler) {
    compiler.hooks.make.tap(PLUGIN_NAME, (compilation) => {
      // Apply loader to JS modules.
      NormalModule.getCompilationHooks(compilation).loader.tap(
        PLUGIN_NAME,
        (loaderContext, module) => {
          if (
            // JavaScript (and Flow) modules
            /\.jsx?/.test(path.extname(module.resource)) ||
            // TypeScript modules
            /\.tsx?/.test(path.extname(module.resource))
          ) {
            // We use .push() here instead of .unshift()
            // Webpack usually runs loaders in reverse order and we want to ideally run
            // our loader before anything else.
            // @ts-expect-error - tmp
            module.loaders.unshift({
              loader: path.resolve(__dirname, "custom-webpack-loader.js"),
              options: { stylexPlugin: this },
            });
          }

          if (
            // JavaScript (and Flow) modules
            /\.css/.test(path.extname(module.resource))
          ) {
            cssFiles.add(module.resource);
          }
        },
      );

      const getStyleXRules = () => {
        if (Object.keys(stylexRules).length === 0) {
          return null;
        }
        // Take styles for the modules that were included in the last compilation.
        const allRules = Object.keys(stylexRules)
        .map((filename) => stylexRules[filename])
        .flat();
        // console.log("!!!processStylexRules allRules: ", allRules);
        return stylexBabelPlugin.processStylexRules(
          allRules,
          this.useCSSLayers,
        );
      };

      if (this.appendTo) {
        compilation.hooks.processAssets.tap(
          {
            name: PLUGIN_NAME,
            stage: Compilation.PROCESS_ASSETS_STAGE_PRE_PROCESS, // see below for more stages
          },
          (assets) => {
            const cssFileName = Object.keys(assets).find(
              typeof this.appendTo === "function"
                ? this.appendTo
                : (filename) => filename.endsWith(this.appendTo),
            );
            const stylexCSS = getStyleXRules();

            if (cssFileName && stylexCSS != null) {
              this.filePath = path.join(process.cwd(), ".next", cssFileName);

              const updatedSource = new ConcatSource(
                // @ts-expect-error - tmp
                new RawSource(assets[cssFileName].source()),
                new RawSource(stylexCSS),
              );

              compilation.updateAsset(cssFileName, updatedSource);
              compilers.add(compiler);
            }
          },
        );
      } else {
        // Consume collected rules and emit the stylex CSS asset
        compilation.hooks.additionalAssets.tap(PLUGIN_NAME, () => {
          try {
            const collectedCSS = getStyleXRules();
            if (collectedCSS) {
              console.log("emitting asset", this.filename, collectedCSS);
              compilation.emitAsset(this.filename, new RawSource(collectedCSS));
              fs.writeFile(this.filename, collectedCSS).then(() =>
                console.log("wrote file", this.filename),
              );
            }
          } catch (e) {
            // @ts-expect-error - tmp
            compilation.errors.push(e);
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
    // if(inputCode.includes("export default function Card") || inputCode.includes("export const buttonTokens")) console.log("originalSource: ", originalSource);
    if(inputCode.includes("Welcome to my MDX page")) console.log("originalSource: ", originalSource);

    if (
      this.stylexImports.some((importName) =>
        originalSource.includes(importName),
    )
  ) {
      let metadataStr = "[]";

      const code = originalSource.replace(
        /\/\/*__stylex_metadata_start__(?<metadata>.+)__stylex_metadata_end__/,
        (...args) => {
          metadataStr = args.at(-1)?.metadata.split('"__stylex_metadata_end__')[0];

          return "";
        },
      );

      const metadata = { stylex: [] };

      try {
        metadata.stylex = JSON.parse(metadataStr);
      } catch (e) {
        console.error("error parsing metadata", e);
      }
      const map = null;

      if (metadata.stylex != null && metadata.stylex.length > 0) {
        const oldRules = stylexRules[filename] || [];
        //@ts-expect-error - tmp
        stylexRules[filename] = metadata.stylex?.map((rule) => [rule.class_name, rule.style, rule.priority]);

        logger.debug(`Read stylex styles from ${filename}:`, metadata.stylex);

        // @ts-expect-error - tmp
        const oldClassNames = new Set(oldRules.map((rule) => rule[0]));
        const newClassNames = new Set(metadata.stylex.map((rule) => rule[0]));

        // If there are any new classNames in the output we need to recompile
        // the CSS bundle.
        if (
          oldClassNames.size !== newClassNames.size ||
          [...newClassNames].some(
            (className) => !oldClassNames.has(className),
          ) ||
          filename.endsWith(".stylex.ts") ||
          filename.endsWith(".stylex.tsx") ||
          filename.endsWith(".stylex.js")
        ) {
          compilers.forEach((compiler) => {
            cssFiles.forEach((cssFile) => {
              compiler.watchFileSystem.watcher.fileWatchers
                .get(cssFile)
                .watcher.emit("change");
            });
          });
        }

        return { code, map };
      }

    }
    return { code: inputCode };
  }
}
export default StylexPlugin;

module.exports = StylexPlugin;
