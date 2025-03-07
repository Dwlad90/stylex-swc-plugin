# StyleX in Rust &middot; [![GitHub license](https://img.shields.io/badge/license-MIT-green.svg)](https://github.com/Dwlad90/stylex-swc-plugin/blob/develop/LICENSE) [![npm version](https://img.shields.io/npm/v/@stylexswc/rs-compiler.svg?style=flat)](https://www.npmjs.com/package/@stylexswc/rs-compiler) ![GitHub tag check runs](https://img.shields.io/github/check-runs/Dwlad90/stylex-swc-plugin/0.6.6?label=Release%20status) ![GitHub Actions Workflow Status](https://img.shields.io/github/actions/workflow/status/Dwlad90/stylex-swc-plugin/pr-validation.yml?branch=develop&label=Project%20Health)

This is a monorepo for an unofficial [`napi-rs`](https://napi.rs/) compiler and
an [SWC](https://swc.rs/) plugin for
[StyleX](https://github.com/facebook/stylex). Using SWC allows us to completely
ditch Babel and make StyleX faster.

**Key Benefits:**

- Faster build times by leveraging NAPI-RS/SWC instead of Babel.
- Seamless integration with Next.js SWC Compiler.
- Almost 100% compatibility with official StyleX tests.

This is specifically useful for Next.js projets as it allows us to use
[SWC Next.js Compiler](https://nextjs.org/docs/architecture/nextjs-compiler).

## Project Structure

This project is organized into several packages:

**Core:**

- [`rs-compiler`](./crates/stylex-rs-compiler) - Rust-based
  [`napi-rs`](https://napi.rs/) compiler for transforming StyleX code.

**Integration:**

- [`nextjs-plugin`](./packages/nextjs-plugin) - A wrapper for
  [`Next.JS configuration`](https://nextjs.org/docs/app/api-reference/next-config-js)
  that integrates the StyleX [napi-rs](https://napi.rs/) compiler into the
  Webpack processing pipeline.

- [`webpack-plugin`](./packages/webpack-plugin) - A `Webpack pluign` that
  integrates the StyleX [napi-rs](https://napi.rs/) compiler.

- [`rollup-plugin`](./packages/rollup-plugin) - A `Rollup plugin` that
  integrates the StyleX [napi-rs](https://napi.rs/) compiler.

- [`unplugin`](./packages/unplugin) - Plugin collection for various build tools
  that integrates the StyleX [napi-rs](https://napi.rs/) compiler.

  Supported build tools and libraries:

  - Farm
  - Rollup
  - Rsbuild
  - Rspack
  - Solid
  - Vite
  - Vue
  - Webpack

- [`postcss-plugin`](./packages/postcss-plugin) - A `PostCSS plugin` that
  integrates the StyleX [napi-rs](https://napi.rs/) compiler.

- [`jest`](./packages/jest) - Jest transformer that integrates the StyleX
  [napi-rs](https://napi.rs/) compiler.

**Utilities:**

- [`stylex-shared`](./crates/stylex-shared) - Shared Rust codebase for the
  StyleX RS compiler and SWC plugin.

- [`path-resolver`](./crates/stylex-path-resolver) - Path handling and resolving
  utilities for the StyleX NAPI-RS/SWC plugin.

- [`test-parser`](./crates/stylex-test-parser) - Parser for
  [StyleX](https://github.com/facebook/stylex) repo Jest tests that helps to
  understand last changes and keeps the project up to date

**Internal Configurations:**

- [`eslint-config`](./packages/eslint-config) - Internal
  [ESLint](https://eslint.org/) configuration

- [`typescript-config`](./packages/typescript-config) - Internal
  [Typescript](https://www.typescriptlang.org/docs/handbook/tsconfig-json.htm)
  configuration

**Other packages:**

- [`design-system`](./packages/design-system) - Design system for the StyleX
  project, intended solely for internal use with in-workspace examples to
  support consistent UI experimentation and prototyping.
