# Next.js plugin with NAPI-RS StyleX compiler integration

Next.js plugin for an unofficial
[`napi-rs`](https://github.com/dwlad90/stylex-swc-plugin/tree/develop/crates/stylex-rs-compiler)
compiler that includes the StyleX SWC code transformation under the hood.

## Overview

This package combines two solutions to enhance your Next.js development
experience with StyleX:

### StyleX SWC Plugin

- Integrates StyleX with the SWC compiler, potentially leading to faster build
  times compared to using Babel.
- Maintains high compatibility with official StyleX tests, ensuring a reliable
  experience.
- Integrates seamlessly with Next.js SWC Compiler for a streamlined workflow.

### StyleX NAPI-RS Compiler

- Utilizes NAPI-RS to compile StyleX code, offering advantages over the SWC
  plugin approach.
- Provides access to StyleX metadata and source maps, enabling advanced plugin
  and tool development.

## Why choose this approach?

- Leverage SWC's speed: Benefit from Next.js's default SWC compiler for
  potentially faster build times.
- Maintain StyleX familiarity: The usage of StyleX remains unchanged for
  developers.

## Installation

To install the package, run the following command:

```bash
npm install --save-dev @stylexswc/nextjs-plugin
```

## Usage

This plugin supports both Webpack and Turbopack configurations in Next.js.

### Using with Webpack

For standard Next.js Webpack builds, use the default import:

```javascript
const stylexPlugin = require('@stylexswc/nextjs-plugin');

module.exports = stylexPlugin({
  // StyleX options here
})({
  // Next.js config here
});
```

### Using with Turbopack

> [!IMPORTANT]
> **Turbopack Limitation**: Turbopack does not support webpack plugins ([see Next.js docs](https://nextjs.org/docs/app/api-reference/turbopack#webpack-plugins)). When using Turbopack, the loader only compiles StyleX code but **does not extract CSS**.
>
> **You must configure the PostCSS plugin for CSS extraction.** Install `@stylexswc/postcss-plugin` and configure it in `postcss.config.js`:
>
> ```javascript
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

```typescript
import withStylexTurbopack from '@stylexswc/nextjs-plugin/turbopack';

export default withStylexTurbopack({
  // StyleX options here same as postcss-plugin
  rsOptions: {
      dev: process.env.NODE_ENV === 'development',
  },
})({
  // Next.js config here
  experimental: {
    turbopack: {
      // Additional Turbopack configuration if needed
    },
  },
});
```

> [!NOTE]
> When using Turbopack, the following options are not supported and will be ignored:
>
> - `useCSSLayers`
> - `nextjsMode`
> - `transformCss`
> - `extractCSS`
> - `transformer`

## Plugin Options

### Basic Options

#### `rsOptions`

- Type: `Partial<StyleXOptions>`
- Optional
- Description: StyleX compiler options that will be passed to the NAPI-RS
  compiler. For standard StyleX options, see the
  [official StyleX documentation](https://stylexjs.com/docs/api/configuration/babel-plugin/).

> [!NOTE]
> **New Features:** The `include` and `exclude` options are exclusive to this NAPI-RS compiler implementation and are not available in the official StyleX Babel plugin.

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
- Description: Custom CSS transformation function. Since the plugin injects CSS
  after all loaders, use this to apply PostCSS or other CSS transformations.

### Example Configuration

#### Webpack Configuration

```javascript
const path = require('path');
const stylexPlugin = require('@stylexswc/nextjs-plugin');
const rootDir = __dirname;

module.exports = stylexPlugin({
  // Add any StyleX options here
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

```typescript
import path from 'path';
import withStylexTurbopack from '@stylexswc/nextjs-plugin/turbopack';

const rootDir = __dirname;

export default withStylexTurbopack({
  // Add any StyleX options here
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
  experimental: {
    turbopack: {
      // Additional Turbopack configuration if needed
    },
  },
  // Optionally, add any other Next.js config below
});
```

##### Required: PostCSS Configuration for CSS Extraction

```javascript
// postcss.config.js
const path = require('path');

module.exports = {
  plugins: {
    '@stylexswc/postcss-plugin': {
      include: [
        'app/**/*.{js,jsx,ts,tsx}',
        'components/**/*.{js,jsx,ts,tsx}',
      ],
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

**Include only specific directories:**

```javascript
stylexPlugin({
  rsOptions: {
    include: ['app/**/*.tsx', 'components/**/*.tsx'],
  },
});
```

**Exclude test and build files:**

```javascript
stylexPlugin({
  rsOptions: {
    exclude: ['**/*.test.*', '**/*.spec.*', '**/dist/**', '**/node_modules/**'],
  },
});
```

**Using regular expressions:**

```javascript
stylexPlugin({
  rsOptions: {
    include: [/app\/.*\.tsx$/, /components\/.*\.tsx$/],
    exclude: [/\.test\./, /\.stories\./],
  },
});
```

**Combined include and exclude (exclude takes precedence):**

```javascript
stylexPlugin({
  rsOptions: {
    include: ['app/**/*.{ts,tsx}', 'components/**/*.{ts,tsx}'],
    exclude: ['**/__tests__/**', '**/__mocks__/**', 'app/api/**'],
  },
});
```

**Exclude node_modules except specific packages:**

```javascript
stylexPlugin({
  rsOptions: {
    // Exclude all node_modules except @stylexjs/open-props
    exclude: [/node_modules(?!\/@stylexjs\/open-props)/],
  },
});
```

**Transform only specific packages from node_modules:**

```javascript
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

## Examples

- [Example repo](https://github.com/Dwlad90/nextjs-app-dir-stylex)
- [CodeSandbox with example repo](https://codesandbox.io/p/github/Dwlad90/nextjs-app-dir-stylex/main)

## Documentation

- [StyleX Documentation](https://stylexjs.com)
- [NAPI-RS compiler for StyleX](https://github.com/Dwlad90/stylex-swc-plugin/tree/develop/crates/stylex-rs-compiler)

## Acknowledgments

This plugin was inspired by
[`stylex-webpack`](https://github.com/SukkaW/stylex-webpack).
