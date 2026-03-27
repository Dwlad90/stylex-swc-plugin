# `stylex-ast`

> Part of the [StyleX SWC Plugin](https://github.com/Dwlad90/stylex-swc-plugin#readme) workspace

## Overview

SWC AST manipulation utilities — factory functions for creating AST nodes and
pure convertor functions for extracting and coercing values. This crate was
split out so that any layer needing to construct or inspect SWC `Expr` / `Lit` /
`Ident` nodes can do so without pulling in the full transform or CSS pipelines.

## Architecture

- **Layer**: 5 — CSS Foundations & AST
- **Depends on**: `stylex-constants`, `stylex-macros`, `stylex-types`,
  `stylex-utils`
- **Depended on by**: `stylex-css`, `stylex-evaluator`, `stylex-rs-compiler`,
  `stylex-transform`

## License

MIT — see [LICENSE](https://github.com/Dwlad90/stylex-swc-plugin/blob/develop/LICENSE)
