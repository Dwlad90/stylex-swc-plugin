# @stylexswc/turbopack-plugin

> StyleX loader for Next.js Turbopack, powered by a Rust compiler (NAPI-RS +
> SWC). Part of the
> [StyleX SWC Plugin](https://github.com/Dwlad90/stylex-swc-plugin#readme)
> workspace.

This loader compiles [StyleX](https://stylexjs.com) code in Turbopack builds
with
[`@stylexswc/rs-compiler`](https://www.npmjs.com/package/@stylexswc/rs-compiler),
a Rust implementation of the StyleX transform, instead of the official Babel
plugin. Your StyleX code stays exactly the same — only the build step changes,
with per-file transforms 2x to 5x faster than Babel
([performance](https://github.com/Dwlad90/stylex-swc-plugin#performance)).

This is a community project and is not affiliated with Meta. It tracks the
official StyleX releases
<!-- stylex-compatibility:start -->(currently compatible with StyleX v0.19.0)<!-- stylex-compatibility:end -->
and requires Node.js 20 or newer. Most Next.js projects should reach for
[`@stylexswc/nextjs-plugin`](https://www.npmjs.com/package/@stylexswc/nextjs-plugin),
whose `/turbopack` export configures this loader for you.

## Installation

```bash
npm install --save-dev @stylexswc/turbopack-plugin
```

The Rust compiler (`@stylexswc/rs-compiler`) is installed automatically as a
dependency. Your application still needs the StyleX runtime:

```bash
npm install @stylexjs/stylex
```

## Usage

> [!IMPORTANT]
> Turbopack does not support webpack plugins
> ([see Next.js docs](https://nextjs.org/docs/app/api-reference/turbopack#webpack-plugins)),
> so this loader only compiles StyleX code — it does not extract CSS.
>
> For CSS extraction, configure
> [`@stylexswc/postcss-plugin`](https://github.com/Dwlad90/stylex-swc-plugin/tree/develop/packages/postcss-plugin#readme)
> in your `postcss.config.js`:
>
> ```js
> // postcss.config.js
> module.exports = {
>   plugins: {
>     '@stylexswc/postcss-plugin': {
>       include: [
>         'app/**/*.{js,jsx,ts,tsx}',
>         'components/**/*.{js,jsx,ts,tsx}',
>       ],
>       rsOptions: {
>         dev: process.env.NODE_ENV === 'development',
>       },
>     },
>     autoprefixer: {},
>   },
> };
> ```

Modify your `next.config.ts` to configure the loader for Turbopack:

```ts
import type { NextConfig } from 'next';

const nextConfig: NextConfig = {
  experimental: {
    turbo: {
      rules: {
        '*.tsx': {
          loaders: ['@stylexswc/turbopack-plugin/loader'],
          options: {
            rsOptions: {
              dev: process.env.NODE_ENV !== 'production',
              // ... other StyleX options
            },
          },
        },
      },
    },
  },
};

export default nextConfig;
```

## Loader Options

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

#### `nextjsMode`

- Type: `boolean`
- Default: `false`
- Description: Enables Next.js-specific optimizations and compatibility
  features.

#### `extractCSS`

- Type: `boolean`
- Optional
- Default: `true`
- Description: Controls whether CSS should be extracted into a separate file.
  Under Turbopack the loader cannot extract CSS itself (see the note above), so
  extraction is handled by the PostCSS plugin.

### Advanced Options

#### `transformCss`

- Type:
  `(css: string, filePath: string | undefined) => string | Buffer | Promise<string | Buffer>`
- Optional
- Description: Custom CSS post-processing hook, applied when the loader emits
  CSS. Use it to run PostCSS or other CSS transformations.

### Example Configuration

```ts
import type { NextConfig } from 'next';

const nextConfig: NextConfig = {
  experimental: {
    turbo: {
      rules: {
        '*.tsx': {
          loaders: ['@stylexswc/turbopack-plugin/loader'],
          options: {
            rsOptions: {
              dev: process.env.NODE_ENV !== 'production',
              include: ['app/**/*.{ts,tsx}', 'components/**/*.{ts,tsx}'],
              exclude: ['**/*.test.*', '**/*.stories.*', '**/__tests__/**'],
            },
            stylexImports: ['@stylexjs/stylex'],
          },
        },
      },
    },
  },
};

export default nextConfig;
```

### Path Filtering Examples

Include only specific directories:

```ts
options: {
  rsOptions: {
    include: ['app/**/*.{ts,tsx}', 'components/**/*.{ts,tsx}'],
  },
},
```

Exclude test and build files:

```ts
options: {
  rsOptions: {
    exclude: ['**/*.test.*', '**/*.spec.*', '**/dist/**'],
  },
},
```

Using regular expressions (exclude always takes precedence over include):

```ts
options: {
  rsOptions: {
    include: [/app\/.*\.tsx$/, /components\/.*\.tsx$/],
    exclude: [/\.test\./, /\.stories\./],
  },
},
```

## FAQ

### Why are my styles missing in the browser?

The loader compiles StyleX code but cannot extract CSS under Turbopack. Make
sure `@stylexswc/postcss-plugin` is configured in `postcss.config.js` and that
your app imports a CSS file containing the `@stylex;` directive.

### Should I use this package directly or `@stylexswc/nextjs-plugin`?

Prefer `@stylexswc/nextjs-plugin` — its `/turbopack` export sets up this loader
and keeps one config surface across Webpack, Rspack, and Turbopack. Use this
package directly only if you need raw control over Turbopack rules.

### Do I still need `@stylexjs/babel-plugin`?

No. The loader replaces the Babel plugin in your build. You only keep
`@stylexjs/stylex` as your app's runtime dependency, and your `stylex.create` /
`stylex.props` code does not change.

### Is this an official StyleX package?

No. It is a community-maintained alternative to the official tooling and is not
affiliated with or supported by Meta.

## Documentation

- [StyleX documentation](https://stylexjs.com)
- [`@stylexswc/rs-compiler` compiler options](https://github.com/Dwlad90/stylex-swc-plugin/tree/develop/crates/stylex-rs-compiler)

## Acknowledgments

This loader was inspired by
[`stylex-webpack`](https://github.com/SukkaW/stylex-webpack).

## License

MIT — see
[LICENSE](https://github.com/Dwlad90/stylex-swc-plugin/blob/develop/LICENSE)
