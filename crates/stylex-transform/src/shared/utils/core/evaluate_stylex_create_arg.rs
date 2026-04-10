use indexmap::IndexMap;
use stylex_macros::{stylex_panic, stylex_unimplemented};
use swc_core::{
  common::{DUMMY_SP, SyntaxContext},
  ecma::{
    ast::{
      ArrowExpr, BinExpr, BinaryOp, BindingIdent, BlockStmtOrExpr, CallExpr, Callee, CondExpr,
      Expr, KeyValueProp, ObjectLit, Pat, Prop, PropOrSpread, UnaryExpr, UnaryOp,
    },
    utils::quote_ident,
  },
};

use crate::shared::enums::data_structures::evaluate_result_value::EvaluateResultValue;
use crate::shared::structures::evaluate_result::EvaluateResult;
use crate::shared::structures::functions::FunctionMap;
use crate::shared::structures::state_manager::StateManager;
use crate::shared::structures::types::{DynamicFns, TInlineStyles};
use crate::shared::utils::ast::convertors::{
  convert_expr_to_str, create_ident_expr, create_null_expr, create_string_expr,
  expand_shorthand_prop,
};
use crate::shared::utils::common::{create_hash, normalize_expr};
use crate::shared::utils::css::common::get_number_suffix;
use crate::shared::utils::js::evaluate::{evaluate, evaluate_obj_key};
use crate::shared::utils::validators::validate_dynamic_style_params;
use stylex_ast::ast::factories::{
  create_expr_or_spread, create_key_value_prop, create_object_expression,
};
use stylex_constants::constants::length_units::LENGTH_UNITS;
use stylex_constants::constants::messages::ILLEGAL_NAMESPACE_VALUE;
use stylex_constants::constants::messages::{
  EVAL_RESULT_EXPECTED, KEY_MUST_EVAL_TO_STRING, SPREAD_NOT_SUPPORTED, VALUE_NOT_EXPRESSION,
};
use stylex_constants::constants::time_units::get_time_units;
use stylex_structures::inline_style::InlineStyle;

/// Prepends a key name to an existing error reason to provide context
/// about which property path triggered the evaluation failure.
fn prepend_key_to_reason(key: &str, reason: Option<String>) -> Option<String> {
  reason.map(|r| format!("{} > {}", key, r))
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
                  return Box::new(EvaluateResult {
                    confident: false,
                    deopt: key_result.deopt,
                    reason: key_result.reason,
                    value: None,
                    inline_styles: None,
                    fns: None,
                  });
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

                    match fn_path.body.as_mut() {
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
                        return Box::new(EvaluateResult {
                          confident: false,
                          deopt: None,
                          reason: Some(
                            "Block statement is not allowed in Dynamic Style functions".to_string(),
                          ),
                          value: None,
                          inline_styles: None,
                          fns: None,
                        });
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
                      EvaluateResultValue::Expr(Expr::Object(obj_expr)) => obj_expr
                        .props
                        .iter()
                        .filter_map(|prop| prop.as_prop().and_then(|prop| prop.as_key_value()))
                        .cloned()
                        .collect::<Vec<_>>(),
                      _ => stylex_panic!("{}", ILLEGAL_NAMESPACE_VALUE),
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
        fns: if fns.is_empty() { None } else { Some(fns) },
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
          return result;
        }
        stylex_unimplemented!("{}", SPREAD_NOT_SUPPORTED);
      },
      PropOrSpread::Prop(prop) => {
        let mut prop = prop.clone();

        expand_shorthand_prop(&mut prop);

        match prop.as_mut() {
          Prop::KeyValue(key_value) => {
            let key_result = evaluate_obj_key(key_value, traversal_state, functions);

            if !key_result.confident {
              return Box::new(EvaluateResult {
                confident: false,
                deopt: key_result.deopt,
                reason: key_result.reason,
                value: None,
                inline_styles: None,
                fns: None,
              });
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

              // When the `key_path` is not empty, the var(--hash) is a `defineConsts` at-rule placeholder and must be kept intact.
              if key_path.is_empty() {
                key_str = inner;
              }
            }

            let value_path = &mut key_value.value;
            match normalize_expr(value_path.as_mut()) {
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
                  return Box::new(EvaluateResult {
                    confident: false,
                    deopt: result.deopt,
                    reason: result.reason,
                    value: None,
                    inline_styles: None,
                    fns: None,
                  });
                }

                let new_prop = create_key_value_prop(
                  &key_str,
                  match result.value.and_then(|v| v.as_expr().cloned()) {
                    Some(expr) => expr,
                    None => stylex_panic!("{}", VALUE_NOT_EXPRESSION),
                  },
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
                    .find(|&k| !k.starts_with(':') && !k.starts_with('@') && k != "default")
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
                    Expr::from(CallExpr {
                      span: DUMMY_SP,
                      callee: Callee::Expr(Box::new(Expr::Arrow(ArrowExpr {
                        span: DUMMY_SP,
                        params: vec![Pat::Ident(BindingIdent::from(quote_ident!("val")))],
                        body: Box::new(BlockStmtOrExpr::Expr(Box::new(Expr::Cond(CondExpr {
                          span: DUMMY_SP,
                          test: Box::new(Expr::from(BinExpr {
                            span: DUMMY_SP,
                            op: BinaryOp::EqEqEq,
                            left: Box::new(Expr::from(UnaryExpr {
                              span: DUMMY_SP,
                              op: UnaryOp::TypeOf,
                              arg: Box::new(val_ident.clone()),
                            })),
                            right: Box::new(create_string_expr("number")),
                          })),
                          cons: Box::new(Expr::from(BinExpr {
                            span: DUMMY_SP,
                            op: BinaryOp::Add,
                            left: Box::new(val_ident.clone()),
                            right: Box::new(create_string_expr(unit)),
                          })),
                          alt: Box::new(Expr::from(CondExpr {
                            span: DUMMY_SP,
                            test: Box::new(Expr::from(BinExpr {
                              span: DUMMY_SP,
                              op: BinaryOp::NotEq,
                              left: Box::new(val_ident.clone()),
                              right: Box::new(create_null_expr()),
                            })),
                            cons: Box::new(val_ident),
                            alt: Box::new(create_ident_expr("undefined")),
                          })),
                        })))),
                        is_async: false,
                        is_generator: false,
                        type_params: None,
                        return_type: None,
                        ctxt: SyntaxContext::empty(),
                      }))),
                      args: vec![create_expr_or_spread(*value_path.clone())],
                      type_args: None,
                      ctxt: SyntaxContext::empty(),
                    })
                  } else {
                    Expr::from(CondExpr {
                      span: DUMMY_SP,
                      test: Box::new(Expr::from(BinExpr {
                        span: DUMMY_SP,
                        op: BinaryOp::NotEq,
                        left: value_path.clone(),
                        right: Box::new(create_null_expr()),
                      })),
                      cons: value_path.clone(),
                      alt: Box::new(create_ident_expr("undefined")),
                    })
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
                    match result.value.and_then(|v| v.as_expr().cloned()) {
                      Some(expr) => expr,
                      None => stylex_panic!("{}", VALUE_NOT_EXPRESSION),
                    },
                  );
                  obj.push(new_prop);
                }
              },
            }
          },
          Prop::Method(_) => {
            return Box::new(EvaluateResult {
              confident: false,
              deopt: None,
              reason: None,
              value: None,
              inline_styles: None,
              fns: None,
            });
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
