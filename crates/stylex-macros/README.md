# `stylex-macros`

> Part of the
> [StyleX SWC Plugin](https://github.com/Dwlad90/stylex-swc-plugin#readme)
> workspace

## Overview

Declarative macros and error types shared across the entire StyleX compiler
workspace. This crate replaces the deleted `stylex-core` crate, consolidating
panic wrappers, the `StyleXError` type, `FxHash` collection constructors, and
type-conversion helpers into a single, lightweight dependency that nearly every
other crate imports.

- **Standardised error handling** — branded `[StyleX]` error messages via
  `stylex_panic!`, `stylex_bail!`, and friends
- **FxHash collection constructors** — convenience macros for building
  `FxHashMap` / `FxHashSet` with zero boilerplate
- **Type conversion helpers** — macro-driven `From` / `Into` implementations

## License

MIT — see
[LICENSE](https://github.com/Dwlad90/stylex-swc-plugin/blob/develop/LICENSE)
