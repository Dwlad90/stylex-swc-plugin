use super::super::*;
use stylex_ast::ast::convertors::{convert_member_prop_to_string, normalize_expr};
use swc_core::ecma::ast::MemberExpr;

pub(in super::super) fn evaluate(
  member: &MemberExpr,
  state: &mut EvaluationState,
  traversal_state: &mut StateManager,
  fns: &FunctionMap,
) -> Option<EvaluateResultValue> {
  let path = Expr::Member(member.clone());
  let path = &path;
  let parent_is_call_expr = traversal_state.is_member_call_callee(member);

  let evaluated_value = if parent_is_call_expr {
    None
  } else {
    // ThemeRef fast-path. Only run for member chains whose base is a plain
    // identifier — the only shape that can resolve to a `ThemeRef` (either a
    // local `ThemeRefMapper` registered in `fns.identifiers` or a cross-file
    // `*.stylex.js` import). Skipping computed / call / object bases avoids
    // paying for a speculative `evaluate_cached` that can never produce a
    // ThemeRef and may early-deopt via `state.confident` for unrelated deep
    // member accesses.
    if let Some((base_path, parts)) = get_full_member_path(member)
      && is_theme_ref_base(&base_path)
    {
      let base_object = evaluate_cached(&base_path, state, traversal_state, fns);

      if !state.confident {
        return None;
      }

      if let Some(EvaluateResultValue::ThemeRef(mut theme_ref)) = base_object {
        let value = theme_ref.get(&parts.join("."), traversal_state);

        let Some(css_var) = value.as_css_var() else {
          deopt_unsupported!(path, state, EXPECTED_CSS_VAR);
        };

        return Some(EvaluateResultValue::Expr(create_string_expr(css_var)));
      }
    }

    evaluate_cached(&member.obj, state, traversal_state, fns)
  };
  match evaluated_value {
    Some(object) => {
      if !state.confident {
        return None;
      }

      let prop_path = &member.prop;

      let property = match prop_path {
        MemberProp::Ident(ident) => Some(EvaluateResultValue::Expr(Expr::from(ident.clone()))),
        MemberProp::Computed(ComputedPropName { expr, .. }) => {
          let result = evaluate_cached(expr, state, traversal_state, fns);

          if !state.confident {
            return None;
          }

          result
        },
        MemberProp::PrivateName(_) => {
          return deopt(path, state, UNEXPECTED_MEMBER_LOOKUP);
        },
      };

      match object {
        EvaluateResultValue::Expr(expr) => match &expr {
          Expr::Array(ArrayLit { elems, .. }) => {
            let Some(eval_res) = property else {
              deopt_unsupported!(path, state, PROPERTY_NOT_FOUND);
            };

            let EvaluateResultValue::Expr(expr) = eval_res else {
              deopt_unsupported!(path, state, PROPERTY_NOT_FOUND);
            };

            // Only a numeric index reads an array element at compile time.
            // `list['length']` and `list[key]` are ordinary JavaScript this
            // evaluator does not fold.
            let Expr::Lit(Lit::Num(Number { value, .. })) = expr else {
              deopt_unsupported!(path, state, MEMBER_NOT_RESOLVED);
            };

            let value = value as usize;

            // An index past the end is `undefined` in the language, and the
            // object arm below now folds the matching case — a key the object
            // does not carry. This arm is deliberately left answering no value
            // instead, because no StyleX source reaches it: an array binding
            // evaluates to the `Vec` variant, which `match object` has no arm
            // for at all, so indexing one refuses at the catch-all below
            // whether the index is in range or not. Making the two agree is a
            // matter of teaching `Vec` to be indexed, which is its own scope.
            let property = elems.get(value)?;

            // An array hole reads as `undefined`, which this arm does not
            // represent — see the note above on why it refuses rather than
            // answering one.
            let Some(expr) = property.as_ref() else {
              deopt_unsupported!(path, state, MEMBER_NOT_RESOLVED);
            };

            let expr = expr.expr.clone();

            Some(EvaluateResultValue::Expr(*expr))
          },
          Expr::Object(ObjectLit { props, .. }) => {
            let Some(eval_res) = property else {
              deopt_unsupported!(path, state, PROPERTY_NOT_FOUND);
            };

            let ident = match eval_res {
              EvaluateResultValue::Expr(ident) => ident,
              EvaluateResultValue::ThemeRef(theme) => {
                // NOTE: it's a very edge case, but it's possible to have a theme ref as a key
                // in an object, when theme import key is same as other variable name.
                // One of the reasons is code minification or obfuscation,
                // when theme import key is renamed to a shorter name.
                // Also it may be a result of a bug in the code.

                warn!(
                  "A theme import key is being used as an object key. This might be caused by code minification or an internal error.\r\nFor additional details, please recompile using debug mode."
                );

                debug!("Evaluating member access on object:");
                debug!("Object expression: {:?}", expr);
                debug!("Theme reference: {:?}", theme);
                debug!("Original property: {:?}", prop_path);

                return deopt(path, state, THEME_IMPORT_KEY_AS_OBJECT_KEY);
              },
              _ => {
                debug!("Property not found for expression: {:?}", expr);
                debug!("Evaluation result: {:?}", eval_res);
                debug!("Original property: {:?}", prop_path);

                deopt_unsupported!(path, state, PROPERTY_NOT_FOUND);
              },
            };

            let normalized_ident = normalize_expr(&ident);

            let ident_string_name = match normalized_ident {
              Expr::Ident(ident) => ident.sym.to_string(),
              // A regex or a BigInt key has no string form the evaluator
              // reads, and a key that is still an expression never resolved.
              Expr::Lit(lit) => match convert_lit_to_string(lit) {
                Some(key) => key,
                None => deopt_unsupported!(path, state, UNEXPECTED_MEMBER_LOOKUP),
              },
              _ => deopt_unsupported!(path, state, UNEXPECTED_MEMBER_LOOKUP),
            };

            // Written as a loop rather than a `find`, because a property the
            // evaluator cannot read has to refuse the whole lookup and a
            // predicate has no way to say so — the closure would have to
            // abort, which is the failure this split exists to remove.
            let mut found = None;

            for prop in props {
              let PropOrSpread::Prop(prop) = prop else {
                // A spread leaves the object's own keys unknown, so a key that
                // is not among the literal ones cannot be called absent.
                deopt_unsupported!(path, state, SPREAD_MUST_BE_OBJECT);
              };

              let mut prop = prop.clone();

              expand_shorthand_prop(&mut prop);

              // A getter, a setter or a method carries no value to read.
              let Prop::KeyValue(key_value) = prop.as_ref() else {
                deopt_unsupported!(path, state, OBJECT_METHOD);
              };

              if ident_string_name == convert_key_value_to_str(key_value) {
                found = Some(key_value.value.clone());
                break;
              }
            }

            // A key the object does not carry reads as `undefined`, which is a
            // value the evaluator is confident about rather than one it failed
            // to resolve. Returning it is what lets `token.missing ?? fallback`
            // fold, where a deopt here would send the whole declaration to the
            // runtime.
            let Some(value) = found else {
              return Some(js_undefined());
            };

            Some(EvaluateResultValue::Expr(*value))
          },
          Expr::Member(member_expr) => evaluate_cached(
            &Expr::Member(member_expr.clone()),
            state,
            traversal_state,
            fns,
          ),
          Expr::Lit(nested_lit) => {
            evaluate_cached(&Expr::Lit(nested_lit.clone()), state, traversal_state, fns)
          },
          Expr::Ident(nested_ident) => evaluate_cached(
            &Expr::Ident(nested_ident.clone()),
            state,
            traversal_state,
            fns,
          ),
          // A member access on a call, an arrow, a class — expression kinds
          // this evaluator reads no properties from.
          _ => deopt_unsupported!(
            path,
            state,
            &unsupported_expression(&format!("{:?}", expr.get_type(get_default_expr_ctx())))
          ),
        },
        EvaluateResultValue::FunctionConfigMap(fc_map) => {
          let key = match property {
            Some(EvaluateResultValue::Expr(Expr::Ident(ident))) => ident,
            _ => deopt_unsupported!(path, state, MEMBER_NOT_RESOLVED),
          };

          if let Some(fc) = fc_map.get(&key.sym) {
            return Some(EvaluateResultValue::FunctionConfig(fc.clone()));
          }

          // Check if this is an env property access on a stylex import.
          if key.sym.as_ref() == STYLEX_ENV {
            if traversal_state.options.env.is_empty() {
              deopt_unsupported!(
                path,
                state,
                "The stylex.env object is not configured. Check that the 'env' option is set in your StyleX configuration."
              );
            }

            return Some(EvaluateResultValue::EnvObject(
              traversal_state.options.env.clone(),
            ));
          }

          deopt_unsupported!(
            path,
            state,
            format!(
              "The property '{}' was not found in the function configuration.",
              key.sym
            )
            .as_str()
          );
        },
        EvaluateResultValue::ThemeRef(mut theme_ref) => {
          let key = match property {
            Some(EvaluateResultValue::Expr(Expr::Ident(Ident { sym, .. }))) => sym.to_string(),
            Some(EvaluateResultValue::Expr(Expr::Lit(lit))) => match convert_lit_to_string(&lit) {
              Some(key) => key,
              None => deopt_unsupported!(path, state, UNEXPECTED_MEMBER_LOOKUP),
            },
            _ => deopt_unsupported!(path, state, MEMBER_NOT_RESOLVED),
          };

          let value = theme_ref.get(&key, traversal_state);

          let Some(css_var) = value.as_css_var() else {
            deopt_unsupported!(path, state, EXPECTED_CSS_VAR);
          };

          Some(EvaluateResultValue::Expr(create_string_expr(css_var)))
        },
        EvaluateResultValue::EnvObject(env_map) => {
          let Some(key) = property.as_ref().and_then(|prop| prop.as_string_key()) else {
            deopt_unsupported!(path, state, UNEXPECTED_MEMBER_LOOKUP);
          };

          let Some(entry) = env_map.get(&key) else {
            deopt_unsupported!(
              path,
              state,
              format!(
                "The property '{}' was not found in the stylex.env configuration.",
                key
              )
              .as_str()
            );
          };

          match resolve_env_entry_to_result(entry, &env_map) {
            Some(result) => Some(result),
            None => deopt_unsupported!(path, state, ILLEGAL_PROP_VALUE),
          }
        },
        // An evaluated value the member path reads no properties from: a
        // callback, an entries map, a raw function configuration.
        _ => deopt_unsupported!(path, state, UNEXPECTED_MEMBER_LOOKUP),
      }
    },
    _ => None,
  }
}

fn get_full_member_path(member: &MemberExpr) -> Option<(Expr, Vec<String>)> {
  let mut parts = Vec::new();
  let mut current = member;

  loop {
    parts.insert(0, convert_member_prop_to_string(&current.prop)?);

    match current.obj.as_ref() {
      Expr::Member(member) => {
        current = member;
      },
      base => {
        if parts.len() < 2 {
          return None;
        }

        return Some((base.clone(), parts));
      },
    }
  }
}

/// Returns `true` when `base` is a plain identifier — the only shape that can
/// resolve to a `ThemeRef` in our evaluator (either via `fns.identifiers` for
/// in-file `defineVars` exports, or via cross-file `*.stylex.js` imports
/// handled in `evaluate::mod`). Any other expression kind (`Member`, `Call`,
/// `Object`, `Array`, …) is guaranteed not to produce a `ThemeRef`, so we
/// skip the fast-path eval to avoid the speculative work the Copilot review
/// flagged.
fn is_theme_ref_base(base: &Expr) -> bool {
  matches!(base, Expr::Ident(_))
}
