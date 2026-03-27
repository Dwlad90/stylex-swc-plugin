# `StyleX CSS Order`

## Overview

CSS property ordering and specificity for the StyleX system.

Manages the deterministic ordering of CSS properties and specificity rules
within the StyleX NAPI-RS compiler.

## Contents

### Constants (`constants/`)

- `application_order` -- `Shorthands` and `Aliases` lookup tables mapping CSS
  shorthand properties (e.g., `animation`, `border`, `flex`, `grid`, `margin`,
  `padding`, `transition`) to their longhand `OrderPair` expansions
- `legacy_expand_shorthands_order` -- Legacy shorthand expansion tables used
  when `StyleResolution::LegacyExpandShorthands` is active
- `property_specificity_order` -- Property specificity ordering constants for
  `StyleResolution::PropertySpecificity` mode

### Structures (`structures/`)

- `ApplicationOrder` -- Implements the `Order` trait with `application_order`
  expansion rules (default StyleX ordering)
- `LegacyExpandShorthandsOrder` -- Implements `Order` using legacy shorthand
  expansion tables
- `PropertySpecificityOrder` -- Implements `Order` using property specificity
  ordering
