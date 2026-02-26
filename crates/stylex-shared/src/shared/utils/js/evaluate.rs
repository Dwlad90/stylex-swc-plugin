use core::panic;
use std::{borrow::Borrow, rc::Rc, sync::Arc};

// Import error handling macros from shared utilities
use crate::{
  collect_confident, expr_to_str_or_deopt, panic_with_context,
  shared::constants::common::{MUTATING_ARRAY_METHODS, MUTATING_OBJECT_METHODS},
  unwrap_or_panic,
};

use indexmap::IndexMap;
use log::{debug, warn};
use rustc_hash::{FxHashMap, FxHashSet};
use swc_core::{
  atoms::Atom,
  common::{DUMMY_SP, EqIgnoreSpan, SyntaxContext},
  ecma::{
    ast::{
      ArrayLit, AssignTarget, BlockStmtOrExpr, CallExpr, Callee, ComputedPropName, Expr,
      ExprOrSpread, Ident, ImportSpecifier, KeyValueProp, Lit, MemberProp, ModuleExportName,
      Number, ObjectLit, OptChainBase, Pat, Prop, PropName, PropOrSpread, SimpleAssignTarget,
      TplElement, UnaryOp, VarDeclarator,
    },
    utils::{ExprExt, drop_span, ident::IdentLike, quote_ident},
  },
};

use crate::shared::{
  constants::{
    common::{INVALID_METHODS, VALID_CALLEES},
    evaluation_errors::{
      IMPORT_PATH_RESOLUTION_ERROR, NON_CONSTANT, OBJECT_METHOD, PATH_WITHOUT_NODE,
      UNEXPECTED_MEMBER_LOOKUP, unsupported_expression, unsupported_operator,
    },
    messages::{BUILT_IN_FUNCTION, ILLEGAL_PROP_ARRAY_VALUE, THEME_IMPORT_KEY_AS_OBJECT_KEY},
  },
  enums::{
    core::TransformationCycle,
    data_structures::{
      evaluate_result_value::EvaluateResultValue,
      import_path_resolution::{ImportPathResolution, ImportPathResolutionType},
      value_with_default::ValueWithDefault,
    },
    js::{ArrayJS, MathJS, ObjectJS, StringJS},
    misc::{BinaryExprType, VarDeclAction},
  },
  structures::{
    evaluate_result::EvaluateResult,
    functions::{CallbackType, FunctionConfig, FunctionConfigType, FunctionMap, FunctionType},
    named_import_source::ImportSources,
    seen_value::SeenValue,
    state::EvaluationState,
    state_manager::{SeenValueWithVarDeclCount, StateManager, add_import_expression},
    theme_ref::ThemeRef,
    types::{FunctionMapIdentifiers, FunctionMapMemberExpression},
  },
  swc::get_default_expr_ctx,
  utils::{
    ast::{
      convertors::{
        atom_to_str, atom_to_string, big_int_to_expression, binary_expr_to_num,
        binary_expr_to_string, bool_to_expression, expr_to_bool, expr_to_num, expr_to_str,
        key_value_to_str, lit_to_string, number_to_expression, string_to_expression,
        tpl_element_cooked_to_string, transform_shorthand_to_key_values,
      },
      factories::{
        array_expression_factory, lit_str_factory, object_expression_factory,
        prop_or_spread_expression_factory,
      },
    },
    common::{
      char_code_at, deep_merge_props, get_hash_map_difference, get_hash_map_value_difference,
      get_import_by_ident, get_key_values_from_object, get_var_decl_by_ident, get_var_decl_from,
      normalize_expr, reduce_ident_count, reduce_member_expression_count, remove_duplicates,
      sort_numbers_factory, stable_hash, sum_hash_map_values,
    },
    js::native_functions::{evaluate_filter, evaluate_join, evaluate_map},
  },
};

use super::check_declaration::{DeclarationType, check_ident_declaration};

/// Helper function to evaluate unary numeric operations (Plus, Minus, Tilde).
/// This reduces code duplication for operations that convert an expression to a number,
/// apply a transformation, and return the result as an expression.
///
/// # Arguments
/// * `arg` - The expression argument to the unary operator
/// * `state` - The evaluation state
/// * `traversal_state` - The state manager for traversal context
/// * `fns` - The function map for evaluating function calls
/// * `transform` - A function to transform the numeric value
///
/// # Example
/// ```ignore
/// UnaryOp::Plus => evaluate_unary_numeric(&arg, state, traversal_state, fns, |v| v),
/// UnaryOp::Minus => evaluate_unary_numeric(&arg, state, traversal_state, fns, |v| -v),
/// ```
#[inline]
fn evaluate_unary_numeric(
  arg: &Expr,
  state: &mut EvaluationState,
  traversal_state: &mut StateManager,
  fns: &FunctionMap,
  transform: impl FnOnce(f64) -> f64,
) -> Option<EvaluateResultValue> {
  let value = unwrap_or_panic!(expr_to_num(arg, state, traversal_state, fns));
  Some(EvaluateResultValue::Expr(number_to_expression(transform(
    value,
  ))))
}

pub(crate) fn evaluate_obj_key(
  prop_kv: &KeyValueProp,
  state: &mut StateManager,
  functions: &FunctionMap,
) -> EvaluateResult {
  let key_path = &prop_kv.key;

  let key = match key_path {
    PropName::Ident(ident) => string_to_expression(&ident.sym),
    PropName::Computed(computed) => {
      let computed_result = evaluate(&computed.expr, state, functions);
      if computed_result.confident {
        match computed_result.value {
          Some(EvaluateResultValue::Expr(ref value)) => value.clone(),
          _ => panic!("Expected expression value"),
        }
      } else {
        return EvaluateResult {
          confident: false,
          deopt: computed_result.deopt,
          reason: computed_result.reason,
          value: None,
          inline_styles: None,
          fns: None,
        };
      }
    }
    PropName::Str(strng) => string_to_expression(&atom_to_string(&strng.value)),
    PropName::Num(num) => number_to_expression(num.value),
    PropName::BigInt(big_int) => big_int_to_expression(big_int.clone()),
  };

  let key_expr = match expr_to_str(&key, state, functions) {
    Some(ref s) => string_to_expression(s),
    None => {
      return EvaluateResult {
        confident: false,
        deopt: Some(key),
        reason: Some("Key is not a string".to_string()),
        value: None,
        inline_styles: None,
        fns: None,
      };
    }
  };

  EvaluateResult {
    confident: true,
    deopt: None,
    reason: None,
    value: Some(EvaluateResultValue::Expr(key_expr)),
    inline_styles: None,
    fns: None,
  }
}

pub fn evaluate(
  path: &Expr,
  traversal_state: &mut StateManager,
  fns: &FunctionMap,
) -> Box<EvaluateResult> {
  let mut state = Box::new(EvaluationState {
    confident: true,
    deopt_path: None,
    deopt_reason: None,
    added_imports: FxHashSet::default(),
    functions: fns.clone(),
  });

  let mut value = evaluate_cached(path, &mut state, traversal_state, fns);

  if !state.confident {
    value = None;
  }

  Box::new(EvaluateResult {
    confident: state.confident,
    value,
    deopt: state.deopt_path,
    reason: state.deopt_reason,
    inline_styles: None,
    fns: None,
  })
}

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

