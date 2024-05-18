/**
 * Copyright (c) Meta Platforms, Inc. and affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 *
 *
 */

import WebpackPluginStylex from "./custom-webpack-plugin";

import type { Configuration } from "webpack";
import type { NextConfig } from "next";
import type { ConfigurationContext } from "next/dist/build/webpack/config/utils";
import type { WebpackConfigContext } from "next/dist/server/config-shared";

let count = 0;
function StylexNextJSPlugin({
  rootDir,
  filename = "stylex-bundle.css",
  ...pluginOptions
}: any) {
  return (nextConfig: NextConfig = {}) => {
    return {
      ...nextConfig,
      transpilePackages: [...(nextConfig.transpilePackages || []), "@stylexjs/open-props"],
      webpack(config: Configuration & ConfigurationContext, options: WebpackConfigContext) {
        if (typeof nextConfig.webpack === "function") {
          config = nextConfig.webpack(config, options);
        }

        const { buildId, dev, isServer } = options;

        console.log(
          [
            "!!!GETTING WEBPACK CONFIG!!!",
            "======================",
            `Count: ${++count}`,
            `Build ID: ${buildId}`,
            `Server: ${isServer}`,
            `Env: ${dev ? "dev" : "prod"}`,
          ].join("\n"),
        );

        // @ts-expect-error - tmp
        config.optimization.splitChunks ||= { cacheGroups: {} };

        // @ts-expect-error - tmp
        if (config.optimization.splitChunks?.cacheGroups?.styles) {
          // @ts-expect-error - tmp
          config.optimization.splitChunks.cacheGroups.styles = {
            name: "styles",
            test: /\.css$/,
            chunks: "all",
            enforce: true,
          };
        }

        const webpackPluginOptions = {
          babelConfig: {
            babelrc: true,
            buildId,
            isServer,
            count,
            dev,
          },
          rootDir,
          appendTo: (name: string) => name.endsWith(".css"),
          filename,
          dev,
          ...pluginOptions,
        };

        const stylexPlugin = new WebpackPluginStylex(webpackPluginOptions);
        // @ts-expect-error - tmp
        config.plugins.push(stylexPlugin);

        return config;
      },
    };
  };
}

export default StylexNextJSPlugin;
module.exports = StylexNextJSPlugin;
