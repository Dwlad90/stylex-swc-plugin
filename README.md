# StyleX in Rust &middot; [![GitHub license](https://img.shields.io/badge/license-MIT-green.svg)](https://github.com/Dwlad90/stylex-swc-plugin/blob/master/LICENSE) [![npm version](https://img.shields.io/npm/v/@stylexswc/swc-plugin.svg?style=flat)](https://www.npmjs.com/package/@stylexswc/swc-plugin) ![GitHub Actions Workflow Status](https://img.shields.io/github/actions/workflow/status/Dwlad90/stylex-swc-plugin/validate.yml?branch=master&label=Validation)

This is a monorepo for an unofficial [`napi-rs`](https://napi.rs/) compiler and
an [SWC](https://swc.rs/) plugin for
[StyleX](https://github.com/facebook/stylex). Using SWC allows us to completely
ditch Babel and make StyleX faster.

**Key Benefits:**

* Faster build times by leveraging NAPI-RS/SWC instead of Babel.
* Seamless integration with Next.js SWC Compiler.
* Almost 100% compatibility with official StyleX tests.

This is specifically useful for Next.js projets as it allows us to use
[SWC Next.js Compiler](https://nextjs.org/docs/architecture/nextjs-compiler).

## Project Structure

This project is organized into several packages:

**Core:**

* [`rs-compiler`](https://github.com/dwlad90/stylex-swc-plugin/tree/master/crates/rs-compiler) -
  Rust-based [`napi-rs`](https://napi.rs/) compiler for transforming StyleX code.

**Integration:**

* [`nextjs-plugin`](https://github.com/dwlad90/stylex-swc-plugin/tree/master/packages/nextjs-plugin) -
  A wrapper for [`Next.JS configuration`](https://nextjs.org/docs/app/api-reference/next-config-js) that integrates the StyleX [napi-rs](https://napi.rs/) compiler into the Webpack processing pipeline.

**Utilities:**

* [`stylex-shared`](https://github.com/dwlad90/stylex-swc-plugin/tree/master/crates/stylex-shared) -
 Shared Rust codebase for the StyleX RS compiler and SWC plugin.

* [`path-resolver`](https://github.com/dwlad90/stylex-swc-plugin/tree/master/crates/stylex-path-resolver) -
   Path handling and resolving utilities for the StyleX NAPI-RS / SWC plugin.

* [`test-parser`](https://github.com/dwlad90/stylex-swc-plugin/tree/master/crates/stylex-test-parser) -
  Parser for [StyleX](https://github.com/facebook/stylex) repo Jest tests that
  helps to understand last changes and keeps the project up to date

**Internal Configurations:**

* [`eslint-config`](https://github.com/dwlad90/stylex-swc-plugin/tree/master/packages/eslint-config) -
  Internal [ESLint](https://eslint.org/) configuration

* [`typescript-config`](https://github.com/dwlad90/stylex-swc-plugin/tree/master/packages/typescript-config) -
  Internal
  [Typescript](https://www.typescriptlang.org/docs/handbook/tsconfig-json.htm)
  configuration

## Deprecated Packages

> [!WARNING]
> The packages below are deprecated as of version `0.3.0` and may be removed in the future. Please use the newer alternatives listed above.

**Core:**

* [`swc-plugin`](https://github.com/dwlad90/stylex-swc-plugin/tree/master/crates/stylex-swc-plugin) -
  Unofficial SWC implementation of the native
  [StyleX](https://github.com/facebook/stylex) plugin. The new alternative is
  [`rs-compiler`](https://github.com/dwlad90/stylex-swc-plugin/tree/master/crates/rs-compiler)

**Integration:**

* [`nextjs-swc-plugin`](https://github.com/dwlad90/stylex-swc-plugin/tree/master/packages/nextjs-plugin) -
  Wrapper for
  [`Next.JS config`](https://nextjs.org/docs/app/api-reference/next-config-js)
  that inject the StyleX SWC plugin to webpack processing. The new alternative
  is
  [`nextjs-plugin`](https://github.com/dwlad90/stylex-swc-plugin/tree/master/packages/nextjs-plugin)
