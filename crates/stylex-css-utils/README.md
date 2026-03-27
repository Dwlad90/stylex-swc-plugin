# `StyleX CSS Utils`

## Overview

CSS utility functions for the StyleX system.

Provides helper functions for pre-rules, conditional styles, and other CSS
processing utilities within the StyleX NAPI-RS compiler.

## Contents

- `pre_rule` -- `sort_pseudos`, `sort_at_rules` -- deterministic ordering of
  pseudo-class and at-rule selectors so that generated CSS class names are
  stable regardless of declaration order
- `vector` -- `intersection` -- set intersection helper for `Vec<String>`
  collections
- `when` -- Relational selector helpers (`from_proxy`, `from_stylex_style`,
  `ancestor`, `descendant`, `sibling_before`, `sibling_after`, `any_sibling`)
  and `get_default_marker_class_name` for marker-class-based conditional styles