fn _evaluate(
  path: &mut Expr,
  state: &mut EvaluationState,
  traversal_state: &mut StateManager,
  fns: &FunctionMap,
) -> Option<EvaluateResultValue> {
  if !state.confident {
    return None;
  }

  let normalized_path = normalize_expr(path);

  if is_mutation_expr(normalized_path) {
    return deopt(path, state, NON_CONSTANT);
  }

  let result: Option<EvaluateResultValue> = match normalized_path {
    Expr::Arrow(arrow) => {
      let body = &arrow.body;
      let params = &arrow.params;

      let ident_params = params
        .iter()
        .filter_map(|param| {
          if let Pat::Ident(ident) = param {
            Some(ident.sym.clone())
          } else {
            None
          }
        })
        .collect::<Vec<Atom>>();

      match body.as_ref() {
        BlockStmtOrExpr::Expr(body_expr) => {
          if ident_params.len() == params.len() {
            let arrow_closure_fabric =
              |identifiers: FunctionMapIdentifiers,
               ident_params: Vec<Atom>,
               body_expr: Box<Expr>,
               traversal_state: StateManager| {
                move |cb_args: Vec<Option<EvaluateResultValue>>| {
                  let mut identifiers = identifiers.clone();

                  let mut member_expressions: FunctionMapMemberExpression = FxHashMap::default();

                  ident_params.iter().enumerate().for_each(|(index, ident)| {
                    if let Some(arg) = cb_args.get(index) {
                      let expr = arg
                        .as_ref()
                        .and_then(|arg| arg.as_expr())
                        .expect("Argument is not an expression");

                      let cl = |arg: Expr| move || arg.clone();

                      let result = (cl)(expr.clone());
                      let function = FunctionConfig {
                        fn_ptr: FunctionType::Mapper(Rc::new(result)),
                        takes_path: false,
                      };
                      identifiers.insert(
                        ident.clone(),
                        Box::new(FunctionConfigType::Regular(function.clone())),
                      );

                      member_expressions.insert(
                        ImportSources::Regular("entry".to_string()),
                        Box::new(identifiers.clone()),
                      );
                    }
                  });

                  let mut local_state = traversal_state.clone();

                  let result = evaluate(
                    &body_expr,
                    &mut local_state,
                    &FunctionMap {
                      identifiers,
                      member_expressions,
                      disable_imports: false,
                    },
                  );

                  let value = result.value;

                  match value {
                    Some(res) => res
                      .as_expr()
                      .expect("Evaluation result must be an expression")
                      .clone(),
                    None => unreachable!("Evaluation result must be non optional"),
                  }
                }
              };

            let identifiers = state.functions.identifiers.clone();

            let arrow_closure = Rc::new(arrow_closure_fabric(
              identifiers,
              ident_params,
              Box::new(*body_expr.clone()),
              traversal_state.clone(),
            ));

            return Some(EvaluateResultValue::Callback(arrow_closure));
          }

          None
        }
        BlockStmtOrExpr::BlockStmt(_) => None,
      }
    }
    Expr::Ident(ident) => {
      let atom_ident_id = &ident.sym;

      if let Some(func) = state.functions.identifiers.get(atom_ident_id) {
        match func.as_ref() {
          FunctionConfigType::Regular(func) => match &func.fn_ptr {
            FunctionType::Mapper(func) => {
              return Some(EvaluateResultValue::Expr(func()));
            }
            FunctionType::DefaultMarker(func) => {
              return Some(EvaluateResultValue::FunctionConfig(FunctionConfig {
                fn_ptr: FunctionType::DefaultMarker(Arc::clone(func)),
                takes_path: false,
              }));
            }
            _ => {
              return deopt(path, state, "Function not found");
            }
          },
          FunctionConfigType::Map(func_map) => {
            return Some(EvaluateResultValue::FunctionConfigMap(func_map.clone()));
          }
          FunctionConfigType::IndexMap(_func_map) => {
            unimplemented!("IndexMap not implemented");
          }
        }
      }

      None
    }
    Expr::TsSatisfies(ts_satisfaies) => {
      evaluate_cached(&ts_satisfaies.expr, state, traversal_state, fns)
    }
    Expr::TsConstAssertion(ts_const) => {
      evaluate_cached(&ts_const.expr, state, traversal_state, fns)
    }
    Expr::TsAs(ts_as) => evaluate_cached(&ts_as.expr, state, traversal_state, fns),
    Expr::TsNonNull(ts_non_null) => evaluate_cached(&ts_non_null.expr, state, traversal_state, fns),
    Expr::TsTypeAssertion(ts_type) => evaluate_cached(&ts_type.expr, state, traversal_state, fns),
    Expr::TsInstantiation(ts_instantiation) => {
      evaluate_cached(&ts_instantiation.expr, state, traversal_state, fns)
    }
    Expr::Seq(sec) => {
      let expr = sec
        .exprs
        .last()
        .expect("Sequence must have at least one expression");

      evaluate_cached(expr, state, traversal_state, fns)
    }
    Expr::Lit(lit_path) => Some(EvaluateResultValue::Expr(Expr::Lit(lit_path.clone()))),
    Expr::Tpl(tpl) => evaluate_quasis(
      &Expr::Tpl(tpl.clone()),
      &tpl.quasis,
      false,
      state,
      traversal_state,
      fns,
    ),
    Expr::TaggedTpl(_tagged_tpl) => {
      panic_with_context!(path, traversal_state, "TaggedTpl not implemented")
      // TODO: Uncomment this for implementation of TaggedTpl
      // evaluate_quasis(
      //   &Expr::TaggedTpl(_tagged_tpl.clone()),
      //   &_tagged_tpl.tpl.quasis,
      //   false,
      //   state,
      // )
    }
    Expr::Cond(cond) => {
      let test_result = evaluate_cached(&cond.test, state, traversal_state, fns);

      if !state.confident {
        return None;
      }

      let test_result = match test_result.expect("Test of condition must be an expression") {
        EvaluateResultValue::Expr(ref expr) => expr_to_bool(expr, traversal_state, fns),
        _ => panic_with_context!(
          path,
          traversal_state,
          "Test of condition must be an expression"
        ),
      };

      if !state.confident {
        return None;
      }

      if test_result {
        evaluate_cached(&cond.cons, state, traversal_state, fns)
      } else {
        evaluate_cached(&cond.alt, state, traversal_state, fns)
      }
    }
    Expr::Paren(_) => panic_with_context!(
      path,
      traversal_state,
      "Paren must be normalized before evaluation"
    ),
    Expr::Member(member) => {
      let parent_is_call_expr = traversal_state
        .all_call_expressions
        .values()
        .any(|call_expr| {
          if let Callee::Expr(callee) = &call_expr.callee {
            callee.eq_ignore_span(&Box::new(Expr::Member(member.clone())))
          } else {
            false
          }
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
            }
            MemberProp::PrivateName(_) => {
              return deopt(path, state, UNEXPECTED_MEMBER_LOOKUP);
            }
          };

          match object {
            EvaluateResultValue::Expr(expr) => match &expr {
              Expr::Array(ArrayLit { elems, .. }) => {
                let eval_res = property.expect("Property not found");

                let expr = match eval_res {
                  EvaluateResultValue::Expr(expr) => expr,
                  _ => panic_with_context!(path, traversal_state, "Property not found"),
                };

                let value = match expr {
                  Expr::Lit(Lit::Num(Number { value, .. })) => value as usize,
                  _ => panic_with_context!(path, traversal_state, "Member not found"),
                };

                let property = elems.get(value)?;

                let Some(expr) = property.as_ref() else {
                  panic_with_context!(path, traversal_state, "Member not found")
                };

                let expr = expr.expr.clone();

                Some(EvaluateResultValue::Expr(*expr))
              }
              Expr::Object(ObjectLit { props, .. }) => {
                let eval_res = property.expect("Property not found");

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
                  }
                  _ => {
                    debug!("Property not found for expression: {:?}", expr);
                    debug!("Evaluation result: {:?}", eval_res);
                    debug!("Original property: {:?}", prop_path);

                    panic_with_context!(
                      path,
                      traversal_state,
                      "Property not found. For additional details, please recompile using debug mode."
                    );
                  }
                };

                let ident = &mut ident.to_owned();
                let normalized_ident = normalize_expr(ident);

                let ident_string_name = match normalized_ident {
                  Expr::Ident(ident) => ident.sym.to_string(),
                  Expr::Lit(lit) => lit_to_string(lit).unwrap_or_else(|| {
                    panic_with_context!(
                      path,
                      traversal_state,
                      "Property must be convertable to string"
                    )
                  }),
                  _ => {
                    panic_with_context!(path, traversal_state, "Member property not implemented")
                  }
                };

                let property = props.iter().find(|prop| match prop {
                  PropOrSpread::Spread(_) => panic_with_context!(
                    path,
                    traversal_state,
                    "Spread properties are not implemented"
                  ),
                  PropOrSpread::Prop(prop) => {
                    let mut prop = prop.clone();

                    transform_shorthand_to_key_values(&mut prop);

                    match prop.as_ref() {
                      Prop::KeyValue(key_value) => {
                        let key = key_value_to_str(key_value);

                        ident_string_name == key
                      }
                      _ => {
                        panic_with_context!(path, traversal_state, "Property not implemented");
                      }
                    }
                  }
                })?;

                if let PropOrSpread::Prop(prop) = property {
                  return Some(EvaluateResultValue::Expr(
                    *prop
                      .as_key_value()
                      .expect("Expression is not a key value")
                      .value
                      .clone(),
                  ));
                } else {
                  panic_with_context!(path, traversal_state, "Member not found");
                }
              }
              Expr::Member(member_expr) => evaluate_cached(
                &Expr::Member(member_expr.clone()),
                state,
                traversal_state,
                fns,
              ),
              Expr::Lit(nested_lit) => {
                evaluate_cached(&Expr::Lit(nested_lit.clone()), state, traversal_state, fns)
              }
              Expr::Ident(nested_ident) => evaluate_cached(
                &Expr::Ident(nested_ident.clone()),
                state,
                traversal_state,
                fns,
              ),
              _ => {
                panic_with_context!(
                  path,
                  traversal_state,
                  "Unimplemented case for object member access"
                );
              }
            },
            EvaluateResultValue::FunctionConfigMap(fc_map) => {
              let key = match property {
                Some(property) => match property {
                  EvaluateResultValue::Expr(expr) => match expr {
                    Expr::Ident(ident) => Box::new(ident.clone()),
                    _ => panic_with_context!(path, traversal_state, "Member not found"),
                  },
                  _ => panic_with_context!(
                    path,
                    traversal_state,
                    "Function config map property not implemented"
                  ),
                },
                None => panic_with_context!(path, traversal_state, "Member not found"),
              };

              let fc = fc_map.get(&key.sym).unwrap();

              return Some(EvaluateResultValue::FunctionConfig(fc.clone()));
            }
            EvaluateResultValue::ThemeRef(mut theme_ref) => {
              let key = match property {
                Some(property) => match property {
                  EvaluateResultValue::Expr(expr) => match expr {
                    Expr::Ident(Ident { sym, .. }) => sym.to_string(),
                    Expr::Lit(lit) => lit_to_string(&lit).expect("Property must be a string"),
                    _ => panic_with_context!(path, traversal_state, "Member not found"),
                  },
                  _ => panic_with_context!(
                    path,
                    traversal_state,
                    "Theme reference property not implemented"
                  ),
                },
                None => {
                  panic_with_context!(path, traversal_state, "Theme reference property not found")
                }
              };

              let value = theme_ref.get(&key, traversal_state);

              return Some(EvaluateResultValue::Expr(string_to_expression(
                value.as_css_var().expect("Expected CSS variable").as_str(),
              )));
            }
            _ => panic_with_context!(
              path,
              traversal_state,
              "Evaluation result value not implemented"
            ),
          }
        }
        _ => None,
      }
    }
    Expr::Unary(unary) => {
      if unary.op == UnaryOp::Void {
        return None;
      }

      let argument = &unary.arg;

      if unary.op == UnaryOp::TypeOf && (argument.is_fn_expr() || argument.is_class()) {
        return Some(EvaluateResultValue::Expr(string_to_expression("function")));
      }

      let arg = evaluate_cached(argument, state, traversal_state, fns);

      if !state.confident {
        return None;
      }

      let arg = match arg.expect("Unary argument is not an expression") {
        EvaluateResultValue::Expr(expr) => expr,
        _ => panic_with_context!(path, traversal_state, "Unary argument is not an expression"),
      };

      match unary.op {
        UnaryOp::Bang => {
          let value = expr_to_bool(&arg, traversal_state, fns);

          Some(EvaluateResultValue::Expr(bool_to_expression(!value)))
        }
        UnaryOp::Plus => evaluate_unary_numeric(&arg, state, traversal_state, fns, |v| v),
        UnaryOp::Minus => evaluate_unary_numeric(&arg, state, traversal_state, fns, |v| -v),
        UnaryOp::Tilde => {
          evaluate_unary_numeric(&arg, state, traversal_state, fns, |v| (!(v as i64)) as f64)
        }
        UnaryOp::TypeOf => {
          let arg_type = match &arg {
            Expr::Lit(Lit::Str(_)) => "string",
            Expr::Lit(Lit::Bool(_)) => "boolean",
            Expr::Lit(Lit::Num(_)) => "number",
            Expr::Lit(Lit::Null(_)) => "object",
            Expr::Fn(_) => "function",
            Expr::Class(_) => "function",
            Expr::Ident(ident) if ident.sym == *"undefined" => "undefined",
            Expr::Object(_) => "object",
            Expr::Array(_) => "object",
            _ => panic_with_context!(path, traversal_state, "Unary expression not implemented"),
          };

          Some(EvaluateResultValue::Expr(string_to_expression(arg_type)))
        }
        UnaryOp::Void => Some(EvaluateResultValue::Expr(Expr::Ident(quote_ident!(
          SyntaxContext::empty(),
          "undefined"
        )))),
        _ => deopt(
          &Expr::from(unary.clone()),
          state,
          &unsupported_operator(unary.op.as_str()),
        ),
      }
    }
    Expr::Array(arr_path) => {
      let mut arr: Vec<Option<EvaluateResultValue>> = Vec::with_capacity(arr_path.elems.len());

      for elem in arr_path.elems.iter().flatten() {
        let elem_value = evaluate(&elem.expr, traversal_state, &state.functions);
        collect_confident!(elem_value, arr);
      }

      Some(EvaluateResultValue::Vec(arr))
    }
    Expr::Object(obj_path) => {
      let mut props = vec![];

      for prop in &obj_path.props {
        match prop {
          PropOrSpread::Spread(prop) => {
            let spread_expression = evaluate_cached(&prop.expr, state, traversal_state, fns);

            if !state.confident {
              return deopt(path, state, OBJECT_METHOD);
            }

            let Some(new_props) = spread_expression.and_then(|s| s.into_object()) else {
              panic_with_context!(path, traversal_state, "Spread must be an object");
            };

            let merged_object = deep_merge_props(props, new_props.props);

            props = merged_object;

            continue;
          }
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

            transform_shorthand_to_key_values(&mut prop);

            match prop.as_ref() {
              Prop::KeyValue(path_key_value) => {
                let key = match &path_key_value.key {
                  PropName::Ident(ident) => Some(ident.sym.to_string()),
                  PropName::Str(strng) => Some(atom_to_string(&strng.value)),
                  PropName::Num(num) => Some(num.value.to_string()),
                  PropName::Computed(computed) => {
                    let evaluated_result =
                      evaluate(&computed.expr, traversal_state, &state.functions);

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
                        "Expression is not a string"
                      ))
                    } else {
                      panic_with_context!(path, traversal_state, "Property must be an expression");
                    }
                  }
                  PropName::BigInt(big_int) => Some(big_int.value.to_string()),
                };

                let eval_value = evaluate(&path_key_value.value, traversal_state, &state.functions);

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
                  panic_with_context!(
                    path,
                    traversal_state,
                    format!(
                      "Value of key '{}' must be present, but got {:?}",
                      key.clone().unwrap_or_else(|| "Unknown".to_string()),
                      path_key_value.value.get_type(get_default_expr_ctx())
                    )
                    .as_ref()
                  );
                };

                let value = match value {
                  EvaluateResultValue::Expr(expr) => Some(expr),
                  EvaluateResultValue::Vec(items) => {
                    let elems = items
                      .iter()
                      .map(|entry| {
                        let expr = entry
                          .as_ref()
                          .and_then(|entry| {
                            entry
                              .as_vec()
                              .map(|vec| {
                                let elems = vec
                                  .iter()
                                  .flatten()
                                  .map(|item| {
                                    let item = item.as_expr().unwrap();
                                    Some(ExprOrSpread {
                                      spread: None,
                                      expr: Box::new(item.clone()),
                                    })
                                  })
                                  .collect();

                                Expr::Array(ArrayLit {
                                  span: DUMMY_SP,
                                  elems,
                                })
                              })
                              .or_else(|| entry.as_expr().cloned())
                          })
                          .expect(ILLEGAL_PROP_ARRAY_VALUE);

                        let expr = match expr {
                          Expr::Array(array) => Expr::Array(array),
                          Expr::Object(obj) => Expr::Object(obj),
                          Expr::Lit(lit) => Expr::Lit(lit),
                          _ => {
                            panic!("{}", ILLEGAL_PROP_ARRAY_VALUE)
                          }
                        };

                        Some(ExprOrSpread {
                          spread: None,
                          expr: Box::new(expr),
                        })
                      })
                      .collect();

                    Some(Expr::Array(ArrayLit {
                      span: DUMMY_SP,
                      elems,
                    }))
                  }
                  EvaluateResultValue::Callback(cb) => match path_key_value.value.as_ref() {
                    Expr::Call(call_expr) => {
                      let cb_args: Vec<Option<EvaluateResultValue>> = call_expr
                        .args
                        .iter()
                        .map(|arg| {
                          let eval_arg = evaluate_cached(&arg.expr, state, traversal_state, fns);

                          if !state.confident {
                            return None;
                          }

                          eval_arg
                        })
                        .collect();

                      Some(cb(cb_args))
                    }
                    Expr::Arrow(arrow_func_expr) => Some(Expr::Arrow(arrow_func_expr.clone())),
                    _ => panic_with_context!(path, traversal_state, "Callback type not supported"),
                  },
                  EvaluateResultValue::ThemeRef(_) => None,
                  _ => panic_with_context!(
                    path,
                    traversal_state,
                    "Property value must be an expression"
                  ),
                };

                if let Some(value) = value {
                  props.push(PropOrSpread::Prop(Box::new(Prop::from(KeyValueProp {
                    key: PropName::Ident(quote_ident!(key.unwrap())),
                    value: Box::new(value),
                  }))));
                }
              }
              _ => panic_with_context!(
                path,
                traversal_state,
                "Evaluation result value not implemented"
              ),
            }
          }
        }
      }

      let obj = ObjectLit {
        props: remove_duplicates(props),
        span: DUMMY_SP,
      };

      return Some(EvaluateResultValue::Expr(Expr::Object(obj)));
    }
    Expr::Bin(bin) => unwrap_or_panic!(
      binary_expr_to_num(bin, state, traversal_state, fns)
        .or_else(|num_error| {
          binary_expr_to_string(bin, state, traversal_state, fns).or_else::<String, _>(
            |str_error| {
              debug!("Binary expression to string error: {}", str_error);
              debug!("Binary expression to number error: {}", num_error);

              Ok(BinaryExprType::Null)
            },
          )
        })
        .map(|result| match result {
          BinaryExprType::Number(num) => Some(EvaluateResultValue::Expr(number_to_expression(num))),
          BinaryExprType::String(strng) => {
            Some(EvaluateResultValue::Expr(string_to_expression(&strng)))
          }
          BinaryExprType::Null => None,
        })
    ),
    Expr::Call(call) => {
      let mut context: Option<Vec<Option<EvaluateResultValue>>> = None;
      let mut func: Option<Box<FunctionConfig>> = None;

      if let Callee::Expr(callee_expr) = &call.callee {
        if get_binding(callee_expr, traversal_state).is_none() && is_valid_callee(callee_expr) {
          // skip built-in function evaluation
        } else if let Expr::Ident(ident) = callee_expr.as_ref() {
          let ident_id = ident.to_id();

          if state.functions.identifiers.contains_key(&ident_id.0) {
            match state
              .functions
              .identifiers
              .get(&ident_id.0)
              .unwrap()
              .as_ref()
            {
              FunctionConfigType::Map(_) => panic_with_context!(
                path,
                traversal_state,
                "FunctionConfigType::Map not implemented"
              ),
              FunctionConfigType::Regular(fc) => func = Some(Box::new(fc.clone())),
              FunctionConfigType::IndexMap(_) => unimplemented!("IndexMap not implemented"),
            }
          } else {
            let _maybe_function = evaluate_cached(callee_expr, state, traversal_state, fns);

            if state.confident {
              match _maybe_function {
                Some(EvaluateResultValue::FunctionConfig(fc)) => func = Some(Box::new(fc.clone())),
                Some(EvaluateResultValue::Callback(cb)) => {
                  return Some(EvaluateResultValue::Callback(cb));
                }
                _ => {
                  return deopt(path, state, NON_CONSTANT);
                }
              }
            } else {
              return deopt(path, state, NON_CONSTANT);
            }
          }
        }

        if let Expr::Member(member) = callee_expr.as_ref() {
          let object = &member.obj;
          let property = &member.prop;

          if object.is_ident() {
            let obj_ident = object.as_ident().expect("Object is not an identifier");

            if property.is_ident() {
              if is_mutating_object_method(property) {
                return deopt(path, state, NON_CONSTANT);
              }

              if is_valid_callee(object) && !is_invalid_method(property) {
                let callee_name = get_callee_name(object);
                let method_name = get_method_name(property);

                match callee_name {
                  "Math" => {
                    let first_arg = call
                      .args
                      .first()
                      .unwrap_or_else(|| panic!("Math.{} requires an argument", method_name));

                    if first_arg.spread.is_some() {
                      panic_with_context!(path, traversal_state, "Spread not implemented");
                    }

                    match method_name {
                      "pow" => {
                        func = Some(Box::new(FunctionConfig {
                          fn_ptr: FunctionType::Callback(Box::new(CallbackType::Math(MathJS::Pow))),
                          takes_path: false,
                        }));

                        let second_arg = call
                          .args
                          .get(1)
                          .expect("Math.pow requires a second argument");

                        if second_arg.spread.is_some() {
                          panic_with_context!(path, traversal_state, "Spread not implemented");
                        }

                        let cached_first_arg =
                          evaluate_cached(&first_arg.expr, state, traversal_state, fns);
                        let cached_second_arg =
                          evaluate_cached(&second_arg.expr, state, traversal_state, fns);

                        context = Some(vec![Some(EvaluateResultValue::Vec(vec![
                          cached_first_arg,
                          cached_second_arg,
                        ]))]);
                      }
                      "round" | "ceil" | "floor" => {
                        func = Some(Box::new(FunctionConfig {
                          fn_ptr: FunctionType::Callback(Box::new(CallbackType::Math(
                            match method_name {
                              "round" => MathJS::Round,
                              "ceil" => MathJS::Ceil,
                              "floor" => MathJS::Floor,
                              _ => unreachable!("Invalid method: {}", method_name),
                            },
                          ))),
                          takes_path: false,
                        }));

                        let cached_first_arg =
                          evaluate_cached(&first_arg.expr, state, traversal_state, fns);

                        context = Some(vec![Some(EvaluateResultValue::Expr(
                          cached_first_arg
                            .and_then(|arg| arg.as_expr().cloned())
                            .expect("First argument should be an expression"),
                        ))]);
                      }
                      "min" | "max" => {
                        func = Some(Box::new(FunctionConfig {
                          fn_ptr: FunctionType::Callback(Box::new(CallbackType::Math(
                            match method_name {
                              "min" => MathJS::Min,
                              "max" => MathJS::Max,
                              _ => unreachable!("Invalid method: {}", method_name),
                            },
                          ))),
                          takes_path: false,
                        }));

                        let cached_first_arg =
                          evaluate_cached(&first_arg.expr, state, traversal_state, fns);

                        let mut result = vec![cached_first_arg];

                        result.extend(
                          call
                            .args
                            .iter()
                            .skip(1)
                            .map(|arg| evaluate_cached(&arg.expr, state, traversal_state, fns))
                            .collect::<Vec<Option<EvaluateResultValue>>>(),
                        );

                        context = Some(vec![Some(EvaluateResultValue::Vec(
                          result.into_iter().collect(),
                        ))]);
                      }
                      "abs" => {
                        let cached_first_arg =
                          evaluate_cached(&first_arg.expr, state, traversal_state, fns);
                        if let Some(cached_first_arg) = cached_first_arg {
                          func = Some(Box::new(FunctionConfig {
                            fn_ptr: FunctionType::Callback(Box::new(CallbackType::Math(
                              MathJS::Abs,
                            ))),
                            takes_path: false,
                          }));

                          context = Some(vec![Some(EvaluateResultValue::Expr(
                            cached_first_arg
                              .as_expr()
                              .expect("First argument should be an expression")
                              .clone(),
                          ))]);
                        }
                      }
                      _ => {
                        panic!("{} - {}:{}", BUILT_IN_FUNCTION, callee_name, method_name)
                      }
                    }
                  }
                  "Object" => {
                    let args = &call.args;

                    let arg = args
                      .first()
                      .unwrap_or_else(|| panic!("Object.{} requires an argument", method_name));

                    if arg.spread.is_some() {
                      panic_with_context!(path, traversal_state, "Spread not implemented");
                    }

                    let cached_arg = evaluate_cached(&arg.expr, state, traversal_state, fns);

                    match method_name {
                      "fromEntries" => {
                        func = Some(Box::new(FunctionConfig {
                          fn_ptr: FunctionType::Callback(Box::new(CallbackType::Object(
                            ObjectJS::FromEntries,
                          ))),
                          takes_path: false,
                        }));

                        let mut from_entries_result = IndexMap::new();

                        match cached_arg.expect("Object.fromEntries requires an argument") {
                          EvaluateResultValue::Expr(expr) => {
                            let array = expr
                              .as_array()
                              .cloned()
                              .expect("Object.fromEntries requires an object");

                            let entries = array
                              .elems
                              .into_iter()
                              .flatten()
                              .collect::<Vec<ExprOrSpread>>();

                            for entry in entries {
                              assert!(entry.spread.is_none(), "Spread");

                              let array = entry.expr.as_array().expect("Entry must be an array");

                              let elems =
                                array.elems.iter().flatten().collect::<Vec<&ExprOrSpread>>();

                              let key = elems
                                .first()
                                .and_then(|e| e.expr.as_lit())
                                .expect("Key must be a literal");

                              let value = elems
                                .get(1)
                                .map(|e| e.expr.clone())
                                .expect("Value must be a literal");

                              from_entries_result.insert(key.clone(), value.clone());
                            }
                          }
                          EvaluateResultValue::Vec(vec) => {
                            for entry in vec {
                              let entry = entry
                                .and_then(|entry| entry.as_vec().cloned())
                                .expect("Entry must be some");

                              let key = entry
                                .first()
                                .and_then(|item| item.clone())
                                .and_then(|item| item.as_expr().cloned())
                                .and_then(|expr| expr.as_lit().cloned())
                                .expect("Key must be a literal");

                              let value = entry
                                .get(1)
                                .and_then(|item| item.clone())
                                .and_then(|item| item.as_expr().cloned())
                                .expect("Value must be a literal");

                              from_entries_result.insert(key.clone(), Box::new(value.clone()));
                            }
                          }
                          _ => {
                            panic!("Object.fromEntries requires an object")
                          }
                        };

                        context = Some(vec![Some(EvaluateResultValue::Entries(
                          from_entries_result,
                        ))]);
                      }
                      "keys" => {
                        func = Some(Box::new(FunctionConfig {
                          fn_ptr: FunctionType::Callback(Box::new(CallbackType::Object(
                            ObjectJS::Keys,
                          ))),
                          takes_path: false,
                        }));

                        let object = normalize_js_object_method_args(cached_arg);

                        let mut keys = vec![];

                        if let Some(object) = object {
                          for prop in &object.props {
                            let expr = prop.as_prop().cloned().expect("Spread");

                            let key_values =
                              expr.as_key_value().expect("Object.keys requires an object");

                            let key = key_value_to_str(key_values);

                            keys.push(Some(ExprOrSpread {
                              spread: None,
                              expr: Box::new(string_to_expression(key.as_str())),
                            }));
                          }
                        }

                        context = Some(vec![Some(EvaluateResultValue::Expr(Expr::Array(
                          ArrayLit {
                            span: DUMMY_SP,
                            elems: keys,
                          },
                        )))]);
                      }
                      "values" => {
                        func = Some(Box::new(FunctionConfig {
                          fn_ptr: FunctionType::Callback(Box::new(CallbackType::Object(
                            ObjectJS::Values,
                          ))),
                          takes_path: false,
                        }));

                        let object = normalize_js_object_method_args(cached_arg);

                        let mut values = vec![];

                        if let Some(object) = object {
                          for prop in &object.props {
                            let prop = prop.as_prop().cloned().expect("Spread");

                            let key_values = prop
                              .as_key_value()
                              .expect("Object.values requires an object");

                            values.push(Some(ExprOrSpread {
                              spread: None,
                              expr: key_values.value.clone(),
                            }));
                          }
                        }

                        context = Some(vec![Some(EvaluateResultValue::Expr(Expr::Array(
                          ArrayLit {
                            span: DUMMY_SP,
                            elems: values,
                          },
                        )))]);
                      }
                      "entries" => {
                        func = Some(Box::new(FunctionConfig {
                          fn_ptr: FunctionType::Callback(Box::new(CallbackType::Object(
                            ObjectJS::Entries,
                          ))),
                          takes_path: false,
                        }));

                        let object = normalize_js_object_method_args(cached_arg);

                        let mut entries: IndexMap<Lit, Box<Expr>> = IndexMap::new();

                        if let Some(object) = object {
                          for prop in &object.props {
                            let expr = prop.as_prop().map(|prop| *prop.clone()).expect("Spread");

                            let key_values = expr
                              .as_key_value()
                              .expect("Object.entries requires an object");

                            let value = key_values.value.clone();

                            let key = key_value_to_str(key_values);

                            entries.insert(lit_str_factory(key.as_str()), value);
                          }
                        }

                        context = Some(vec![Some(EvaluateResultValue::Entries(entries))]);
                      }
                      _ => {
                        panic!("{} - {}:{}", BUILT_IN_FUNCTION, callee_name, method_name)
                      }
                    }
                  }
                  _ => panic!("{} - {}", BUILT_IN_FUNCTION, callee_name),
                }
              } else {
                let prop_ident = property.as_ident().unwrap();

                let obj_name = obj_ident.sym.to_string();
                let prop_id = prop_ident.sym.to_id();

                if let Some(member_expr) = state
                  .functions
                  .member_expressions
                  .get(&ImportSources::Regular(obj_name))
                  && let Some(member_expr_fn) = member_expr.get(&prop_id.0)
                {
                  match member_expr_fn.as_ref() {
                    FunctionConfigType::Regular(fc) => {
                      func = Some(Box::new(fc.clone()));
                    }
                    FunctionConfigType::Map(_) => panic_with_context!(
                      path,
                      traversal_state,
                      "FunctionConfigType::Map not implemented"
                    ),
                    FunctionConfigType::IndexMap(_) => unimplemented!("IndexMap not implemented"),
                  }
                }
              }
            }

            if let Some(prop_id) = is_id_prop(property) {
              let obj_name = obj_ident.sym.to_string();

              if let Some(member_expr) = state
                .functions
                .member_expressions
                .get(&ImportSources::Regular(obj_name))
                && member_expr.contains_key(prop_id)
              {
                panic_with_context!(path, traversal_state, "Check what's happening here");

                // context = Some(member_expr.clone());

                // TODO: uncomment this for implementation of member expressions
                // match member_expr.get(&prop_id).unwrap() {
                //   FunctionConfigType::Regular(fc) => {
                //     func = Some(Box::new(fc.clone()));
                //   }
                //   FunctionConfigType::Map(_) => unimplemented!("FunctionConfigType::Map"),
                // }
              }
            }
          }

          if object.is_lit() {
            let obj_lit = object.as_lit().unwrap();

            if property.is_ident()
              && let Lit::Bool(_) = obj_lit
            {
              panic_with_context!(path, traversal_state, "Boolean object not implemented");
            }
          }

          if func.is_none() {
            let parsed_obj = evaluate(object, traversal_state, &state.functions);

            if parsed_obj.confident {
              if property.is_ident() {
                let prop_ident = property.as_ident().expect("Property is not an identifier");
                let prop_name = prop_ident.sym.to_string();

                if is_mutating_array_method(property) {
                  return deopt(path, state, NON_CONSTANT);
                }

                let value = match parsed_obj.value {
                  Some(v) => v,
                  None => {
                    panic_with_context!(
                      path,
                      traversal_state,
                      format!(
                        "Parsed object has no value when accessing property '.{}'",
                        prop_name
                      )
                      .as_str()
                    );
                  }
                };

                match value.clone() {
                  EvaluateResultValue::Map(map) => {
                    let result_fn = map.get(&Expr::from(prop_ident.clone()));

                    func = match result_fn {
                      Some(_) => panic_with_context!(
                        path,
                        traversal_state,
                        "EvaluateResultValue::Map not implemented"
                      ),
                      None => None,
                    };
                  }
                  EvaluateResultValue::Vec(expr) => {
                    func = Some(Box::new(FunctionConfig {
                      fn_ptr: FunctionType::Callback(Box::new(match prop_name.as_str() {
                        "map" => CallbackType::Array(ArrayJS::Map),
                        "filter" => CallbackType::Array(ArrayJS::Filter),
                        "join" => CallbackType::Array(ArrayJS::Join),
                        "entries" => CallbackType::Object(ObjectJS::Entries),
                        _ => panic_with_context!(
                          path,
                          traversal_state,
                          format!("Array method '{}' not implemented yet", prop_name).as_str()
                        ),
                      })),
                      takes_path: false,
                    }));

                    context = Some(expr.clone())
                  }
                  EvaluateResultValue::Expr(expr) => match expr {
                    Expr::Array(ArrayLit { elems, .. }) => {
                      func = Some(Box::new(FunctionConfig {
                        fn_ptr: FunctionType::Callback(Box::new(match prop_name.as_str() {
                          "map" => CallbackType::Array(ArrayJS::Map),
                          "filter" => CallbackType::Array(ArrayJS::Filter),
                          "entries" => CallbackType::Object(ObjectJS::Entries),
                          _ => panic_with_context!(
                            path,
                            traversal_state,
                            format!("Method '{}' not implemented yet", prop_name).as_str()
                          ),
                        })),
                        takes_path: false,
                      }));

                      let expr = elems
                        .iter()
                        .map(|elem| Some(EvaluateResultValue::Expr(*elem.clone().unwrap().expr)))
                        .collect::<Vec<Option<EvaluateResultValue>>>();

                      context = Some(vec![Some(EvaluateResultValue::Vec(expr))]);
                    }
                    Expr::Lit(Lit::Str(_)) => {
                      func = Some(Box::new(FunctionConfig {
                        fn_ptr: FunctionType::Callback(Box::new(match prop_name.as_str() {
                          "concat" => CallbackType::String(StringJS::Concat),
                          "charCodeAt" => CallbackType::String(StringJS::CharCodeAt),
                          _ => panic_with_context!(
                            path,
                            traversal_state,
                            format!("Method '{}' not implemented yet", prop_name).as_str()
                          ),
                        })),
                        takes_path: false,
                      }));

                      context = Some(vec![Some(EvaluateResultValue::Expr(expr.clone()))]);
                    }
                    Expr::Object(object) => {
                      let key_values = get_key_values_from_object(&object);

                      let key_value =
                        key_values
                          .into_iter()
                          .find(|key_value| match key_value.key.as_ident() {
                            Some(key_ident) => key_ident.sym == prop_name,
                            _ => false,
                          });

                      let Some(key_value) = key_value else {
                        panic_with_context!(path, traversal_state, "Property not found");
                      };

                      func = Some(Box::new(FunctionConfig {
                        fn_ptr: FunctionType::Callback(Box::new(CallbackType::Custom(
                          *key_value.value,
                        ))),
                        takes_path: false,
                      }));

                      let args: Vec<Option<EvaluateResultValue>> = call
                        .args
                        .iter()
                        .map(|arg| {
                          let arg = evaluate_cached(&arg.expr, state, traversal_state, fns);

                          if !state.confident {
                            return None;
                          }

                          arg
                        })
                        .collect();

                      context = Some(args);
                    }
                    Expr::Lit(Lit::Regex(_)) => {
                      // Regex methods like .test(), .exec(), etc. require runtime evaluation
                      // We can't statically evaluate them, so we deopt
                      return deopt(path, state, "Regex methods cannot be statically evaluated");
                    }
                    _ => {
                      panic_with_context!(
                        path,
                        traversal_state,
                        "Expression evaluation not implemented"
                      )
                    }
                  },
                  EvaluateResultValue::FunctionConfig(fc) => match fc.fn_ptr {
                    FunctionType::StylexFnsFactory(sxfns) => {
                      let fc = sxfns(prop_name);

                      func = Some(Box::new(FunctionConfig {
                        fn_ptr: FunctionType::StylexTypeFn(fc),
                        takes_path: false,
                      }));

                      context = Some(vec![Some(value)]);
                    }
                    FunctionType::DefaultMarker(default_marker) => {
                      if let Some(expr_fn) = default_marker.get(&prop_name) {
                        func = Some(Box::new(FunctionConfig {
                          fn_ptr: FunctionType::StylexExprFn(*expr_fn),
                          takes_path: false,
                        }));

                        context = Some(vec![Some(value)]);
                      };
                    }
                    _ => panic_with_context!(
                      path,
                      traversal_state,
                      "FunctionType::StylexFnsFactory not implemented"
                    ),
                  },
                  _ => panic_with_context!(
                    path,
                    traversal_state,
                    "Evaluation result value not implemented"
                  ),
                }
              } else if let Some(prop_id) = is_id_prop(property) {
                let prop_id_owned = prop_id.to_string();

                let value = match parsed_obj.value {
                  Some(v) => v,
                  None => {
                    panic_with_context!(
                      path,
                      traversal_state,
                      format!(
                        "Parsed object has no value when accessing computed property '{}'",
                        prop_id_owned
                      )
                      .as_str()
                    );
                  }
                };
                let map = match value.as_map() {
                  Some(m) => m,
                  None => {
                    panic_with_context!(
                      path,
                      traversal_state,
                      format!(
                        "Expected object map when accessing computed property '{}', got {:?}",
                        prop_id_owned, value
                      )
                      .as_str()
                    );
                  }
                };

                let result_fn = map.get(&string_to_expression(&prop_id_owned));

                func = match result_fn {
                  Some(_) => panic_with_context!(path, traversal_state, "Result function is some"),
                  None => None,
                };
              }
            }
          }
        }
      }

      if let Some(func) = func {
        if func.takes_path {
          let args = call.args.iter().map(|arg| &*arg.expr).collect::<Vec<_>>();

          match func.fn_ptr {
            FunctionType::ArrayArgs(func) => {
              let func_result = (func)(
                args.iter().map(|arg| (*arg).clone()).collect(),
                traversal_state,
                fns,
              );
              return Some(EvaluateResultValue::Expr(func_result));
            }
            FunctionType::StylexExprFn(func) => {
              let func_result = (func)((**args.first().unwrap()).clone(), traversal_state);

              return Some(EvaluateResultValue::Expr(func_result));
            }
            FunctionType::StylexTypeFn(_) => {
              panic_with_context!(path, traversal_state, "StylexFnsFactory not implemented")
            }
            FunctionType::StylexFnsFactory(_) => {
              panic_with_context!(path, traversal_state, "StylexFnsFactory")
            }
            FunctionType::Callback(_) => {
              panic_with_context!(path, traversal_state, "Arrow function")
            }
            FunctionType::Mapper(_) => panic_with_context!(path, traversal_state, "Mapper"),
            FunctionType::DefaultMarker(_) => {
              panic_with_context!(path, traversal_state, "DefaultMarker")
            }
          }
        } else {
          if !state.confident {
            return None;
          }

          match func.fn_ptr {
            FunctionType::ArrayArgs(func) => {
              let args = evaluate_func_call_args(call, state, traversal_state, fns);
              let func_result = (func)(
                args
                  .into_iter()
                  .map(|arg| {
                    arg
                      .as_expr()
                      .cloned()
                      .expect("Argument is not an expression")
                  })
                  .collect(),
                traversal_state,
                fns,
              );
              return Some(EvaluateResultValue::Expr(func_result));
            }
            FunctionType::StylexExprFn(func) => {
              let args = evaluate_func_call_args(call, state, traversal_state, fns);
              let func_result = (func)(
                args.first().and_then(|arg| arg.as_expr().cloned()).unwrap(),
                traversal_state,
              );
              return Some(EvaluateResultValue::Expr(func_result));
            }
            FunctionType::StylexTypeFn(func) => {
              let args = evaluate_func_call_args(call, state, traversal_state, fns);
              let mut fn_args = IndexMap::default();
              let expr = args
                .first()
                .and_then(|expr| expr.as_expr())
                .expect("Argument is not an expression");

              match expr {
                Expr::Object(obj) => {
                  for prop in &obj.props {
                    let prop = prop.as_prop().unwrap();
                    let key_value = prop.as_key_value().unwrap();

                    let key = key_value
                      .key
                      .as_ident()
                      .expect("Key not an ident")
                      .sym
                      .to_string();

                    let value = key_value.value.as_lit().expect("Value not a literal");

                    fn_args.insert(key, ValueWithDefault::String(lit_to_string(value).unwrap()));
                  }
                }
                Expr::Lit(lit) => {
                  fn_args.insert(
                    "default".to_string(),
                    ValueWithDefault::String(lit_to_string(lit).unwrap()),
                  );
                }
                _ => {}
              }

              let func_result = (func)(ValueWithDefault::Map(fn_args));
              return Some(EvaluateResultValue::Expr(func_result));
            }
            FunctionType::Callback(func) => {
              let context = context.expect("Object.entries requires a context");

              match func.as_ref() {
                CallbackType::Array(ArrayJS::Map) => {
                  let args = evaluate_func_call_args(call, state, traversal_state, fns);

                  return evaluate_map(&args, &context);
                }
                CallbackType::Array(ArrayJS::Filter) => {
                  let args = evaluate_func_call_args(call, state, traversal_state, fns);

                  return evaluate_filter(&args, &context);
                }
                CallbackType::Array(ArrayJS::Join) => {
                  let args = evaluate_func_call_args(call, state, traversal_state, fns);

                  return evaluate_join(&args, &context, traversal_state, &state.functions);
                }
                CallbackType::Object(ObjectJS::Entries) => {
                  let Some(Some(eval_result)) = context.first() else {
                    panic_with_context!(
                      path,
                      traversal_state,
                      "Object.entries requires an argument"
                    )
                  };

                  let EvaluateResultValue::Entries(entries) = eval_result else {
                    panic_with_context!(
                      path,
                      traversal_state,
                      "Object.entries requires an argument"
                    )
                  };

                  let mut entry_elems: Vec<Option<ExprOrSpread>> = vec![];

                  for (key, value) in entries {
                    let key: ExprOrSpread = ExprOrSpread {
                      spread: None,
                      expr: Box::new(Expr::from(key.clone())),
                    };

                    let value: ExprOrSpread = ExprOrSpread {
                      spread: None,
                      expr: value.clone(),
                    };

                    entry_elems.push(Some(ExprOrSpread {
                      spread: None,
                      expr: Box::new(array_expression_factory(vec![Some(key), Some(value)])),
                    }));
                  }

                  return Some(EvaluateResultValue::Expr(array_expression_factory(
                    entry_elems,
                  )));
                }
                CallbackType::Object(ObjectJS::Keys) => {
                  let Some(Some(EvaluateResultValue::Expr(keys))) = context.first() else {
                    panic_with_context!(path, traversal_state, "Object.keys requires an argument")
                  };

                  return Some(EvaluateResultValue::Expr(keys.clone()));
                }
                CallbackType::Object(ObjectJS::Values) => {
                  let Some(Some(EvaluateResultValue::Expr(values))) = context.first() else {
                    panic_with_context!(path, traversal_state, "Object.keys requires an argument")
                  };

                  return Some(EvaluateResultValue::Expr(values.clone()));
                }
                CallbackType::Object(ObjectJS::FromEntries) => {
                  let Some(Some(EvaluateResultValue::Entries(entries))) = context.first() else {
                    panic_with_context!(
                      path,
                      traversal_state,
                      "Object.fromEntries requires an argument"
                    )
                  };

                  let mut entry_elems = vec![];

                  for (key, value) in entries {
                    let ident_name = if let Lit::Str(lit_str) = key {
                      quote_ident!(atom_to_str(&lit_str.value))
                    } else {
                      panic_with_context!(path, traversal_state, "Expected a string literal")
                    };

                    let prop = PropOrSpread::Prop(Box::new(Prop::from(KeyValueProp {
                      key: PropName::Ident(ident_name),
                      value: value.clone(),
                    })));

                    entry_elems.push(prop);
                  }

                  return Some(EvaluateResultValue::Expr(object_expression_factory(
                    entry_elems,
                  )));
                }
                CallbackType::Math(MathJS::Pow) => {
                  let Some(Some(EvaluateResultValue::Vec(args))) = context.first() else {
                    panic_with_context!(path, traversal_state, "Math.pow requires an argument")
                  };

                  let num_args = args
                    .iter()
                    .flatten()
                    .map(|arg| {
                      arg
                        .as_expr()
                        .map(|expr| {
                          unwrap_or_panic!(expr_to_num(expr, state, traversal_state, fns))
                        })
                        .expect("All arguments must be a number")
                    })
                    .collect::<Vec<f64>>();

                  let result = num_args.first().unwrap().powf(*num_args.get(1).unwrap());

                  return Some(EvaluateResultValue::Expr(number_to_expression(result)));
                }
                CallbackType::Math(MathJS::Round | MathJS::Floor | MathJS::Ceil) => {
                  let Some(Some(EvaluateResultValue::Expr(expr))) = context.first() else {
                    panic_with_context!(
                      path,
                      traversal_state,
                      "Math.(round | ceil | floor) requires an argument"
                    )
                  };

                  let num =
                    expr_to_num(expr, state, traversal_state, fns).unwrap_or_else(|error| {
                      panic_with_context!(path, traversal_state, error.to_string().as_str())
                    });

                  let result = match func.as_ref() {
                    CallbackType::Math(MathJS::Round) => num.round(),
                    CallbackType::Math(MathJS::Ceil) => num.ceil(),
                    CallbackType::Math(MathJS::Floor) => num.floor(),
                    _ => unreachable!("Invalid function type"),
                  };

                  return Some(EvaluateResultValue::Expr(number_to_expression(result)));
                }
                CallbackType::Math(MathJS::Min | MathJS::Max) => {
                  let Some(Some(EvaluateResultValue::Vec(args))) = context.first() else {
                    panic_with_context!(
                      path,
                      traversal_state,
                      "Math.(min | max) requires an argument"
                    )
                  };

                  let num_args = args_to_numbers(args, state, traversal_state, fns);

                  let result = match func.as_ref() {
                    CallbackType::Math(MathJS::Min) => {
                      num_args.iter().cloned().min_by(sort_numbers_factory())
                    }
                    CallbackType::Math(MathJS::Max) => {
                      num_args.iter().cloned().max_by(sort_numbers_factory())
                    }
                    _ => unreachable!("Invalid function type"),
                  }
                  .unwrap();

                  return Some(EvaluateResultValue::Expr(number_to_expression(result)));
                }
                CallbackType::Math(MathJS::Abs) => {
                  let Some(Some(EvaluateResultValue::Expr(expr))) = context.first() else {
                    panic_with_context!(path, traversal_state, "Math.abs requires an argument")
                  };

                  let num =
                    expr_to_num(expr, state, traversal_state, fns).unwrap_or_else(|error| {
                      panic_with_context!(path, traversal_state, error.to_string().as_str())
                    });

                  return Some(EvaluateResultValue::Expr(number_to_expression(num.abs())));
                }
                CallbackType::String(StringJS::Concat) => {
                  let Some(Some(EvaluateResultValue::Expr(base_str))) = context.first() else {
                    panic_with_context!(path, traversal_state, "String concat requires an argument")
                  };

                  let args = evaluate_func_call_args(call, state, traversal_state, fns);

                  let mut str_args_vec = Vec::new();
                  for arg in &args {
                    match arg.as_expr() {
                      Some(expr) => {
                        str_args_vec.push(expr_to_str_or_deopt!(
                          expr,
                          state,
                          traversal_state,
                          fns,
                          "Expression is not a string"
                        ));
                      }
                      None => {
                        deopt(path, state, "All arguments must be a string");
                        return None;
                      }
                    }
                  }
                  let str_args = str_args_vec.join("");

                  let base_str = expr_to_str_or_deopt!(
                    base_str,
                    state,
                    traversal_state,
                    fns,
                    "Expression is not a string"
                  );

                  return Some(EvaluateResultValue::Expr(string_to_expression(
                    format!("{}{}", base_str, str_args).as_str(),
                  )));
                }
                CallbackType::String(StringJS::CharCodeAt) => {
                  let Some(Some(EvaluateResultValue::Expr(base_str))) = context.first() else {
                    panic_with_context!(path, traversal_state, "String concat requires an argument")
                  };

                  let base_str = expr_to_str_or_deopt!(
                    base_str,
                    state,
                    traversal_state,
                    fns,
                    "Expression is not a string"
                  );

                  let args = evaluate_func_call_args(call, state, traversal_state, fns);

                  let num_args = args
                    .iter()
                    .map(|arg| {
                      arg
                        .as_expr()
                        .map(|expr| {
                          unwrap_or_panic!(expr_to_num(expr, state, traversal_state, fns))
                        })
                        .expect("First argument must be a number")
                    })
                    .collect::<Vec<f64>>();

                  let char_index = num_args
                    .first()
                    .expect("First argument of 'charCodeAt' method must be a number");

                  let char_code = char_code_at(&base_str, *char_index as usize)
                    .expect("Char code not found for index");

                  return Some(EvaluateResultValue::Expr(number_to_expression(
                    char_code as f64,
                  )));
                }
                CallbackType::Custom(arrow_fn) => {
                  let args = evaluate_func_call_args(call, state, traversal_state, fns);

                  let evaluation_result = evaluate_cached(arrow_fn, state, traversal_state, fns);

                  let expr_result = match evaluation_result.as_ref() {
                    Some(EvaluateResultValue::Callback(cb)) => {
                      cb(args.into_iter().map(Some).collect())
                    }
                    _ => panic_with_context!(path, traversal_state, "Arrow function not found"),
                  };

                  return Some(EvaluateResultValue::Expr(expr_result));
                }
              }
            }
            FunctionType::DefaultMarker(default_marker) => {
              return Some(EvaluateResultValue::FunctionConfig(FunctionConfig {
                fn_ptr: FunctionType::DefaultMarker(Arc::clone(&default_marker)),
                takes_path: false,
              }));
            }
            _ => panic_with_context!(path, traversal_state, "Function type"),
          }
        }
      }

      return deopt(
        normalized_path,
        state,
        &unsupported_expression(&format!(
          "{:?}",
          normalized_path.get_type(get_default_expr_ctx())
        )),
      );
    }
    Expr::Await(await_expr) => evaluate_cached(&await_expr.arg, state, traversal_state, fns),
    Expr::OptChain(opt_chain) => {
      // Evaluate the base object/callee first
      let base_result = match opt_chain.base.as_ref() {
        OptChainBase::Member(member) => evaluate_cached(&member.obj, state, traversal_state, fns),
        OptChainBase::Call(call) => evaluate_cached(&call.callee, state, traversal_state, fns),
      };

      // Check if we should short-circuit:
      // 1. Base is null literal
      // 2. Base is undefined identifier
      // 3. Base evaluation failed (returned None)
      let should_short_circuit = match &base_result {
        Some(EvaluateResultValue::Expr(base_expr)) => {
          matches!(base_expr, Expr::Lit(Lit::Null(_)))
            || (matches!(base_expr, Expr::Ident(ident) if ident.sym == *"undefined"))
        }
        None => true,
        // For other result types (Object, Array, FunctionConfig, etc.), don't short-circuit
        _ => false,
      };

      if should_short_circuit {
        None
      } else {
        // Otherwise, evaluate the full optional chain expression
        match opt_chain.base.as_ref() {
          OptChainBase::Member(member) => {
            let member_expr = Expr::Member(member.clone());
            evaluate_cached(&member_expr, state, traversal_state, fns)
          }
          OptChainBase::Call(call) => {
            let call_expr = Expr::Call(call.clone().into());
            evaluate_cached(&call_expr, state, traversal_state, fns)
          }
        }
      }
    }
    _ => {
      warn!(
        "Unsupported type of expression: {:?}. If its not enough, please run in debug mode to see more details",
        normalized_path.get_type(get_default_expr_ctx())
      );

      debug!("Unsupported type of expression: {:?}", normalized_path);

      return deopt(
        normalized_path,
        state,
        &unsupported_expression(&format!(
          "{:?}",
          normalized_path.get_type(get_default_expr_ctx())
        )),
      );
    }
  };

  if result.is_none() && normalized_path.is_ident() {
    let Some(ident) = normalized_path.as_ident() else {
      panic_with_context!(path, traversal_state, "Identifier not foun")
    };

    let binding = get_var_decl_by_ident(
      ident,
      traversal_state,
      &state.functions,
      if traversal_state.cycle == TransformationCycle::TransformExit {
        // NOTE: We don't want to reduce the binding count of stylex.props arguments
        VarDeclAction::None
      } else {
        VarDeclAction::Reduce
      },
    );

    if let Some(init) = binding.and_then(|var_decl| var_decl.init.clone()) {
      return evaluate_cached(&init, state, traversal_state, fns);
    }

    let name = ident.sym.to_string();

    if name == "undefined" || name == "Infinity" || name == "NaN" {
      return Some(EvaluateResultValue::Expr(Expr::from(ident.clone())));
    }

    if let Some(import_path) = get_import_by_ident(ident, traversal_state)
      && !state.functions.disable_imports
    {
      let (local_name, imported) = import_path
        .specifiers
        .iter()
        .find_map(|import| {
          let (local_name, imported) = match import {
            ImportSpecifier::Default(default) => (
              default.local.clone(),
              Some(ModuleExportName::Ident(default.local.clone())),
            ),
            ImportSpecifier::Named(named) => (named.local.clone(), named.imported.clone()),
            ImportSpecifier::Namespace(namespace) => (
              namespace.local.clone(),
              Some(ModuleExportName::Ident(namespace.local.clone())),
            ),
          };

          if ident.sym == local_name.sym {
            Some((local_name, imported))
          } else {
            None
          }
        })
        .expect("Import specifier not found");

      let imported = imported
        .clone()
        .unwrap_or_else(|| ModuleExportName::Ident(local_name.clone()));

      let abs_path = traversal_state.import_path_resolver(
        atom_to_str(&import_path.src.value),
        &mut FxHashMap::default(),
      );

      let imported_name = match imported {
        ModuleExportName::Ident(ident) => ident.sym.to_string(),
        ModuleExportName::Str(strng) => atom_to_string(&strng.value),
      };

      let return_value = match abs_path {
        ImportPathResolution::Tuple(ImportPathResolutionType::ThemeNameRef, value) => {
          evaluate_theme_ref(&value, imported_name, traversal_state)
        }
        _ => return deopt(path, state, IMPORT_PATH_RESOLUTION_ERROR),
      };

      if state.confident {
        let import_path_src = atom_to_string(&import_path.src.value);

        if !state.added_imports.contains(&import_path_src)
          && traversal_state.get_treeshake_compensation()
        {
          let prepend_import_module_item = add_import_expression(&import_path_src);

          if !traversal_state
            .prepend_import_module_items
            .contains(&prepend_import_module_item)
          {
            traversal_state
              .prepend_import_module_items
              .push(prepend_import_module_item);
          }

          state.added_imports.insert(import_path_src);
        }

        return Some(EvaluateResultValue::ThemeRef(return_value));
      }
    }

    return check_ident_declaration(
      ident,
      &[
        (
          DeclarationType::Class,
          &traversal_state.class_name_declarations,
        ),
        (
          DeclarationType::Function,
          &traversal_state.function_name_declarations,
        ),
      ],
      state,
      normalized_path,
    );
  }

  if result.is_none() {
    return deopt(
      normalized_path,
      state,
      &unsupported_expression(&format!(
        "{:?}",
        normalized_path.get_type(get_default_expr_ctx())
      )),
    );
  }

  result
}

