use super::super::*;
use crate::deopt_unsupported;
use swc_core::ecma::ast::ObjectLit;

pub(in super::super) fn evaluate(
  obj_path: &ObjectLit,
  state: &mut EvaluationState,
  traversal_state: &mut StateManager,
  fns: &FunctionMap,
) -> Option<EvaluateResultValue> {
  let path = Expr::Object(obj_path.clone());
  let path = &path;
  let mut props = Vec::with_capacity(obj_path.props.len());

  for prop in &obj_path.props {
    match prop {
      PropOrSpread::Spread(prop) => {
        let spread_expression = evaluate_cached(&prop.expr, state, traversal_state, fns);

        if !state.confident {
          return deopt(path, state, OBJECT_METHOD);
        }

        // `{ ...1 }` and `{ ...fn }` are legal JavaScript that spread nothing;
        // a value with no object form is an input this evaluator does not fold.
        let Some(new_props) = spread_expression.and_then(|s| s.into_object()) else {
          deopt_unsupported!(path, state, SPREAD_MUST_BE_OBJECT);
        };

        let merged_object = deep_merge_props(props, new_props.props);

        props = merged_object;

        continue;
      },
      PropOrSpread::Prop(prop) => {
        if prop.is_method() {
          let deopt_reason = state
            .deopt_reason
            .as_deref()
            .unwrap_or("unknown error")
            .to_string();

          return deopt(path, state, &deopt_reason);
        }

        let mut prop = prop.clone();

        expand_shorthand_prop(&mut prop);

        match prop.as_ref() {
          Prop::KeyValue(path_key_value) => {
            let key = match &path_key_value.key {
              PropName::Ident(ident) => Some(ident.sym.to_string()),
              PropName::Str(strng) => Some(convert_atom_to_string(&strng.value)),
              PropName::Num(num) => Some(num.value.to_string()),
              PropName::Computed(computed) => {
                let evaluated_result = evaluate_with_functions(
                  &computed.expr,
                  traversal_state,
                  Rc::clone(&state.functions),
                );

                if !evaluated_result.confident {
                  if let Some(deopt_val) = evaluated_result.deopt {
                    let deopt_reason = state
                      .deopt_reason
                      .as_deref()
                      .unwrap_or(
                        evaluated_result
                          .reason
                          .as_deref()
                          .unwrap_or("unknown error"),
                      )
                      .to_string();

                    deopt(&deopt_val, state, &deopt_reason);
                  }

                  return None;
                }

                if let Some(expr) = evaluated_result
                  .value
                  .as_ref()
                  .and_then(|value| value.as_expr())
                {
                  Some(expr_to_str_or_deopt!(
                    expr,
                    state,
                    traversal_state,
                    &state.functions,
                    EXPRESSION_IS_NOT_A_STRING
                  ))
                } else {
                  deopt_unsupported!(path, state, ILLEGAL_PROP_VALUE);
                }
              },
              PropName::BigInt(big_int) => Some(big_int.value.to_string()),
            };

            let eval_value = evaluate_with_functions(
              &path_key_value.value,
              traversal_state,
              Rc::clone(&state.functions),
            );

            if !eval_value.confident {
              if let Some(deopt_val) = eval_value.deopt {
                let base_reason = state
                  .deopt_reason
                  .as_deref()
                  .unwrap_or(eval_value.reason.as_deref().unwrap_or("unknown error"))
                  .to_string();

                let deopt_reason = if let Some(ref k) = key {
                  format!("{} > {}", k, base_reason)
                } else {
                  base_reason
                };

                deopt(&deopt_val, state, &deopt_reason);
              }

              return None;
            }

            let Some(value) = eval_value.value else {
              deopt_unsupported!(
                path,
                state,
                format!(
                  "Value of key '{}' has no compile-time value, but got {}",
                  key.clone().unwrap_or_else(|| "Unknown".to_string()),
                  get_expr_node_kind(&path_key_value.value)
                )
                .as_ref()
              );
            };

            let value = match value {
              EvaluateResultValue::Expr(expr) => Some(expr),
              EvaluateResultValue::Vec(items) => match evaluate_result_vec_to_array_expr(&items) {
                Some(expr) => Some(expr),
                None => deopt_unsupported!(path, state, ILLEGAL_PROP_ARRAY_VALUE),
              },
              EvaluateResultValue::Callback(cb) => match path_key_value.value.as_ref() {
                Expr::Call(call_expr) => {
                  let cb_args: Vec<EvaluateResultValue> = call_expr
                    .args
                    .iter()
                    .map(|arg| {
                      let eval_arg = evaluate_cached(&arg.expr, state, traversal_state, fns);

                      if !state.confident {
                        return EvaluateResultValue::Null;
                      }

                      eval_arg.unwrap_or(EvaluateResultValue::Null)
                    })
                    .collect();

                  Some(cb(cb_args, traversal_state))
                },
                Expr::Arrow(arrow_func_expr) => Some(Expr::Arrow(arrow_func_expr.clone())),
                _ => deopt_unsupported!(path, state, ILLEGAL_PROP_VALUE),
              },
              EvaluateResultValue::ThemeRef(_) => None,
              _ => deopt_unsupported!(path, state, ILLEGAL_PROP_VALUE),
            };

            if let Some(value) = value {
              props.push(create_ident_key_value_prop(
                &match key {
                  Some(k) => k,
                  None => stylex_panic!("Property key must be present in the style object."),
                },
                value,
              ));
            }
          },
          // A getter, a setter or an assignment pattern: object properties
          // with no compile-time value of their own.
          _ => deopt_unsupported!(path, state, OBJECT_METHOD),
        }
      },
    }
  }

  Some(EvaluateResultValue::Expr(Expr::Object(create_object_lit(
    remove_duplicates(props),
  ))))
}
