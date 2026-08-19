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

/// Refuse to fold an input shape the evaluator does not handle, and return from
/// the evaluation.
///
/// This is the *ordinary* failure of a static evaluation: the author wrote
/// something with no compile-time value, so the expression falls to the
/// runtime. It is the counterpart of the reference implementation's terminal
/// `deopt(path, state, errMsgs.UNSUPPORTED_EXPRESSION(path.node.type))`.
///
/// A broken invariant is [`stylex_panic_with_context!`] instead, and the two are
/// told apart by their argument: this one takes the [`EvaluationState`] it
/// records the refusal on, the panicking one takes the [`StateManager`] it
/// builds a code frame from. Why they are separate constructs at all is
/// `docs/adr/0002-a-refusal-and-a-broken-invariant-are-separate-constructs.md`.
///
/// A refusal is not a silent one: the reason lands on the evaluation state, and
/// a deopt that reaches a position requiring a static value — inside
/// `stylex.create()`, say — is reported there with a code frame built from it.
///
/// # Usage
/// ```ignore
/// deopt_unsupported!(path, state, &unsupported_expression("TaggedTemplateExpression"));
/// ```
///
/// # Arguments
/// - `$expr`: The expression that could not be folded, recorded as the deopt
///   path
/// - `$state`: Mutable reference to the `EvaluationState`
/// - `$reason`: Why it could not be folded, as a `&str`
///
/// [`EvaluationState`]: crate::shared::structures::state::EvaluationState
/// [`StateManager`]: crate::shared::structures::state_manager::StateManager
#[macro_export]
macro_rules! deopt_unsupported {
  ($expr:expr, $state:expr, $reason:expr) => {{
    $crate::shared::utils::js::evaluate::deopt($expr, $state, $reason);

    return None;
  }};
}
