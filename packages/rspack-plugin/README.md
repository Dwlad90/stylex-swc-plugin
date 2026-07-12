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

Then import the carrier stylesheet **once** at the entry point of your app
(e.g. `index.js`, `App.tsx`):

```js
import '@stylexswc/rspack-plugin/stylex.css';
```

The plugin appends the extracted StyleX CSS to this asset during
the build. Like a regular CSS file, it must flow through your CSS pipeline, so
a `css-loader` + `CssExtractRspackPlugin.loader` rule has to cover `.css`
files.

The carrier import is a **recommendation, not a hard requirement**. The plugin
also appends tiny per-module CSS imports to every StyleX module, so any part
of the bundle that renders a StyleX component pulls the stylesheet in on its
own — most builds emit correct CSS even without the carrier. What the carrier
adds is a guarantee that doesn't depend on your module graph: the stylesheet
is always present and loaded with the entrypoint. That matters when something
consumes StyleX **output** without rendering a StyleX **component** — plain
CSS reading `defineVars` custom properties (`var(--x…)`), or injected markup
carrying StyleX class names. If styles would actually be lost (no CSS asset
exists to receive them at all), the plugin raises a compilation warning; it
stays silent as long as the output is correct, carrier or not.

> [!IMPORTANT]
> **Migrating from 0.17.x**: version 0.18.0 reworks the CSS extraction
> architecture. The CSS is no longer injected through auto-generated
> `stylex.virtual.css` imports — add the
> `import '@stylexswc/rspack-plugin/stylex.css';` carrier import to your app
> entrypoint (recommended; see above for when you can skip it). The package no
> longer depends on `@stylexswc/webpack-plugin`; shared logic lives in
> `@stylexswc/plugin-shared`. Paths embedded in module identifiers are now
> relative to `compiler.context`, which changes chunk hashes once and makes
> builds reproducible across machines.

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
split into files, cached, or grouped. A custom cache group replaces the
plugin's default one entirely, with standard `splitChunks` semantics — e.g.
omitting `test` matches every module, which funnels all extracted CSS into the
StyleX chunk. Only `name` falls back to the default chunk name; include
`type: 'css/mini-extract'`, `chunks` and `enforce` yourself when you need them.
`name` must be a static string. For webpack compatibility, string and RegExp
shorthand values are treated as `test` and normalized to a Rspack cache group.
`false` disables the plugin's cache group entirely — extracted styles then
have no CSS asset to land in and the build warns; to turn off extraction, use
`extractCSS: false` instead.

### `stylexPackages`

- Type: `string[]`
- Default: `['@stylexjs/']`

`node_modules` path fragments that must be processed by the StyleX loader.

### `carrierCss`

- Type: `string`
- Default: the packaged `@stylexswc/rspack-plugin/stylex.css`

Path to a custom carrier stylesheet that receives the extracted StyleX CSS —
the file you import once at your app entrypoint. Absolute, or relative to
`compiler.context`. Replaces the default packaged carrier: useful when another
file named `stylex.css` in your project would collide with the default
filename pattern, or when you want the carrier to live in your own source
tree.

```js
new StylexPlugin({
  carrierCss: './src/styles/stylex-carrier.css',
});
```

If styles get extracted but no CSS asset is emitted to receive them (e.g. a
custom `cacheGroup` renamed the chunk), the plugin raises a compilation
warning instead of silently dropping the CSS.

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
