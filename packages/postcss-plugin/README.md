# PostCSS plugin with NAPI-RS StyleX compiler integration

> Part of the
> [StyleX SWC Plugin](https://github.com/Dwlad90/stylex-swc-plugin#readme)
> workspace

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
> **Features:** The `include` and `exclude` options are exclusive to
> this NAPI-RS compiler implementation and are not available in the official
> StyleX Babel plugin.

### `include`

- Type: `(string | RegExp)[]`
- Default: auto-discovered
- Description: **RS-compiler Only** An array of glob patterns or regular
  expressions to include specific files for StyleX transformation. When
  specified, only files matching at least one of these patterns will be
  discovered and transformed. Patterns are matched against paths relative to the
  current working directory. Supports regex lookahead/lookbehind for advanced
  filtering.

When omitted, the plugin auto-discovers source files in the project `cwd` and
direct dependencies that use StyleX.

### `exclude`

- Type: `(string | RegExp)[]`
- Optional
- Description: **RS-compiler Only** An array of glob patterns or regular
  expressions to exclude specific files from StyleX transformation. Files
  matching any of these patterns will not be transformed, even if they match an
  `include` pattern. Patterns are matched against paths relative to the current
  working directory. Supports regex lookahead/lookbehind for advanced filtering.

When `include` is omitted, the plugin automatically excludes common build and
dependency folders (for example `node_modules`, `.next`, `dist`, `build`) to
keep discovery focused on source files.

### `rsOptions`

- Type: `StyleXOptions`
- Optional
- Default: `{}`
- Description: StyleX compiler options passed to the StyleX compiler. For
  standard StyleX options, see the
  [official StyleX documentation](https://stylexjs.com/docs/api/configuration/babel-plugin/).

### `useCSSLayers`

- Type: `boolean`
- Optional
- Default: `false`
- Description: Whether to use CSS layers for better style isolation

### `cwd`

- Type: `string`
- Optional
- Default: `process.cwd()`
- Description: Current working directory for resolving files. Dependency paths
  and config resolution use this value.

### `isDev`

- Type: `boolean`
- Optional
- Description: Whether the plugin is running in development mode

### `importSources`

- Type: `Array<string | { from: string, as: string }>`
- Optional
- Description: Override import sources at the PostCSS plugin level.

When provided, takes precedence over `rsOptions.importSources`. When omitted,
falls back to `rsOptions.importSources`, then the built-in defaults
(`@stylexjs/stylex`, `stylex`).

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

## Debugging auto-discovery

Set `STYLEX_POSTCSS_DEBUG=1` to print resolved plugin inputs, including:

- resolved `importSources` and where they came from
- final `include` and `exclude` globs
- discovered dependency directories

## License

MIT — see
[LICENSE](https://github.com/Dwlad90/stylex-swc-plugin/blob/develop/LICENSE)
