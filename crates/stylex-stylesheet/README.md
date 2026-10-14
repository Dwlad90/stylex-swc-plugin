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
- `ProcessStylexRulesConfig` deserializes every shape the reference accepts
  from JavaScript: a boolean standing for `useLayers`, `null`, or the options
  object with `useLayers` read by the `!== false` rule, the flags by
  truthiness and a `null` layer option as its default (`CONTEXT.md`, "Config")
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
