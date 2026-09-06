# StyleX CSS Parser

> Part of the
> [StyleX SWC Plugin](https://github.com/Dwlad90/stylex-swc-plugin#readme)
> workspace

## Overview

A high-performance CSS value parser (~28 k lines) providing comprehensive
parsing and validation for CSS properties, types, and at-rules. This was already
an independent crate before the monorepo refactor; it is now a clean leaf with
no internal dependencies beyond `stylex-macros` and `stylex-utils`.

- Full CSS type coverage: colors, lengths, angles, calc expressions, transform
  functions, easing, filters, and more
- Flexible parser combinator system (`FlexParser`, `FlexCombinators`) with
  backtracking support for composable, zero-copy parsing
- Media query parsing and transformation with "last media query wins" semantics
  via `last_media_query_wins_transform`

## License

MIT — see
[LICENSE](https://github.com/Dwlad90/stylex-swc-plugin/blob/develop/LICENSE)
