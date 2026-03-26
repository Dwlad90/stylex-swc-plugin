# `StyleX Evaluator`

## Overview

Pure utility functions extracted from the StyleX JS expression evaluator
and common utilities.

## Contents

### Common Utils (`common`)
- `create_hash`, `create_short_hash`, `stable_hash` -- Hashing utilities
- `evaluate_bin_expr` -- Binary expression evaluation (arithmetic, bitwise)
- `get_expr_from_var_decl` -- Variable declaration accessor
- `hash_f64`, `round_f64` -- Numeric utilities
- `wrap_key_in_quotes` -- String formatting
- `normalize_expr`, `char_code_at`, `sort_numbers_factory` -- Misc helpers

### JS Helpers (`js/helpers`)
- `is_valid_callee`, `get_callee_name` -- Call expression inspection
- `is_invalid_method`, `is_mutating_object_method`,
  `is_mutating_array_method` -- Method validation
- `is_mutation_expr` -- Mutation detection
- `get_method_name`, `is_id_prop` -- Property inspection

## Note

The core evaluator (`evaluate.rs`, `check_declaration.rs`,
`native_functions.rs`) remains in `stylex-transform` due to deep
`StateManager` coupling. This crate contains only the pure helper
functions that were extractable without code changes.

## Layer

Layer 1. Dependencies: `stylex-constants`, `stylex-types`, `stylex-ast`,
`stylex-macros`, `stylex-core`, `stylex-path-resolver`.
