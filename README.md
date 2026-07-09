# StyleX SWC Compiler — StyleX in Rust

[![GitHub license](https://img.shields.io/badge/license-MIT-green.svg)](https://github.com/Dwlad90/stylex-swc-plugin/blob/develop/LICENSE) [![npm version](https://img.shields.io/npm/v/@stylexswc/rs-compiler.svg?style=flat)](https://www.npmjs.com/package/@stylexswc/rs-compiler) ![GitHub tag check runs](https://img.shields.io/github/check-runs/Dwlad90/stylex-swc-plugin/0.17.1?label=Release%20status) ![GitHub Actions Workflow Status](https://img.shields.io/github/actions/workflow/status/Dwlad90/stylex-swc-plugin/pr-validation.yml?branch=develop&label=Project%20Health) <!-- stylex-compatibility:start -->[![StyleX compatibility](https://img.shields.io/badge/StyleX%20compatibility-v0.19.0-blue)](https://stylexjs.com/blog)<!-- stylex-compatibility:end -->

> **Rust/NAPI-RS/SWC compiler for [StyleX](https://stylexjs.com) that replaces
> the official Babel transform and adds fast integrations for Next.js, Vite,
> webpack, Rspack, Rollup, Turbopack, PostCSS, Jest, and custom Node.js
> tooling.**

> [!IMPORTANT]
>
> This is a **community-written** implementation of StyleX tooling. It aims to
> provide a high-performance alternative to the official StyleX tooling and is
> not affiliated with or officially supported by Meta.

StyleX is Meta's CSS-in-JS library for compile-time atomic CSS extraction. This
project is a community implementation of the StyleX compiler written in Rust and
exposed to Node.js through [NAPI-RS](https://napi.rs). It keeps the StyleX
authoring API unchanged while moving compilation from Babel to
[SWC](https://swc.rs)-native tooling.

Nothing changes in how you write StyleX: `stylex.create` and `stylex.props` work
exactly as documented. Only your build gets faster, and on Next.js you drop the
Babel fallback entirely and keep the fast SWC toolchain.

## What problems does this solve?

| Developer need                   | How this project helps                                                                                   |
| -------------------------------- | -------------------------------------------------------------------------------------------------------- |
| Compile StyleX without Babel     | Replaces the official StyleX Babel plugin with a Rust compiler running on SWC                            |
| Keep Next.js on the SWC pipeline | Avoids adding `.babelrc`, which would force Next.js files through Babel                                  |
| Use StyleX with modern bundlers  | Provides plugins for Next.js, Vite, webpack, Rspack, Rollup, Turbopack, esbuild, Farm, Rsbuild, and more |
| Test StyleX components in Jest   | Provides `@stylexswc/jest`, a Jest transformer backed by the same Rust compiler                          |
| Build custom StyleX tooling      | Exposes `@stylexswc/rs-compiler` with `transform()`, metadata, CSS extraction, source maps, and filters  |

## Performance

This compiler transforms StyleX **2x to 5x faster per file** than the official
`@stylexjs/babel-plugin`, measured with the benchmark suite in
[`crates/stylex-rs-compiler/benchmark`](./crates/stylex-rs-compiler/benchmark).
The entire transform — parsing, evaluation, and code generation — is
implemented in Rust and runs as a native Node.js addon on SWC, with none of the
JavaScript-side AST overhead that Babel carries. The advantage grows with the
workload: complex `stylex.create` calls, theme creation, and large design-token
sheets see the biggest wins.

Per-file speed is only half of the story. On SWC-based frameworks like Next.js,
using this compiler keeps Babel out of the pipeline entirely. Adding the
official plugin drags every file through Babel as soon as a `.babelrc` appears,
while this toolchain stays on the fast SWC path.

## Which package do I need?

| Your setup                                | Install                                                      | Notes                                                     |
| ----------------------------------------- | ------------------------------------------------------------ | --------------------------------------------------------- |
| Next.js (Webpack, Rspack, Turbopack)      | [`@stylexswc/nextjs-plugin`](./packages/nextjs-plugin)       | One config surface for all three bundlers                 |
| Vite, esbuild, Farm, Rsbuild, Nuxt, Astro | [`@stylexswc/unplugin`](./packages/unplugin)                 | Universal plugin with per-bundler entry points            |
| webpack (standalone)                      | [`@stylexswc/webpack-plugin`](./packages/webpack-plugin)     | Loader ordering and cache-group control                   |
| Rspack (standalone)                       | [`@stylexswc/rspack-plugin`](./packages/rspack-plugin)       | Native Rspack rule registration                           |
| Rollup                                    | [`@stylexswc/rollup-plugin`](./packages/rollup-plugin)       | Lightning CSS post-processing built in                    |
| Turbopack (raw loader)                    | [`@stylexswc/turbopack-plugin`](./packages/turbopack-plugin) | Usually driven via `nextjs-plugin`                        |
| PostCSS pipeline / Turbopack CSS          | [`@stylexswc/postcss-plugin`](./packages/postcss-plugin)     | Replaces an `@stylex;` directive with generated CSS       |
| Jest                                      | [`@stylexswc/jest`](./packages/jest)                         | Transformer so StyleX components run in tests             |
| Custom tooling                            | [`@stylexswc/rs-compiler`](./crates/stylex-rs-compiler)      | The compiler itself: `transform()`, metadata, source maps |

Every plugin drives the same Rust compiler under the hood, so options like
`rsOptions`, `include`/`exclude` filtering, and CSS extraction behave
consistently across build tools.

## Quick Start

### Next.js

```bash
npm install --save-dev @stylexswc/nextjs-plugin
npm install @stylexjs/stylex
```

```js
// next.config.js
const stylexPlugin = require('@stylexswc/nextjs-plugin');

module.exports = stylexPlugin({
  rsOptions: {
    dev: process.env.NODE_ENV !== 'production',
  },
})({
  // Next.js config
});
```

Using Turbopack? Import from `@stylexswc/nextjs-plugin/turbopack` instead and
add `@stylexswc/postcss-plugin` for CSS extraction — see the
[plugin README](./packages/nextjs-plugin/README.md#using-with-turbopack).

### Vite (and other bundlers via unplugin)

```bash
npm install --save-dev @stylexswc/unplugin
npm install @stylexjs/stylex
```

```ts
// vite.config.ts
import StylexRsPlugin from '@stylexswc/unplugin/vite';
import { defineConfig } from 'vite';

export default defineConfig({
  plugins: [
    StylexRsPlugin({
      rsOptions: {
        dev: process.env.NODE_ENV !== 'production',
      },
    }),
  ],
});
```

Working example apps for every integration — Next.js (Webpack, Rspack,
Turbopack), Vite, webpack, Rollup, Rspack, Rsbuild, Farm, esbuild, Vue, and
Solid — live in the [`apps/`](./apps) directory.

## Compatibility

- Tracks official StyleX releases; currently compatible with **StyleX v0.19.0**
  (see the badge above, updated automatically)
- Validated against the official StyleX test suite
- Node.js **20 or newer**
- Prebuilt binaries for macOS (x64, arm64), Linux (glibc and musl, x64, arm64),
  and Windows (x64, arm64) — no Rust toolchain needed to install

## FAQ

### Do I have to know Rust to use this?

No. Everything installs from npm with prebuilt native binaries, and all
configuration happens in plain JavaScript or TypeScript config files. Rust is
only needed to contribute to the compiler itself.

### Does my StyleX code change?

No. This project swaps the build-time compiler, not the API. Your
`stylex.create`, `stylex.props`, themes, and tokens keep working unchanged, and
the generated atomic CSS is compatible with the official output.

### How is this different from the official StyleX toolchain?

The official transform runs as a Babel plugin. This one is a Rust
reimplementation running on SWC, which makes per-file transforms 2x to 5x
faster and — on SWC-based frameworks like Next.js — avoids adding Babel to the
pipeline at all.
It also adds compiler-only features such as `include`/`exclude` file filtering,
SWC WASM plugin chaining, and `inputSourceMap` chaining.

### How do I use StyleX with Next.js without Babel?

Install [`@stylexswc/nextjs-plugin`](./packages/nextjs-plugin) and wrap your
Next.js config with it — see the [Quick Start](#quick-start). StyleX is then
compiled by the Rust compiler inside the SWC pipeline, so no `.babelrc` is
needed and Next.js never falls back to Babel. This works with Webpack, Rspack,
and Turbopack, with the App Router and the Pages Router, and with React Server
Components.

### Is it production-ready?

The compiler is validated against the official StyleX test suite, ships prebuilt
binaries for all major platforms, and powers the example apps in this repository
across ten build tools. As with any community project, pin your versions and
report regressions in the
[issue tracker](https://github.com/Dwlad90/stylex-swc-plugin/issues).

## Development

```bash
# Clone the repository
git clone https://github.com/Dwlad90/stylex-swc-plugin.git

# Setup development environment (Node.js and Rust)
make setup

# Build all packages
make build

# Run tests
make test

# Run quality checks (format, lint, typecheck)
make quick-check
```

Run `make help` for the full command list, or use `pnpm` directly
(`pnpm install`, `pnpm build`, `pnpm test`, `pnpm lint:check`,
`pnpm format:check`, `pnpm typecheck`).

Curious how the compiler is organized internally? The Rust workspace is a
layered graph of single-concern crates — see
[Project Structure](./guidelines/STRUCTURE.md) for the crate map and dependency
graph.

## Documentation

- [StyleX documentation](https://stylexjs.com)
- [SWC documentation](https://swc.rs)
- [NAPI-RS documentation](https://napi.rs)

## Contributing

Contributions are welcome! Please read the guidelines in
[`guidelines/`](./guidelines) and submit pull requests to the `develop` branch.

## License

MIT Licensed. See [LICENSE](./LICENSE) for details.
