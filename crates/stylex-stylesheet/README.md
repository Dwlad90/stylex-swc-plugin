# StyleX stylesheet

> Part of the
> [StyleX SWC Plugin](https://github.com/Dwlad90/stylex-swc-plugin#readme)
> workspace

## Overview

The stylesheet step of StyleX: the rules a compile emits, assembled into one
sheet in the order the browser needs them. A port of `processStylexRules` from
`@stylexjs/babel-plugin`, function by function, producing the same string.

- The media-query width sort key and its total order (mobile first: `min-width`
  ascending, then `max-width` descending, then rules with no single px bound)
- The default collation the reference sorts with (`localeCompare`): a table for
  printable ASCII, root collation for the rest
- Pinned against the reference over a matrix of 1,305 at-rule chains
  (`tests/fixtures/width-sort-key-matrix.json`)

## License

MIT — see
[LICENSE](https://github.com/Dwlad90/stylex-swc-plugin/blob/develop/LICENSE)
