# @stylexswc/unplugin

> Universal StyleX plugin for Vite, webpack, Rspack, Rollup, esbuild, Farm,
> Rsbuild, Nuxt, and Astro — powered by a Rust compiler (NAPI-RS + SWC). Part of
> the [StyleX SWC Plugin](https://github.com/Dwlad90/stylex-swc-plugin#readme)
> workspace.

Built on [unplugin](https://unplugin.unjs.io/), this package gives every major
bundler the same [StyleX](https://stylexjs.com) integration: it compiles your
StyleX code with
[`@stylexswc/rs-compiler`](https://www.npmjs.com/package/@stylexswc/rs-compiler),
a Rust implementation of the StyleX transform, and extracts the generated CSS.
Your StyleX code stays exactly the same — only the build step changes, with
per-file transforms 2x to 5x faster than Babel
([performance](https://github.com/Dwlad90/stylex-swc-plugin#performance)).

This is a community project and is not affiliated with Meta. It tracks the
official StyleX releases
<!-- stylex-compatibility:start -->(currently compatible with StyleX v0.19.0)<!-- stylex-compatibility:end -->
and requires Node.js 20 or newer.

## Installation

```bash
npm install --save-dev @stylexswc/unplugin
```

The Rust compiler (`@stylexswc/rs-compiler`) is installed automatically as a
dependency. Your application still needs the StyleX runtime:

```bash
npm install @stylexjs/stylex
```

## Usage

Import the entry point matching your build tool and add it to the plugin list. A
working example for each bundler lives in the
[`apps/{pluginName}-unplugin-example`](https://github.com/Dwlad90/stylex-swc-plugin/tree/develop/apps)
folders.

<details>
<summary>Vite</summary><br>

```ts
// vite.config.ts
import StylexRsPlugin from '@stylexswc/unplugin/vite';

export default defineConfig({
  plugins: [StylexRsPlugin({/* options */})],
});
```

<br></details>

<details>
<summary>Rollup</summary><br>

```ts
// rollup.config.js
import StylexRsPlugin from '@stylexswc/unplugin/rollup';

export default {
  plugins: [StylexRsPlugin({/* options */})],
};
```

<br></details>

<details>
<summary>Webpack</summary><br>

```ts
// webpack.config.js
module.exports = {
  /* ... */
  plugins: [require('@stylexswc/unplugin/webpack')({/* options */})],
};
```

<br></details>

<details>
<summary>Rspack</summary><br>

```ts
// rspack.config.js
module.exports = {
  /* ... */
  plugins: [require('@stylexswc/unplugin/rspack')({/* options */})],
};
```

<br></details>

<details>
<summary>Nuxt</summary><br>

```ts
// nuxt.config.js
export default defineNuxtConfig({
  modules: [['@stylexswc/unplugin/nuxt', {/* options */}]],
});
```

> This module works for both Nuxt 2 and
> [Nuxt Vite](https://github.com/nuxt/vite)

<br></details>

<details>
<summary>Vue CLI</summary><br>

```ts
// vue.config.js
module.exports = {
  configureWebpack: {
    plugins: [require('@stylexswc/unplugin/webpack')({/* options */})],
  },
};
```

<br></details>

<details>
<summary>esbuild</summary><br>

```ts
// esbuild.config.js
import { build } from 'esbuild';
import StylexRsPlugin from '@stylexswc/unplugin/esbuild';

build({
  plugins: [StylexRsPlugin()],
});
```

<br></details>

## Plugin Options

### Basic Options

#### `rsOptions`

- Type: `Partial<StyleXOptions>`
- Optional
- Description: StyleX compiler options passed to `@stylexswc/rs-compiler`. For
  the standard options, see the
  [official StyleX documentation](https://stylexjs.com/docs/api/configuration/babel-plugin/).

> [!NOTE]
> The `include` and `exclude` options are exclusive to the Rust compiler
> and are not available in the official StyleX Babel plugin.

##### `rsOptions.include`

- Type: `(string | RegExp)[]`
- Optional
- Description: Glob patterns or regular expressions selecting the files to
  transform. When specified, only files matching at least one pattern are
  transformed. Patterns are matched against paths relative to the current
  working directory.

##### `rsOptions.exclude`

- Type: `(string | RegExp)[]`
- Optional
- Description: Glob patterns or regular expressions excluding files from the
  transform. A file matching any exclude pattern is skipped even if it matches
  an `include` pattern. Patterns are matched against paths relative to the
  current working directory.

#### `fileName`

- Type: `string`
- Default: `'stylex.css'`
- Description: Name of the generated CSS file.

#### `useCSSLayers`

- Type: `UseLayersType`
- Default: `false`
- Description: Wraps the generated CSS in cascade layers for better style
  isolation.

#### `extractCSS`

- Type: `boolean`
- Default: `true`
- Description: Controls whether the generated CSS is extracted into a separate
  file.

#### `pageExtensions`

- Type: `string[]`
- Default: `['js', 'jsx', 'ts', 'tsx', 'mjs', 'mts']`
- Description: File extensions to process for StyleX transformations.

#### `transformCss`

- Type: `CSSTransformer`
- Optional
- Description: Transforms the extracted StyleX CSS before it is emitted or
  injected. Matches the `@stylexswc/webpack-plugin` API. Use it to run the
  generated CSS through PostCSS, Lightning CSS, a minifier, or any custom
  post-processing step.

```ts
type CSSTransformer = (
  css: string,
  filePath: string | undefined
) => string | Buffer | Promise<string | Buffer>;
```

```ts
import postcss from 'postcss';
import autoprefixer from 'autoprefixer';

StylexRsPlugin({
  async transformCss(css) {
    const result = await postcss([autoprefixer]).process(css, {
      from: undefined,
    });
    return result.css;
  },
});
```

The `filePath` argument identifies the CSS destination and is bundler-specific:

- webpack/rspack/rollup injection: the output asset name (e.g. `app.css`)
- esbuild disk writes: the absolute path of the written file
- Vite placeholder replacement: the id of the CSS module being loaded
- generated assets: the configured `fileName`, with `[hash]` left unresolved
  (the hash is computed from the transformed CSS, so it cannot be known earlier)
- `undefined` when no destination is known

> [!NOTE]
> `Buffer` results are decoded as UTF-8. Results are memoized per
> `filePath` while the input CSS is unchanged, so the callback must be a pure
> function of its arguments — the same input may be served from cache instead of
> invoking the callback again.

#### `useCssPlaceholder`

- Type: `boolean | string`
- Default: `false`
- Description: Injects the generated CSS into an existing CSS file via a
  placeholder marker.
  - When set to `true`, the plugin looks for the default `@stylex;` marker
  - When set to a string, the plugin uses that string as the custom marker

Routing the StyleX output through a real CSS file has practical benefits:

- The generated CSS goes through the bundler's CSS pipeline (PostCSS, Lightning
  CSS, css-loader, and so on)
- Deterministic builds — no race conditions or hash instability from virtual
  modules
- All CSS follows the same processing rules and bundling strategy
- CSS can be code-split and optimized alongside other stylesheets
- The same approach works for Vite, webpack, Rspack, esbuild, Rollup, and Farm

How to use it:

1. Create a CSS file with a marker (e.g. `global.css`):

```css
/* global.css */
:root {
  --brand-color: #663399;
}

body {
  margin: 0;
  font-family: system-ui, sans-serif;
}

@stylex;
```

2. Import the CSS file in your entry point:

```ts
// src/main.ts
import './global.css';
import { App } from './App';
```

3. Configure the plugin with `useCssPlaceholder`:

```ts
// vite.config.ts (or webpack.config.js, rspack.config.js, etc.)
import StylexRsPlugin from '@stylexswc/unplugin/vite';
import { defineConfig } from 'vite';

export default defineConfig({
  plugins: [
    StylexRsPlugin({
      useCssPlaceholder: true, // Uses default '@stylex;' marker
      useCSSLayers: true,
    }),
  ],
});
```

Or with a custom marker string:

```css
/* global.css */
:root {
  --brand-color: #663399;
}

/* INJECT_STYLEX_HERE */
```

```ts
StylexRsPlugin({
  useCssPlaceholder: '/* INJECT_STYLEX_HERE */',
  useCSSLayers: true,
});
```

The plugin replaces the marker with the generated StyleX CSS during the build.

> [!NOTE]
> When `useCssPlaceholder` is enabled, the plugin no longer injects CSS
> automatically into HTML or emits a separate `stylex.css` file. The CSS goes
> into your specified CSS file instead.

> [!IMPORTANT]
> **Migration from `useViteCssPipeline`**
>
> The `useViteCssPipeline` option (which used virtual CSS modules) has been
> replaced by `useCssPlaceholder`. The new approach uses real CSS files instead
> of virtual modules, which provides better compatibility across all bundlers,
> no race conditions or timing issues, and deterministic builds with stable
> hashes. To migrate, create a CSS file with a marker and set
> `useCssPlaceholder: true` (or use a custom marker string).

### Example Configuration

```ts
// vite.config.ts
import StylexRsPlugin from '@stylexswc/unplugin/vite';
import { defineConfig } from 'vite';

export default defineConfig({
  plugins: [
    StylexRsPlugin({
      rsOptions: {
        dev: process.env.NODE_ENV !== 'production',
        include: ['src/**/*.{ts,tsx}', 'components/**/*.{ts,tsx}'],
        exclude: ['**/*.test.*', '**/*.stories.*', '**/__tests__/**'],
      },
      useCSSLayers: true,
      useCssPlaceholder: true,
    }),
  ],
});
```

### Path Filtering Examples

Include only specific directories:

```ts
StylexRsPlugin({
  rsOptions: {
    include: ['src/**/*.tsx', 'app/**/*.tsx'],
  },
});
```

Exclude test and build files:

```ts
StylexRsPlugin({
  rsOptions: {
    exclude: ['**/*.test.*', '**/*.spec.*', '**/dist/**'],
  },
});
```

Exclude `node_modules` except specific packages (negative lookahead):

```ts
StylexRsPlugin({
  rsOptions: {
    exclude: [/node_modules(?!\/@stylexjs)/],
  },
});
```

Transform only specific packages from `node_modules`:

```ts
StylexRsPlugin({
  rsOptions: {
    include: [
      'src/**/*.{ts,tsx}',
      'node_modules/@stylexjs/open-props/**/*.js',
      'node_modules/@my-org/design-system/**/*.js',
    ],
    exclude: ['**/*.test.*'],
  },
});
```

Combined include and exclude (exclude takes precedence):

```ts
StylexRsPlugin({
  rsOptions: {
    include: ['src/**/*.{ts,tsx}'],
    exclude: ['**/__tests__/**', '**/__mocks__/**'],
  },
});
```

## FAQ

### Should I use this package or the bundler-specific plugin?

If you are on Vite, esbuild, Farm, Rsbuild, Nuxt, or Astro, this is the package
to use. For webpack and Rspack, the dedicated `@stylexswc/webpack-plugin` and
`@stylexswc/rspack-plugin` packages offer a few deeper integrations (loader
ordering, cache groups); this plugin covers the common cases with one consistent
API. For Next.js, use `@stylexswc/nextjs-plugin`.

### Do I still need `@stylexjs/babel-plugin`?

No. This plugin replaces the Babel plugin in your build. You only keep
`@stylexjs/stylex` as your app's runtime dependency, and your `stylex.create` /
`stylex.props` code does not change.

### How do I run the generated CSS through PostCSS or Tailwind pipelines?

Either pass a `transformCss` function, or enable `useCssPlaceholder` so the
generated CSS lands in a real CSS file and flows through your bundler's normal
CSS pipeline.

### Does hot module replacement work?

Yes. The plugin participates in each bundler's standard transform pipeline, so
style changes update through the dev server like any other module.

### Is this an official StyleX package?

No. It is a community-maintained alternative to the official tooling and is not
affiliated with or supported by Meta.

## Documentation

- [StyleX documentation](https://stylexjs.com)
- [unplugin documentation](https://unplugin.unjs.io/)
- [`@stylexswc/rs-compiler` compiler options](https://github.com/Dwlad90/stylex-swc-plugin/tree/develop/crates/stylex-rs-compiler)

## License

MIT — see
[LICENSE](https://github.com/Dwlad90/stylex-swc-plugin/blob/develop/LICENSE)
