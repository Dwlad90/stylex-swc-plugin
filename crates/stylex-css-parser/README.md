# `stylex-css-parser`

> Part of the [StyleX SWC Plugin](https://github.com/Dwlad90/stylex-swc-plugin#readme) workspace

## Overview

A high-performance CSS value parser (~28 k lines) providing comprehensive
parsing and validation for CSS properties, types, and at-rules. This was
already an independent crate before the monorepo refactor; it is now a
clean leaf with no internal dependencies beyond `stylex-macros`.

## Architecture

- **Layer**: 2 — Domain Leaves
- **Depends on**: `stylex-macros`
- **Depended on by**: `stylex-css`, `stylex-transform`

## License

MIT — see [LICENSE](https://github.com/Dwlad90/stylex-swc-plugin/blob/develop/LICENSE)
