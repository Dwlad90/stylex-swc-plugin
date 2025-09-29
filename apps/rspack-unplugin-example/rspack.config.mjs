import styleXRSPlugin from '@toss/stylexswc-unplugin/rspack'
import path from 'node:path'
import rspack from '@rspack/core'

const isDev = process.env.NODE_ENV === 'development'

const dirname = process.cwd()

export default {
  context: dirname,
  mode: isDev ? 'development' : 'production',
  target: 'web',
  cache: true,
  entry: {
    main: './src/index.jsx',
  },
  output: {
    path: path.resolve(dirname, 'dist'),
    filename: 'bundle.js',
  },
  resolve: {
    extensions: ['.js', '.jsx'],
  },
  module: {
    rules: [
      {
        test: /\.jsx?$/,
        loader: 'builtin:swc-loader',
        options: {
          jsc: {
            parser: {
              syntax: 'ecmascript',
              jsx: true,
            },
            target: "es2015",
            transform: {
              react: {
                runtime: 'automatic',
              },
            },
          },
          isModule: 'unknown',
        },
        type: 'javascript/auto',
      },
    ],
  },
  devServer: {
    hot: true,
    port: 4321,
  },
  plugins: [
    styleXRSPlugin({
      rsOptions: {
        dev: isDev,
        useCSSLayers: true,
      },
    }),
    new rspack.HtmlRspackPlugin({
      template: 'public/index.html',
    }),
  ],
}