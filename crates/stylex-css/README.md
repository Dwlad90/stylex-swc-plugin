# `stylex-css`

> Part of the [StyleX SWC Plugin](https://github.com/Dwlad90/stylex-swc-plugin#readme) workspace

## Overview

Pure CSS generation and normalization functions for the StyleX compiler. This
crate was isolated from the former `stylex-shared` monolith to encapsulate CSS
output logic — LTR/RTL generation and whitespace normalization — without
depending on compiler state (`StateManager`).

## Architecture

- **Layer**: 7 — CSS Processing
- **Depends on**: `stylex-ast`, `stylex-constants`, `stylex-css-order`,
  `stylex-css-parser`, `stylex-css-utils`, `stylex-css-values`, `stylex-enums`,
  `stylex-evaluator`, `stylex-macros`, `stylex-regex`, `stylex-structures`,
  `stylex-types`
- **Depended on by**: `stylex-transform`

## License

MIT — see [LICENSE](https://github.com/Dwlad90/stylex-swc-plugin/blob/develop/LICENSE)
