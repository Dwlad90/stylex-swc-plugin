# `stylex-structures`

> Part of the [StyleX SWC Plugin](https://github.com/Dwlad90/stylex-swc-plugin#readme) workspace

## Overview

Core data structures and configuration types for the StyleX compiler pipeline.
This crate defines the foundational structs — plugin state, style
representations, CSS ordering primitives, and compiler options — that every
higher-level crate depends on. It was isolated so that data definitions stay
decoupled from transform logic and CSS generation.

## Architecture

- **Layer**: 3 — Core Data Structures
- **Depends on**: `stylex-constants`, `stylex-enums`, `stylex-macros`
- **Depended on by**: `stylex-css`, `stylex-css-order`, `stylex-css-utils`,
  `stylex-rs-compiler`, `stylex-transform`, `stylex-types`

## License

MIT — see [LICENSE](https://github.com/Dwlad90/stylex-swc-plugin/blob/develop/LICENSE)
