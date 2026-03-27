# `stylex-evaluator`

> Part of the [StyleX SWC Plugin](https://github.com/Dwlad90/stylex-swc-plugin#readme) workspace

## Overview

Pure utility functions for JS expression evaluation — expression
traversal, value extraction, and type coercion helpers used by the
transform layer. This crate was extracted so that evaluation helpers
with no `StateManager` dependency can be reused by `stylex-css` and
tested in isolation from the full transform pipeline.

## Architecture

- **Layer**: 6 — Evaluation
- **Depends on**: `stylex-ast`, `stylex-constants`, `stylex-js`,
  `stylex-macros`, `stylex-path-resolver`, `stylex-types`
- **Depended on by**: `stylex-css`

## License

MIT — see [LICENSE](https://github.com/Dwlad90/stylex-swc-plugin/blob/develop/LICENSE)
