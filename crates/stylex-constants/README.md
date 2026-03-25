# `StyleX Constants`

## Overview

This package provides constants, regex patterns and static data for the StyleX SWC plugin. Includes CSS property orders, priorities, shorthands, error messages and compiled regex patterns.

## Architecture

Layer 0 leaf crate. No internal dependencies on other StyleX crates.

## Key Exports

- **Constants modules**: CSS property orders, priorities, valid properties, browser prefixes, etc.
- **Regex patterns**: Pre-compiled regex patterns for StyleX syntax parsing
- **SWC utilities**: Common SWC helpers and conversion utilities

## Dependencies

- `phf` — perfect hashing for static data
- `once_cell` — lazy static initialization
- `fancy-regex` — advanced regex patterns with lookahead/lookbehind
- `lazy_static` — simple lazy static data
- `swc_core` (common only) — SWC type definitions

## Usage

```rust
use stylex_constants::{css_properties, regex_patterns};

// Access CSS property metadata
if let Some(props) = css_properties::get_property_info("backgroundColor") {
  println!("Property: {}", props.name);
}

// Use pre-compiled regex patterns
if regex_patterns::STYLEX_CALL_PATTERN.is_match(code) {
  // Process StyleX call
}
```
