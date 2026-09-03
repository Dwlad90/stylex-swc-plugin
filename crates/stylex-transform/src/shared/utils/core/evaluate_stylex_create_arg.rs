use indexmap::IndexMap;
use stylex_ast::ast::objects::assign_props;
use stylex_css::css::common::get_number_suffix;
use stylex_macros::{stylex_panic, stylex_unimplemented};
use swc_core::{
  common::DUMMY_SP,
  ecma::{
    ast::{
      BinaryOp, BindingIdent, BlockStmtOrExpr, Expr, KeyValueProp, ObjectLit, Pat, Prop,
      PropOrSpread, UnaryExpr, UnaryOp,
    },
    utils::quote_ident,
  },
};

use crate::shared::utils::validators::validate_dynamic_style_params;
use stylex_ast::ast::convertors::{
  create_ident_expr, create_null_expr, create_string_expr, expand_shorthand_prop, normalize_expr,
};
use stylex_ast::ast::factories::{
  create_arrow_expression_with_params, create_bin_expr, create_call_expr, create_cond_expr,
  create_expr_or_spread, create_key_value_prop, create_object_expression,
};
use stylex_constants::constants::{
  length_units::LENGTH_UNITS,
  messages::{
    EVAL_RESULT_EXPECTED, ILLEGAL_NAMESPACE_VALUE, ILLEGAL_PROP_ARRAY_VALUE, ILLEGAL_PROP_VALUE,
    KEY_MUST_EVAL_TO_STRING, SPREAD_NOT_SUPPORTED, VALUE_NOT_EXPRESSION,
  },
  time_units::get_time_units,
};
use stylex_css::utils::pseudo::is_pseudo_selector;
use stylex_diagnostics::code_frame::build_code_frame_error_and_panic_at;
use stylex_evaluator::evaluate::{
  evaluate, evaluate_obj_key, evaluate_result_vec_to_array_expr, function_fold_to_object,
  spread_own_properties,
};
use stylex_evaluator::evaluate_result::EvaluateResult;
use stylex_state::resolution::convertors::convert_expr_to_str;
use stylex_state::{
  evaluate_result_value::EvaluateResultValue,
  functions::FunctionMap,
  state_manager::StateManager,
  types::{DynamicFns, TInlineStyles},
};
use stylex_structures::inline_style::InlineStyle;
use stylex_utils::hash::create_hash;

/// The key-value properties of an evaluated object, which is what a namespace
/// carries. A spread or a method has no key and value of its own and is dropped,
/// as it was before an object could arrive here from more than one place.
fn key_value_props_of(object: &ObjectLit) -> Vec<KeyValueProp> {
  object
    .props
    .iter()
    .filter_map(|prop| prop.as_prop().and_then(|prop| prop.as_key_value()))
    .cloned()
    .collect()
}

/// Prepends a key name to an existing error reason to provide context
/// about which property path triggered the evaluation failure.
///
/// **A deliberate divergence, kept on purpose.** The reference compiler prefixes
/// nothing: a nullish refusal is `unknown error` there where it is
/// `a > flexGrow > unknown error` here, and its object-property deopt is
/// `deopt(prop, state, state.deoptReason ?? 'unknown error')` with no key path
/// anywhere in it. Both were measured by running the two compilers on the same
/// source, not inferred.
///
/// It is kept because the divergence costs nothing and pays for itself. This
/// text is a build-failure message: no class name is hashed from it, no CSS
/// differs, and no output a consumer can observe moves. What it buys is the one
/// thing `unknown error` withholds -- which of a namespace's properties the
/// build stopped on. Most of the diagnostics that carry a path are this
/// compiler's own additions anyway (`Expression is too deeply nested` has no
/// counterpart upstream, which raises no error for it at all), so for those
/// there is nothing to be faithful to.
///
/// Removing it was tried and reverted. If it is proposed again, the question to
/// answer first is what an author gains from the shorter sentence, because the
/// last two attempts could not name anything.
fn prepend_key_to_reason(key: &str, reason: Option<String>) -> Option<String> {
  reason.map(|r| format!("{} > {}", key, r))
}

