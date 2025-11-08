use indexmap::IndexMap;
use swc_core::{
  common::{DUMMY_SP, SyntaxContext},
  ecma::{
    ast::{
      ArrowExpr, BinExpr, BinaryOp, BindingIdent, BlockStmtOrExpr, CallExpr, Callee, CondExpr,
      Expr, ExprOrSpread, KeyValueProp, ObjectLit, Pat, Prop, PropOrSpread, UnaryExpr, UnaryOp,
    },
    utils::quote_ident,
  },
};

use crate::shared::{
  constants::{
    length_units::LENGTH_UNITS, messages::ILLEGAL_NAMESPACE_VALUE, time_units::get_time_units,
  },
  enums::data_structures::evaluate_result_value::EvaluateResultValue,
  structures::{
    evaluate_result::EvaluateResult,
    functions::FunctionMap,
    inline_style::InlineStyle,
    state_manager::StateManager,
    types::{DynamicFns, TInlineStyles},
  },
  utils::{
    ast::{
      convertors::{
        expr_to_str, ident_to_expression, null_to_expression, string_to_expression,
        transform_shorthand_to_key_values,
      },
      factories::{object_expression_factory, prop_or_spread_expression_factory},
    },
    common::{create_hash, normalize_expr},
    css::common::get_number_suffix,
    js::evaluate::{evaluate, evaluate_obj_key},
    validators::validate_dynamic_style_params,
  },
};

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
          PropOrSpread::Spread(_) => unimplemented!("Spread"),
          PropOrSpread::Prop(prop) => {
            let mut prop = prop.clone();

            transform_shorthand_to_key_values(&mut prop);

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

                let key = key_result.value.as_ref().unwrap();
                let key_expr = key.as_expr().unwrap();
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
                            return Box::new(EvaluateResult {
                              confident: eval_result.confident,
                              deopt: eval_result.deopt,
                              reason: eval_result.reason,
                              value: eval_result.value,
                              inline_styles: None,
                              fns: None,
                            });
                          }

                          let value = eval_result
                            .value
                            .as_ref()
                            .and_then(|value| value.as_expr())
                            .and_then(|expr| expr.as_object())
                            .expect("Value not an object");

                          let key = expr_to_str(key_expr, traversal_state, functions)
                            .expect("Key is not a string");

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
                      }
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
                      }
                    }
                  }
                  _ => {
                    let val = evaluate(value_path, traversal_state, functions);

                    if !val.confident {
                      return val;
                    }

                    let value_to_insert = match val.value.as_ref().unwrap() {
                      EvaluateResultValue::Expr(expr) => match expr {
                        Expr::Object(obj_expr) => obj_expr
                          .props
                          .iter()
                          .filter_map(|prop| prop.as_prop().and_then(|prop| prop.as_key_value()))
                          .cloned()
                          .collect::<Vec<_>>(),
                        _ => panic!("{}", ILLEGAL_NAMESPACE_VALUE),
                      },
                      _ => panic!("{}", ILLEGAL_NAMESPACE_VALUE),
                    };

                    result_value.insert(key_expr.clone(), value_to_insert);

                    continue;
                  }
                }
              }
              _ => {
                return evaluate(path, traversal_state, functions);
              }
            }
          }
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
    }
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
        unimplemented!();
      }
      PropOrSpread::Prop(prop) => {
        let mut prop = prop.clone();

        transform_shorthand_to_key_values(&mut prop);

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

            let key = key_result
              .value
              .as_ref()
              .and_then(|v| v.as_expr())
              .expect("Evaluated key value is not a string");

            let mut key_str =
              expr_to_str(key, traversal_state, functions).expect("Key is not a string");

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

                let new_prop = prop_or_spread_expression_factory(
                  &key_str,
                  result
                    .value
                    .and_then(|v| v.as_expr().cloned())
                    .expect("Value not an expression"),
                );
                obj.push(new_prop);

                if let Some(result_inline_styles) = result.inline_styles {
                  inline_styles.extend(result_inline_styles);
                }
              }
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

                  let new_prop = prop_or_spread_expression_factory(
                    &key_str,
                    string_to_expression(&format!("var({})", var_name)),
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
                    String::new()
                  };

                  let inline_style_expression = if !unit.is_empty() {
                    let val_ident = ident_to_expression("val");
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
                            right: Box::new(string_to_expression("number")),
                          })),
                          cons: Box::new(Expr::from(BinExpr {
                            span: DUMMY_SP,
                            op: BinaryOp::Add,
                            left: Box::new(val_ident.clone()),
                            right: Box::new(string_to_expression(&unit)),
                          })),
                          alt: Box::new(Expr::from(CondExpr {
                            span: DUMMY_SP,
                            test: Box::new(Expr::from(BinExpr {
                              span: DUMMY_SP,
                              op: BinaryOp::NotEq,
                              left: Box::new(val_ident.clone()),
                              right: Box::new(null_to_expression()),
                            })),
                            cons: Box::new(val_ident),
                            alt: Box::new(ident_to_expression("undefined")),
                          })),
                        })))),
                        is_async: false,
                        is_generator: false,
                        type_params: None,
                        return_type: None,
                        ctxt: SyntaxContext::empty(),
                      }))),
                      args: vec![ExprOrSpread {
                        spread: None,
                        expr: value_path.clone(),
                      }],
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
                        right: Box::new(null_to_expression()),
                      })),
                      cons: value_path.clone(),
                      alt: Box::new(ident_to_expression("undefined")),
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
                  let new_prop = prop_or_spread_expression_factory(
                    &key_str,
                    result
                      .value
                      .and_then(|v| v.as_expr().cloned())
                      .expect("Value not an expression"),
                  );
                  obj.push(new_prop);
                }
              }
            }
          }
          Prop::Method(_) => {
            return Box::new(EvaluateResult {
              confident: false,
              deopt: None,
              reason: None,
              value: None,
              inline_styles: None,
              fns: None,
            });
          }
          _ => {}
        }
      }
    }
  }

  Box::new(EvaluateResult {
    confident: true,
    deopt: None,
    reason: None,
    value: Some(EvaluateResultValue::Expr(object_expression_factory(obj))),
    inline_styles: Some(inline_styles),
    fns: None,
  })
}
