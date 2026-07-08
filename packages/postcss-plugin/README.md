# @stylexswc/postcss-plugin

> PostCSS plugin that extracts StyleX styles with a Rust compiler (NAPI-RS +
> SWC). Part of the
> [StyleX SWC Plugin](https://github.com/Dwlad90/stylex-swc-plugin#readme)
> workspace.

This plugin scans your source files for [StyleX](https://stylexjs.com) code,
compiles it with
[`@stylexswc/rs-compiler`](https://www.npmjs.com/package/@stylexswc/rs-compiler)
— a Rust implementation of the StyleX transform — and replaces an `@stylex;`
directive in your CSS with the generated rules. Because it plugs into any
PostCSS pipeline, it is the standard way to get CSS extraction where bundler
plugins cannot run, most notably Next.js with Turbopack.

This is a community project and is not affiliated with Meta. It tracks the
official StyleX releases
<!-- stylex-compatibility:start -->(currently compatible with StyleX v0.19.0)<!-- stylex-compatibility:end -->
and requires Node.js 20 or newer.

## Installation

```bash
npm install --save-dev @stylexswc/postcss-plugin
```

## Usage

Add the plugin to `postcss.config.js`:

```js
module.exports = {
  plugins: {
    '@stylexswc/postcss-plugin': {
      include: ['src/**/*.{js,jsx,ts,tsx}'],
    },
    autoprefixer: {},
  },
};
```

Add a CSS file containing the `@stylex` directive to your project:

```css
/* stylex.css */

/**
 * The @stylex directive is replaced with the generated CSS during builds.
 */
@stylex;
```

And import it in your JS/TS entry point:

```js
import './stylex.css';
```

The JS/TS files themselves still need a StyleX transform so the class names
exist at runtime — pair this plugin with one of the `@stylexswc` bundler
plugins. For example with Next.js:

```js
// next.config.js
const path = require('path');
const stylexPlugin = require('@stylexswc/nextjs-plugin');
const rootDir = __dirname;

module.exports = stylexPlugin({
  rsOptions: {
    aliases: {
      '@/*': [path.join(rootDir, '*')],
    },
    unstable_moduleResolution: {
      type: 'commonJS',
    },
  },
  // Important: prevents generating the StyleX CSS twice
  extractCSS: false,
})({
  transpilePackages: ['@stylexjs/open-props'],
});
```

> [!WARNING]
> Every `@stylexswc` bundler plugin accepts an `extractCSS` option.
> When this PostCSS plugin owns extraction, set `extractCSS: false` on the
> bundler plugin to avoid generating the StyleX CSS twice.

> [!NOTE]
> This approach transforms JS/TS files with StyleX code twice: once for
> the bundle and once inside the PostCSS plugin. On Next.js with Webpack you can
> avoid that by skipping this plugin entirely and passing a `transformCss`
> function to `@stylexswc/nextjs-plugin` instead. With Turbopack, the double
> transform is currently the only way to extract CSS.

## Plugin Options

> [!NOTE]
> The `include` and `exclude` options are exclusive to the Rust compiler
> and are not available in the official StyleX Babel plugin.

### `include`

- Type: `(string | RegExp)[]`
- Default: auto-discovered
- Description: Glob patterns or regular expressions selecting the files to scan
  and transform. When specified, only files matching at least one pattern are
  processed. Patterns are matched against paths relative to the current working
  directory. Regular expressions support lookahead and lookbehind.

When omitted, the plugin auto-discovers source files in the project `cwd` and
direct dependencies that use StyleX.

### `exclude`

- Type: `(string | RegExp)[]`
- Optional
- Description: Glob patterns or regular expressions excluding files from the
  transform. A file matching any exclude pattern is skipped even if it matches
  an `include` pattern. Patterns are matched against paths relative to the
  current working directory. Regular expressions support lookahead and
  lookbehind.

When `include` is omitted, the plugin automatically excludes common build and
dependency folders (for example `node_modules`, `.next`, `dist`, `build`) to
keep discovery focused on source files.

### `rsOptions`

- Type: `StyleXOptions`
- Optional
- Default: `{}`
- Description: StyleX compiler options passed to the compiler. For the standard
  options, see the
  [official StyleX documentation](https://stylexjs.com/docs/api/configuration/babel-plugin/).

### `useCSSLayers`

- Type: `boolean`
- Optional
- Default: `false`
- Description: Wraps the generated CSS in cascade layers for better style
  isolation.

### `cwd`

- Type: `string`
- Optional
- Default: `process.cwd()`
- Description: Working directory for resolving files. Dependency paths and
  config resolution use this value.

### `isDev`

- Type: `boolean`
- Optional
- Description: Whether the plugin is running in development mode.

### `importSources`

- Type: `Array<string | { from: string, as: string }>`
- Optional
- Description: Overrides import sources at the PostCSS plugin level.

When provided, takes precedence over `rsOptions.importSources`. When omitted,
falls back to `rsOptions.importSources`, then the built-in defaults
(`@stylexjs/stylex`, `stylex`).

## Path Filtering Examples

Include only specific directories:

```js
{
  '@stylexswc/postcss-plugin': {
    include: ['src/**/*.{ts,tsx}', 'components/**/*.{ts,tsx}'],
  },
}
```

Exclude test and story files:

```js
{
  '@stylexswc/postcss-plugin': {
    include: ['src/**/*.{ts,tsx}'],
    exclude: ['**/*.test.*', '**/*.stories.*', '**/__tests__/**'],
  },
}
```

Using regular expressions:

```js
{
  '@stylexswc/postcss-plugin': {
    include: ['src/**/*.{ts,tsx}', /components\/.*\.tsx$/],
    exclude: [/\.test\./, /\.stories\./],
  },
}
```

Exclude `node_modules` except specific packages (negative lookahead):

```js
{
  '@stylexswc/postcss-plugin': {
    include: ['src/**/*.{ts,tsx}', 'node_modules/**/*.js'],
    exclude: [/node_modules(?!\/@stylexjs\/open-props)/],
  },
}
```

## Debugging auto-discovery

Set `STYLEX_POSTCSS_DEBUG=1` to print resolved plugin inputs, including:

- resolved `importSources` and where they came from
- final `include` and `exclude` globs
- discovered dependency directories

## FAQ

### When do I need this plugin instead of a bundler plugin?

Whenever your bundler cannot extract CSS itself: Next.js with Turbopack is the
main case, and any pipeline where CSS is produced exclusively through PostCSS
(for example alongside Tailwind). If you build with Webpack, Rspack, Rollup, or
Vite, the corresponding `@stylexswc` plugin extracts CSS on its own and this
plugin is unnecessary.

### My generated CSS appears twice. What happened?

Both the bundler plugin and this plugin extracted it. Set `extractCSS: false` on
the bundler plugin so the PostCSS plugin is the only producer.

### Nothing is injected in place of `@stylex;`. What should I check?

Confirm the CSS file with the directive is actually imported by your app, that
your `include` globs match the files defining styles, and run with
`STYLEX_POSTCSS_DEBUG=1` to see which files the plugin discovered.

### Is this an official StyleX package?

No. It is a community-maintained alternative to the official tooling and is not
affiliated with or supported by Meta.

## Documentation

- [StyleX documentation](https://stylexjs.com)
- [`@stylexswc/rs-compiler` compiler options](https://github.com/Dwlad90/stylex-swc-plugin/tree/develop/crates/stylex-rs-compiler)

## License

MIT — see
[LICENSE](https://github.com/Dwlad90/stylex-swc-plugin/blob/develop/LICENSE)
