# `StyleX Macros`

## Overview

Error handling and utility macros for the StyleX SWC plugin.

## Contents

- `panic_macros` -- `stylex_panic!`, `stylex_unimplemented!`,
  `stylex_unreachable!`, `stylex_bail!`, `stylex_anyhow!`,
  `stylex_unwrap!`, `unwrap_or_panic!`
- `stylex_error` -- `StyleXError` structured error type,
  `SuppressPanicStderr` guard, `format_panic_message`
- `collection_macros` -- `collect_confident!` macro for evaluation
  iteration
- `conversion_macros` -- `expr_to_str_or_err!`, `as_expr_or_err!`,
  `as_expr_or_opt_err!`, `as_expr_or_panic!`

## Layer

Layer 0 leaf crate. Dependencies: `stylex-constants`, `ctor`, `colored`,
`log`, `pretty_env_logger`.
