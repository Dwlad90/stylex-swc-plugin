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
- Description: StyleX compiler options that extend from `@stylexswc/rs-compiler`.
  For standard StyleX options, see the [official StyleX documentation](https://stylexjs.com/docs/api/configuration/babel-plugin/).

> [!NOTE]
> **New Features:** The `include` and `exclude` options are exclusive to this NAPI-RS compiler implementation and are not available in the official StyleX Babel plugin.

#### `rsOptions.include`

- Type: `(string | RegExp)[]`
- Optional
- Description: **RS-compiler Only** An array of glob patterns or regular expressions to include specific files for StyleX transformation.
  When specified, only files matching at least one of these patterns will be transformed.
  Patterns are matched against paths relative to the current working directory.
  Supports regex lookahead/lookbehind for advanced filtering.

#### `rsOptions.exclude`

- Type: `(string | RegExp)[]`
- Optional
- Description: **RS-compiler Only** An array of glob patterns or regular expressions to exclude specific files from StyleX transformation.
  Files matching any of these patterns will not be transformed, even if they match an `include` pattern.
  Patterns are matched against paths relative to the current working directory.
  Supports regex lookahead/lookbehind for advanced filtering.

### Path Filtering Examples

**Include only specific directories:**

```javascript
stylexPlugin({
  rsOptions: {
    include: ['src/**/*.{ts,tsx,js,jsx}'],
  },
})
```

**Exclude test and build files:**

```javascript
stylexPlugin({
  rsOptions: {
    exclude: ['**/*.test.*', '**/*.spec.*', '**/dist/**'],
  },
})
```

**Using RegExp with lookahead/lookbehind - exclude node_modules except specific packages:**

```javascript
stylexPlugin({
  rsOptions: {
    // Exclude all node_modules except @stylexjs packages
    exclude: [/node_modules(?!\/@stylexjs)/],
  },
})
```

**Transform only specific packages from node_modules:**

```javascript
stylexPlugin({
  rsOptions: {
    include: [
      'src/**/*.{ts,tsx,js,jsx}',
      'node_modules/@stylexjs/open-props/**/*.js',
    ],
    exclude: ['**/*.test.*'],
  },
})
```

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
