use super::super::*;
use swc_core::ecma::ast::MemberExpr;

pub(in super::super) fn evaluate(
  member: &MemberExpr,
  state: &mut EvaluationState,
  traversal_state: &mut StateManager,
  fns: &FunctionMap,
) -> Option<EvaluateResultValue> {
  let path = Expr::Member(member.clone());
  let path = &path;
  let parent_is_call_expr = traversal_state
    .all_call_expressions
    .values()
    .any(|call_expr_callee| match call_expr_callee {
      Callee::Expr(callee) => match callee.as_ref() {
        Expr::Member(call_member) => call_member.eq_ignore_span(member),
        _ => false,
      },
      _ => false,
    });

  let evaluated_value = if parent_is_call_expr {
    None
  } else {
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
            let eval_res = match property {
              Some(p) => p,
              #[cfg_attr(coverage_nightly, coverage(off))]
              None => stylex_panic!("{}", PROPERTY_NOT_FOUND),
            };

            let expr = match eval_res {
              EvaluateResultValue::Expr(expr) => expr,
              _ => stylex_panic_with_context!(path, traversal_state, PROPERTY_NOT_FOUND),
            };

            let value = match expr {
              Expr::Lit(Lit::Num(Number { value, .. })) => value as usize,
              _ => stylex_panic_with_context!(path, traversal_state, MEMBER_NOT_RESOLVED),
            };

            let property = elems.get(value)?;

            let Some(expr) = property.as_ref() else {
              stylex_panic_with_context!(path, traversal_state, MEMBER_NOT_RESOLVED)
            };

            let expr = expr.expr.clone();

            Some(EvaluateResultValue::Expr(*expr))
          },
          Expr::Object(ObjectLit { props, .. }) => {
            let eval_res = match property {
              Some(p) => p,
              #[cfg_attr(coverage_nightly, coverage(off))]
              None => stylex_panic!("{}", PROPERTY_NOT_FOUND),
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

                stylex_panic_with_context!(
                  path,
                  traversal_state,
                  "Property not found. For additional details, please recompile using debug mode."
                );
              },
            };

            let ident = &mut ident.to_owned();
            let normalized_ident = normalize_expr(ident);

            let ident_string_name = match normalized_ident {
              Expr::Ident(ident) => ident.sym.to_string(),
              Expr::Lit(lit) => convert_lit_to_string(lit).unwrap_or_else(|| {
                stylex_panic_with_context!(
                  path,
                  traversal_state,
                  "The property key must be convertible to a string."
                )
              }),
              _ => {
                stylex_panic_with_context!(
                  path,
                  traversal_state,
                  "Computed member properties are not supported in static evaluation."
                )
              },
            };

            let property = props.iter().find(|prop| match prop {
                  PropOrSpread::Spread(_) => stylex_panic_with_context!(
                    path,
                    traversal_state,
                    "The spread operator (...) is not supported in this context. Declare each property explicitly."
                  ),
                  PropOrSpread::Prop(prop) => {
                    let mut prop = prop.clone();

                    expand_shorthand_prop(&mut prop);

                    match prop.as_ref() {
                      Prop::KeyValue(key_value) => {
                        let key = convert_key_value_to_str(key_value);

                        ident_string_name == key
                      }
                      _ => {
                        stylex_panic_with_context!(
                          path,
                          traversal_state,
                          "Computed property keys are not supported in static evaluation."
                        );
                      }
                    }
                  }
                })?;

            if let PropOrSpread::Prop(prop) = property {
              return Some(EvaluateResultValue::Expr(
                *match prop.as_key_value() {
                  Some(kv) => kv,
                  #[cfg_attr(coverage_nightly, coverage(off))]
                  None => stylex_panic!("{}", KEY_VALUE_EXPECTED),
                }
                .value
                .clone(),
              ));
            } else {
              stylex_panic_with_context!(path, traversal_state, MEMBER_NOT_RESOLVED);
            }
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
          _ => {
            stylex_panic_with_context!(
              path,
              traversal_state,
              "This type of object member access is not yet supported in static evaluation."
            );
          },
        },
        EvaluateResultValue::FunctionConfigMap(fc_map) => {
          let key = match property {
            Some(property) => match property {
              EvaluateResultValue::Expr(expr) => match expr {
                Expr::Ident(ident) => Box::new(ident.clone()),
                _ => stylex_panic_with_context!(path, traversal_state, MEMBER_NOT_RESOLVED),
              },
              _ => stylex_panic_with_context!(
                path,
                traversal_state,
                "This function configuration property is not yet supported."
              ),
            },
            None => stylex_panic_with_context!(path, traversal_state, MEMBER_NOT_RESOLVED),
          };

          if let Some(fc) = fc_map.get(&key.sym) {
            return Some(EvaluateResultValue::FunctionConfig(fc.clone()));
          }

          // Check if this is an env property access on a stylex import.
          if key.sym.as_ref() == STYLEX_ENV {
            if traversal_state.options.env.is_empty() {
              stylex_panic_with_context!(
                path,
                traversal_state,
                "The stylex.env object is not configured. Check that the 'env' option is set in your StyleX configuration."
              );
            }

            return Some(EvaluateResultValue::EnvObject(
              traversal_state.options.env.clone(),
            ));
          }

          stylex_panic_with_context!(
            path,
            traversal_state,
            format!(
              "The property '{}' was not found in the function configuration.",
              key.sym
            )
            .as_str()
          );
        },
        EvaluateResultValue::ThemeRef(mut theme_ref) => {
          let key = match property {
            Some(property) => match property {
              EvaluateResultValue::Expr(expr) => match expr {
                Expr::Ident(Ident { sym, .. }) => sym.to_string(),
                Expr::Lit(lit) => match convert_lit_to_string(&lit) {
                  Some(s) => s,
                  #[cfg_attr(coverage_nightly, coverage(off))]
                  None => stylex_panic!("Property key must be a string value."),
                },
                _ => stylex_panic_with_context!(path, traversal_state, MEMBER_NOT_RESOLVED),
              },
              _ => stylex_panic_with_context!(
                path,
                traversal_state,
                "This theme reference property type is not yet supported."
              ),
            },
            None => {
              stylex_panic_with_context!(
                path,
                traversal_state,
                "The referenced property was not found on the theme object. Ensure it was declared in defineVars()."
              )
            },
          };

          let value = theme_ref.get(&key, traversal_state);

          return Some(EvaluateResultValue::Expr(create_string_expr(
            match value.as_css_var() {
              Some(css_var) => css_var,
              #[cfg_attr(coverage_nightly, coverage(off))]
              None => stylex_panic!("{}", EXPECTED_CSS_VAR),
            },
          )));
        },
        EvaluateResultValue::EnvObject(env_map) => {
          let key = property
            .as_ref()
            .and_then(|prop| prop.as_string_key())
            .unwrap_or_else(|| {
              stylex_panic_with_context!(
                path,
                traversal_state,
                "The referenced property was not found in the stylex.env configuration."
              )
            });

          match env_map.get(&key) {
            Some(entry) => match resolve_env_entry_to_result(entry, &env_map) {
              Some(result) => return Some(result),
              None => stylex_panic_with_context!(
                path,
                traversal_state,
                "The stylex.env value could not be converted to a static expression."
              ),
            },
            None => {
              stylex_panic_with_context!(
                path,
                traversal_state,
                format!(
                  "The property '{}' was not found in the stylex.env configuration.",
                  key
                )
                .as_str()
              );
            },
          }
        },
        _ => stylex_panic_with_context!(
          path,
          traversal_state,
          "This evaluation result type is not yet supported in static evaluation."
        ),
      }
    },
    _ => None,
  }
}
