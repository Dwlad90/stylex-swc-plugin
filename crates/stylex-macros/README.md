# `stylex-macros`

> Part of the [StyleX SWC Plugin](https://github.com/Dwlad90/stylex-swc-plugin#readme) workspace

## Overview

Declarative macros used throughout the StyleX compiler. This crate
replaces the deleted `stylex-core` and provides panic wrappers,
error types, and collection constructors that every other crate
imports.

## Architecture

- **Layer**: 1 — Macros
- **Depends on**: `stylex-constants`
- **Depended on by**: ast, css, css-parser, css-values, enums,
  evaluator, js, logs, path-resolver, rs-compiler, structures,
  transform, types

## License

MIT — see [LICENSE](https://github.com/Dwlad90/stylex-swc-plugin/blob/develop/LICENSE)