/// Normalizes different argument types into an ObjectLit for JavaScript object methods.
fn normalize_js_object_method_args(cached_arg: Option<EvaluateResultValue>) -> Option<ObjectLit> {
  cached_arg.and_then(|arg| match arg {
    EvaluateResultValue::Expr(expr) => expr.as_object().cloned().or_else(|| {
      if let Expr::Lit(Lit::Str(ref strng)) = expr {
        let keys = atom_to_string(&strng.value)
          .chars()
          .enumerate()
          .map(|(i, c)| {
            prop_or_spread_expression_factory(&i.to_string(), string_to_expression(&c.to_string()))
          })
          .collect::<Vec<PropOrSpread>>();

        Some(ObjectLit {
          props: keys,
          span: DUMMY_SP,
        })
      } else {
        None
      }
    }),

    EvaluateResultValue::Vec(arr) => {
      let props = arr
        .iter()
        .enumerate()
        .filter_map(|(index, elem)| {
          elem.as_ref().map(|elem_value| {
            let expr = match elem_value {
              EvaluateResultValue::Expr(expr) => expr.clone(),
              EvaluateResultValue::Vec(vec) => normalize_js_object_method_nested_vector_arg(vec),
              _ => panic!("{}", ILLEGAL_PROP_ARRAY_VALUE),
            };

            prop_or_spread_expression_factory(&index.to_string(), expr)
          })
        })
        .collect();

      Some(ObjectLit {
        props,
        span: DUMMY_SP,
      })
    }

    _ => None,
  })
}

