use super::super::*;
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

  let test_result = match match test_result {
    Some(v) => v,
    #[cfg_attr(coverage_nightly, coverage(off))]
    None => {
      stylex_panic!("The test condition of a conditional expression must be a static expression.")
    },
  } {
    EvaluateResultValue::Expr(ref expr) => convert_expr_to_bool(expr, traversal_state, fns),
    _ => {
      let path = Expr::Cond(cond.clone());
      stylex_panic_with_context!(
        &path,
        traversal_state,
        "The test condition of a conditional expression must be a static expression."
      )
    },
  };

  if !state.confident {
    return None;
  }

  if test_result {
    evaluate_cached(&cond.cons, state, traversal_state, fns)
  } else {
    evaluate_cached(&cond.alt, state, traversal_state, fns)
  }
}
