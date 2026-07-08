# @stylexswc/rspack-plugin

> StyleX plugin for Rspack, powered by a Rust compiler (NAPI-RS + SWC). Part of
> the [StyleX SWC Plugin](https://github.com/Dwlad90/stylex-swc-plugin#readme)
> workspace.

This plugin compiles [StyleX](https://stylexjs.com) code in your Rspack build
with
[`@stylexswc/rs-compiler`](https://www.npmjs.com/package/@stylexswc/rs-compiler),
a Rust implementation of the StyleX transform, instead of the official Babel
plugin. Your StyleX code stays exactly the same — only the build step changes,
with per-file transforms 2x to 5x faster than Babel
([performance](https://github.com/Dwlad90/stylex-swc-plugin#performance)).
StyleX rules are extracted through virtual CSS imports and appended to a
dedicated CSS chunk.

This is a community project and is not affiliated with Meta. It tracks the
official StyleX releases
<!-- stylex-compatibility:start -->(currently compatible with StyleX v0.19.0)<!-- stylex-compatibility:end -->
and requires Node.js 20 or newer. For Next.js projects, use
[`@stylexswc/nextjs-plugin/rspack`](https://www.npmjs.com/package/@stylexswc/nextjs-plugin),
which wires this plugin into `next-rspack` for you.

## Installation

```bash
npm install --save-dev @stylexswc/rspack-plugin
```

The Rust compiler (`@stylexswc/rs-compiler`) is installed automatically as a
dependency. Your application still needs the StyleX runtime:

```bash
npm install @stylexjs/stylex
```

## Usage

Add the plugin to your Rspack config:

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
      rsOptions: {
        dev: argv.mode === 'development',
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
- `node_modules` is excluded by default. Rspack invokes JS loaders across a
  native boundary, so touching every module just to bail out is not free.
  Packages that ship untransformed StyleX source must be allowlisted via the
  `stylexPackages` option (path fragments, default `['@stylexjs/']`):

```js
new StylexPlugin({
  stylexPackages: ['@stylexjs/', 'my-design-system'],
});
```

### Source maps

The StyleX loader automatically forwards the previous loader's source map to the
compiler as `inputSourceMap`. With `loaderOrder: 'last'` — where the loader
receives code already rewritten by earlier loaders — this keeps debug source-map
annotations (`debug: true`) and the emitted source map pointing at the original
authored file. See the
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

Specify where StyleX is imported from.

### `useCSSLayers`

- Type: `UseLayersType`
- Default: `false`

Whether to wrap the generated CSS in cascade layers.

### `nextjsMode`

- Type: `boolean`
- Default: `false`

Enable when the plugin is driven by the Next.js integration.

### `transformCss`

- Type:
  `(css: string, filePath: string | undefined) => string | Buffer | Promise<string | Buffer>`

Post-process the extracted CSS. The plugin injects CSS after all loaders have
run, so `postcss-loader` cannot be used on it — invoke `postcss()` here instead:

```js
new StylexPlugin({
  transformCss: async (css, filePath) => {
    const postcss = require('postcss');
    const result = await postcss([require('autoprefixer')]).process(css, {
      from: filePath,
    });
    return result.css;
  },
});
```

### `extractCSS`

- Type: `boolean`
- Default: `true`

Whether to extract CSS into the dedicated StyleX chunk.

### `loaderOrder`

- Type: `'first' | 'last'`
- Default: `'first'`

When the StyleX transformation runs relative to other Rspack loaders — see
[Loader behavior](#loader-behavior).

### `cacheGroup`

- Type:
  [`splitChunks.cacheGroups` entry](https://rspack.rs/plugins/webpack/split-chunks-plugin#splitchunkscachegroups)

Customizes the cache group configuration for extracted CSS chunks — how CSS is
split into files, cached, or grouped.

### `stylexPackages`

- Type: `string[]`
- Default: `['@stylexjs/']`

`node_modules` path fragments that must be processed by the StyleX loader.

## FAQ

### Do I still need `@stylexjs/babel-plugin`?

No. This plugin replaces the Babel plugin in your build. You only keep
`@stylexjs/stylex` as your app's runtime dependency, and your `stylex.create` /
`stylex.props` code does not change.

### My styles from a component library are missing. Why?

`node_modules` is excluded by default for performance. Add the package to
`stylexPackages` (for example
`stylexPackages: ['@stylexjs/', 'my-design-system']`) so the StyleX loader
processes it.

### How is this different from `@stylexswc/webpack-plugin`?

Same options, same compiler — but this package registers its loader through
Rspack's native rule system instead of patching the webpack loader chain, which
is both faster and more predictable in Rspack.

### Can I use this with Next.js?

Yes, through `@stylexswc/nextjs-plugin/rspack`, which composes this plugin with
the `next-rspack` adapter for you.

### Is this an official StyleX package?

No. It is a community-maintained alternative to the official tooling and is not
affiliated with or supported by Meta.

## Documentation

- [StyleX documentation](https://stylexjs.com)
- [Rspack documentation](https://rspack.rs)
- [`@stylexswc/rs-compiler` compiler options](https://github.com/Dwlad90/stylex-swc-plugin/tree/develop/crates/stylex-rs-compiler)

## License

MIT — see
[LICENSE](https://github.com/Dwlad90/stylex-swc-plugin/blob/develop/LICENSE)
