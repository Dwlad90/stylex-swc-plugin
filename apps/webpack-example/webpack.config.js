const StylexPlugin = require('@toss/stylexswc-webpack-plugin');
const path = require('node:path');
const MiniCssExtractPlugin = require('mini-css-extract-plugin');


const config = (env, argv) => ({
  entry: {
    main: './js/index.js',
  },
  output: {
    path: path.resolve(__dirname, 'dist'),
    filename: '[name].js',
  },
  module: {
    rules: [
      // Just like your normal CSS setup, a css-loader and MiniCssExtractPlugin.loader
      {
        test: /\.css$/i,
        use: [MiniCssExtractPlugin.loader, 'css-loader'],
        // NOTE: SideEffect is required to css extraction if not imported real css file into the entry code.
        sideEffects: true
      }
    ]
  },
  plugins: [
    new StylexPlugin({
      // get webpack mode and set value for dev
      dev: argv.mode === 'development',
      // See all options in the babel plugin configuration docs:
      // https://stylexjs.com/docs/api/configuration/babel-plugin/
      rsOptions: {}
    }),
    new MiniCssExtractPlugin(),
  ],
  cache: true,
});

module.exports = config;
