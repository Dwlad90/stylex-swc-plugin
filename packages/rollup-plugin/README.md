# @stylexswc/rollup-plugin

> StyleX plugin for Rollup, powered by a Rust compiler (NAPI-RS + SWC). Part of
> the [StyleX SWC Plugin](https://github.com/Dwlad90/stylex-swc-plugin#readme)
> workspace.

This plugin compiles [StyleX](https://stylexjs.com) code in your Rollup build
with
[`@stylexswc/rs-compiler`](https://www.npmjs.com/package/@stylexswc/rs-compiler),
a Rust implementation of the StyleX transform, instead of the official Babel
plugin. Your StyleX code stays exactly the same — only the build step changes,
with per-file transforms 2x to 5x faster than Babel
([performance](https://github.com/Dwlad90/stylex-swc-plugin#performance)).
The extracted CSS is processed with [Lightning CSS](https://lightningcss.dev)
before it is written to disk.

This is a community project and is not affiliated with Meta. It tracks the
official StyleX releases
<!-- stylex-compatibility:start -->(currently compatible with StyleX v0.19.0)<!-- stylex-compatibility:end -->
and requires Node.js 20 or newer.

## Installation

```bash
npm install --save-dev @stylexswc/rollup-plugin
```

The Rust compiler (`@stylexswc/rs-compiler`) is installed automatically as a
dependency. Your application still needs the StyleX runtime:

```bash
npm install @stylexjs/stylex
```

## Usage

Add the plugin to your Rollup config:

```js
// rollup.config.mjs
import stylexPlugin from '@stylexswc/rollup-plugin';

export default {
  input: 'src/index.js',
  output: {
    file: 'dist/bundle.js',
    format: 'cjs',
  },
  plugins: [
    stylexPlugin.default({
      fileName: 'stylex.css',
      rsOptions: {
        dev: process.env.NODE_ENV !== 'production',
      },
    }),
  ],
};
```

> [!NOTE]
> In an ESM config file the plugin is exposed on the `default` property
> because the package is compiled to CommonJS — hence
> `stylexPlugin.default(...)`. With a CommonJS config
> (`const stylexPlugin = require('@stylexswc/rollup-plugin')`), call
> `stylexPlugin(...)` directly.

A complete working setup, including JSX handling via `@rollup/plugin-swc`, lives
in the
[`rollup-example` app](https://github.com/Dwlad90/stylex-swc-plugin/tree/develop/apps/rollup-example).

## Plugin Options

### `rsOptions`

- Type: `StyleXOptions`
- Optional
- Default: `{}`
- Description: StyleX compiler options passed to `@stylexswc/rs-compiler`. For
  the standard options, see the
  [official StyleX documentation](https://stylexjs.com/docs/api/configuration/babel-plugin/).

> [!NOTE]
> The `include` and `exclude` options are exclusive to the Rust compiler
> and are not available in the official StyleX Babel plugin.

#### `rsOptions.include`

- Type: `(string | RegExp)[]`
- Optional
- Description: Glob patterns or regular expressions selecting the files to
  transform. When specified, only files matching at least one pattern are
  transformed. Patterns are matched against paths relative to the current
  working directory. Regular expressions support lookahead and lookbehind.

#### `rsOptions.exclude`

- Type: `(string | RegExp)[]`
- Optional
- Description: Glob patterns or regular expressions excluding files from the
  transform. A file matching any exclude pattern is skipped even if it matches
  an `include` pattern. Patterns are matched against paths relative to the
  current working directory. Regular expressions support lookahead and
  lookbehind.

### `fileName`

- Type: `string`
- Optional
- Default: `'stylex.css'`
- Description: Name of the emitted CSS asset.

### `useCSSLayers`

- Type: `boolean`
- Optional
- Default: `false`
- Description: Wraps the generated CSS in cascade layers for better style
  isolation.

### `lightningcssOptions`

- Type: `TransformOptions`
- Optional
- Description: Options forwarded to the Lightning CSS transform that
  post-processes the extracted CSS (everything except `code`, `filename`, and
  `visitor`).

### `extractCSS`

- Type: `boolean`
- Optional
- Default: `true`
- Description: Controls whether the generated CSS is extracted into a separate
  file.

## Path Filtering Examples

Include only specific directories:

```js
stylexPlugin.default({
  rsOptions: {
    include: ['src/**/*.{ts,tsx,js,jsx}'],
  },
});
```

Exclude test and build files:

```js
stylexPlugin.default({
  rsOptions: {
    exclude: ['**/*.test.*', '**/*.spec.*', '**/dist/**'],
  },
});
```

Exclude all of `node_modules` except specific packages (negative lookahead):

```js
stylexPlugin.default({
  rsOptions: {
    exclude: [/node_modules(?!\/@stylexjs)/],
  },
});
```

Transform only specific packages from `node_modules`:

```js
stylexPlugin.default({
  rsOptions: {
    include: [
      'src/**/*.{ts,tsx,js,jsx}',
      'node_modules/@stylexjs/open-props/**/*.js',
    ],
    exclude: ['**/*.test.*'],
  },
});
```

## FAQ

### Do I still need `@stylexjs/babel-plugin`?

No. This plugin replaces the Babel plugin in your build. You only keep
`@stylexjs/stylex` as your app's runtime dependency, and your `stylex.create` /
`stylex.props` code does not change.

### Does this work together with other Rollup plugins?

Yes. Run StyleX after resolution/commonjs plugins and alongside your transpiler
plugin (SWC or Babel). The
[example app](https://github.com/Dwlad90/stylex-swc-plugin/tree/develop/apps/rollup-example)
shows a full pipeline with `@rollup/plugin-swc`.

### How do I post-process the generated CSS?

The plugin already runs Lightning CSS on the extracted stylesheet; tune it with
`lightningcssOptions` (for example `targets` from a browserslist query).

### Is this an official StyleX package?

No. It is a community-maintained alternative to the official tooling and is not
affiliated with or supported by Meta.

## Documentation

- [StyleX documentation](https://stylexjs.com)
- [`@stylexswc/rs-compiler` compiler options](https://github.com/Dwlad90/stylex-swc-plugin/tree/develop/crates/stylex-rs-compiler)

## License

MIT — see
[LICENSE](https://github.com/Dwlad90/stylex-swc-plugin/blob/develop/LICENSE)
