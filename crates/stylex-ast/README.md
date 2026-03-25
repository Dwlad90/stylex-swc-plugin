# `StyleX AST`

## Overview

This package provides SWC AST manipulation utilities for the StyleX compiler. Includes AST factories, convertors, helpers, code frame error infrastructure, and logging utilities.

## Architecture

Layer 0 crate providing AST construction and error reporting primitives for downstream layers.

## Key Modules

- **`ast/factories.rs`** — AST factory functions for common patterns
  - `spread_element_factory()` — `...expr` spread syntax
  - `call_expr_member_factory()` — `obj.method(...args)` calls
  - `arrow_expr_factory()` — `() => expr` arrow functions
  - Plus 8 more factories for common AST construction patterns

- **`ast/convertors.rs`** — Expression-to-value conversions
  - `expr_to_string()` — convert Expr to string
  - `expr_to_bool()` — convert Expr to boolean
  - Full AST traversal and type conversions

- **`log/build_code_frame_error.rs`** — Error reporting with source context
  - Generic over `<T: EvaluationContext>` for context-aware errors
  - `build_code_frame_error()` — creates detailed error messages with line/column
  - `panic_with_context()` — panics with formatted error context

## Dependencies

- `stylex-constants` — CSS metadata
- `stylex-types` — Type definitions and traits
- `swc_core` — SWC AST types and utilities
- `swc_compiler_base` — SWC base utilities
- `anyhow` — error handling

## Usage

```rust
use stylex_ast::factories::*;
use stylex_ast::log::build_code_frame_error;
use stylex_types::traits::EvaluationContext;

// Create AST nodes using factories
let call = call_expr_ident_factory(
  Atom::from("getValue"),
  vec![],
);

// Build detailed error messages with source context
fn validate<T: EvaluationContext>(
  expr: &Expr,
  ctx: &mut T,
) -> Result<()> {
  if !is_valid(expr) {
    build_code_frame_error(
      expr,
      "Invalid expression format",
      ctx.get_filename(),
      ctx,
    );
  }
  Ok(())
}
```
