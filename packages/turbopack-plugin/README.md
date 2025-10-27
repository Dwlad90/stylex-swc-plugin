# Turbopack Loader with NAPI-RS StyleX compiler integration

`Turbopack loader` for an unofficial
[`napi-rs`](https://github.com/dwlad90/stylex-swc-plugin/tree/develop/crates/stylex-rs-compiler)
compiler that includes the StyleX SWC code transformation under the hood.

## Installation

To install the package, run the following command:

```bash
npm install --save-dev @stylexswc/turbopack-plugin
```

Please install `@stylexswc/rs-compiler` if you haven't done so already:

```bash
npm install --save-dev @stylexswc/rs-compiler
```

## Usage

> [!IMPORTANT] **Turbopack Limitation**: Turbopack does not support webpack
> plugins
> ([see Next.js docs](https://nextjs.org/docs/app/api-reference/turbopack#webpack-plugins)).
> This loader only compiles StyleX code but **does not extract CSS**.
>
> For CSS extraction, you must use the
> [`@stylexswc/postcss-plugin`](../postcss-plugin) in your `postcss.config.js`:
>
> ```javascript
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
- Description: StyleX compiler options that will be passed to the NAPI-RS
  compiler. For standard StyleX options, see the
  [official StyleX documentation](https://stylexjs.com/docs/api/configuration/babel-plugin/).

> [!NOTE] **New Features:** The `include` and `exclude` options are exclusive to
> this NAPI-RS compiler implementation and are not available in the official
> StyleX Babel plugin.

##### `rsOptions.include`

- Type: `(string | RegExp)[]`
- Optional
- Description: **RS-compiler Only** An array of glob patterns or regular
  expressions to include specific files for StyleX transformation. When
  specified, only files matching at least one of these patterns will be
  transformed. Patterns are matched against paths relative to the current
  working directory.

##### `rsOptions.exclude`

- Type: `(string | RegExp)[]`
- Optional
- Description: **RS-compiler Only** An array of glob patterns or regular
  expressions to exclude specific files from StyleX transformation. Files
  matching any of these patterns will not be transformed, even if they match an
  `include` pattern. Patterns are matched against paths relative to the current
  working directory.

#### `stylexImports`

- Type: `Array<string | { as: string, from: string }>`
- Default: `['stylex', '@stylexjs/stylex']`
- Description: Specifies where StyleX will be imported from. Supports both
  string paths and import aliases.

#### `useCSSLayers`

- Type: `boolean`
- Default: `false`
- Description: Enables CSS cascade layers support for better style isolation.

#### `nextjsMode`

- Type: `boolean`
- Default: `false`
- Description: Enables Next.js-specific optimizations and compatibility
  features.

#### `extractCSS`

- Type: `boolean`
- Optional
- Default: `true`
- Description: Controls whether CSS should be extracted into a separate file

### Advanced Options

#### `transformCss`

- Type:
  `(css: string, filePath: string | undefined) => string | Buffer | Promise<string | Buffer>`
- Optional
- Description: Custom CSS transformation function. Since the loader injects CSS
  after all loaders, use this to apply PostCSS or other CSS transformations.

### Example Configuration

```typescript
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
              // Include only specific directories
              include: ['app/**/*.{ts,tsx}', 'components/**/*.{ts,tsx}'],
              // Exclude test files and stories
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

#### Path Filtering Examples

**Include only specific directories:**

```typescript
import type { NextConfig } from 'next';

const nextConfig: NextConfig = {
  experimental: {
    turbo: {
      rules: {
        '*.tsx': {
          loaders: ['@stylexswc/turbopack-plugin/loader'],
          options: {
            rsOptions: {
              include: ['app/**/*.{ts,tsx}', 'components/**/*.{ts,tsx}'],
            },
          },
        },
      },
    },
  },
};

export default nextConfig;
```

**Exclude test and build files:**

```typescript
import type { NextConfig } from 'next';

const nextConfig: NextConfig = {
  experimental: {
    turbo: {
      rules: {
        '*.tsx': {
          loaders: ['@stylexswc/turbopack-plugin/loader'],
          options: {
            rsOptions: {
              exclude: ['**/*.test.*', '**/*.spec.*', '**/dist/**'],
            },
          },
        },
      },
    },
  },
};

export default nextConfig;
```

**Using regular expressions:**

```typescript
import type { NextConfig } from 'next';

const nextConfig: NextConfig = {
  experimental: {
    turbo: {
      rules: {
        '*.tsx': {
          loaders: ['@stylexswc/turbopack-plugin/loader'],
          options: {
            rsOptions: {
              include: [/app\/.*\.tsx$/, /components\/.*\.tsx$/],
              exclude: [/\.test\./, /\.stories\./],
            },
          },
        },
      },
    },
  },
};

export default nextConfig;
```

**Combined include and exclude (exclude takes precedence):**

```typescript
import type { NextConfig } from 'next';

const nextConfig: NextConfig = {
  experimental: {
    turbo: {
      rules: {
        '*.tsx': {
          loaders: ['@stylexswc/turbopack-plugin/loader'],
          options: {
            rsOptions: {
              include: ['app/**/*.{ts,tsx}', 'components/**/*.{ts,tsx}'],
              exclude: ['**/__tests__/**', '**/__mocks__/**'],
            },
          },
        },
      },
    },
  },
};

export default nextConfig;
```

## Documentation

- [StyleX Documentation](https://stylexjs.com)
- [NAPI-RS compiler for StyleX](https://github.com/Dwlad90/stylex-swc-plugin/tree/develop/crates/stylex-rs-compiler)

## Acknowledgments

This loader was inspired by
[`stylex-webpack`](https://github.com/SukkaW/stylex-webpack).
