# `stylex-css-order`

> Part of the [StyleX SWC Plugin](https://github.com/Dwlad90/stylex-swc-plugin#readme) workspace

## Overview

Deterministic CSS property ordering and specificity ranking for StyleX. This
crate encapsulates the three ordering strategies (`ApplicationOrder`,
`LegacyExpandShorthandsOrder`, `PropertySpecificityOrder`) and their associated
constant tables. It was extracted so ordering logic can evolve independently of
CSS generation and transform passes.

## Architecture

- **Layer**: 5 — CSS Foundations & AST
- **Depends on**: `stylex-constants`, `stylex-css-values`, `stylex-structures`,
  `stylex-types`
- **Depended on by**: `stylex-css`, `stylex-transform`

## License

MIT — see [LICENSE](https://github.com/Dwlad90/stylex-swc-plugin/blob/develop/LICENSE)