/// Helper function to convert a nested vector of EvaluateResultValues to an array expression
fn normalize_js_object_method_nested_vector_arg(vec: &[Option<EvaluateResultValue>]) -> Expr {
  let elems = vec
    .iter()
    .map(|entry| {
      let expr = entry
        .as_ref()
        .and_then(|entry| {
          entry
            .as_vec()
            .map(|nested_vec| {
              let nested_elems = nested_vec
                .iter()
                .flatten()
                .map(|item| {
                  let item = item.as_expr().unwrap();
                  Some(ExprOrSpread {
                    spread: None,
                    expr: Box::new(item.clone()),
                  })
                })
                .collect();

              Expr::Array(ArrayLit {
                span: DUMMY_SP,
                elems: nested_elems,
              })
            })
            .or_else(|| entry.as_expr().cloned())
        })
        .expect(ILLEGAL_PROP_ARRAY_VALUE);

      Some(ExprOrSpread {
        spread: None,
        expr: Box::new(expr),
      })
    })
    .collect();

  Expr::Array(ArrayLit {
    span: DUMMY_SP,
    elems,
  })
}

fn evaluate_func_call_args(
  call: &CallExpr,
  state: &mut EvaluationState,
  traversal_state: &mut StateManager,
  fns: &FunctionMap,
) -> Vec<EvaluateResultValue> {
  call
    .args
    .iter()
    .filter_map(|arg| evaluate_cached(&arg.expr, state, traversal_state, fns))
    .collect()
}

