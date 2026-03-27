# `stylex-logs`

> Part of the [StyleX SWC Plugin](https://github.com/Dwlad90/stylex-swc-plugin#readme) workspace

## Overview

Branded logging utilities for the StyleX compiler. Provides a
`[StyleX]`-prefixed, ANSI-colored log formatter and one-time logger
initialization. Isolated so that any crate needing diagnostics output
can pull in logging without depending on compiler internals.

> **Note:** This crate was formerly named `stylex-logger`.

## Architecture

- **Layer**: 2 — Domain Leaves
- **Depends on**: `stylex-macros`
- **Depended on by**: `stylex-rs-compiler`, `stylex-transform`

## License

MIT — see [LICENSE](https://github.com/Dwlad90/stylex-swc-plugin/blob/develop/LICENSE)
