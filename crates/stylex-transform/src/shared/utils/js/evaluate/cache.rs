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
      let evaluate_value: &SeenValueWithVarDeclCount = evaluate_value.borrow();

      let evaluated_value = &evaluate_value.seen_value;
      let var_decl_count_value_diff = &evaluate_value.var_decl_count;

      if evaluated_value.resolved {
        let resolved = evaluated_value.value.clone();

        match path {
          Expr::Ident(ident) => reduce_ident_count(traversal_state, ident),
          Expr::Member(member) => reduce_member_expression_count(traversal_state, member),
          Expr::Object(_) => {
            if let Some(var_decl_count_value_diff) = var_decl_count_value_diff {
              traversal_state.var_decl_count_map = sum_hash_map_values(
                var_decl_count_value_diff,
                &traversal_state.var_decl_count_map,
              );
            }
          },
          _ => {},
        }

        return resolved;
      }

      deopt(path, state, PATH_WITHOUT_NODE)
    },
    None => {
      let mut cleaned_path = drop_span(path.clone());
      let should_save_var_decl_count = path.is_object();

      let var_decl_count_map_orig =
        should_save_var_decl_count.then(|| traversal_state.var_decl_count_map.clone());

      let val = _evaluate(&mut cleaned_path, state, traversal_state, fns);

      let var_decl_count_value_diff = var_decl_count_map_orig.as_ref().map(|orig| {
        let var_decl_count_map_diff =
          get_hash_map_difference(&traversal_state.var_decl_count_map, orig);

        get_hash_map_value_difference(&var_decl_count_map_diff, orig)
      });

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
        .or_insert_with(|| {
          Rc::new(SeenValueWithVarDeclCount {
            seen_value,
            var_decl_count: var_decl_count_value_diff,
          })
        });

      val
    },
  }
}
