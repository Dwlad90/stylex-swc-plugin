# Unplugin with NAPI-RS StyleX compiler integration

`Uplugin` for an unofficial
[`napi-rs`](https://github.com/dwlad90/stylex-swc-plugin/tree/develop/crates/stylex-rs-compiler)
compiler that includes the StyleX SWC code transformation under the hood.

## Installation

To install the package, run the following command:

```bash
npm install --save-dev @toss/stylexswc-unplugin
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
import StylexRsPlugin from '@toss/stylexswc-unplugin/vite';

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
import StylexRsPlugin from '@toss/stylexswc-unplugin/rollup';

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
    require('@toss/stylexswc-unplugin/webpack')({
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
      '@toss/stylexswc-unplugin/nuxt',
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
      require('@toss/stylexswc-unplugin/webpack')({
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
import StylexRsPlugin from '@toss/stylexswc-unplugin/esbuild';

build({
  plugins: [StylexRsPlugin()],
});
```

<br></details>
