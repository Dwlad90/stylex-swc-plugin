# SWC Stylex plugin for Next.js

> [!WARNING]
> **Deprecated**: This package is deprecated as of version `0.3.0` and may be removed in the future. Please use the [`nextjs-plugin`](https://github.com/dwlad90/stylex-swc-plugin/tree/develop/packages/nextjs-plugin) instead.

## Breaking Changes in v0.5.0

> [!IMPORTANT]
> The plugin API has been updated since version [0.5.0](https://www.npmjs.com/package/@stylexswc/swc-plugin/v/0.5.0). If you're upgrading from an earlier version, please note that the configuration options have changed. See the [Plugin Options](#plugin-options) section for the updated API.

Next.js plugin for an unofficial
[`StyleX SWC`](https://github.com/dwlad90/stylex-swc-plugin/tree/develop/crates/stylex-swc-plugin)
plugin.

## Why SWC instead of Babel

Since version 12, Next.js uses SWC Compiler by default.
[According to Vercel](https://nextjs.org/docs/architecture/nextjs-compiler),
compilation using the SWC Compiler is 17x faster than Babel.

However, if you have a Babel config, the application will out put of SWC
Compiler and continue to use Babel.

This plugin allows us to use StyleX and take advantage of SWC Compiler.

**The usage of StyleX does not change**, all changes are internal. All you need
to do, is install SWC StyleX plugin and update Next.js config.

## Installation

Install the package and SWC plugin by using:

```bash
npm install --save-dev @stylexswc/nextjs-plugin
```

## Plugin Options

### Basic Options

#### `stylexImports`

- Type: `Array<string | { as: string, from: string }>`
- Default: `['stylex', '@stylexjs/stylex']`
- Description: Specifies where StyleX will be imported from. Supports both
  string paths and import aliases.

#### `useCSSLayers`

- Type: `boolean`
- Default: `false`
- Description: Enables CSS cascade layers support for better style isolation.

### Advanced Options

#### `transformCss`

- Type: `(css: string) => string | Buffer | Promise<string | Buffer>`
- Optional
- Description: Custom CSS transformation function. Since the plugin injects CSS
  after all loaders, use this to apply PostCSS or other CSS transformations.

### SWC Plugin Options

- Type: `Partial<StyleXOptions>`
- Optional
- Description: StyleX compiler options that will be passed to the NAPI-RS
  compiler. See
  [StyleX configuration docs](https://stylexjs.com/docs/api/configuration/babel-plugin/)
  for details.

### Example Configuration

```javascript
const path = require('path');
const stylexPlugin = require('@stylexswc/nextjs-swc-plugin');
const rootDir = __dirname;

module.exports = stylexPlugin({})({
  transpilePackages: ['@stylexjs/open-props'],
  // Optionally, add any other Next.js config below
  swcMinify: true,
  experimental: {
    swcPlugins: [[
      "@stylexswc/swc-plugin",
      {
        dev: process.env.NODE_ENV === 'development',
        genConditionalClasses: true,
        treeshakeCompensation: true,
        aliases: {
          '@/*': [path.join(rootDir, '*')],
        },
        unstable_moduleResolution: {
          type: 'commonJS',
          rootDir: rootDir,
        },
      },
    ]],
  },
});
```

## Examples

- [Example repo](https://github.com/Dwlad90/nextjs-app-dir-stylex/tree/swc)
- [CodeSandbox with example repo](https://codesandbox.io/p/github/Dwlad90/nextjs-app-dir-stylex/swc)

## Documentation

- [StyleX Documentation](https://stylexjs.com)
- [SWC plugin for StyleX](https://github.com/Dwlad90/stylex-swc-plugin/tree/develop/packages/swc-plugin)
