# `StyleX Transformers`

## Overview

This package provides the StyleX API transformer implementations for the SWC plugin. Handles all StyleX function calls and transforms them into compiled CSS. ~5,500 lines including tests.

## Architecture

Layer 1 processing engine crate. Depends on all Layer 0 + Layer 1 crates.

## StyleX Functions Implemented

- **`stylex.create()`** — Define and transform style objects into class names
- **`stylex.defineVars()`** — Define CSS custom properties (CSS variables)
- **`stylex.keyframes()`** — Define CSS animations
- **`stylex.createTheme()`** — Create theme variables from base definitions
- **`stylex.firstThatWorks()`** — Conditional fallback for style variants
- **`stylex.types()`** — Type definitions (no-op in compiled output)
- **`stylex.positionTry()`** — CSS position-try API integration
- **`stylex.viewTransitionClass()`** — View transition API integration
- **`stylex.defineConsts()`** — Define compile-time constants
- **`stylex.defaultMarker()`** — Default styling for conditions

## Key Modules

Each transformer module implements one or more StyleX functions:

- **`stylex_create.rs`** — `stylex.create()` implementation
  - Generic over `<T: TransformState + Clone>`
  - Handles property validation, class name assignment, CSS generation
  - Supports conditional styles and fallbacks

- **`stylex_define_vars.rs`** — `stylex.defineVars()` implementation
  - CSS variable declaration and registration

- **`stylex_keyframes.rs`** — `stylex.keyframes()` implementation
  - Keyframe animation generation

- **`stylex_create_theme.rs`** — `stylex.createTheme()` implementation
  - Theme variable creation and inheritance

- **`stylex_first_that_works.rs`** — `stylex.firstThatWorks()` implementation
  - Conditional style fallback logic

- Plus modules for `types`, `positionTry`, `viewTransitionClass`, `defineConsts`, `defaultMarker`

## Dependencies

- All Layer 0: `stylex-constants`, `stylex-types`, `stylex-ast`
- All Layer 1: `stylex-evaluator`, `stylex-css`
- `swc_core` — SWC AST types
- `indexmap`, `rustc-hash` — ordered, fast maps
- `log` — logging

## Trait Requirements

All transformer functions are generic over `<T: TransformState + Clone>` to access:

- **StyleX imports** — `stylex_import()`, `stylex_create_import()`, etc.
- **Variable tracking** — `declarations()`, `var_decl_count_map()`, etc.
- **Style registration** — `style_map()`, code frame context
- **Evaluation** — `top_imports()`, `top_level_expressions()`

## Usage

```rust
use stylex_transformers::stylex_create;
use stylex_types::traits::TransformState;

// Transform a stylex.create() call
fn transform_stylex_create<T: TransformState + Clone>(
  config_expr: &Expr,
  path: &Path,
  state: &mut T,
  fns: &FunctionMap,
) -> Result<Expr> {
  stylex_create::transform(config_expr, path, state, fns)
}

// Transform a stylex.defineVars() call
fn transform_define_vars<T: TransformState + Clone>(
  config_expr: &Expr,
  state: &mut T,
) -> Result<Expr> {
  stylex_define_vars::transform(config_expr, state)
}
```
