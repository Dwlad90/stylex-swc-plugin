# Rspack plugin with NAPI-RS StyleX compiler integration

> Part of the [StyleX SWC Plugin](https://github.com/Dwlad90/stylex-swc-plugin#readme) workspace

`Rspack plugin` for an unofficial
[`napi-rs`](https://github.com/dwlad90/stylex-swc-plugin/tree/develop/crates/stylex-rs-compiler)
compiler that includes the StyleX SWC code transformation under the hood.

A faithful port of
[`@stylexswc/webpack-plugin`](https://github.com/Dwlad90/stylex-swc-plugin/tree/develop/packages/webpack-plugin):
StyleX rules are extracted through virtual CSS imports and appended to a
dedicated CSS chunk. For Next.js projects use
[`@stylexswc/nextjs-plugin/rspack`](https://github.com/Dwlad90/stylex-swc-plugin/tree/develop/packages/nextjs-plugin),
which wires this plugin into `next-rspack`.

## Installation

To install the package, run the following command:

```bash
npm install --save-dev @stylexswc/rspack-plugin
```

Please install `@stylexswc/rs-compiler` if you haven't done so already:

```bash
npm install --save-dev @stylexswc/rs-compiler
```

## Usage

Modify Rspack config. For example:

```js
const StylexPlugin = require('@stylexswc/rspack-plugin');
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
});

module.exports = config;
```

## Differences from `@stylexswc/webpack-plugin`

Rspack computes loader lists natively, so the plugin registers a static module
rule instead of injecting its loader per module. Two user-visible consequences:

- `loaderOrder` maps to `Rule.enforce`: `'first'` (default) runs the StyleX
  transform before normal loaders (`enforce: 'pre'`), `'last'` runs it after
  (`enforce: 'post'`).
- `node_modules` is **excluded by default**. Rspack invokes JS loaders across a
  native boundary, so touching every module just to bail out is not free like
  it is in webpack. Packages that ship untransformed StyleX source must be
  allowlisted via the `stylexPackages` option (path fragments, default
  `['@stylexjs/']`):

```js
new StylexPlugin({
  stylexPackages: ['@stylexjs/', 'my-design-system'],
});
```

## Plugin Options

The plugin accepts the same options as
[`@stylexswc/webpack-plugin`](https://github.com/Dwlad90/stylex-swc-plugin/tree/develop/packages/webpack-plugin#plugin-options),
plus:

### `stylexPackages`

- Type: `string[]`
- Default: `['@stylexjs/']`

`node_modules` path fragments that must be processed by the StyleX loader.

## Documentation

- [StyleX Documentation](https://stylexjs.com)
- [SWC Documentation](https://swc.rs)
- [Rspack Documentation](https://rspack.rs)
