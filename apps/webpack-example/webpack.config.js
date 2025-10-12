/* eslint-disable @typescript-eslint/no-require-imports */
const StylexPlugin = require('@stylexswc/webpack-plugin');
const path = require('path');
const HtmlWebpackPlugin = require('html-webpack-plugin');
const ReactRefreshWebpackPlugin = require('@pmmmwh/react-refresh-webpack-plugin');
const MiniCssExtractPlugin = require('mini-css-extract-plugin');
/* eslint-enable @typescript-eslint/no-require-imports */

const config = (env, argv) => {
  const isHot = argv.hot;
  const isDev = argv.mode === 'development';

  return {
    entry: {
      main: path.resolve(__dirname, 'src/index.js'),
    },
    output: {
      path: path.resolve(__dirname, './dist'),
    },
    devServer: {
      static: {
        directory: path.resolve(__dirname, 'dist'),
      },
    },
    module: {
      rules: [
        {
          test: /\.(js|jsx)$/,
          exclude: /node_modules/,
          use: {
            loader: 'swc-loader',
            options: {
              jsc: {
                parser: {
                  syntax: 'ecmascript',
                  jsx: true,
                },
                target: 'es2015',
                transform: {
                  react: {
                    runtime: 'automatic',
                    refresh: isDev,
                    development: isDev,
                  },
                },
              },
            },
          },
        },
        {
          test: /\.(css)$/,
          use: [MiniCssExtractPlugin.loader, 'css-loader', 'postcss-loader'],
        },
        {
          test: /\.svg$/i,
          issuer: /\.[jt]sx?$/,
          use: ['file-loader'],
        },
      ],
    },
    resolve: {
      extensions: ['*', '.js', '.jsx'],
    },
    plugins: [
      new StylexPlugin({
        // get webpack mode and set value for dev
        dev: isDev,
        // See all options in the babel plugin configuration docs:
        // https://stylexjs.com/docs/api/configuration/babel-plugin/
        rsOptions: {}
      }),
      new HtmlWebpackPlugin({
        inject: true,
        template: path.resolve(__dirname, 'index.html'),
      }),
      new MiniCssExtractPlugin(),
      isHot && new ReactRefreshWebpackPlugin(),
    ].filter(Boolean),
    cache: true,
  };
};

module.exports = config;
