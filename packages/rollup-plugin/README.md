# Webpack plugin with NAPI-RS StyleX compiler integration

`Webpack plugin` for an unofficial
[`napi-rs`](https://github.com/dwlad90/stylex-swc-plugin/tree/develop/crates/stylex-rs-compiler)
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

The plugin accepts the following configuration options:

### `rsOptions`

- Type: `StyleXOptions`
- Optional
- Default: `{}`
- Description: StyleX compiler options that extend from `@stylexswc/rs-compiler`

### `fileName`

- Type: `string`
- Optional
- Default: `'stylex.css'`
- Description: Name of the output CSS file

### `useCSSLayers`

- Type: `boolean`
- Optional
- Default: `false`
- Description: Enable CSS Layers support for better style isolation

### `lightningcssOptions`

- Type: `TransformOptions`
- Optional
- Description: LightningCSS transform options (excluding code, filename, and
  visitor properties)

### `extractCSS`

- Type: `boolean`
- Optional
- Default: `true`
- Description: Controls whether CSS should be extracted into a separate file

## Documentation

- [StyleX Documentation](https://stylexjs.com)
- [NAPI-RS compiler for StyleX](https://github.com/Dwlad90/stylex-swc-plugin/tree/develop/crates/stylex-rs-compiler)
