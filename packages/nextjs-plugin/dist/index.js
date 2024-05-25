"use strict";
/**
 * Copyright (c) Meta Platforms, Inc. and affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 *
 *
 */
var __importDefault = (this && this.__importDefault) || function (mod) {
    return (mod && mod.__esModule) ? mod : { "default": mod };
};
Object.defineProperty(exports, "__esModule", { value: true });
const custom_webpack_plugin_1 = __importDefault(require("./custom-webpack-plugin"));
let count = 0;
function StylexNextJSPlugin({ rootDir, filename = "stylex-bundle.css", ...pluginOptions }) {
    return (nextConfig = {}) => {
        return {
            ...nextConfig,
            transpilePackages: [
                ...(nextConfig.transpilePackages || []),
                "@stylexjs/open-props",
            ],
            webpack(config, options) {
                if (typeof nextConfig.webpack === "function") {
                    config = nextConfig.webpack(config, options);
                }
                const { buildId, dev, isServer } = options;
                console.log([
                    "!!!GETTING WEBPACK CONFIG!!!",
                    "======================",
                    `Count: ${++count}`,
                    `Build ID: ${buildId}`,
                    `Server: ${isServer}`,
                    `Env: ${dev ? "dev" : "prod"}`,
                ].join("\n"));
                if (config.optimization?.splitChunks) {
                    config.optimization.splitChunks ||= { cacheGroups: {} };
                    if (config.optimization.splitChunks.cacheGroups) {
                        config.optimization.splitChunks.cacheGroups.stylex = {
                            name: "stylex",
                            chunks: "all",
                            enforce: true,
                        };
                    }
                }
                const webpackPluginOptions = {
                    rootDir,
                    appendTo: (name) => name.endsWith(".css"),
                    filename,
                    dev,
                    ...pluginOptions,
                };
                const stylexPlugin = new custom_webpack_plugin_1.default(webpackPluginOptions);
                config.plugins?.push(stylexPlugin);
                return config;
            },
        };
    };
}
exports.default = StylexNextJSPlugin;
module.exports = StylexNextJSPlugin;
