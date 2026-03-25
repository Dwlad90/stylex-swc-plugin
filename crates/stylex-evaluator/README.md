# `StyleX Evaluator`

## Overview

This package provides the JavaScript expression evaluation engine for the StyleX SWC plugin. Resolves compile-time constant expressions, handles variable lookups, function calls, and import path resolution. ~5,500 lines including `evaluate.rs` (2,938 lines).

## Architecture

Layer 1 processing engine crate. Depends on all Layer 0 crates (constants, types, AST).

## Key Modules

- **`js/evaluate.rs`** — Core expression evaluation engine (2,938 lines)
  - `evaluate<T: EvaluationContext + Clone>()` — main evaluation dispatcher
  - Recursive expression resolution with cycle detection
  - Support for: variables, member access, function calls, arrays, objects, conditionals
  - Arrow closure evaluation with state cloning
  - Caching of evaluation results via `EvaluationContext::span_cache`

- **`js/check_declaration.rs`** — Variable and import declaration resolution
  - Lookup variable declarations from `EvaluationContext::declarations()`
  - Resolve imported symbols and their sources

- **`js/native_functions.rs`** — Built-in JavaScript functions
  - Array methods: `map`, `filter`, `reduce`, etc.
  - Object methods: `keys`, `values`, `entries`, `assign`, etc.
  - String methods: `split`, `replace`, `toUpperCase`, etc.

- **`common.rs`** — Shared utilities
  - Variable and identifier tracking via `EvaluationContext`
  - `reduce_ident_count()`, `increase_ident_count()`
  - `get_var_decl_by_ident()`

- **`macros/`** — Converted to functions
  - `expr_to_str_or_deopt()` — converts expr to string or deoptimizes with `?` operator

## Dependencies

- `stylex-constants`, `stylex-types`, `stylex-ast` — Layer 0
- `stylex-path-resolver` — Import path resolution
- `swc_core` — SWC AST types
- `indexmap`, `rustc-hash` — fast, ordered maps
- `murmur2`, `radix_fmt`, `base62`, `md5` — hashing and encoding
- `anyhow`, `log` — error handling and logging

## Usage

```rust
use stylex_evaluator::js::evaluate;
use stylex_types::traits::EvaluationContext;

// Evaluate a JS expression at compile-time
fn resolve_constant<T: EvaluationContext + Clone>(
  expr: &Expr,
  context: &mut T,
  functions: &FunctionMap,
) -> Box<EvaluateResult> {
  evaluate(expr, context, functions)
}

// Check if expression is resolvable to string
fn get_config_value<T: EvaluationContext + Clone>(
  expr: &Expr,
  context: &mut T,
  functions: &FunctionMap,
) -> Option<String> {
  expr_to_str_or_deopt(expr, context, functions)
}
```
