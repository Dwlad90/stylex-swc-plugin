# PostCSS plugin with NAPI-RS StyleX compiler integration

`PostCSS plugin` for an unofficial
[`napi-rs`](https://github.com/dwlad90/stylex-swc-plugin/tree/develop/crates/stylex-rs-compiler)
compiler that includes the StyleX SWC code transformation under the hood.

## Installation

To install the package, run the following command:

```bash
npm install --save-dev @stylexswc/postcss-plugin
```

## Usage

Modify `postcss.config.js`. For example:

```js
module.exports = {
  plugins: {
    '@stylexjs/postcss-plugin': {
      include: ['src/**/*.{js,jsx,ts,tsx}'],
    },
    autoprefixer: {},
  },
};
```

Use on of the plugins to process JS/TS files with StyleX code. For example:

```js
/// next.config.js
const path = require('path');
const stylexPlugin = require('@stylexswc/nextjs-plugin');
const rootDir = __dirname;

module.exports = stylexPlugin({
  // Add any StyleX options here
  rsOptions: {
    aliases: {
      '@/*': [path.join(rootDir, '*')],
    },
    unstable_moduleResolution: {
      type: 'commonJS',
    },
  },
  // It's important to prevent creating a new CSS file with StyleX classes twice
  extractCSS: false,
})({
  transpilePackages: ['@stylexjs/open-props'],
  // Optionally, add any other Next.js config below
});
```

> [!WARNING]
> Each plugin of `@stylexswc` namespace accepts an `extractCSS`
> option to control CSS extraction. When using the `postcss` plugin, this option
> should be set to `false` to avoid double generation of CSS files with StyleX
> styles.

&nbsp;

> [!NOTE]
> This approach requires transpiling JS/TS files with StyleX code twice:
> first the source code and then using the PostCSS plugin. To avoid this
> behavior when using `NextJS`, use the regular `@stylexswc/nextjs-plugin`
> passing the `transformCss` parameter to transform the generated CSS if it's
> possible, for example:
>
> ```js
> /// next.config.js
>
> //...other code
> transformCss: async css => {
>   const postcss = require('postcss');
>   const result = await postcss([require('autoprefixer')]).process(css);
>   return result.css;
> },
> //...other code
> ```

Add the following CSS file to your project:

```css
/*[fileName].css*/

/**
 * The @stylex directive is used by the @stylexjs/postcss-plugin.
 * It is automatically replaced with generated CSS during builds.
 */
@stylex;
```

And import it in your JS/TS files:

```js
import '[fileName].css';
```

## Plugin Options

The plugin accepts the following configuration options:

> [!NOTE]
> **Features:** The `include` and `exclude` options are exclusive to this NAPI-RS compiler implementation and are not available in the official StyleX Babel plugin.

### `include`

- Type: `(string | RegExp)[]`
- Optional
- Description: **RS-compiler Only** An array of glob patterns or regular expressions to include specific files for StyleX transformation.
  When specified, only files matching at least one of these patterns will be discovered and transformed.
  Patterns are matched against paths relative to the current working directory.
  Supports regex lookahead/lookbehind for advanced filtering.

### `exclude`

- Type: `(string | RegExp)[]`
- Optional
- Description: **RS-compiler Only** An array of glob patterns or regular expressions to exclude specific files from StyleX transformation.
  Files matching any of these patterns will not be transformed, even if they match an `include` pattern.
  Patterns are matched against paths relative to the current working directory.
  Supports regex lookahead/lookbehind for advanced filtering.

### `rsOptions`

- Type: `StyleXOptions`
- Optional
- Default: `{}`
- Description: StyleX compiler options passed to the StyleX compiler.
  For standard StyleX options, see the [official StyleX documentation](https://stylexjs.com/docs/api/configuration/babel-plugin/).

### `useCSSLayers`

- Type: `boolean`
- Optional
- Default: `false`
- Description: Whether to use CSS layers for better style isolation

### `cwd`

- Type: `string`
- Optional
- Default: `process.cwd()`
- Description: Current working directory for resolving files

### `isDev`

- Type: `boolean`
- Optional
- Description: Whether the plugin is running in development mode

## Path Filtering Examples

**Include only specific directories:**

```javascript
{
  '@stylexswc/postcss-plugin': {
    include: ['src/**/*.{ts,tsx}', 'components/**/*.{ts,tsx}'],
  },
}
```

**Exclude test and story files:**

```javascript
{
  '@stylexswc/postcss-plugin': {
    include: ['src/**/*.{ts,tsx}'],
    exclude: ['**/*.test.*', '**/*.stories.*', '**/__tests__/**'],
  },
}
```

**Using regular expressions:**

```javascript
{
  '@stylexswc/postcss-plugin': {
    include: ['src/**/*.{ts,tsx}', /components\/.*\.tsx$/],
    exclude: [/\.test\./, /\.stories\./],
  },
}
```

**Exclude node_modules except specific packages (using lookahead):**

```javascript
{
  '@stylexswc/postcss-plugin': {
    include: ['src/**/*.{ts,tsx}', 'node_modules/**/*.js'],
    // Exclude all node_modules except @stylexjs/open-props
    exclude: [/node_modules(?!\/@stylexjs\/open-props)/],
  },
}
```

**Transform only specific packages from node_modules:**

```javascript
{
  '@stylexswc/postcss-plugin': {
    include: [
      'src/**/*.{ts,tsx}',
      'node_modules/@stylexjs/open-props/**/*.js',
      'node_modules/@my-org/design-system/**/*.js',
    ],
    exclude: ['**/*.test.*'],
  },
}
```
