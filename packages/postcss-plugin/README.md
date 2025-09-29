# PostCSS plugin with NAPI-RS StyleX compiler integration

`PostCSS plugin` for an unofficial
[`napi-rs`](https://github.com/dwlad90/stylex-swc-plugin/tree/develop/crates/stylex-rs-compiler)
compiler that includes the StyleX SWC code transformation under the hood.

## Installation

To install the package, run the following command:

```bash
npm install --save-dev @toss/stylexswc-postcss-plugin
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
const stylexPlugin = require('@toss/stylexswc-nextjs-plugin');
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
> behavior when using `NextJS`, use the regular `@toss/stylexswc-nextjs-plugin`
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

### `rsOptions`

- Type: `StyleXOptions`
- Optional
- Default: `{}`
- Description: StyleX compiler options passed to the StyleX compiler

### `useCSSLayers`

- Type: `boolean`
- Optional
- Default: `false`
- Description: Whether to use CSS layers for better style isolation

### `exclude`

- Type: `string[]`
- Optional
- Description: Array of glob patterns to exclude from processing

### `include`

- Type: `string[]`
- Optional
- Description: Array of glob patterns to include for processing

### `cwd`

- Type: `string`
- Optional
- Default: `process.cwd()`
- Description: Current working directory for resolving files

### `isDev`

- Type: `boolean`
- Optional
- Description: Whether the plugin is running in development mode
