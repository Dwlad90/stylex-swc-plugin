# `stylex-logs`

> Part of the
> [StyleX SWC Plugin](https://github.com/Dwlad90/stylex-swc-plugin#readme)
> workspace

## Overview

Branded logging utilities for the StyleX compiler. Provides a
`[StyleX]`-prefixed, ANSI-colored log formatter and one-time logger
initialization. Isolated so that any crate needing diagnostics output can pull
in logging without depending on compiler internals.

> **Note:** This crate was formerly named `stylex-logger`.

- Emits all diagnostics with a recognizable `[StyleX]` prefix using ANSI color
  codes for terminal readability
- Thread-safe one-time initialization ensures the logger is set up exactly once
  per process
- Leaf-level crate — depends only on `stylex-macros`, keeping the dependency
  footprint minimal

## License

MIT — see
[LICENSE](https://github.com/Dwlad90/stylex-swc-plugin/blob/develop/LICENSE)