/// The expression a style value carries, materializing a folded function map as
/// the object it stands for.
///
/// The fold has no expression form, so this position used to abort with a
/// message about a static expression instead. `function_fold_to_object`
/// owns which folds have an object form and why, and the static object
/// evaluator asks it the same question -- a value read where a style value
/// belongs is refused by the same sentence whether or not a dynamic style holds
/// it.
///
/// A theme reference has no expression form either, and is not a fold that
/// function can materialize: it stands for a `defineVars` group, whose keys live
/// in another file. The reference implementation folds it to an object its
/// namespace validation refuses because it is not a plain object, so it is
/// refused here with that same message rather than materialized -- the static
/// position refuses it the same way, in `nodes/object_expression.rs`. Named
/// ahead of the fold arm because that arm answers `None` for it and the message
/// differs.
///
/// An array is folded the way the static position folds one, through the same
/// function, and refused with the array-specific message when an element has no
/// array-element form. `EvaluateResultValue::Vec` is what an array literal
/// evaluates to and it has no expression form either, so every array written
/// inside a dynamic style's body used to abort here -- including the ones the
/// reference implementation compiles. What decides an array is not this
/// position: an element that is not a string or a number is refused by namespace
/// validation, from the folded `Expr::Array`, with the message upstream gives.
///
/// Every other evaluated shape with no expression form still falls through to
/// the old message. Each is a refusal of its own rather than a fold this
/// understands, and the set is not audited here.
///
/// Both refusals report at the value the author wrote, with a code frame:
/// everything reaching them is author input, and
/// `stylex-evaluator/docs/adr/0002-a-refusal-and-a-broken-invariant-are-separate-constructs.md`
/// reserves a bare abort for an invariant this code established itself.
fn materialize_style_value(
  value: Option<EvaluateResultValue>,
  value_path: &Expr,
  traversal_state: &mut StateManager,
) -> Expr {
  match value {
    Some(EvaluateResultValue::Expr(expr)) => expr,
    Some(EvaluateResultValue::ThemeRef(_)) => {
      build_code_frame_error_and_panic_at(value_path, ILLEGAL_PROP_VALUE, traversal_state)
    },
    Some(EvaluateResultValue::Vec(items)) => match evaluate_result_vec_to_array_expr(&items) {
      Some(expr) => expr,
      None => {
        build_code_frame_error_and_panic_at(value_path, ILLEGAL_PROP_ARRAY_VALUE, traversal_state)
      },
    },
    value => match value.as_ref().and_then(function_fold_to_object) {
      Some(object) => Expr::from(object),
      None => {
        build_code_frame_error_and_panic_at(value_path, VALUE_NOT_EXPRESSION, traversal_state)
      },
    },
  }
}

