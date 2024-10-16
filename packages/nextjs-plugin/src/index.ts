import WebpackPluginStylex from './custom-webpack-plugin';

import type { Configuration } from 'webpack';
import type { NextConfig } from 'next';
import type { ConfigurationContext } from 'next/dist/build/webpack/config/utils';
import type { WebpackConfigContext } from 'next/dist/server/config-shared';
import type { StyleXOptions } from '@stylexswc/rs-compiler';

let count = 0;

interface StylexNextJSPluginOptions extends StyleXOptions {
  rootDir: string;
  filename?: string;
}

function StylexNextJSPlugin({
  rootDir,
  filename = 'stylex-bundle.css',
  ...pluginOptions
}: StylexNextJSPluginOptions) {
  return (nextConfig: NextConfig = {}) => {
    return {
      ...nextConfig,
      transpilePackages: [...(nextConfig.transpilePackages || []), '@stylexjs/open-props'],
      webpack(config: Configuration & ConfigurationContext, options: WebpackConfigContext) {
        if (typeof nextConfig.webpack === 'function') {
          config = nextConfig.webpack(config, options);
        }

        const { buildId, dev, isServer } = options;

        console.log(
          [
            '!!!GETTING WEBPACK CONFIG!!!',
            '======================',
            `Count: ${++count}`,
            `Build ID: ${buildId}`,
            `Server: ${isServer}`,
            `Env: ${dev ? 'dev' : 'prod'}`,
          ].join('\n')
        );

        if (config.optimization?.splitChunks) {
          config.optimization.splitChunks ||= { cacheGroups: {} };

          if (config.optimization.splitChunks.cacheGroups) {
            config.optimization.splitChunks.cacheGroups.stylex = {
              name: 'stylex',
              chunks: 'all',
              test: /\.css$/,
              enforce: true,
            };
          }
        }

        const webpackPluginOptions = {
          rootDir,
          appendTo: (name: string) => name.endsWith('.css'),
          filename,
          dev,
          rsOptions: pluginOptions,
        };

        const stylexPlugin = new WebpackPluginStylex(webpackPluginOptions);
        config.plugins?.push(stylexPlugin);

        return config;
      },
    };
  };
}

export default StylexNextJSPlugin;
module.exports = StylexNextJSPlugin;
