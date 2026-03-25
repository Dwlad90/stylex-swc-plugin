# `StyleX Types`

## Overview

This package provides type definitions for the StyleX SWC plugin. Includes enums, structs, type aliases and trait definitions used across the compiler pipeline.

## Architecture

Layer 0 crate providing the semantic contract for all downstream layers.

### Core Traits (Object-Safe Design)

Three-tier trait hierarchy eliminating coupling between types and StateManager:

1. **`StyleOptions`** — Minimal interface for CSS generation, CSS property tracking
   - Object-safe, used as `dyn StyleOptions` in function pointers
   - Methods: `options()`, `css_property_seen()`, `css_property_seen_mut()`

2. **`EvaluationContext: StyleOptions`** — Extended interface for JS evaluation
   - Object-safe on its own
   - Methods: declarations, imports, variable tracking, span caching
   - Used with `<T: EvaluationContext + Clone>` for arrow closure handling

3. **`TransformState: EvaluationContext`** — Full transform state and import tracking
   - Used via generics: `<T: TransformState + Clone>`
   - Methods: StyleX import sets, style maps, transform registration

## Key Exports

- **Enums**: `FunctionType`, `EvaluateResultValue`, `TransformationCycle`, etc.
- **Structures**: `StyleXOptions`, `FunctionConfig`, `FunctionMap`, `StylesObjectMap`, etc.
- **Type aliases**: `StylexExprFn` → `fn(Expr, &mut dyn StyleOptions) -> Expr`
- **Traits**: `StyleOptions`, `EvaluationContext`, `TransformState`

## Dependencies

- `stylex-constants` — CSS properties, regex patterns
- `swc_core` — SWC AST types and utilities
- `indexmap` — ordered maps for deterministic iteration
- `rustc-hash` — fast hashing (FxHashMap/FxHashSet)
- `serde`, `serde_json` — serialization for metadata
- `derive_more` — convenient derive macros
- `anyhow`, `log` — error handling and logging

## Usage

```rust
use stylex_types::traits::{StyleOptions, EvaluationContext, TransformState};
use stylex_types::structures::*;

// Define a function accepting any style context
fn generate_css<T: StyleOptions>(style: &Expr, options: &mut T) -> String {
  let seen = options.css_property_seen();
  // Process CSS generation
}

// Evaluate expressions with access to declarations and imports
fn evaluate_expr<T: EvaluationContext + Clone>(
  expr: &Expr,
  context: &mut T,
) -> EvaluateResultValue {
  let decls = context.declarations();
  // Resolve and evaluate expression
}
```
