# Next.js plugin with NAPI-RS StyleX compiler integration

Next.js plugin for an unofficial
[`napi-rs`](https://github.com/dwlad90/stylex-swc-plugin/tree/develop/crates/stylex-rs-compiler)
compiler that includes the StyleX SWC code transformation under the hood.

## Breaking Changes in v0.5.0

> [!IMPORTANT]
> The plugin API has been updated since version
> [0.5.0](https://www.npmjs.com/package/@stylexswc/swc-plugin/v/0.5.0). If
> you're upgrading from an earlier version, please note that the configuration
> options have changed. See the [Plugin Options](#plugin-options) section for
> the updated API.

## Overview

This package combines two solutions to enhance your Next.js development
experience with StyleX:

### StyleX SWC Plugin

- Integrates StyleX with the SWC compiler, potentially leading to faster build
  times compared to using Babel.
- Maintains high compatibility with official StyleX tests, ensuring a reliable
  experience.
- Integrates seamlessly with Next.js SWC Compiler for a streamlined workflow.

### StyleX NAPI-RS Compiler

- Utilizes NAPI-RS to compile StyleX code, offering advantages over the SWC
  plugin approach.
- Provides access to StyleX metadata and source maps, enabling advanced plugin
  and tool development.

## Why choose this approach?

- Leverage SWC's speed: Benefit from Next.js's default SWC compiler for
  potentially faster build times.
- Maintain StyleX familiarity: The usage of StyleX remains unchanged for
  developers.

## Installation

To install the package, run the following command:

```bash
npm install --save-dev @stylexswc/nextjs-plugin
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

#### `extractCSS`

- Type: `boolean`
- Optional
- Default: `true`
- Description: Controls whether CSS should be extracted into a separate file

### Advanced Options

#### `transformCss`

- Type: `(css: string) => string | Buffer | Promise<string | Buffer>`
- Optional
- Description: Custom CSS transformation function. Since the plugin injects CSS
  after all loaders, use this to apply PostCSS or other CSS transformations.

### Example Configuration

```javascript
const path = require('path');
const stylexPlugin = require('@stylexswc/nextjs-plugin');
const rootDir = __dirname;

module.exports = stylexPlugin({
  // Add any StyleX options here
  rsOptions: {
    dev: process.env.NODE_ENV !== 'production',
    useRemForFontSize: true,
    aliases: {
      '@/*': [path.join(rootDir, '*')],
    },
    unstable_moduleResolution: {
      type: 'commonJS',
    },
  },
  stylexImports: ['@stylexjs/stylex', { from: './theme', as: 'tokens' }],
  useCSSLayers: true,
  transformCss: async css => {
    const postcss = require('postcss');
    const result = await postcss([require('autoprefixer')]).process(css);
    return result.css;
  },
})({
  transpilePackages: ['@stylexjs/open-props'],
  // Optionally, add any other Next.js config below
});
```

## Examples

- [Example repo](https://github.com/Dwlad90/nextjs-app-dir-stylex)
- [CodeSandbox with example repo](https://codesandbox.io/p/github/Dwlad90/nextjs-app-dir-stylex/main)

## Documentation

- [StyleX Documentation](https://stylexjs.com)
- [NAPI-RS compiler for StyleX](https://github.com/Dwlad90/stylex-swc-plugin/tree/develop/crates/stylex-rs-compiler)

## Acknowledgments

This plugin was inspired by
[`stylex-webpack`](https://github.com/SukkaW/stylex-webpack).
