# `StyleX CSS`

## Overview

CSS generation, normalization, and utility functions for the StyleX SWC
plugin.

## Contents

### CSS Utilities (`css/`)
- `parser` -- CSS value parsing using `cssparser` (format_ident, parse_css)
- `generate_ltr` -- LTR style generation with logical property mapping
- `normalizers/whitespace_normalizer` -- CSS whitespace normalization
  (math signs, brackets, functions, URLs)

### General Utilities (`utils/`)
- `pre_rule` -- Pseudo-class and at-rule sorting (sort_pseudos,
  sort_at_rules)
- `vector` -- Vector intersection utility
- `when` -- Relational selector functions (ancestor, descendant,
  sibling_before, sibling_after, any_sibling) with 15 inline tests

## Note

CSS generation code that references `StateManager` (`css/common.rs`,
`generate_rtl.rs`, `core/*` utils) remains in `stylex-transform`. This
crate contains only the pure functions extractable without code changes.

## Layer

Layer 1. Dependencies: `stylex-constants`, `stylex-types`, `stylex-core`,
`stylex-regex`, `cssparser`.
