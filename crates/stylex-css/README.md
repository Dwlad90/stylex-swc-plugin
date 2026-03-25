# `StyleX CSS`

## Overview

This package provides CSS generation, normalization and validation for the StyleX SWC plugin. Includes class name generation, LTR/RTL support, shorthand expansion, CSS property validation and style merging. ~5,000 lines.

## Architecture

Layer 1 processing engine crate. Depends on all Layer 0 crates + `stylex-evaluator`.

## Key Modules

- **`css/`** — CSS class name generation and manipulation
  - `class_name_generation.rs` — hashing and class name assignment
  - LTR/RTL style pair generation
  - Media query and pseudo-class handling

- **`core/`** — Core CSS processing
  - `flatten_raw_style_object.rs` — flatten nested style objects
  - `evaluate_stylex_create_arg.rs` — evaluate style arguments
  - `stylex_merge.rs` — merge style objects
  - `parse_nullable_style.rs` — handle optional/conditional styles
  - `member_expression.rs` — resolve member access patterns
  - `convert_style_to_class_name.rs` — style object → class names

- **`object.rs`** — Object manipulation utilities
  - `obj_map()` — map over object properties with context
  - Working with `<T: StyleOptions>` and `<T: EvaluationContext>`

- **`pre_rule.rs`** — CSS rule generation
  - `PreRule::compiled()` — compile to final CSS rules
  - Works with `&mut dyn StyleOptions` for context-agnostic compilation

- **`validators.rs`** — Style validation
  - Validate CSS property values
  - Check import requirements (`<T: TransformState>`)
  - Media query and pseudo-class validation

- **`when.rs`** — Conditional style handling
- **`vector.rs`** — Utility functions for vector/array operations

## Dependencies

- All Layer 0 crates: `stylex-constants`, `stylex-types`, `stylex-ast`
- `stylex-evaluator` — expression evaluation
- `stylex-css-parser` — CSS parsing
- `swc_core` — SWC AST types
- `cssparser` — CSS specification parsing
- `fancy-regex` — pattern matching
- `indexmap`, `rustc-hash` — fast, ordered maps
- `log` — logging

## Usage

```rust
use stylex_css::core::*;
use stylex_types::traits::TransformState;

// Flatten nested style objects
fn process_styles<T: EvaluationContext + Clone>(
  raw_object: &Expr,
  context: &mut T,
) -> Result<StylesObjectMap> {
  flatten_raw_style_object(raw_object, context)
}

// Merge style objects with precedence
fn merge_styles<T: TransformState + Clone>(
  base: &Expr,
  overrides: &Expr,
  context: &mut T,
) -> Result<Expr> {
  stylex_merge(base, overrides, context)
}

// Convert to class names
fn generate_classes<T: EvaluationContext + Clone>(
  styles: &Expr,
  context: &mut T,
) -> Vec<String> {
  convert_style_to_class_name(styles, context)
}
```
