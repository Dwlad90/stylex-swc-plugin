# `stylex-transform`

> Part of the [StyleX SWC Plugin](https://github.com/Dwlad90/stylex-swc-plugin#readme) workspace

## Overview

Main SWC transform orchestration crate for the StyleX compiler. It owns the
`StyleXTransform` entry point, the `StateManager`, the SWC `Fold`
implementation, and all code that depends on per-file compiler state. This is
the largest crate in the workspace (108 files, ~27,700 lines) and replaces the
former `stylex-shared` monolith.

## Architecture

- **Layer**: 8 — StyleX Transform
- **Depends on**: `stylex-ast`, `stylex-constants`, `stylex-css`,
  `stylex-css-order`, `stylex-css-parser`, `stylex-css-utils`,
  `stylex-css-values`, `stylex-enums`, `stylex-logs`, `stylex-macros`,
  `stylex-path-resolver`, `stylex-regex`, `stylex-structures`, `stylex-types`,
  `stylex-utils`
- **Depended on by**: `stylex-rs-compiler`

## License

MIT — see [LICENSE](https://github.com/Dwlad90/stylex-swc-plugin/blob/develop/LICENSE)
