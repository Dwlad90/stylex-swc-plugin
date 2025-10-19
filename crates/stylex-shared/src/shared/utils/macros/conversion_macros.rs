//! Conversion and type extraction macros for safe value unwrapping.
//!
//! These macros provide standardized patterns for converting between different
//! types and extracting values with proper error handling.

/// Macro to safely convert an expression to a string with error Result handling.
/// Returns the string on success, or returns Err(anyhow::Error) on failure.
///
/// This macro is designed for use in functions that return Result<T, anyhow::Error>.
///
/// # Usage
/// ```ignore
/// let str_value = expr_to_str_or_err!(expr, traversal_state, fns, "Expression is not a string");
/// ```
///
/// # Arguments
/// - `$expr`: The expression to convert
/// - `$traversal_state`: Mutable reference to StateManager
/// - `$fns`: Reference to FunctionMap
/// - `$error_msg`: Error message string literal
#[macro_export]
macro_rules! expr_to_str_or_err {
  ($expr:expr, $traversal_state:expr, $fns:expr, $error_msg:expr) => {
    match $crate::shared::utils::ast::convertors::expr_to_str($expr, $traversal_state, $fns) {
      Some(s) => s,
      None => return Err(anyhow!($error_msg)),
    }
  };
}

/// Macro to unwrap an Option<EvaluateResultValue> to Expr or return an error.
/// Returns the expression on success, or returns Err(anyhow::Error) on failure.
///
/// This macro is designed for use in functions that return Result<T, anyhow::Error>.
///
/// # Usage
/// ```ignore
/// let expr = as_expr_or_err!(result_value, "Argument not expression");
/// ```
///
/// # Arguments
/// - `$opt`: The EvaluateResultValue to unwrap
/// - `$error_msg`: Error message string literal
#[macro_export]
macro_rules! as_expr_or_err {
  ($opt:expr, $error_msg:expr) => {
    match $opt.as_expr() {
      Some(expr) => expr,
      None => return Err(anyhow!($error_msg)),
    }
  };
}

/// Macro to unwrap an Option<EvaluateResultValue> to Expr for functions returning Option<Result>.
/// Returns the expression on success, or returns Some(Err(anyhow::Error)) on failure.
///
/// This macro is designed for use in functions that return Option<Result<T, anyhow::Error>>.
///
/// # Usage
/// ```ignore
/// let expr = as_expr_or_opt_err!(result_value, "Argument not expression");
/// ```
///
/// # Arguments
/// - `$opt`: The EvaluateResultValue to unwrap
/// - `$error_msg`: Error message string literal
#[macro_export]
macro_rules! as_expr_or_opt_err {
  ($opt:expr, $error_msg:expr) => {
    match $opt.as_expr() {
      Some(expr) => expr,
      None => return Some(Err(anyhow!($error_msg))),
    }
  };
}

/// Macro to unwrap an Option<EvaluateResultValue> to Expr for functions returning primitives.
/// Returns the expression on success, or panics with the error message on failure.
///
/// This macro is designed for use in functions that return primitive types like f64
/// where error handling must be done via panic.
///
/// # Usage
/// ```ignore
/// let expr = as_expr_or_panic!(result_value, "Argument not expression");
/// ```
///
/// # Arguments
/// - `$opt`: The EvaluateResultValue to unwrap
/// - `$error_msg`: Error message string literal
#[macro_export]
macro_rules! as_expr_or_panic {
  ($opt:expr, $error_msg:expr) => {
    match $opt.as_expr() {
      Some(expr) => expr,
      None => panic!("{}", $error_msg),
    }
  };
}
