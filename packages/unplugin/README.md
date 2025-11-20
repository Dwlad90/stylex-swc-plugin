# Unplugin with NAPI-RS StyleX compiler integration

`Uplugin` for an unofficial
[`napi-rs`](https://github.com/dwlad90/stylex-swc-plugin/tree/develop/crates/stylex-rs-compiler)
compiler that includes the StyleX SWC code transformation under the hood.

## Installation

To install the package, run the following command:

```bash
npm install --save-dev @stylexswc/unplugin
```

## Usage

To use the plugin, you need to add it to your build tool configuration.

For every plugin have an example of how to use it in
[`apps/{pluginName}-unplugin-example`](https://github.com/Dwlad90/stylex-swc-plugin/tree/develop/apps)
folder.

## Plugins

<details>
<summary>Vite</summary><br>

```ts
// vite.config.ts
import StylexRsPlugin from '@stylexswc/unplugin/vite';

export default defineConfig({
  plugins: [
    StylexRsPlugin({
      /* options */
    }),
  ],
});
```

<br></details>

<details>
<summary>Rollup</summary><br>

```ts
// rollup.config.js
import StylexRsPlugin from '@stylexswc/unplugin/rollup';

export default {
  plugins: [
    StylexRsPlugin({
      /* options */
    }),
  ],
};
```

<br></details>

<details>
<summary>Webpack</summary><br>

```ts
// webpack.config.js
module.exports = {
  /* ... */
  plugins: [
    require('@stylexswc/unplugin/webpack')({
      /* options */
    }),
  ],
};
```

<br></details>

<details>
<summary>Rspack</summary><br>

```ts
// rspack.config.js
module.exports = {
  /* ... */
  plugins: [
    require('@stylexswc/unplugin/rspack')({
      /* options */
    }),
  ],
};
```

<br></details>

<details>
<summary>Nuxt</summary><br>

```ts
// nuxt.config.js
export default defineNuxtConfig({
  modules: [
    [
      '@stylexswc/unplugin/nuxt',
      {
        /* options */
      },
    ],
  ],
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
    plugins: [
      require('@stylexswc/unplugin/webpack')({
        /* options */
      }),
    ],
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
- Description: StyleX compiler options that will be passed to the NAPI-RS compiler.
  For standard StyleX options, see the [official StyleX documentation](https://stylexjs.com/docs/api/configuration/babel-plugin/).

> [!NOTE]
> **New Features:** The `include` and `exclude` options are exclusive to this NAPI-RS compiler implementation and are not available in the official StyleX Babel plugin.

##### `rsOptions.include`

- Type: `(string | RegExp)[]`
- Optional
- Description: **RS-compiler Only** An array of glob patterns or regular expressions to include specific files for StyleX transformation.
  When specified, only files matching at least one of these patterns will be transformed.
  Patterns are matched against paths relative to the current working directory.

##### `rsOptions.exclude`

- Type: `(string | RegExp)[]`
- Optional
- Description: **RS-compiler Only** An array of glob patterns or regular expressions to exclude specific files from StyleX transformation.
  Files matching any of these patterns will not be transformed, even if they match an `include` pattern.
  Patterns are matched against paths relative to the current working directory.

#### `fileName`

- Type: `string`
- Default: `'stylex.css'`
- Description: Name of the generated CSS file.

#### `useCSSLayers`

- Type: `boolean`
- Default: `false`
- Description: Enables CSS cascade layers support for better style isolation.

#### `extractCSS`

- Type: `boolean`
- Default: `true`
- Description: Controls whether CSS should be extracted into a separate file.

#### `pageExtensions`

- Type: `string[]`
- Default: `['js', 'jsx', 'ts', 'tsx', 'mjs', 'mts']`
- Description: File extensions to process for StyleX transformations.

#### `useViteCssPipeline`

- Type: `boolean`
- Default: `false`
- Description: **(Vite only)** Integrates StyleX-generated CSS into Vite's CSS processing pipeline as a virtual module. When enabled, StyleX CSS will be processed through Vite's CSS transformers (including PostCSS, LightningCSS, etc.) and benefit from proper HMR support.

##### Benefits

- **CSS Processing**: Generated StyleX CSS goes through Vite's CSS pipeline (PostCSS, LightningCSS, etc.)
- **Better HMR**: CSS updates are handled through Vite's native CSS HMR with proper source maps
- **Consistent Output**: All CSS follows the same processing rules and bundling strategy
- **Build Optimization**: CSS can be code-split and optimized alongside other stylesheets

##### How to Use

```typescript
// vite.config.ts
import StylexRsPlugin from '@stylexswc/unplugin/vite';
import { defineConfig } from 'vite';

export default defineConfig({
  plugins: [
    StylexRsPlugin({
      useViteCssPipeline: true,
    }),
  ],
});
```

Then import the virtual CSS module in your entry file:

```typescript
// src/main.ts
import 'virtual:stylex.css';
import { App } from './App';
```

**TypeScript Support:**

For TypeScript projects, add the type definition to your `tsconfig.json`:

```json
{
  "compilerOptions": {
    "types": ["@stylexswc/unplugin/virtual-css"]
  }
}
```

> [!NOTE]
> When `useViteCssPipeline` is enabled, you need to explicitly import `virtual:stylex.css` in your application. The plugin will no longer inject CSS automatically into the HTML.

> [!IMPORTANT]
> **Reset CSS and Other Global Styles**
>
> The `virtual:stylex.css` module should only contain StyleX-generated CSS. If you need to include reset CSS, global styles, or other non-StyleX CSS, import them from separate CSS files:
>
> ```typescript
> // src/main.ts
> import './reset.css';        // Your reset CSS
> import './global.css';       // Other global styles
> import 'virtual:stylex.css'; // StyleX-generated CSS
> ```
>
> **Do not** put reset CSS or other styles inside the virtual module, as they should be managed separately from the StyleX-generated styles. See the examples in [`apps/vite-unplugin-virtual-css-example`](../../apps/vite-unplugin-virtual-css-example) and [`apps/vue-unplugin-virtual-css-example`](../../apps/vue-unplugin-virtual-css-example) for reference.

### Example Configuration

```typescript
// vite.config.ts
import StylexRsPlugin from '@stylexswc/unplugin/vite';
import { defineConfig } from 'vite';

export default defineConfig({
  plugins: [
    StylexRsPlugin({
      rsOptions: {
        dev: process.env.NODE_ENV !== 'production',
        // Include only specific directories
        include: ['src/**/*.{ts,tsx}', 'components/**/*.{ts,tsx}'],
        // Exclude test files and stories
        exclude: ['**/*.test.*', '**/*.stories.*', '**/__tests__/**'],
      },
      useCSSLayers: true,
      fileName: 'stylex.[hash].css',
    }),
  ],
});
```

### Path Filtering Examples

**Include only specific directories:**

```typescript
StylexRsPlugin({
  rsOptions: {
    include: ['src/**/*.tsx', 'app/**/*.tsx'],
  },
})
```

**Exclude test and build files:**

```typescript
StylexRsPlugin({
  rsOptions: {
    exclude: ['**/*.test.*', '**/*.spec.*', '**/dist/**', '**/node_modules/**'],
  },
})
```

**Using RegExp with lookahead/lookbehind - exclude node_modules except specific packages:**

```typescript
StylexRsPlugin({
  rsOptions: {
    // Exclude all node_modules except @stylexjs packages
    exclude: [/node_modules(?!\/@stylexjs)/],
  },
})
```

**Transform only specific packages from node_modules:**

```typescript
StylexRsPlugin({
  rsOptions: {
    include: [
      'src/**/*.{ts,tsx}',
      'node_modules/@stylexjs/open-props/**/*.js',
      'node_modules/@my-org/design-system/**/*.js',
    ],
    exclude: ['**/*.test.*'],
  },
})
```

**Using regular expressions:**

```typescript
StylexRsPlugin({
  rsOptions: {
    include: [/src\/.*\.tsx$/],
    exclude: [/\.test\./, /\.stories\./],
  },
})
```

**Combined include and exclude (exclude takes precedence):**

```typescript
StylexRsPlugin({
  rsOptions: {
    include: ['src/**/*.{ts,tsx}'],
    exclude: ['**/__tests__/**', '**/__mocks__/**'],
  },
})
```

**Exclude node_modules except specific packages:**

```typescript
StylexRsPlugin({
  rsOptions: {
    // Exclude all node_modules except @stylexjs/open-props
    exclude: [/node_modules(?!\/@stylexjs\/open-props)/],
  },
})
```

**Transform only specific packages from node_modules:**

```typescript
StylexRsPlugin({
  rsOptions: {
    include: [
      'src/**/*.{ts,tsx}',
      'node_modules/@stylexjs/open-props/**/*.js',
      'node_modules/@my-org/design-system/**/*.js',
    ],
    exclude: ['**/*.test.*'],
  },
})
```
