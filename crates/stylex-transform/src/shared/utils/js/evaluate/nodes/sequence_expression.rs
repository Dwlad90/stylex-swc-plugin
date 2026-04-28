use super::super::*;
use swc_core::ecma::ast::SeqExpr;

pub(in super::super) fn evaluate(
  sec: &SeqExpr,
  state: &mut EvaluationState,
  traversal_state: &mut StateManager,
  fns: &FunctionMap,
) -> Option<EvaluateResultValue> {
  let expr = match sec.exprs.last() {
    Some(e) => e,
    #[cfg_attr(coverage_nightly, coverage(off))]
    None => stylex_panic!("Sequence expression must contain at least one expression."),
  };

  evaluate_cached(expr, state, traversal_state, fns)
}
