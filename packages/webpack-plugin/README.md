# Webpack plugin with NAPI-RS StyleX compiler integration

`Webpack plugin` for an unofficial
[`napi-rs`](https://github.com/dwlad90/stylex-swc-plugin/tree/master/crates/stylex-rs-compiler)
compiler that includes the StyleX SWC code transformation under the hood.

## Installation

To install the package, run the following command:

```bash
npm install --save-dev @stylexswc/webpack-plugin
```

Please install `@stylexswc/rs-compiler` if you haven't done so already:

```bash
npm install --save-dev @stylexswc/rs-compiler
```

## Usage

Modify Webpack config. For example:

```js
const StylexPlugin = require('@stylexswc/webpack-plugin');
const path = require('path');

const config = (env, argv) => ({
  entry: {
    main: './js/index.js',
  },
  output: {
    path: path.resolve(__dirname, 'dist'),
    filename: '[name].js',
  },
  plugins: [
    new StylexPlugin({
      filename: 'styles.[contenthash].css',
      dev: argv.mode === 'development',
    }),
  ],
  cache: true,
});

module.exports = config;
```

## Plugin Options
The options are similar like `@stylexjs/babel-plugin` and can be found [here](https://stylexjs.com/docs/api/configuration/babel-plugin/)

## Documentation

* [StyleX Documentation](https://stylexjs.com)
* [NAPI-RS compiler for StyleX](https://github.com/Dwlad90/stylex-swc-plugin/tree/master/crates/stylex-rs-compiler)
