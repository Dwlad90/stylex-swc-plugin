"use strict";
var __importDefault = (this && this.__importDefault) || function (mod) {
    return (mod && mod.__esModule) ? mod : { "default": mod };
};
Object.defineProperty(exports, "__esModule", { value: true });
const path_1 = __importDefault(require("path"));
const babel_plugin_1 = __importDefault(require("@stylexjs/babel-plugin"));
const webpack_1 = __importDefault(require("webpack"));
const promises_1 = __importDefault(require("fs/promises"));
const { NormalModule, Compilation } = webpack_1.default;
const PLUGIN_NAME = "stylex";
const IS_DEV_ENV = process.env.NODE_ENV === "development" ||
    process.env.BABEL_ENV === "development";
const { RawSource, ConcatSource } = webpack_1.default.sources;
const stylexRules = {};
const cssFiles = new Set();
const compilers = new Set();
class StylexPlugin {
    filesInLastRun = null;
    filePath = null;
    dev;
    appendTo;
    filename;
    babelConfig;
    stylexImports;
    babelPlugin;
    useCSSLayers;
    constructor({ dev = IS_DEV_ENV, appendTo, filename = appendTo == null ? "stylex.css" : undefined, stylexImports = ["stylex", "@stylexjs/stylex"], useCSSLayers = false, } = {}) {
        this.dev = dev;
        this.appendTo = appendTo;
        this.filename = filename;
        this.stylexImports = stylexImports;
        this.useCSSLayers = useCSSLayers;
    }
    apply(compiler) {
        compiler.hooks.make.tap(PLUGIN_NAME, (compilation) => {
            // Apply loader to JS modules.
            NormalModule.getCompilationHooks(compilation).loader.tap(PLUGIN_NAME, (_, module) => {
                if (
                // JavaScript (and Flow) modules
                /\.jsx?/.test(path_1.default.extname(module.resource)) ||
                    // TypeScript modules
                    /\.tsx?/.test(path_1.default.extname(module.resource))) {
                    // We use .push() here instead of .unshift()
                    // Webpack usually runs loaders in reverse order and we want to ideally run
                    // our loader before anything else.
                    module.loaders.unshift({
                        loader: path_1.default.resolve(__dirname, "custom-webpack-loader.js"),
                        options: { stylexPlugin: this },
                        ident: null,
                        type: null,
                    });
                }
                if (
                // JavaScript (and Flow) modules
                /\.css/.test(path_1.default.extname(module.resource))) {
                    cssFiles.add(module.resource);
                }
            });
            const getStyleXRules = () => {
                if (Object.keys(stylexRules).length === 0) {
                    return null;
                }
                // Take styles for the modules that were included in the last compilation.
                const allRules = Object.keys(stylexRules)
                    .map((filename) => stylexRules[filename])
                    .filter(Boolean)
                    .flat();
                return babel_plugin_1.default.processStylexRules(allRules, this.useCSSLayers);
            };
            if (this.appendTo) {
                compilation.hooks.processAssets.tap({
                    name: PLUGIN_NAME,
                    stage: Compilation.PROCESS_ASSETS_STAGE_PRE_PROCESS, // see below for more stages
                }, (assets) => {
                    const cssFileName = Object.keys(assets).find(typeof this.appendTo === "function"
                        ? this.appendTo
                        : (filename) => filename.endsWith(this.appendTo));
                    const stylexCSS = getStyleXRules();
                    if (cssFileName && stylexCSS != null) {
                        this.filePath = path_1.default.join(process.cwd(), ".next", cssFileName);
                        const source = assets?.[cssFileName]?.source();
                        if (source) {
                            const updatedSource = new ConcatSource(new RawSource(source), new RawSource(stylexCSS));
                            compilation.updateAsset(cssFileName, updatedSource);
                            compilers.add(compiler);
                        }
                    }
                });
            }
            else {
                // Consume collected rules and emit the stylex CSS asset
                compilation.hooks.additionalAssets.tap(PLUGIN_NAME, () => {
                    try {
                        const collectedCSS = getStyleXRules();
                        if (collectedCSS) {
                            console.log("emitting asset", this.filename, collectedCSS);
                            compilation.emitAsset(this.filename, new RawSource(collectedCSS));
                            promises_1.default.writeFile(this.filename, collectedCSS).then(() => console.log("wrote file", this.filename));
                        }
                    }
                    catch (e) {
                        compilation.errors.push(e);
                    }
                });
            }
        });
    }
    // This function is not called by Webpack directly.
    // Instead, `NormalModule.getCompilationHooks` is used to inject a loader
    // for JS modules. The loader than calls this function.
    async transformCode(inputCode, filename, logger) {
        const originalSource = inputCode;
        if (inputCode.includes("Welcome to my MDX page"))
            console.log("originalSource: ", originalSource);
        if (this.stylexImports.some((importName) => originalSource.includes(importName))) {
            let metadataStr = "[]";
            const code = originalSource.replace(/\/\/*__stylex_metadata_start__(?<metadata>.+)__stylex_metadata_end__/, (...args) => {
                metadataStr = args
                    .at(-1)
                    ?.metadata.split('"__stylex_metadata_end__')[0];
                return "";
            });
            const metadata = { stylex: [] };
            try {
                metadata.stylex = JSON.parse(metadataStr);
            }
            catch (e) {
                console.error("error parsing metadata", e);
            }
            const map = null;
            if (metadata.stylex != null && metadata.stylex.length > 0) {
                const oldRules = stylexRules[filename] || [];
                stylexRules[filename] = metadata.stylex?.map((rule) => [rule.class_name, rule.style, rule.priority]);
                logger.debug(`Read stylex styles from ${filename}:`, metadata.stylex);
                const oldClassNames = new Set(oldRules.map((rule) => rule[0]));
                const newClassNames = new Set(metadata.stylex.map((rule) => rule[0]));
                // If there are any new classNames in the output we need to recompile
                // the CSS bundle.
                if (oldClassNames.size !== newClassNames.size ||
                    [...newClassNames].some((className) => !oldClassNames.has(className)) ||
                    filename.endsWith(".stylex.ts") ||
                    filename.endsWith(".stylex.tsx") ||
                    filename.endsWith(".stylex.js")) {
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
exports.default = StylexPlugin;
module.exports = StylexPlugin;
