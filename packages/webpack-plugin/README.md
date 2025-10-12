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
- Description: StyleX compiler options that will be passed to the NAPI-RS compiler.
  For standard StyleX options, see the [official StyleX documentation](https://stylexjs.com/docs/api/configuration/babel-plugin/).

> [!NOTE]
> **New Features:** The `include` and `exclude` options are exclusive to this NAPI-RS compiler implementation and are not available in the official StyleX Babel plugin.

##### `rsOptions.include`

- Type: `(string | RegExp)[]`
- Optional
- Description: **[NAPI-RS Only]** An array of glob patterns or regular expressions to include specific files for StyleX transformation.
  When specified, only files matching at least one of these patterns will be transformed.
  Patterns are matched against paths relative to the current working directory.

##### `rsOptions.exclude`

- Type: `(string | RegExp)[]`
- Optional
- Description: **[NAPI-RS Only]** An array of glob patterns or regular expressions to exclude specific files from StyleX transformation.
  Files matching any of these patterns will not be transformed, even if they match an `include` pattern.
  Patterns are matched against paths relative to the current working directory.

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

- Type:
  `(css: string, filePath: string | undefined) => string | Buffer | Promise<string | Buffer>`
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
        // Include only specific directories
        include: ['src/**/*.{ts,tsx}', 'components/**/*.{ts,tsx}'],
        // Exclude test files and stories
        exclude: ['**/*.test.*', '**/*.stories.*', '**/__tests__/**'],
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

#### Path Filtering Examples

**Include only specific directories:**

```javascript
new StylexPlugin({
  rsOptions: {
    include: ['src/**/*.tsx', 'app/**/*.tsx'],
  },
})
```

**Exclude test and build files:**

```javascript
new StylexPlugin({
  rsOptions: {
    exclude: ['**/*.test.*', '**/*.spec.*', '**/dist/**', '**/node_modules/**'],
  },
})
```

**Using regular expressions:**

```javascript
new StylexPlugin({
  rsOptions: {
    include: [/src\/.*\.tsx$/],
    exclude: [/\.test\./, /\.stories\./],
  },
})
```

**Combined include and exclude (exclude takes precedence):**

```javascript
new StylexPlugin({
  rsOptions: {
    include: ['src/**/*.{ts,tsx}'],
    exclude: ['**/__tests__/**', '**/__mocks__/**'],
  },
})
```

**Exclude node_modules except specific packages:**

```javascript
new StylexPlugin({
  rsOptions: {
    // Exclude all node_modules except @stylexjs/open-props
    exclude: [/node_modules(?!\/@stylexjs\/open-props)/],
  },
})
```

**Transform only specific packages from node_modules:**

```javascript
new StylexPlugin({
  rsOptions: {
    include: [
      'src/**/*.{ts,tsx}',
      'node_modules/@stylexjs/open-props/**/*.js',
      'node_modules/@my-org/design-system/**/*.js',
    ],
    exclude: ['**/*.test.*'],
  },
})
```

## Documentation

- [StyleX Documentation](https://stylexjs.com)
- [NAPI-RS compiler for StyleX](https://github.com/Dwlad90/stylex-swc-plugin/tree/develop/crates/stylex-rs-compiler)

## Acknowledgments

This plugin was inspired by
[`stylex-webpack`](https://github.com/SukkaW/stylex-webpack).
