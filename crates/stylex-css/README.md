# `stylex-css`

> Part of the
> [StyleX SWC Plugin](https://github.com/Dwlad90/stylex-swc-plugin#readme)
> workspace

## Overview

Unified CSS processing crate for the StyleX compiler pipeline. This crate
consolidates all CSS-related functionality — generation, value parsing, property
ordering, and utility helpers — into a single, cohesive package.

- **Stateless CSS generation** — produces CSS strings from StyleX declarations
  without requiring a `StateManager`, making every function a pure input →
  output transform.
- **Bidirectional (LTR / RTL) output** — dedicated modules generate
  left-to-right and right-to-left stylesheets, enabling automatic bidirectional
  support in downstream consumers.
- **CSS value parsing** — tokenises and parses CSS value strings using the
  `cssparser` crate, splitting shorthand properties into their individual
  components (top, right, bottom, left).
- **Property ordering strategies** — implements three ordering strategies
  (`ApplicationOrder`, `LegacyExpandShorthandsOrder`,
  `PropertySpecificityOrder`) for deterministic shorthand expansion and CSS
  property sorting.
- **Pseudo-class and selector utilities** — provides `when::ancestor`,
  `when::descendant`, `when::sibling_*` and other helpers for generating
  conditional CSS selectors from StyleX state options.
- **Whitespace normalization** — canonicalises whitespace in generated CSS so
  output is deterministic and diff-friendly.
- **Deterministic output** — given identical input declarations and
  configuration, the crate always produces byte-identical CSS, which simplifies
  snapshot testing and caching.

## License

MIT — see
[LICENSE](https://github.com/Dwlad90/stylex-swc-plugin/blob/develop/LICENSE)
