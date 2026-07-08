# @stylexswc/webpack-plugin

> StyleX plugin for webpack, powered by a Rust compiler (NAPI-RS + SWC). Part of
> the [StyleX SWC Plugin](https://github.com/Dwlad90/stylex-swc-plugin#readme)
> workspace.

This plugin compiles [StyleX](https://stylexjs.com) code in your webpack build
with
[`@stylexswc/rs-compiler`](https://www.npmjs.com/package/@stylexswc/rs-compiler),
a Rust implementation of the StyleX transform, instead of the official Babel
plugin. Your StyleX code stays exactly the same — only the build step changes,
with per-file transforms 2x to 5x faster than Babel
([performance](https://github.com/Dwlad90/stylex-swc-plugin#performance)).
The plugin transforms your source files, collects the generated rules, and
extracts them into a dedicated CSS chunk.

This is a community project and is not affiliated with Meta. It tracks the
official StyleX releases
<!-- stylex-compatibility:start -->(currently compatible with StyleX v0.19.0)<!-- stylex-compatibility:end -->
and requires Node.js 20 or newer. For Next.js projects, use
[`@stylexswc/nextjs-plugin`](https://www.npmjs.com/package/@stylexswc/nextjs-plugin)
instead — it wires this plugin into the Next.js build for you.

## Installation

```bash
npm install --save-dev @stylexswc/webpack-plugin
```

The Rust compiler (`@stylexswc/rs-compiler`) is installed automatically as a
dependency. Your application still needs the StyleX runtime:

```bash
npm install @stylexjs/stylex
```

## Usage

Add the plugin to your webpack config:

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
      rsOptions: {
        dev: argv.mode === 'development',
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

#### `stylexImports`

- Type: `Array<string | { as: string, from: string }>`
- Default: `['stylex', '@stylexjs/stylex']`
- Description: Specifies where StyleX is imported from. Supports both string
  paths and import aliases.

#### `useCSSLayers`

- Type: `boolean`
- Default: `false`
- Description: Wraps the generated CSS in cascade layers for better style
  isolation.

#### `nextjsMode`

- Type: `boolean`
- Default: `false`
- Description: Enables Next.js-specific optimizations and compatibility
  features. Leave off unless the plugin is driven by the Next.js integration.

#### `extractCSS`

- Type: `boolean`
- Optional
- Default: `true`
- Description: Controls whether the generated CSS is extracted into a separate
  file.

#### `loaderOrder`

- Type: `'first' | 'last'`
- Optional
- Default: `'first'`
- Description: When the StyleX transformation runs relative to other webpack
  loaders.
  - `'first'` (recommended): StyleX processes the source code before any other
    loaders run. Automatically enables `injectStylexSideEffects` so tree-shaking
    cannot remove `.stylex` and `.consts` imports.
  - `'last'`: StyleX processes after all other loaders have completed. Use this
    if other loaders (TypeScript, SWC plugins) must transform your code before
    StyleX sees it.

  Why `'first'` is recommended: after StyleX rewrites your code, imports from
  `.stylex` and `.consts` files can look unused to subsequent loaders and
  bundler passes and get tree-shaken away:

  ```ts
  // Before transformation
  import { colors } from './theme.stylex';
  const styles = stylex.create({
    root: { backgroundColor: colors.primary },
  });

  // After StyleX transformation (appears unused to the bundler)
  import { colors } from './theme.stylex'; // may be tree-shaken!
  const styles = { root: { backgroundColor: 'x1a2b3c', $$css: true } };
  ```

  With `loaderOrder: 'first'`, the plugin preserves these imports by injecting
  side-effect imports automatically.

#### `stylexPackages`

- Type: `string[]`
- Optional
- Default: `['@stylexjs/']`
- Description: `node_modules` is excluded from the transform by default, even
  for files that reference a StyleX import. Packages that ship untransformed
  StyleX source (component libraries, token packages) must be allowlisted here
  with path fragments:

```js
new StylexPlugin({
  stylexPackages: ['@stylexjs/', 'my-design-system'],
});
```

### Advanced Options

#### `transformCss`

- Type:
  `(css: string, filePath: string | undefined) => string | Buffer | Promise<string | Buffer>`
- Optional
- Description: Custom CSS post-processing. The plugin injects CSS after all
  loaders have run, so `postcss-loader` never sees it — apply PostCSS,
  autoprefixer, or a minifier here instead:

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

#### `cacheGroup`

- Type: `CacheGroupOptions` (webpack cache group configuration)
- Optional
- Description: Overrides the default webpack cache group used for StyleX CSS
  extraction. By default the plugin creates a dedicated cache group named
  `stylex`. Use this to customize chunk naming, priority, or other split chunks
  options.

Default cache group configuration:

```js
{
  name: 'stylex',
  test: /\.stylex\.virtual\.css$/,
  type: 'css/mini-extract',
  chunks: 'all',
  enforce: true,
}
```

Custom cache group:

```js
new StylexPlugin({
  cacheGroup: {
    name: 'my-stylex-bundle',
    chunks: 'initial',
    priority: 20,
    enforce: true,
  },
});
```

### Example Configuration

```js
const StylexPlugin = require('@stylexswc/webpack-plugin');

module.exports = {
  plugins: [
    new StylexPlugin({
      rsOptions: {
        dev: process.env.NODE_ENV !== 'production',
        include: ['src/**/*.{ts,tsx}', 'components/**/*.{ts,tsx}'],
        exclude: ['**/*.test.*', '**/*.stories.*', '**/__tests__/**'],
      },
      stylexImports: ['@stylexjs/stylex', { from: './theme', as: 'tokens' }],
      useCSSLayers: true,
      loaderOrder: 'first',
      transformCss: async css => {
        const postcss = require('postcss');
        const result = await postcss([require('autoprefixer')]).process(css);
        return result.css;
      },
    }),
  ],
};
```

### Path Filtering Examples

Include only specific directories:

```js
new StylexPlugin({
  rsOptions: {
    include: ['src/**/*.tsx', 'app/**/*.tsx'],
  },
});
```

Exclude test and build files:

```js
new StylexPlugin({
  rsOptions: {
    exclude: ['**/*.test.*', '**/*.spec.*', '**/dist/**'],
  },
});
```

Using regular expressions:

```js
new StylexPlugin({
  rsOptions: {
    include: [/src\/.*\.tsx$/],
    exclude: [/\.test\./, /\.stories\./],
  },
});
```

Transform only specific packages from `node_modules` — note that `node_modules`
is excluded regardless of `rsOptions.include`/`exclude`, so allowlist packages
with `stylexPackages` instead:

```js
new StylexPlugin({
  rsOptions: {
    include: ['src/**/*.{ts,tsx}'],
    exclude: ['**/*.test.*'],
  },
  stylexPackages: ['@stylexjs/open-props', '@my-org/design-system'],
});
```

## FAQ

### Do I still need `@stylexjs/babel-plugin`?

No. This plugin replaces the Babel plugin in your build. You only keep
`@stylexjs/stylex` as your app's runtime dependency, and your `stylex.create` /
`stylex.props` code does not change.

### My styles from a component library are missing. Why?

`node_modules` is excluded by default. Add the package to `stylexPackages` (for
example `stylexPackages: ['@stylexjs/', 'my-design-system']`) so the StyleX
loader processes it.

### Imports from my `.stylex.ts` token files disappear after the build

That is tree-shaking removing imports that look unused after the transform. Keep
the default `loaderOrder: 'first'`, which injects side-effect imports to protect
them.

### Can I run PostCSS on the generated CSS?

Yes — through the `transformCss` option. The StyleX CSS is produced after the
regular loader pipeline, so `postcss-loader` alone will not see it.

### Is this an official StyleX package?

No. It is a community-maintained alternative to the official tooling and is not
affiliated with or supported by Meta.

## Documentation

- [StyleX documentation](https://stylexjs.com)
- [`@stylexswc/rs-compiler` compiler options](https://github.com/Dwlad90/stylex-swc-plugin/tree/develop/crates/stylex-rs-compiler)

## Acknowledgments

This plugin was inspired by
[`stylex-webpack`](https://github.com/SukkaW/stylex-webpack).

## License

MIT — see
[LICENSE](https://github.com/Dwlad90/stylex-swc-plugin/blob/develop/LICENSE)
