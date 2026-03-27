# `stylex-css-values`

> Part of the [StyleX SWC Plugin](https://github.com/Dwlad90/stylex-swc-plugin#readme) workspace

## Overview

CSS value validation, tokenization, and shorthand-expansion utilities.
Isolated into its own crate so that higher-level CSS generation and
ordering crates can share a single, well-tested value-handling layer
without duplicating parsing logic.

## Architecture

- **Layer**: 2 — Domain Leaves
- **Depends on**: `stylex-macros`
- **Depended on by**: `stylex-css`, `stylex-css-order`,
  `stylex-transform`

## License

MIT — see [LICENSE](https://github.com/Dwlad90/stylex-swc-plugin/blob/develop/LICENSE)
