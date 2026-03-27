# `StyleX CSS Values`

## Overview

CSS value validation and parsing for the StyleX system.

Handles validation and parsing of CSS property values within the StyleX NAPI-RS
compiler.

## Contents

- `parser` -- `parse_css` -- tokenizes a CSS value string into a `Vec<String>`
  of individual tokens using `cssparser`; `format_ident` -- serializes a CSS
  identifier according to the CSS spec
- `common` -- `split_value` -- splits a CSS shorthand value string into its
  top/right/bottom/left components returning `Option` for omitted sides;
  `split_value_required` -- same but falls back to the preceding value for any
  missing side (CSS expansion rules)
