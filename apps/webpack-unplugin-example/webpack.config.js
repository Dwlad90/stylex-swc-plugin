/* eslint-disable @typescript-eslint/no-require-imports */
const path = require('node:path')
const HtmlWebpackPlugin = require('html-webpack-plugin')
const styleXRSPlugin = require('@stylexswc/unplugin/webpack')
/* eslint-enable @typescript-eslint/no-require-imports */

const isDev = process.env.NODE_ENV === 'development'

module.exports = {
  context: __dirname,
  mode: isDev ? 'development' : 'production',
  target: 'web',
  cache: true,
  entry: {
    main: './src/index.jsx',
  },
  output: {
    path: path.resolve(__dirname, 'dist'),
    filename: 'bundle.js',
  },
  resolve: {
    extensions: ['.js', '.jsx'],
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
              target: "es2015",
              transform: {
                react: {
                  runtime: 'automatic', // Use 'classic' if you are not using the new JSX transform
                },
              },
            },
          },
        },
      },
    ],
  },
  devServer: {
    hot: true,
    port: 8080,
  },
  performance: {
    hints: false,
    maxEntrypointSize: 512000,
    maxAssetSize: 512000,
  },
  plugins: [
    styleXRSPlugin({
      rsOptions: {
        dev: true,
        useCSSLayers: true,
      },
    }),
    new HtmlWebpackPlugin({
      template: 'public/index.html',
    }),
  ],
}