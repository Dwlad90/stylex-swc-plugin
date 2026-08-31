# `stylex-constants`

> Part of the
> [StyleX SWC Plugin](https://github.com/Dwlad90/stylex-swc-plugin#readme)
> workspace

## Overview

Single source of truth for every compile-time constant in the StyleX compiler.
This crate centralises CSS global values, error messages, property
classifications, shorthand expansion tables, and style priority rankings so that
all downstream crates share one canonical definition without risk of duplication
or circular dependencies.

- **Performance-first collections** — all lookup tables use `FxHashMap` /
  `FxHashSet` from `rustc-hash`
- **Zero internal dependencies** — leaf crate with no workspace deps
- **Extracted for deduplication** — eliminates scattered constant definitions
  across the compiler pipeline

## License

MIT — see
[LICENSE](https://github.com/Dwlad90/stylex-swc-plugin/blob/develop/LICENSE)