fn args_to_numbers(
  args: &[Option<EvaluateResultValue>],
  state: &mut EvaluationState,
  traversal_state: &mut StateManager,
  fns: &FunctionMap,
) -> Vec<f64> {
  args
    .iter()
    .flat_map(|arg| match arg {
      Some(arg) => match arg {
        EvaluateResultValue::Expr(expr) => {
          vec![
            expr_to_num(expr, state, traversal_state, fns).unwrap_or_else(|error| {
              panic_with_context!(expr, traversal_state, error.to_string().as_str())
            }),
          ]
        }
        EvaluateResultValue::Vec(vec) => args_to_numbers(vec, state, traversal_state, fns),
        _ => unreachable!("Math.min/max requires a number"),
      },
      None => vec![],
    })
    .collect::<Vec<f64>>()
}

fn get_binding<'a>(callee: &'a Expr, state: &'a StateManager) -> Option<&'a VarDeclarator> {
  match callee {
    Expr::Ident(ident) => get_var_decl_from(state, ident),
    _ => None,
  }
}

fn is_valid_callee(callee: &Expr) -> bool {
  if let Expr::Ident(ident) = callee {
    VALID_CALLEES.contains(ident.sym.as_ref())
  } else {
    false
  }
}

fn get_callee_name(callee: &Expr) -> &str {
  match callee {
    Expr::Ident(ident) => &ident.sym,
    _ => panic!("Callee is not an identifier"),
  }
}