pub fn evaluate_stylex_create_arg(
  path: &mut Expr,
  traversal_state: &mut StateManager,
  functions: &FunctionMap,
) -> Box<EvaluateResult> {
  match path {
    Expr::Object(style_object) => {
      let mut result_value: IndexMap<Expr, Vec<KeyValueProp>> = IndexMap::new();
      let mut fns: DynamicFns = IndexMap::new();

      for prop in &style_object.props {
        match prop {
          PropOrSpread::Spread(_) => stylex_unimplemented!("{}", SPREAD_NOT_SUPPORTED),
          PropOrSpread::Prop(prop) => {
            let mut prop = prop.clone();

            expand_shorthand_prop(&mut prop);

            match prop.as_mut() {
              Prop::KeyValue(key_value_prop) => {
                let key_result = evaluate_obj_key(key_value_prop, traversal_state, functions);

                if !key_result.confident {
                  return Box::new(EvaluateResult::refused(key_result.deopt, key_result.reason));
                }

                let key = match key_result.value.as_ref() {
                  Some(val) => val,
                  None => stylex_panic!("{}", EVAL_RESULT_EXPECTED),
                };
                let key_expr = match key.as_expr() {
                  Some(expr) => expr,
                  None => stylex_panic!("Expected an expression from evaluation result."),
                };
                let value_path = &mut key_value_prop.value;

                match value_path.as_mut() {
                  Expr::Arrow(fn_path) => {
                    let all_params = fn_path.params.clone();
                    validate_dynamic_style_params(fn_path, &all_params, traversal_state);

                    let params = all_params
                      .into_iter()
                      .filter_map(|param| param.as_ident().cloned())
                      .collect::<Vec<BindingIdent>>();

                    match fn_path.body.as_ref() {
                      BlockStmtOrExpr::Expr(expr) => {
                        if let Expr::Object(fn_body_object) = normalize_expr(expr) {
                          let eval_result = evaluate_partial_object_recursively(
                            fn_body_object,
                            traversal_state,
                            functions,
                            None,
                          );

                          if !eval_result.confident {
                            let reason =
                              match convert_expr_to_str(key_expr, traversal_state, functions) {
                                Some(key_name) => {
                                  prepend_key_to_reason(&key_name, eval_result.reason)
                                },
                                None => eval_result.reason,
                              };
                            // Not `EvaluateResult::refused`, and the difference
                            // is the point: this refusal carries the value the
                            // evaluation did reach, which the constructor
                            // deliberately forces to `None`. Every other refusal
                            // in this file goes through it.
                            return Box::new(EvaluateResult {
                              confident: false,
                              deopt: eval_result.deopt,
                              reason,
                              value: eval_result.value,
                              inline_styles: None,
                              fns: None,
                            });
                          }

                          let value = match eval_result
                            .value
                            .as_ref()
                            .and_then(|value| value.as_expr())
                            .and_then(|expr| expr.as_object())
                          {
                            Some(obj) => obj,
                            None => stylex_panic!(
                              "Expected an object value in style evaluation, but received a different type."
                            ),
                          };

                          let key = match convert_expr_to_str(key_expr, traversal_state, functions)
                          {
                            Some(k) => k,
                            None => stylex_panic!("{}", KEY_MUST_EVAL_TO_STRING),
                          };

                          fns.insert(key, (params, eval_result.inline_styles.unwrap_or_default()));

                          result_value.insert(
                            key_expr.clone(),
                            value
                              .props
                              .iter()
                              .filter_map(|prop| {
                                prop.as_prop().and_then(|prop| prop.as_key_value())
                              })
                              .cloned()
                              .collect(),
                          );
                        } else {
                          return evaluate(path, traversal_state, functions);
                        }
                      },
                      _ => {
                        return Box::new(EvaluateResult::refused(
                          None,
                          Some(
                            "Block statement is not allowed in Dynamic Style functions".to_string(),
                          ),
                        ));
                      },
                    }
                  },
                  _ => {
                    let mut val = evaluate(value_path, traversal_state, functions);

                    if !val.confident {
                      if let Some(key_name) =
                        convert_expr_to_str(key_expr, traversal_state, functions)
                      {
                        val.reason = prepend_key_to_reason(&key_name, val.reason);
                      }
                      return val;
                    }

                    let value_to_insert = match match val.value.as_ref() {
                      Some(v) => v,
                      None => stylex_panic!("{}", EVAL_RESULT_EXPECTED),
                    } {
                      EvaluateResultValue::Expr(Expr::Object(obj_expr)) => {
                        key_value_props_of(obj_expr)
                      },
                      // A folded function map written where a namespace
                      // belongs, materialized as the object it stands for so
                      // namespace validation refuses its keys -- which is what
                      // the reference implementation refuses, having folded the
                      // same reference to a plain object. Everything else with
                      // no object form is a namespace this cannot read.
                      value => match function_fold_to_object(value) {
                        Some(object) => key_value_props_of(&object),
                        None => stylex_panic!("{}", ILLEGAL_NAMESPACE_VALUE),
                      },
                    };

                    result_value.insert(key_expr.clone(), value_to_insert);

                    continue;
                  },
                }
              },
              _ => {
                return evaluate(path, traversal_state, functions);
              },
            }
          },
        }
      }

      Box::new(EvaluateResult {
        confident: true,
        deopt: None,
        reason: None,
        value: Some(EvaluateResultValue::Map(result_value)),
        inline_styles: None,
        fns: (!fns.is_empty()).then_some(fns),
      })
    },
    _ => evaluate(path, traversal_state, functions),
  }
}

