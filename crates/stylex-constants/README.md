# `stylex-constants`

> Part of the [StyleX SWC Plugin](https://github.com/Dwlad90/stylex-swc-plugin#readme) workspace

## Overview

Core constant definitions for the StyleX compiler. This crate holds all static
values — CSS keywords, error messages, property classifications, and priority
rankings — used across the entire plugin. It was extracted so that every
downstream crate can share one canonical source of truth without circular
dependencies.

## Architecture

- **Layer**: 0 — Primitives (no internal deps)
- **Depends on**: None (leaf crate)
- **Depended on by**: macros, ast, css, css-order, evaluator, js, structures,
  transform, types

## License

MIT — see [LICENSE](https://github.com/Dwlad90/stylex-swc-plugin/blob/develop/LICENSE)
