# `stylex-types`

> Part of the [StyleX SWC Plugin](https://github.com/Dwlad90/stylex-swc-plugin#readme) workspace

## Overview

Injectable style types and metadata structures for the StyleX
compiler. This crate defines the `InjectableStyle` family of structs
and enums, the `MetaData` output type, and the `StyleOptions` trait
that decouples function-pointer types from `StateManager`. It was
extracted so that every crate needing compiled-style representations
can depend on a slim type package without pulling in transform logic.

## Architecture

- **Layer**: 4 — Type System
- **Depends on**: `stylex-constants`, `stylex-enums`,
  `stylex-macros`, `stylex-structures`, `stylex-utils`
- **Depended on by**: `stylex-ast`, `stylex-css`, `stylex-css-order`,
  `stylex-evaluator`, `stylex-rs-compiler`, `stylex-transform`

## License

MIT — see [LICENSE](https://github.com/Dwlad90/stylex-swc-plugin/blob/develop/LICENSE)
