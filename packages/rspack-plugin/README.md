# Rspack plugin with NAPI-RS StyleX compiler integration

> Part of the [StyleX SWC Plugin](https://github.com/Dwlad90/stylex-swc-plugin#readme) workspace

`Rspack plugin` for an unofficial
[`napi-rs`](https://github.com/dwlad90/stylex-swc-plugin/tree/develop/crates/stylex-rs-compiler)
compiler that includes the StyleX SWC code transformation under the hood.

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

## Loader behavior

Rspack computes loader lists natively, so the plugin registers a static module
rule with its StyleX loader. Two user-visible consequences:

- `loaderOrder` maps to `Rule.enforce`: `'first'` (default) runs the StyleX
  transform before normal loaders (`enforce: 'pre'`), `'last'` runs it after
  (`enforce: 'post'`).
- `node_modules` is **excluded by default**. Rspack invokes JS loaders across a
  native boundary, so touching every module just to bail out is not free.
  Packages that ship untransformed StyleX source must be allowlisted via the
  `stylexPackages` option (path fragments, default `['@stylexjs/']`):

```js
new StylexPlugin({
  stylexPackages: ['@stylexjs/', 'my-design-system'],
});
```

### Source maps

The StyleX loader automatically forwards the previous loader's source map to
the compiler as `inputSourceMap`. With `loaderOrder: 'last'` — where the
loader receives code already rewritten by earlier loaders — this keeps debug
source-map annotations (`debug: true`) and the emitted source map pointing at
the original authored file. See the
[`inputSourceMap` compiler option](https://github.com/Dwlad90/stylex-swc-plugin/tree/develop/crates/stylex-rs-compiler#inputsourcemap)
for details.

## Plugin Options

### `rsOptions`

- Type: `Partial<StyleXOptions>`

StyleX compiler options passed to
[`@stylexswc/rs-compiler`](https://github.com/Dwlad90/stylex-swc-plugin/tree/develop/crates/stylex-rs-compiler).
See the
[StyleX configuration docs](https://stylexjs.com/docs/api/configuration/babel-plugin/)
for the shared option semantics.

### `stylexImports`

- Type: `StyleXOptions['importSources']`
- Default: `['stylex', '@stylexjs/stylex']`

Specify where StyleX will be imported from.

### `useCSSLayers`

- Type: `UseLayersType`
- Default: `false`

Whether to use CSS layers.

### `nextjsMode`

- Type: `boolean`
- Default: `false`

Enable when the plugin is driven by the Next.js integration.

### `transformCss`

- Type:
  `(css: string, filePath: string | undefined) => string | Buffer | Promise<string | Buffer>`

Post-process the extracted CSS. Since the plugin only injects CSS after all
loaders, `postcss-loader` cannot be used on it — invoke `postcss()` here
instead.

### `extractCSS`

- Type: `boolean`
- Default: `true`

Whether to extract CSS into the dedicated StyleX chunk.

### `loaderOrder`

- Type: `'first' | 'last'`
- Default: `'first'`

When the StyleX transformation is applied relative to other rspack loaders —
see [Loader behavior](#loader-behavior).

### `cacheGroup`

- Type:
  [`splitChunks.cacheGroups` entry](https://rspack.rs/plugins/webpack/split-chunks-plugin#splitchunkscachegroups)

Customizes the cache group configuration for extracted CSS chunks — how CSS is
split into files, cached, or grouped.

### `stylexPackages`

- Type: `string[]`
- Default: `['@stylexjs/']`

`node_modules` path fragments that must be processed by the StyleX loader.

## Documentation

- [StyleX Documentation](https://stylexjs.com)
- [SWC Documentation](https://swc.rs)
- [Rspack Documentation](https://rspack.rs)
