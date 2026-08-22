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

  // A test that folded to nothing has no compile-time truthiness, so neither
  // branch can be chosen. `fn() ? a : b` is ordinary JavaScript and belongs in
  // the output unfolded.
  let Some(ref test_value) = test_result else {
    let path = Expr::Cond(cond.clone());

    deopt_unsupported!(&path, state, ILLEGAL_PROP_VALUE);
  };

  // Read through the same `ToBoolean` bridge the logical operators read, and
  // read off the evaluated *value* rather than off an expression form of it.
  // Both halves matter. The bridge is the only truthiness table -- a second
  // copy beside it drifted, and called `NaN` true where `NaN || x` called it
  // false. And a value with no expression form is still a value with a
  // truthiness: the evaluator spells an array as its own vector and a folded
  // namespace as a function map, and every one of those stands for an object,
  // which is truthy whatever it holds. Requiring an expression form here
  // refused `[] ? a : b` on a test the language has no doubt about.
  let Some(test_result) = evaluate_result_to_js_boolean(test_value) else {
    let path = Expr::Cond(cond.clone());

    deopt_unsupported!(&path, state, ILLEGAL_PROP_VALUE);
  };

  if test_result {
    evaluate_cached(&cond.cons, state, traversal_state, fns)
  } else {
    evaluate_cached(&cond.alt, state, traversal_state, fns)
  }
}
