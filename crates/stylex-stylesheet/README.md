# StyleX stylesheet

> Part of the
> [StyleX SWC Plugin](https://github.com/Dwlad90/stylex-swc-plugin#readme)
> workspace

## Overview

The stylesheet step of StyleX: the rules a compile emits, assembled into one
sheet in the order the browser needs them. A port of `processStylexRules` from
`@stylexjs/babel-plugin`, function by function, producing the same string.

- `process_stylex_rules(rules, config)`: constants resolved and substituted,
  the rules sorted (priority, declared property, breakpoint, text), grouped by
  thousand of priority, and written as `@layer` blocks or with the specificity
  polyfill, RTL twins included
- `process_stylex_rule_values(rules, config)`: the same step over two
  JavaScript values, read as the reference's JavaScript reads them, a value
  outside the published types included (`CONTEXT.md`, "Value reading")
- Typed errors (`ProcessStylexRulesError`) where the reference throws, and
  where it would cut a character in two. The addon reads any error as "run
  the reference" (`CONTEXT.md`, "Fallback"), so a consumer of the addon sees
  the reference's own errors
- The rules sorted with V8's sort algorithm, so a `NaN` priority gives the
  order Node gives (`CONTEXT.md`, "Sort")
- The media-query width sort key and its total order (mobile first: `min-width`
  ascending, then `max-width` descending, then rules with no single px bound)
- The default collation the reference sorts with (`localeCompare`): a table for
  printable ASCII, root collation for the rest
- Pinned against the reference: every case of its own test file
  (`tests/fixtures/process-stylex-rules-cases.json`) and a matrix of 1,305
  at-rule chains (`tests/fixtures/width-sort-key-matrix.json`)

## License

MIT — see
[LICENSE](https://github.com/Dwlad90/stylex-swc-plugin/blob/develop/LICENSE)
