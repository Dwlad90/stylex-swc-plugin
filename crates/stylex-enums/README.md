# `stylex-enums`

> Part of the [StyleX SWC Plugin](https://github.com/Dwlad90/stylex-swc-plugin#readme) workspace

## Overview

Shared enum and type-alias definitions used throughout the StyleX compiler
crates. Extracted from the monolithic `stylex-shared` crate so that every
consumer can depend on a small, stable set of domain types without pulling in
transformation or CSS logic.

## Architecture

- **Layer**: 2 — Domain Leaves
- **Depends on**: `stylex-macros`
- **Depended on by**: `stylex-css`, `stylex-rs-compiler`, `stylex-structures`,
  `stylex-transform`, `stylex-types`

## License

MIT — see [LICENSE](https://github.com/Dwlad90/stylex-swc-plugin/blob/develop/LICENSE)
