use super::*;

pub(crate) fn evaluate_cached(
  path: &Expr,
  state: &mut EvaluationState,
  traversal_state: &mut StateManager,
  fns: &FunctionMap,
) -> Option<EvaluateResultValue> {
  let cleaned_path_hash = stable_hash_unspanned(path);

  let existing = traversal_state.seen.get(&cleaned_path_hash);

  match existing {
    Some(evaluate_value) => {
      let evaluated_value: &SeenValue = evaluate_value.borrow();

      if evaluated_value.resolved {
        return evaluated_value.value.clone();
      }

      deopt(path, state, PATH_WITHOUT_NODE)
    },
    None => {
      let val = _evaluate(path, state, traversal_state, fns);

      let seen_value = if state.confident {
        SeenValue {
          value: val.clone(),
          resolved: true,
        }
      } else {
        SeenValue {
          value: None,
          resolved: false,
        }
      };

      traversal_state
        .seen
        .entry(cleaned_path_hash)
        .or_insert_with(|| Rc::new(seen_value));

      val
    },
  }
}