fn is_invalid_method(prop: &MemberProp) -> bool {
  match prop {
    MemberProp::Ident(ident_prop) => INVALID_METHODS.contains(&*ident_prop.sym),
    _ => false,
  }
}

/// Checks if a member property represents a mutating object method (Object.assign, etc.)
fn is_mutating_object_method(prop: &MemberProp) -> bool {
  if let MemberProp::Ident(ident_prop) = prop {
    MUTATING_OBJECT_METHODS.contains(&*ident_prop.sym)
  } else {
    false
  }
}

/// Checks if a member property represents a mutating array method (push, pop, splice, etc.)
fn is_mutating_array_method(prop: &MemberProp) -> bool {
  if let MemberProp::Ident(ident_prop) = prop {
    MUTATING_ARRAY_METHODS.contains(&*ident_prop.sym)
  } else {
    false
  }
}

/// Checks if an expression represents a mutation operation
/// Returns true if any of the following conditions are met:
/// - Assignment to a member expression (e.g., `a.x = 1` or `a[0] = 1`)
/// - Update expression on a member (e.g., `++a.x` or `a[0]++`)
/// - Delete operation on a member (e.g., `delete a.x`)
fn is_mutation_expr(expr: &Expr) -> bool {
  match expr {
    // Check for assignment to member: a.x = 1 or a[0] = 1
    Expr::Assign(assign)
      if matches!(
        &assign.left,
        AssignTarget::Simple(SimpleAssignTarget::Member(member)) if member.obj.is_ident()
      ) =>
    {
      true
    }

    // Check for update on member: ++a.x or a[0]++
    Expr::Update(update) if matches!(&*update.arg, Expr::Member(member) if member.obj.is_ident()) => {
      true
    }

    // Check for delete on member: delete a.x
    Expr::Unary(unary)
      if unary.op == UnaryOp::Delete
        && matches!(&*unary.arg, Expr::Member(member) if member.obj.is_ident()) =>
    {
      true
    }

    _ => false,
  }
}

