use super::super::*;
use crate::deopt_unsupported;
use stylex_ast::ast::convertors::normalize_expr;
use stylex_utils::number::to_js_string;
use swc_core::ecma::ast::ObjectLit;

/// The own enumerable properties a spread operand contributes, mirroring
/// `Object.assign({}, value)` — which is literally what the reference
/// implementation calls, on a real JavaScript value, at this point.
///
/// Spreading a non-object is not an error in the language and is not one here:
/// a number, a boolean, `null`, `undefined` and a function have no own
/// enumerable properties, so they contribute nothing and the object is built
/// from whatever else it holds. A string and an array do have them — their
/// indices — so `{ ...'ab' }` is `{ 0: 'a', 1: 'b' }` and `{ ...[1, 2] }` is
/// `{ 0: 1, 1: 2 }`, with each value keeping the type it had, which is why the
/// second declares `1px` and not `'1'`.
///
/// `None` is a refusal, kept for the two readings this evaluator cannot write
/// down rather than used as a general "not an object" answer — that answer is
/// `Some(vec![])`.
/// The properties an indexable value contributes: one per element, keyed by its
/// position.
///
/// The three indexable readings -- a string's code units, an evaluated array's
/// elements, and the elements of an array a fold produced -- differ only in how
/// each element becomes an expression, so they differ only in the iterator they
/// hand over. `None` from that iterator is an element with no expression form,
/// and refuses the whole receiver.
fn indexed_props(
  elements: impl ExactSizeIterator<Item = Option<Expr>>,
) -> Option<Vec<PropOrSpread>> {
  let mut props = Vec::with_capacity(elements.len());

  for (index, element) in elements.enumerate() {
    props.push(create_ident_key_value_prop(&index.to_string(), element?));
  }

  Some(props)
}

