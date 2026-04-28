use super::*;

pub(crate) fn deopt(
  path: &Expr,
  state: &mut EvaluationState,
  reason: &str,
) -> Option<EvaluateResultValue> {
  if state.confident {
    state.confident = false;
    state.deopt_path = Some(path.clone());
    state.deopt_reason = Some(reason.to_string());
  }

  None
}
