//! Error handling helper macros for consistent error handling across the workspace.
//!
//! These macros provide standardized patterns for handling common error cases
//! when working with expressions, conversions, and evaluations.

/// Macro to unwrap a Result or panic with the error message.
/// This is a cleaner replacement for `.unwrap_or_else(|error| panic!("{}", error))`.
///
/// # Usage
/// ```ignore
/// let value = unwrap_or_panic!(expr_to_num(&arg, state, traversal_state, fns));
/// // Or with additional context:
/// let value = unwrap_or_panic!(expr_to_num(&arg, state, traversal_state, fns), "Converting to number");
/// ```
///
/// # Arguments
/// - `$result`: A Result type to unwrap
/// - `$context` (optional): Additional context string to prepend to the error message
#[macro_export]
macro_rules! unwrap_or_panic {
  ($result:expr) => {
    $result.unwrap_or_else(|error| panic!("{}", error))
  };
  ($result:expr, $context:expr) => {
    $result.unwrap_or_else(|error| panic!("{}: {}", $context, error))
  };
}

/// Macro to create a ParenExpr-wrapped error and panic with a code frame.
/// This is a common pattern when reporting errors with build_code_frame_error_and_panic.
///
/// # Usage
/// ```ignore
/// panic_with_context!(path, traversal_state, "Unary expression not implemented");
/// ```
///
/// # Arguments
/// - `$expr`: The expression to wrap and report
/// - `$state`: State manager for error context
/// - `$msg`: Error message string
#[macro_export]
macro_rules! panic_with_context {
  ($expr:expr, $state:expr, $msg:expr) => {{
    let paren_expr = $crate::shared::utils::ast::factories::wrap_in_paren_ref($expr);
    $crate::shared::utils::log::build_code_frame_error::build_code_frame_error_and_panic(
      &paren_expr,
      $expr,
      $msg,
      $state,
    )
  }};
}

/// Macro to safely convert an expression to a string with proper error handling.
/// Returns the string on success, or calls deopt and returns None on failure.
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
    match $crate::shared::utils::ast::convertors::expr_to_str($expr, $traversal_state, $fns) {
      Some(s) => s,
      None => {
        $crate::shared::utils::js::evaluate::deopt($expr, $state, $error_msg);
        return None;
      }
    }
  };
}

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