fn spread_own_properties(value: EvaluateResultValue, operand: &Expr) -> Option<Vec<PropOrSpread>> {
  // An array hole has no key of its own, so an operand carrying one would answer
  // `{ 0: 1 }` where the language says `{ 1: 1 }` if the hole were dropped
  // rather than kept. Read off the operand's own literal, where it has one, so
  // the wrong key is refused rather than written. A trailing comma is not a
  // hole: `[1, ]` has one element.
  //
  // A guard rather than a live path, since `array_expression` refuses a holey
  // array outright and the spread operand is evaluated before it reaches here.
  // Both refusals read the same sentence upstream gives, so which one fires is
  // invisible to an author — and the check is a bounds test against a wrong key,
  // which is worth keeping if that order ever changes.
  if let Expr::Array(array) = normalize_expr(operand)
    && array.elems.iter().any(|elem| elem.is_none())
  {
    return None;
  }

  match value {
    EvaluateResultValue::Expr(Expr::Object(object)) => Some(object.props),

    // A string's own properties are its indices, one per UTF-16 code unit.
    //
    // An astral character occupies two of them and each is a lone surrogate,
    // which no Rust string can hold — upstream duly emits two replacement
    // characters into the stylesheet. Refused rather than approximated: the
    // alternative is writing a value the source does not describe, and this is
    // the same rule `char_code_at` and `json_stringify` already leave
    // unreachable for the same reason.
    EvaluateResultValue::Expr(Expr::Lit(Lit::Str(strng))) => {
      let value = strng.value.as_str()?;
      let characters = value.chars().collect::<Vec<_>>();

      indexed_props(
        characters
          .iter()
          .map(|character| match character.len_utf16() {
            1 => Some(create_string_expr(&character.to_string())),
            _ => None,
          }),
      )
    },

    // An array's own properties are its indices. Both readings an array can
    // arrive as are handled: the `Vec` an array literal evaluates to, and the
    // `ArrayLit` a fold such as `Object.keys` answers.
    EvaluateResultValue::Vec(items) => indexed_props(items.iter().map(|item| match item {
      EvaluateResultValue::Vec(nested) => evaluate_result_vec_to_array_expr(nested),
      _ => item.as_expr().cloned(),
    })),
    EvaluateResultValue::Expr(Expr::Array(array)) => {
      indexed_props(array.elems.iter().map(|elem| match elem {
        // A hole written as one in a fold's own output means what it means
        // above, and a spread there is no more countable than anywhere else.
        Some(elem) if elem.spread.is_none() => Some(*elem.expr.clone()),
        _ => None,
      }))
    },

    // Everything with no own enumerable properties. A number, a boolean and
    // `null` have none, and neither does a function: `Object.assign({}, () =>
    // 1)` is `{}`. A function reaches here as the callback the evaluator folded
    // it to rather than as an `Expr`, which is why both readings are named.
    EvaluateResultValue::Expr(
      Expr::Lit(Lit::Num(_) | Lit::Bool(_) | Lit::Null(_)) | Expr::Arrow(_) | Expr::Fn(_),
    )
    | EvaluateResultValue::Callback(_) => Some(vec![]),

    // A folded function map or function config is a plain object upstream, not a
    // function, so its keys are own enumerable properties and a spread of it
    // contributes every one. Answering empty here spread nothing and compiled a
    // style object the author did not write -- where the reference
    // implementation spreads the keys and refuses the first value that is not a
    // style value.
    value
    @ (EvaluateResultValue::FunctionConfig(_) | EvaluateResultValue::FunctionConfigMap(_)) => {
      function_fold_to_object(&value).map(|object| object.props)
    },
    // A global primitive still spelled as an identifier. In practice that is
    // `undefined` alone: the resolution chain answers `NaN` and `Infinity` with
    // the numbers they are, so those arrive as `Lit::Num` and are answered by
    // the arm above. The predicate stays the whole set rather than the one
    // name, because an identifier reaching here from somewhere the chain did
    // not resolve -- a binding in an imported file, which this compiler does
    // not evaluate -- may still be any of the three, and all three have no own
    // enumerable properties either way.
    EvaluateResultValue::Expr(Expr::Ident(ident)) if is_global_spelled_as_an_identifier(&ident) => {
      Some(vec![])
    },

    // A value carried in a representation of the evaluator's own — a theme
    // reference, an entries map, a callback. Refused rather than answered
    // empty, because answering empty would silently drop whatever it holds.
    _ => None,
  }
}

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

        let Some(new_props) =
          spread_expression.and_then(|value| spread_own_properties(value, &prop.expr))
        else {
          deopt_unsupported!(path, state, SPREAD_PROPERTIES_UNREADABLE);
        };

        let merged_object = assign_props(props, new_props);

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
              // Rendered as JavaScript spells a number, not as Rust does:
              // `{ 1e21: x }` names the property `"1e+21"`, where
              // `f64::to_string` would name it `"1000000000000000000000"`. The
              // same reader decides whether two keys collide, so two spellings
              // here is how one key comes to be two.
              PropName::Num(num) => Some(to_js_string(num.value)),
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

                // The key path is this compiler's own, and deliberately so --
                // the reference compiler's counterpart here is
                // `deopt(prop, state, state.deoptReason ?? 'unknown error')`,
                // with no key in it. See `prepend_key_to_reason` in
                // `utils::core::evaluate_stylex_create_arg` for why the
                // divergence is kept rather than closed.
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

            // Every arm answers an expression or refuses. An arm that answered
            // "no value" would drop the property and compile the object as if
            // the author had not written it -- which is what a theme reference
            // read without a member access used to do, silently, where upstream
            // refuses it. A value this evaluator cannot write down is a refusal,
            // never an omission.
            let value = match value {
              EvaluateResultValue::Expr(expr) => expr,
              EvaluateResultValue::Vec(items) => match evaluate_result_vec_to_array_expr(&items) {
                Some(expr) => expr,
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

                  cb(cb_args, traversal_state)
                },
                Expr::Arrow(arrow_func_expr) => Expr::Arrow(arrow_func_expr.clone()),
                _ => deopt_unsupported!(path, state, ILLEGAL_PROP_VALUE),
              },
              // A folded function map or function config, materialized as the
              // object it stands for so it reaches whatever validates this
              // position -- the same object the dynamic style's value position
              // builds, from the same function. Everything else with no
              // expression form is a refusal.
              value => match function_fold_to_object(&value) {
                Some(object) => Expr::from(object),
                None => deopt_unsupported!(path, state, ILLEGAL_PROP_VALUE),
              },
            };

            props.push(create_ident_key_value_prop(
              &match key {
                Some(k) => k,
                None => stylex_panic!("Property key must be present in the style object."),
              },
              value,
            ));
          },
          // A getter, a setter or an assignment pattern: object properties
          // with no compile-time value of their own.
          _ => deopt_unsupported!(path, state, OBJECT_METHOD),
        }
      },
    }
  }

  // Ordered last, once, rather than maintained as properties are added: an
  // array-index key can arrive from a literal, from a spread, or from a
  // computed key, and the language's answer depends only on the set that ends
  // up here.
  Some(EvaluateResultValue::Expr(Expr::Object(create_object_lit(
    order_own_keys(remove_duplicates(props)),
  ))))
}
