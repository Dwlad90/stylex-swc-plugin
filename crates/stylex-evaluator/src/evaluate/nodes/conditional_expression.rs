use super::super::*;
use swc_core::ecma::ast::CondExpr;

pub(in super::super) fn evaluate(
  cond: &CondExpr,
  state: &mut EvaluationState,
  traversal_state: &mut StateManager,
  fns: &FunctionMap,
) -> Option<EvaluateResultValue> {
  // One question rather than two. A fold that answered nothing is a fold that
  // refused, and the refusal is already recorded on the state -- so asking
  // whether the state is still confident and then whether there is a value
  // asks the same thing twice, and leaves the second arm unreachable.
  let test_value = evaluate_cached(&cond.test, state, traversal_state, fns)?;

  // Read through the same `ToBoolean` bridge the logical operators read, and
  // read off the evaluated *value* rather than off an expression form of it.
  // Both halves matter. The bridge is the only truthiness table -- a second
  // copy beside it drifted, and called `NaN` true where `NaN || x` called it
  // false. And a value with no expression form is still a value with a
  // truthiness: the evaluator spells an array as its own vector and a folded
  // namespace as a function map, and every one of those stands for an object,
  // which is truthy whatever it holds. Requiring an expression form here
  // refused `[] ? a : b` on a test the language has no doubt about.
  //
  // A value with no truthiness at all reads as `false` rather than refusing,
  // and is a reading no case can be written for: the one such value is the
  // absent element of an array, which the evaluator only ever holds *inside* a
  // list. `false` is what the language gives a missing test, and an arm no case
  // can enter is a claim nothing checks.
  let takes_the_consequent = evaluate_result_to_js_boolean(&test_value).unwrap_or(false);

  match takes_the_consequent {
    true => evaluate_cached(&cond.cons, state, traversal_state, fns),
    false => evaluate_cached(&cond.alt, state, traversal_state, fns),
  }
}