fn get_method_name(prop: &MemberProp) -> &str {
  match prop {
    MemberProp::Ident(ident_prop) => &ident_prop.sym,
    _ => panic!("Method is not an identifier"),
  }
}

fn is_id_prop(prop: &MemberProp) -> Option<&Atom> {
  if let MemberProp::Computed(comp_prop) = prop
    && let Expr::Lit(Lit::Str(strng)) = comp_prop.expr.as_ref()
  {
    return Some(
      strng
        .value
        .as_atom()
        .expect("Failed to convert Str to Atom"),
    );
  }

  None
}

pub(crate) fn evaluate_quasis(
  tpl_expr: &Expr,
  quasis: &[TplElement],
  raw: bool,
  state: &mut EvaluationState,
  traversal_state: &mut StateManager,
  fns: &FunctionMap,
) -> Option<EvaluateResultValue> {
  let mut strng = String::new();

  let exprs = match tpl_expr {
    Expr::Tpl(tpl) => &tpl.exprs,
    Expr::TaggedTpl(tagged_tpl) => &tagged_tpl.tpl.exprs,
    _ => panic_with_context!(
      tpl_expr,
      traversal_state,
      "The expression is not a template"
    ),
  };

  for (i, elem) in quasis.iter().enumerate() {
    if !state.confident {
      return None;
    }

    if raw {
      strng.push_str(&elem.raw);
    } else {
      strng.push_str(&tpl_element_cooked_to_string(elem));
    }

    if let Some(expr) = exprs.get(i)
      && let Some(evaluated_expr) = evaluate_cached(expr, state, traversal_state, fns)
      && let Some(lit_str) = evaluated_expr
        .as_expr()
        .and_then(|expr| expr.as_lit())
        .and_then(lit_to_string)
    {
      strng.push_str(&lit_str);
    }
  }

  if !state.confident {
    return None;
  }

  Some(EvaluateResultValue::Expr(string_to_expression(&strng)))
}

pub(crate) fn evaluate_cached(
  path: &Expr,
  state: &mut EvaluationState,
  traversal_state: &mut StateManager,
  fns: &FunctionMap,
) -> Option<EvaluateResultValue> {
  let mut cleaned_path = drop_span(path.clone());

  let cleaned_path_hash = stable_hash(&cleaned_path);

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
          }
          _ => {}
        }

        return resolved;
      }

      deopt(path, state, PATH_WITHOUT_NODE)
    }
    None => {
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
    }
  }
}

fn evaluate_theme_ref(file_name: &str, export_name: String, state: &StateManager) -> ThemeRef {
  ThemeRef::new(
    file_name.to_owned(),
    export_name,
    state.options.class_name_prefix.clone(),
  )
}
