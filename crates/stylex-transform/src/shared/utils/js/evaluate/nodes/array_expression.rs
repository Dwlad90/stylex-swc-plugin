use super::super::*;
use swc_core::ecma::ast::ArrayLit;

pub(in super::super) fn evaluate(
  arr_path: &ArrayLit,
  state: &mut EvaluationState,
  traversal_state: &mut StateManager,
) -> Option<EvaluateResultValue> {
  let mut arr: Vec<EvaluateResultValue> = Vec::with_capacity(arr_path.elems.len());

  for elem in arr_path.elems.iter().flatten() {
    let elem_value =
      evaluate_with_functions(&elem.expr, traversal_state, Rc::clone(&state.functions));
    if elem_value.confident {
      arr.push(elem_value.value.unwrap_or(EvaluateResultValue::Null));
    } else {
      return None;
    }
  }

  Some(EvaluateResultValue::Vec(arr))
}
