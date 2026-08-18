use super::super::*;
use crate::deopt_unsupported;
use swc_core::ecma::ast::CondExpr;

pub(in super::super) fn evaluate(
  cond: &CondExpr,
  state: &mut EvaluationState,
  traversal_state: &mut StateManager,
  fns: &FunctionMap,
) -> Option<EvaluateResultValue> {
  let test_result = evaluate_cached(&cond.test, state, traversal_state, fns);

  if !state.confident {
    return None;
  }

  // A test with no expression form has no compile-time truthiness, so neither
  // branch can be chosen. `fn() ? a : b` is ordinary JavaScript and belongs in
  // the output unfolded.
  let Some(EvaluateResultValue::Expr(ref expr)) = test_result else {
    let path = Expr::Cond(cond.clone());

    deopt_unsupported!(&path, state, ILLEGAL_PROP_VALUE);
  };

  let test_result = convert_expr_to_bool(expr, traversal_state, fns);

  if !state.confident {
    return None;
  }

  if test_result {
    evaluate_cached(&cond.cons, state, traversal_state, fns)
  } else {
    evaluate_cached(&cond.alt, state, traversal_state, fns)
  }
}
