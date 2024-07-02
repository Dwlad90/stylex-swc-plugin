use indexmap::IndexMap;
use swc_core::{
  common::DUMMY_SP,
  ecma::ast::{
    ArrowExpr, BinExpr, BinaryOp, BindingIdent, BlockStmtOrExpr, CallExpr, Callee, CondExpr, Expr,
    ExprOrSpread, Ident, KeyValueProp, ObjectLit, Pat, Prop, PropOrSpread, UnaryExpr, UnaryOp,
  },
};

use crate::shared::{
  constants::{
    length_units::LENGTH_UNITS, messages::ILLEGAL_NAMESPACE_VALUE, time_units::get_time_units,
  },
  enums::data_structures::evaluate_result_value::EvaluateResultValue,
  structures::{
    evaluate_result::EvaluateResult, functions::FunctionMap, state_manager::StateManager,
    types::EvaluateResultFns,
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
      let mut result_value: IndexMap<Box<Expr>, Vec<KeyValueProp>> = IndexMap::new();

      let mut fns: EvaluateResultFns = IndexMap::new();

      for prop in &mut style_object.props {
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
                    value: None,
                    inline_styles: None,
                    fns: None,
                  });
                }

                let key = key_result.value.unwrap();

                let key_expr = key.as_expr().unwrap();

                let value_path = &mut key_value_prop.value;

                match value_path.as_mut() {
                  Expr::Arrow(fn_path) => {
                    let all_params = fn_path.params.clone();
                    validate_dynamic_style_params(&all_params);

                    let params = all_params
                      .into_iter()
                      .filter_map(|param| param.as_ident().cloned())
                      .collect::<Vec<BindingIdent>>();

                    if let BlockStmtOrExpr::Expr(expr) = fn_path.body.as_mut() {
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
                            value: eval_result.value,
                            inline_styles: None,
                            fns: None,
                          });
                        }

                        let value = eval_result
                          .value
                          .and_then(|value| value.as_expr().cloned())
                          .and_then(|expr| expr.as_object().cloned())
                          .expect("Value not an object");

                        let key = expr_to_str(key_expr, traversal_state, functions);

                        fns.insert(key, (params, eval_result.inline_styles.unwrap_or_default()));

                        result_value.insert(
                          Box::new(key_expr.clone()),
                          value
                            .props
                            .into_iter()
                            .filter_map(|prop| {
                              prop.as_prop().and_then(|prop| prop.as_key_value()).cloned()
                            })
                            .collect(),
                        );
                      } else {
                        return evaluate(path, traversal_state, functions);
                      }
                    } else {
                      unimplemented!("Block statement")
                    }
                  }
                  _ => {
                    let val = evaluate(value_path, traversal_state, functions);

                    if !val.confident {
                      return val;
                    }

                    let value_to_insert = match val.value.unwrap().as_ref() {
                      EvaluateResultValue::Expr(expr) => match expr.as_ref() {
                        Expr::Object(obj_expr) => {
                          let mut obj_expr_props: Vec<KeyValueProp> = vec![];

                          for prop in obj_expr.clone().props {
                            match prop {
                              PropOrSpread::Spread(_) => unimplemented!("Spread"),
                              PropOrSpread::Prop(mut prop) => {
                                transform_shorthand_to_key_values(&mut prop);

                                match prop.as_ref() {
                                  Prop::KeyValue(obj_expr_prop_kv) => {
                                    obj_expr_props.push(obj_expr_prop_kv.clone())
                                  }

                                  _ => unimplemented!(),
                                }
                              }
                            }
                          }

                          obj_expr_props
                        }
                        _ => panic!("{}", ILLEGAL_NAMESPACE_VALUE),
                      },
                      _ => panic!("{}", ILLEGAL_NAMESPACE_VALUE),
                    };

                    result_value.insert(Box::new(key_expr.clone()), value_to_insert);

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
        value: Some(Box::new(EvaluateResultValue::Map(result_value))),
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
  let mut key_path = key_path.unwrap_or_default();

  let mut inline_styles: IndexMap<String, Box<Expr>> = IndexMap::new();

  let mut obj: Vec<PropOrSpread> = vec![];

  for prop in path.props.clone() {
    match prop {
      PropOrSpread::Spread(spread) => {
        let result = evaluate(&spread.expr, traversal_state, functions);

        if !result.confident {
          return result;
        }

        unimplemented!();
      }
      PropOrSpread::Prop(mut prop) => {
        transform_shorthand_to_key_values(&mut prop);

        match prop.as_mut() {
          Prop::KeyValue(key_value) => {
            let key_result = evaluate_obj_key(key_value, traversal_state, functions);

            if !key_result.confident {
              return Box::new(EvaluateResult {
                confident: false,
                deopt: key_result.deopt,
                value: None,
                inline_styles: None,
                fns: None,
              });
            }

            let Some(key) = key_result.value else {
              panic!("Evaluated key value in not found");
            };

            let Some(key) = key.as_expr() else {
              panic!("Evaluated key value in not a string");
            };

            let mut key = expr_to_str(key, traversal_state, functions);

            if key.starts_with("var(") && key.ends_with(')') {
              key = key[4..key.len() - 1].to_string();
            }

            let value_path = &mut key_value.value;

            match normalize_expr(value_path.as_mut()) {
              Expr::Object(object) => {
                key_path.push(key.clone());

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
                    value: None,
                    inline_styles: None,
                    fns: None,
                  });
                }

                let new_prop = prop_or_spread_expression_factory(
                  key.as_str(),
                  result
                    .value
                    .and_then(|value| value.as_expr().cloned())
                    .expect("Value not an expression")
                    .clone(),
                );

                obj.push(new_prop);

                if let Some(result_inline_styles) = result.inline_styles {
                  inline_styles.extend(result_inline_styles);
                }
              }
              _ => {
                let result = evaluate(value_path, traversal_state, functions);

                if !result.confident {
                  let var_name = if !key_path.is_empty() {
                    key_path.push(key.clone());
                    format!("--{}", create_hash(key_path.join("_").as_str()))
                  } else {
                    format!("--{}", key)
                  };

                  let new_prop = prop_or_spread_expression_factory(
                    key.as_str(),
                    string_to_expression(format!("var({}, revert)", var_name).as_str()),
                  );

                  obj.push(new_prop);

                  let unit = if get_time_units().contains(key.as_str())
                    || LENGTH_UNITS.contains(key.as_str())
                  {
                    get_number_suffix(key.as_str())
                  } else {
                    String::new()
                  };

                  let result_expression = if !unit.is_empty() {
                    let val_ident = ident_to_expression("val");

                    Expr::from(CallExpr {
                      span: DUMMY_SP,
                      callee: Callee::Expr(Box::new(Expr::Arrow(ArrowExpr {
                        span: DUMMY_SP,
                        params: vec![Pat::Ident(BindingIdent::from(Ident::new(
                          "val".into(),
                          DUMMY_SP,
                        )))],
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
                            right: Box::new(string_to_expression(unit.as_str())),
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
                            alt: Box::new(string_to_expression("initial")),
                          })),
                        })))),
                        is_async: false,
                        is_generator: false,
                        type_params: None,
                        return_type: None,
                      }))),
                      args: vec![ExprOrSpread {
                        spread: None,
                        expr: value_path.clone(),
                      }],
                      type_args: None,
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
                      alt: Box::new(string_to_expression("initial")),
                    })
                  };

                  inline_styles.insert(var_name, Box::new(result_expression));
                } else {
                  let new_prop = prop_or_spread_expression_factory(
                    key.as_str(),
                    result
                      .value
                      .and_then(|value| value.as_expr().cloned())
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
    value: Some(Box::new(EvaluateResultValue::Expr(Box::new(
      object_expression_factory(obj),
    )))),
    inline_styles: Some(inline_styles),
    fns: None,
  })
}
