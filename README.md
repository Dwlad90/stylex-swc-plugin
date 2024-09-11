# StyleX SWC plugin &middot; [![GitHub license](https://img.shields.io/badge/license-MIT-green.svg)](https://github.com/Dwlad90/stylex-swc-plugin/blob/master/LICENSE) [![npm version](https://img.shields.io/npm/v/@stylexswc/swc-plugin.svg?style=flat)](https://www.npmjs.com/package/@stylexswc/swc-plugin) ![GitHub Actions Workflow Status](https://img.shields.io/github/actions/workflow/status/Dwlad90/stylex-swc-plugin/validate.yml?branch=master&label=Validation)


This is a monorepo for an unofficial [SWC](https://swc.rs/) plugin for
[StyleX](https://github.com/facebook/stylex). Using SWC allows us to completely
ditch Babel and make StyleX faster.

This plugin passes almost 100% of tests of the official StyleX library. It is
intended as a drop-in replacement for the official StyleX babel plugin.

This is specifically useful for Next.js projets as it allows us to use
[SWC Next.js Compiler](https://nextjs.org/docs/architecture/nextjs-compiler).

## Packages

- [`eslint-config`](https://github.com/dwlad90/stylex-swc-plugin/tree/master/packages/eslint-config) -
  internal [ESLint](https://eslint.org/) configuration
- [`nextjs-plugin`](https://github.com/dwlad90/stylex-swc-plugin/tree/master/packages/nextjs-plugin) -
  wrapper for
  [`Next.JS config`](https://nextjs.org/docs/app/api-reference/next-config-js)
  that inject the StyleX SWC plugin to webpack processing
- [`swc-plugin`](https://github.com/dwlad90/stylex-swc-plugin/tree/master/crates/stylex-swc-plugin) -
  unofficial SWC implementation of the native
- [`path-resolver`](https://github.com/dwlad90/stylex-swc-plugin/tree/master/crates/stylex-path-resolver) -
  path utilities for
  [StyleX SWC plugin](https://github.com/dwlad90/stylex-swc-plugin/tree/master/crates/stylex-swc-plugin)
- [`test-parser`](https://github.com/dwlad90/stylex-swc-plugin/tree/master/crates/stylex-test-parser) -
  parser for [StyleX](https://github.com/facebook/stylex) repo Jest tests that
  helps to understand last changes and keeps the project up to date
- [`typescript-config`](https://github.com/dwlad90/stylex-swc-plugin/tree/master/packages/typescript-config) -
  internal
  [Typescript](https://www.typescriptlang.org/docs/handbook/tsconfig-json.htm)
  configuration