fn evaluate_partial_object_recursively(
  path: &ObjectLit,
  traversal_state: &mut StateManager,
  functions: &FunctionMap,
  key_path: Option<Vec<String>>,
) -> Box<EvaluateResult> {
  let key_path = key_path.unwrap_or_default();
  let mut inline_styles: TInlineStyles = IndexMap::new();
  let mut obj: Vec<PropOrSpread> = vec![];

  for prop in &path.props {
    match prop {
      PropOrSpread::Spread(spread) => {
        let result = evaluate(&spread.expr, traversal_state, functions);
        if !result.confident {
          // The reason is dropped here, and this compiler's choice rather than
          // the reference compiler's placement. Worth separating the two,
          // because the comment here used to run them together.
          //
          // Upstream does not drop it at the spread: `:142-144` returns the
          // result whole, reason and all. The drop happens one frame up, at
          // `:107-109`, where `evaluatePartialObjectRecursively`'s caller
          // destructures `{ confident, value, deopt }` and leaves `reason`
          // behind -- for *every* refusal out of this function, not for spreads
          // in particular. Matching that shape would therefore mean dropping the
          // reason for the computed-key refusal too, and losing the
          // `Referenced constant is not defined` that
          // `a_shadowing_param_as_a_computed_key` pins. The divergence is kept
          // deliberately, which the two sites named in the `docs(transform)`
          // commit that introduced it both say.
          //
          // Dropped *at this arm* because a spread asks the evaluator to
          // enumerate the value's keys rather than to fold it to a value: there
          // is no inline-style fall-through to deopt into, the build stops, and
          // the complaint is the whole of what the author is handed. The useful
          // one names the position -- a `create()` argument that is not static,
          // which is what the reference compiler says -- and naming the binding
          // instead answers a question nobody asked, since there is nothing an
          // author can do with a resolvable binding in a position that admits no
          // dynamic value at all. The caller supplies `non_static_value` for
          // whichever call it is, and this function cannot know that.
          //
          // `prepend_key_to_reason` is `reason.map(..)`, so it is a no-op on
          // `None` and the arrow and nested-object branches forward the erased
          // reason unchanged. The deopt path is kept either way, so the code
          // frame still points where it did.
          return Box::new(EvaluateResult::refused(result.deopt, None));
        }
        // `Object.assign(obj, result.value)` upstream
        // (`visitors/parse-stylex-create-arg.js:146`, 0.19.0): a spread whose
        // operand folded contributes that value's own enumerable properties, and
        // the fold carries on.
        //
        // The same two helpers the object-expression path uses, rather than a
        // reader of its own. `spread_own_properties` already answers what a
        // spread operand contributes for every reading one can arrive as -- an
        // object, a string's code units, an array's indices -- and `assign_props`
        // already *is* `Object.assign`: shallow, a repeated key taking the later
        // value and keeping the position it first took. Both are the semantics
        // the language fixes, so spelling them again here would be a second
        // answer to a question asked once.
        let Some(new_props) = result
          .value
          .and_then(|value| spread_own_properties(value, &spread.expr))
        else {
          // A value with no own-properties reading: a number, a boolean, a
          // callback. Nothing to enumerate, so the refusal stands.
          stylex_unimplemented!("{}", SPREAD_NOT_SUPPORTED);
        };

        obj = assign_props(obj, new_props);
      },
      PropOrSpread::Prop(prop) => {
        let mut prop = prop.clone();

        expand_shorthand_prop(&mut prop);

        match prop.as_mut() {
          Prop::KeyValue(key_value) => {
            let key_result = evaluate_obj_key(key_value, traversal_state, functions);

            if !key_result.confident {
              return Box::new(EvaluateResult::refused(key_result.deopt, key_result.reason));
            }

            let key = match key_result.value.as_ref().and_then(|v| v.as_expr()) {
              Some(expr) => expr,
              None => stylex_panic!("{}", KEY_MUST_EVAL_TO_STRING),
            };

            let mut key_str = match convert_expr_to_str(key, traversal_state, functions) {
              Some(s) => s,
              None => stylex_panic!("{}", KEY_MUST_EVAL_TO_STRING),
            };

            if key_str.starts_with("var(") && key_str.ends_with(')') {
              let inner = key_str[4..key_str.len() - 1].to_string();

              // When the `key_path` is not empty, the var(--hash) is a `defineConsts` at-rule
              // placeholder and must be kept intact.
              if key_path.is_empty() {
                key_str = inner;
              }
            }

            let value_path = &key_value.value;
            match normalize_expr(value_path.as_ref()) {
              Expr::Object(object) => {
                let mut key_path = key_path.clone();

                key_path.push(key_str.clone());

                let result = evaluate_partial_object_recursively(
                  object,
                  traversal_state,
                  functions,
                  Some(key_path.clone()),
                );

                if !result.confident {
                  return Box::new(EvaluateResult::refused(result.deopt, result.reason));
                }

                let new_prop = create_key_value_prop(
                  &key_str,
                  materialize_style_value(result.value, value_path, traversal_state),
                );
                obj.push(new_prop);

                if let Some(result_inline_styles) = result.inline_styles {
                  inline_styles.extend(result_inline_styles);
                }
              },
              _ => {
                let result = evaluate(value_path, traversal_state, functions);

                if !result.confident {
                  let mut full_key_path = key_path.clone();

                  full_key_path.push(key_str.clone());

                  let var_name = if !key_path.is_empty() {
                    let mut key_path = key_path.clone();

                    key_path.push(key_str.clone());

                    format!("--x-{}", create_hash(&key_path.join("_")))
                  } else {
                    format!("--x-{}", key_str)
                  };

                  let new_prop = create_key_value_prop(
                    &key_str,
                    create_string_expr(&format!("var({})", var_name)),
                  );
                  obj.push(new_prop);

                  let expression = &value_path;

                  let prop_name = full_key_path
                    .iter()
                    .find(|&k| !is_pseudo_selector(k) && !k.starts_with('@') && k != "default")
                    .unwrap_or(&key_str)
                    .clone();

                  let unit = if get_time_units().contains(prop_name.as_str())
                    || LENGTH_UNITS.contains(prop_name.as_str())
                  {
                    get_number_suffix(prop_name.as_str())
                  } else {
                    ""
                  };

                  let inline_style_expression = if !unit.is_empty() {
                    let val_ident = create_ident_expr("val");
                    Expr::from(create_call_expr(
                      create_arrow_expression_with_params(
                        vec![Pat::Ident(BindingIdent::from(quote_ident!("val")))],
                        create_cond_expr(
                          create_bin_expr(
                            BinaryOp::EqEqEq,
                            Expr::from(UnaryExpr {
                              span: DUMMY_SP,
                              op: UnaryOp::TypeOf,
                              arg: Box::new(val_ident.clone()),
                            }),
                            create_string_expr("number"),
                          ),
                          create_bin_expr(
                            BinaryOp::Add,
                            val_ident.clone(),
                            create_string_expr(unit),
                          ),
                          create_cond_expr(
                            create_bin_expr(BinaryOp::NotEq, val_ident.clone(), create_null_expr()),
                            val_ident,
                            create_ident_expr("undefined"),
                          ),
                        ),
                      ),
                      vec![create_expr_or_spread(*value_path.clone())],
                    ))
                  } else {
                    create_cond_expr(
                      create_bin_expr(BinaryOp::NotEq, *value_path.clone(), create_null_expr()),
                      *value_path.clone(),
                      create_ident_expr("undefined"),
                    )
                  };

                  let mut key_path = key_path.clone();

                  key_path.push(key_str.clone());

                  inline_styles.insert(
                    var_name,
                    Box::new(InlineStyle {
                      path: key_path,
                      original_expression: *(*expression).clone(),
                      expression: inline_style_expression,
                    }),
                  );
                } else {
                  let new_prop = create_key_value_prop(
                    &key_str,
                    materialize_style_value(result.value, value_path, traversal_state),
                  );
                  obj.push(new_prop);
                }
              },
            }
          },
          Prop::Method(_) => {
            return Box::new(EvaluateResult::refused(None, None));
          },
          _ => {},
        }
      },
    }
  }

  Box::new(EvaluateResult {
    confident: true,
    deopt: None,
    reason: None,
    value: Some(EvaluateResultValue::Expr(create_object_expression(obj))),
    inline_styles: Some(inline_styles),
    fns: None,
  })
}
