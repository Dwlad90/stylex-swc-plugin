# @stylexswc/nextjs-plugin

> StyleX plugin for Next.js, powered by a Rust compiler (NAPI-RS + SWC).
> Supports Webpack, Rspack, and Turbopack builds. Part of the
> [StyleX SWC Plugin](https://github.com/Dwlad90/stylex-swc-plugin#readme)
> workspace.

This plugin integrates [StyleX](https://stylexjs.com) into Next.js using
[`@stylexswc/rs-compiler`](https://www.npmjs.com/package/@stylexswc/rs-compiler),
a Rust implementation of the StyleX transform, instead of the official Babel
plugin. That means you keep Next.js's fast SWC toolchain — no `.babelrc`, no
Babel fallback — and your StyleX code stays exactly the same. Per-file
transforms are 2x to 5x faster than with Babel — see
[performance](https://github.com/Dwlad90/stylex-swc-plugin#performance).

This is a community project and is not affiliated with Meta. It tracks the
official StyleX releases
<!-- stylex-compatibility:start -->(currently compatible with StyleX v0.19.0)<!-- stylex-compatibility:end -->,
requires Node.js 20 or newer, and supports Next.js 15+ (App Router and Pages
Router).

## Installation

```bash
npm install --save-dev @stylexswc/nextjs-plugin
```

Your application still needs the StyleX runtime:

```bash
npm install @stylexjs/stylex
```

## Usage

The plugin supports all three Next.js bundlers. Pick the export that matches
your setup.

For the Webpack and Rspack integrations, import the carrier stylesheet
**once** at your app entrypoint — the root layout (`app/layout.tsx`) for the
App Router, or `pages/_app.tsx` for the Pages Router:

```tsx
// App Router: app/layout.tsx (Webpack)
import '@stylexswc/webpack-plugin/stylex.css';
// or for the /rspack export:
import '@stylexswc/rspack-plugin/stylex.css';
```

The plugin replaces this asset's content with the extracted StyleX CSS during
the build; without it, no StyleX CSS is emitted.

> [!IMPORTANT]
> **Migrating from 0.17.x**: version 0.18.0 requires the carrier import
> above. The App Router cross-compiler rule registry is now enabled by
> default (`nextjsAppRouterMode: true`), so styles authored in Server
> Components reach the client CSS; pass `nextjsAppRouterMode: false` when
> using the Pages Router. `experimental.webpackBuildWorker` is force-disabled
> because the registry requires all compilers to share one process.

### Using with Webpack

For standard Next.js Webpack builds, use the default import:

```js
// next.config.js
const stylexPlugin = require('@stylexswc/nextjs-plugin');

module.exports = stylexPlugin({
  // StyleX options here
})({
  // Next.js config here
});
```

### Using with Rspack

For Next.js with [Rspack](https://rspack.rs), use the `/rspack` export. It
applies the experimental
[`next-rspack`](https://www.npmjs.com/package/next-rspack) adapter for you — no
manual `withRspack` composition needed:

```ts
import stylexPlugin from '@stylexswc/nextjs-plugin/rspack';

module.exports = stylexPlugin({
  // StyleX options here
})({
  // Next.js config here
});
```

> [!NOTE]
> Run `next dev` and `next build` without the `--webpack`/`--turbopack`
> flags. For `next start`, set `NEXT_RSPACK=true` in the environment (the
> production server only serves prebuilt output, but it still evaluates the
> config).

The Rspack integration extracts StyleX CSS from both Server and Client
Components, with the same options as the Webpack plugin, plus `stylexPackages`
from
[`@stylexswc/rspack-plugin`](https://github.com/Dwlad90/stylex-swc-plugin/tree/develop/packages/rspack-plugin).

> [!NOTE]
> Packages listed in `transpilePackages` are automatically added to the
> `stylexPackages` allowlist, so StyleX source shipped in `node_modules` (e.g.
> `@stylexjs/open-props`) is picked up without extra configuration.

### Using with Turbopack

> [!IMPORTANT]
> Turbopack does not support webpack plugins
> ([see Next.js docs](https://nextjs.org/docs/app/api-reference/turbopack#webpack-plugins)).
> When using Turbopack, the loader only compiles StyleX code but does not
> extract CSS.
>
> You must configure the PostCSS plugin for CSS extraction. Install
> `@stylexswc/postcss-plugin` and configure it in `postcss.config.js`:
>
> ```js
> // postcss.config.js
> module.exports = {
>   plugins: {
>     '@stylexswc/postcss-plugin': {
>       rsOptions: {
>         dev: process.env.NODE_ENV === 'development',
>       },
>     },
>     autoprefixer: {},
>   },
> };
> ```

For Next.js with Turbopack, use the `/turbopack` export:

```ts
import withStylexTurbopack from '@stylexswc/nextjs-plugin/turbopack';

export default withStylexTurbopack({
  // StyleX options here, same as postcss-plugin
  rsOptions: {
    dev: process.env.NODE_ENV === 'development',
  },
})({
  // Next.js config here
});
```

> [!NOTE]
> When using Turbopack, the following options are not supported and will
> be ignored:
>
> - `useCSSLayers`
> - `nextjsMode`
> - `transformCss`
> - `extractCSS`

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

- Type: `UseLayersType`
- Default: `false`
- Description: Wraps the generated CSS in cascade layers for better style
  isolation.

#### `extractCSS`

- Type: `boolean`
- Optional
- Default: `true`
- Description: Controls whether the generated CSS is extracted into a separate
  file. Set to `false` when `@stylexswc/postcss-plugin` owns extraction.

#### `carrierCss`

- Type: `string`
- Optional
- Default: the packaged `<plugin package>/stylex.css`
- Description: Path to a custom carrier stylesheet (the file imported in your
  root layout / `_app`) that receives the extracted StyleX CSS. Absolute, or
  relative to the project directory. Replaces the default packaged carrier.
  When styles are extracted but no carrier asset exists to receive them, the
  build raises a compilation warning instead of silently dropping the CSS.
- Note: carrier matching compares resolved absolute paths, which assumes the
  default symlink resolution. With `resolve.symlinks: false` or
  `node --preserve-symlinks`, Node and the bundler can disagree about the
  carrier's real path — if the missing-carrier warning appears in such a
  setup, point `carrierCss` at a file inside your own source tree.

### Advanced Options

#### `transformCss`

- Type:
  `(css: string, filePath: string | undefined) => string | Buffer | Promise<string | Buffer>`
- Optional
- Description: Custom CSS post-processing. The plugin injects CSS after all
  loaders have run, so use this to apply PostCSS or other CSS transformations.

### Example Configuration

#### Webpack Configuration

```js
const path = require('path');
const stylexPlugin = require('@stylexswc/nextjs-plugin');
const rootDir = __dirname;

module.exports = stylexPlugin({
  rsOptions: {
    dev: process.env.NODE_ENV !== 'production',
    // Include only specific directories
    include: [
      'app/**/*.{ts,tsx}',
      'components/**/*.{ts,tsx}',
      'src/**/*.{ts,tsx}',
    ],
    // Exclude test files and API routes
    exclude: ['**/*.test.*', '**/*.stories.*', '**/__tests__/**', 'app/api/**'],
    aliases: {
      '@/*': [path.join(rootDir, '*')],
    },
    unstable_moduleResolution: {
      type: 'commonJS',
    },
  },
  stylexImports: ['@stylexjs/stylex', { from: './theme', as: 'tokens' }],
  useCSSLayers: true,
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
})({
  transpilePackages: ['@stylexjs/open-props'],
  // Optionally, add any other Next.js config below
});
```

#### Turbopack Configuration

```ts
import path from 'path';
import withStylexTurbopack from '@stylexswc/nextjs-plugin/turbopack';

const rootDir = __dirname;

export default withStylexTurbopack({
  rsOptions: {
    dev: process.env.NODE_ENV !== 'production',
    aliases: {
      '@/*': [path.join(rootDir, '*')],
    },
    unstable_moduleResolution: {
      type: 'commonJS',
    },
  },
  stylexImports: ['@stylexjs/stylex'],
})({
  transpilePackages: ['@stylexjs/open-props'],
  // Optionally, add any other Next.js config below
});
```

Required PostCSS configuration for CSS extraction under Turbopack:

```js
// postcss.config.js
const path = require('path');

module.exports = {
  plugins: {
    '@stylexswc/postcss-plugin': {
      include: ['app/**/*.{js,jsx,ts,tsx}', 'components/**/*.{js,jsx,ts,tsx}'],
      rsOptions: {
        aliases: {
          '@/*': [path.join(__dirname, '*')],
        },
        unstable_moduleResolution: {
          type: 'commonJS',
        },
        dev: process.env.NODE_ENV === 'development',
      },
    },
    autoprefixer: {},
  },
};
```

### Path Filtering Examples

Include only specific directories:

```js
stylexPlugin({
  rsOptions: {
    include: ['app/**/*.tsx', 'components/**/*.tsx'],
  },
});
```

Exclude test files and API routes:

```js
stylexPlugin({
  rsOptions: {
    exclude: ['**/*.test.*', '**/*.stories.*', '**/__tests__/**', 'app/api/**'],
  },
});
```

Using regular expressions (exclude always takes precedence over include):

```js
stylexPlugin({
  rsOptions: {
    include: [/app\/.*\.tsx$/, /components\/.*\.tsx$/],
    exclude: [/\.test\./, /\.stories\./],
  },
});
```

Exclude `node_modules` except specific packages (negative lookahead):

```js
stylexPlugin({
  rsOptions: {
    exclude: [/node_modules(?!\/@stylexjs\/open-props)/],
  },
});
```

Transform only specific packages from `node_modules`:

```js
stylexPlugin({
  rsOptions: {
    include: [
      'app/**/*.{ts,tsx}',
      'components/**/*.{ts,tsx}',
      'node_modules/@stylexjs/open-props/**/*.js',
      'node_modules/@my-org/design-system/**/*.js',
    ],
    exclude: ['**/*.test.*', 'app/api/**'],
  },
});
```

## FAQ

### Which bundler mode should I pick?

Webpack (the default export) is the most battle-tested path and supports every
option. Turbopack gives the fastest dev server but needs the PostCSS plugin for
CSS extraction. Rspack is experimental in Next.js itself but works end to end
through the `/rspack` export.

### Do I still need `@stylexjs/babel-plugin` or a `.babelrc`?

No — that is the point of this plugin. StyleX is compiled by the Rust compiler
inside the SWC pipeline, so Next.js never falls back to Babel.

### My styles from `@stylexjs/open-props` (or another library) are missing

Add the package to `transpilePackages` in your Next.js config. It is then
transpiled by Next.js and automatically allowlisted for the StyleX transform.

### Does this work with React Server Components?

Yes. Styles are extracted at build time from both Server and Client Components
into static CSS, so there is no runtime cost either way.

### Is this an official StyleX package?

No. It is a community-maintained alternative to the official tooling and is not
affiliated with or supported by Meta.

## Examples

- [Example repo](https://github.com/Dwlad90/nextjs-app-dir-stylex)
- [CodeSandbox with example repo](https://codesandbox.io/p/github/Dwlad90/nextjs-app-dir-stylex/main)

## Documentation

- [StyleX documentation](https://stylexjs.com)
- [`@stylexswc/rs-compiler` compiler options](https://github.com/Dwlad90/stylex-swc-plugin/tree/develop/crates/stylex-rs-compiler)

## Acknowledgments

This plugin was inspired by
[`stylex-webpack`](https://github.com/SukkaW/stylex-webpack).

## License

MIT — see
[LICENSE](https://github.com/Dwlad90/stylex-swc-plugin/blob/develop/LICENSE)
