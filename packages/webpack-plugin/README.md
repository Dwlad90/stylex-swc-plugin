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
      // ... Other StyleX options
      transformCss: async (css, filePath) => {
        const postcss = require('postcss');
        const result = await postcss([require('autoprefixer')]).process(css, {
          from: filePath,
          map: {
            inline: false,
            annotation: false,
          },
        });
        return result.css;
      },
    }),
  ],
  cache: true,
});

module.exports = config;
```

## Plugin Options

### Basic Options

#### `rsOptions`

- Type: `Partial<StyleXOptions>`
- Optional
- Description: StyleX compiler options that will be passed to the NAPI-RS
  compiler. See
  [StyleX configuration docs](https://stylexjs.com/docs/api/configuration/babel-plugin/)
  for details.

#### `stylexImports`

- Type: `Array<string | { as: string, from: string }>`
- Default: `['stylex', '@stylexjs/stylex']`
- Description: Specifies where StyleX will be imported from. Supports both
  string paths and import aliases.

#### `useCSSLayers`

- Type: `boolean`
- Default: `false`
- Description: Enables CSS cascade layers support for better style isolation.

#### `nextjsMode`

- Type: `boolean`
- Default: `false`
- Description: Enables Next.js-specific optimizations and compatibility
  features.

#### `extractCSS`

- Type: `boolean`
- Optional
- Default: `true`
- Description: Controls whether CSS should be extracted into a separate file

### Advanced Options

#### `transformCss`

- Type: `(css: string, filePath: string | undefined) => string | Buffer | Promise<string | Buffer>`
- Optional
- Description: Custom CSS transformation function. Since the plugin injects CSS
  after all loaders, use this to apply PostCSS or other CSS transformations.

### Example Configuration

```javascript
const StylexPlugin = require('@stylexswc/webpack-plugin');

module.exports = {
  plugins: [
    new StylexPlugin({
      rsOptions: {
        dev: process.env.NODE_ENV !== 'production',
        useRemForFontSize: true,
      },
      stylexImports: ['@stylexjs/stylex', { from: './theme', as: 'tokens' }],
      useCSSLayers: true,
      nextjsMode: false,
      transformCss: async css => {
        const postcss = require('postcss');
        const result = await postcss([require('autoprefixer')]).process(css);
        return result.css;
      },
    }),
  ],
};
```

## Documentation

- [StyleX Documentation](https://stylexjs.com)
- [NAPI-RS compiler for StyleX](https://github.com/Dwlad90/stylex-swc-plugin/tree/develop/crates/stylex-rs-compiler)

## Acknowledgments

This plugin was inspired by
[`stylex-webpack`](https://github.com/SukkaW/stylex-webpack).
