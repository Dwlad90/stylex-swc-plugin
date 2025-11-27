import styleXRSPlugin from '@stylexswc/unplugin/rspack'
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
    cssFilename: '[name].css',
  },
  resolve: {
    extensions: ['.js', '.jsx'],
  },
  experiments: {
    css: true,
  },
  optimization: {
    minimize: true, // Ensure minification is enabled
    minimizer: [
      // If using a specific CSS minifier plugin
      new rspack.LightningCssMinimizerRspackPlugin({
        // options for cssnano or Lightning CSS
      }),
    ],
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
      {
        test: /\.css$/,
        type: 'css',
      },
    ],
  },
  devServer: {
    hot: true,
    port: 4321,
    // Disable partial CSS updates - always reload full CSS file
    // This ensures HMR CSS includes all styles including those from
    // modules not in the current changed set (like @stylexjs/open-props)
    client: {
      overlay: false,
      // Force refresh on style updates instead of live update
    },
  },
  plugins: [
    styleXRSPlugin({
      useCSSLayers: true,
      useCssPlaceholder: true,
      rsOptions: {
        dev: isDev,
      },
    }),
    new rspack.HtmlRspackPlugin({
      template: 'public/index.html',
    }),
  ],
}