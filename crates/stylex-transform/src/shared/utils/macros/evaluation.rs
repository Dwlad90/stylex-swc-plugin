//! Evaluation macros for consistent error handling across the workspace.
//!
//! These macros provide standardized patterns for handling common error cases
//! when working with expressions, conversions, and evaluations.

/// Panic with a `[StyleX]`-prefixed code frame error.
///
/// Wraps the expression in a `ParenExpr` and delegates to
/// `build_code_frame_error_and_panic` which produces a source-located
/// `[StyleX] <message>` diagnostic on stderr before panicking.
///
/// # Usage
/// ```ignore
/// stylex_panic_with_context!(path, traversal_state, "Unary expression not implemented");
/// ```
///
/// # Arguments
/// - `$expr`: The expression to wrap and report
/// - `$state`: State manager for error context
/// - `$msg`: Error message string
#[macro_export]
macro_rules! stylex_panic_with_context {
  ($expr:expr, $state:expr, $msg:expr) => {{
    let paren_expr = stylex_ast::ast::factories::wrap_in_paren_ref($expr);
    $crate::shared::utils::log::build_code_frame_error::build_code_frame_error_and_panic(
      &paren_expr,
      $expr,
      $msg,
      $state,
    )
  }};
}

/// Macro to safely convert an expression to a string with proper error
/// handling. Returns the string on success, or calls deopt and returns None on
/// failure.
///
/// This macro is designed for use in evaluation contexts where we need to:
/// - Convert an expression to a string
/// - Call deopt() if conversion fails
/// - Return None to indicate failure
///
/// # Usage
/// ```ignore
/// let str_value = expr_to_str_or_deopt!(expr, state, traversal_state, fns, "Expression is not a string");
/// ```
///
/// # Arguments
/// - `$expr`: The expression to convert
/// - `$state`: Mutable reference to EvaluationState
/// - `$traversal_state`: Mutable reference to StateManager
/// - `$fns`: Reference to FunctionMap
/// - `$error_msg`: Error message string literal
#[macro_export]
macro_rules! expr_to_str_or_deopt {
  ($expr:expr, $state:expr, $traversal_state:expr, $fns:expr, $error_msg:expr) => {
    match $crate::shared::utils::ast::convertors::convert_expr_to_str($expr, $traversal_state, $fns)
    {
      Some(s) => s,
      None => {
        $crate::shared::utils::js::evaluate::deopt($expr, $state, $error_msg);
        return None;
      },
    }
  };
}
