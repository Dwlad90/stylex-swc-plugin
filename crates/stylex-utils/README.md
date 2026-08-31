# `stylex-utils`

> Part of the
> [StyleX SWC Plugin](https://github.com/Dwlad90/stylex-swc-plugin#readme)
> workspace

## Overview

Small, standalone utilities that don't belong in any domain crate. Formerly
named `stylex-misc`, this crate provides deterministic floating-point hashing
and SWC helper functions shared by several higher-level crates in the compiler
pipeline.

- **Deterministic hashing** — `hash_f64` converts `f64` values to a stable,
  hashable representation
- **SWC helpers** — factory for a default `ExprCtx` used in expression type
  checking
- **No internal dependencies** — sits at the primitives layer and depends on
  nothing else in the workspace

## License

MIT — see
[LICENSE](https://github.com/Dwlad90/stylex-swc-plugin/blob/develop/LICENSE)
