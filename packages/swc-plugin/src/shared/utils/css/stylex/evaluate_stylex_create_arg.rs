use indexmap::IndexMap;
use swc_core::{
  common::DUMMY_SP,
  ecma::{
    ast::{
      ArrowExpr, BinExpr, BinaryOp, BindingIdent, BlockStmtOrExpr, CallExpr, Callee, CondExpr,
      Expr, ExprOrSpread, Ident, KeyValueProp, Lit, Null, ObjectLit, Pat, Prop, PropOrSpread, Str,
      UnaryExpr, UnaryOp,
    },
    utils::ExprExt,
  },
};

use crate::shared::{
  constants::{self, length_units::LENGTH_UNITS, time_units::get_time_units},
  structures::{
    evaluate_result::{EvaluateResult, EvaluateResultValue},
    functions::FunctionMap,
    state_manager::StateManager,
    types::EvaluateResultFns,
  },
  utils::{
    common::{
      create_hash, expr_to_str, normalize_expr, prop_or_spread_expression_creator,
      string_to_expression, transform_shorthand_to_key_values,
    },
    css::utils::get_number_suffix,
    validators::validate_dynamic_style_params,
  },
};

use super::evaluate::{evaluate, evaluate_obj_key};

pub fn evaluate_stylex_create_arg(
  path: &Expr,
  traversal_state: &mut StateManager,
  functions: &FunctionMap,
) -> Box<EvaluateResult> {
  match path {
    Expr::Object(object) => {
      let mut style_object = object.clone();

      let mut result_value: IndexMap<Box<Expr>, Vec<KeyValueProp>> = IndexMap::new();

      let mut fns: EvaluateResultFns = IndexMap::new();

      for prop in &mut style_object.props {
        match prop {
          PropOrSpread::Spread(_) => todo!("Spread not implemented yet"),
          PropOrSpread::Prop(prop) => {
            let mut prop = prop.clone();

            transform_shorthand_to_key_values(&mut prop);

            match prop.as_ref() {
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

                let value_path = &key_value_prop.value;

                match value_path.as_ref() {
                  Expr::Arrow(fn_path) => {
                    let all_params = fn_path.params.clone();
                    validate_dynamic_style_params(&all_params);

                    let params = all_params
                      .into_iter()
                      .filter_map(|param| param.as_ident().cloned())
                      .collect::<Vec<BindingIdent>>();

                    let fn_body = fn_path.body.clone();
                    if let BlockStmtOrExpr::Expr(expr) = fn_body.as_ref() {
                      if let Expr::Object(fn_body_object) = normalize_expr(expr) {

                        let eval_result = evaluate_partial_object_recursively(
                          fn_body_object,
                          traversal_state,
                          functions,
                          Option::None,
                        );

                        if !eval_result.confident {
                          return Box::new(EvaluateResult {
                            confident: eval_result.confident,
                            deopt: eval_result.deopt,
                            value: eval_result.value,
                            inline_styles: Option::None,
                            fns: Option::None,
                          });
                        }


                        let value = eval_result
                          .value
                          .clone()
                          .and_then(|value| value.as_expr().cloned())
                          .and_then(|expr| expr.as_object().cloned())
                          .expect("Value not an object");


                        let key = expr_to_str(key_expr, traversal_state, functions);

                        fns.insert(key, (params, eval_result.inline_styles.unwrap_or_default()));

                        result_value.insert(
                          Box::new(key_expr.as_expr().clone()),
                          value
                            .props
                            .into_iter()
                            .filter_map(|prop| {
                              prop.as_prop().and_then(|prop| prop.as_key_value()).cloned()
                            })
                            .collect(),
                        );


                        // return EvaluateResult {
                        //   confident: true,
                        //   deopt: Option::None,
                        //   value: Option::Some(EvaluateResultValue::Map(result_value)),
                        //   inline_styles: Option::None,
                        //   fns: Option::Some(fns),
                        // };
                      } else {
                        return evaluate(path, traversal_state, functions);
                      }
                    } else {
                      todo!("BlockStmt not implemented yet")
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
                              PropOrSpread::Spread(_) => todo!(),
                              PropOrSpread::Prop(mut prop) => {
                                transform_shorthand_to_key_values(&mut prop);

                                match prop.as_ref() {
                                  Prop::KeyValue(obj_expr_prop_kv) => {
                                    obj_expr_props.push(obj_expr_prop_kv.clone())
                                  }

                                  _ => todo!(),
                                }
                              }
                            }
                          }

                          obj_expr_props
                        }
                        _ => panic!("{}", constants::messages::ILLEGAL_NAMESPACE_VALUE),
                      },
                      _ => panic!("{}", constants::messages::ILLEGAL_NAMESPACE_VALUE),
                    };

                    result_value.insert(Box::new(key_expr.as_expr().clone()), value_to_insert);


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
        deopt: Option::None,
        value: Option::Some(Box::new(EvaluateResultValue::Map(result_value))),
        inline_styles: Option::None,
        fns: if fns.is_empty() {
          Option::None
        } else {
          Option::Some(fns)
        },
      })
    }
    _ => {
      evaluate(path, traversal_state, functions)
    }
  }
}

fn evaluate_partial_object_recursively(
  path: &ObjectLit,
  traversal_state: &mut StateManager,
  functions: &FunctionMap,
  key_path: Option<Vec<String>>,
) -> Box<EvaluateResult> {
  let key_path = key_path.unwrap_or_default();

  let mut inline_styles: IndexMap<String, Box<Expr>> = IndexMap::new();

  let mut obj: Vec<PropOrSpread> = vec![];

  let props = path.props.clone();

  for prop in props.into_iter() {
    match prop {
      PropOrSpread::Spread(spread) => {
        let result = evaluate(&spread.expr, traversal_state, functions);

        if !result.confident {
          return result;
        }

        todo!("Check what to do with spread");
      }
      PropOrSpread::Prop(mut prop) => {
        transform_shorthand_to_key_values(&mut prop);

        match prop.as_ref() {
          Prop::KeyValue(key_value) => {

            let key_result = evaluate_obj_key(key_value, traversal_state, functions);

            if !key_result.confident {
              return Box::new(EvaluateResult {
                confident: false,
                deopt: key_result.deopt,
                value: Option::None,
                inline_styles: Option::None,
                fns: Option::None,
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

            let value_path = &key_value.value;

            match normalize_expr(value_path.as_ref()) {
              Expr::Object(object) => {
                let mut extended_key_path = key_path.clone();
                extended_key_path.push(key.clone());

                let result = evaluate_partial_object_recursively(
                  object,
                  traversal_state,
                  functions,
                  Option::Some(extended_key_path),
                );


                if !result.confident {
                  return Box::new(EvaluateResult {
                    confident: false,
                    deopt: result.deopt,
                    value: Option::None,
                    inline_styles: Option::None,
                    fns: Option::None,
                  });
                }

                let new_prop = prop_or_spread_expression_creator(
                  key.as_str(),
                  Box::new(
                    result
                      .value
                      .and_then(|value| value.as_expr().cloned())
                      .expect("Value not an expression")
                      .clone(),
                  ),
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
                    let mut keys = key_path.clone();
                    keys.push(key.clone());
                    format!("--{}", create_hash(keys.join("_").as_str()))
                  } else {
                    format!("--{}", key)
                  };

                  let new_prop = prop_or_spread_expression_creator(
                    key.as_str(),
                    Box::new(
                      string_to_expression(format!("var({}, revert)", var_name).as_str()).unwrap(),
                    ),
                  );

                  obj.push(new_prop);

                  let expression = value_path.as_ref().clone();

                  let unit = if get_time_units().contains(key.as_str())
                    || LENGTH_UNITS.contains(key.as_str())
                  {
                    get_number_suffix(key.as_str())
                  } else {
                    String::new()
                  };

                  let result_expression = if !unit.is_empty() {
                    Expr::Call(CallExpr {
                      span: DUMMY_SP,
                      callee: Callee::Expr(Box::new(Expr::Arrow(ArrowExpr {
                        span: DUMMY_SP,
                        params: vec![Pat::Ident(BindingIdent::from(Ident::new(
                          "val".into(),
                          DUMMY_SP,
                        )))],
                        body: Box::new(BlockStmtOrExpr::Expr(Box::new(Expr::Cond(CondExpr {
                          span: DUMMY_SP,
                          test: Box::new(Expr::Bin(BinExpr {
                            span: DUMMY_SP,
                            op: BinaryOp::EqEqEq,
                            left: Box::new(Expr::Unary(UnaryExpr {
                              span: DUMMY_SP,
                              op: UnaryOp::TypeOf,
                              arg: Box::new(Expr::Ident(Ident::new("val".into(), DUMMY_SP))),
                            })),
                            right: Box::new(Expr::Lit(Lit::Str(Str {
                              span: DUMMY_SP,
                              value: "number".into(),
                              raw: Option::None,
                            }))),
                          })),
                          cons: Box::new(Expr::Bin(BinExpr {
                            span: DUMMY_SP,
                            op: BinaryOp::Add,
                            left: Box::new(Expr::Ident(Ident::new("val".into(), DUMMY_SP))),
                            right: Box::new(Expr::Lit(Lit::Str(Str {
                              span: DUMMY_SP,
                              value: unit.into(), // replace with your unit
                              raw: Option::None,
                            }))),
                          })),
                          alt: Box::new(Expr::Cond(CondExpr {
                            span: DUMMY_SP,
                            test: Box::new(Expr::Bin(BinExpr {
                              span: DUMMY_SP,
                              op: BinaryOp::NotEq,
                              left: Box::new(Expr::Ident(Ident::new("val".into(), DUMMY_SP))),
                              right: Box::new(Expr::Lit(Lit::Null(Null { span: DUMMY_SP }))),
                            })),
                            cons: Box::new(Expr::Ident(Ident::new("val".into(), DUMMY_SP))),
                            alt: Box::new(Expr::Lit(Lit::Str(Str {
                              span: DUMMY_SP,
                              value: "initial".into(),
                              raw: Option::None,
                            }))),
                          })),
                        })))),
                        is_async: false,
                        is_generator: false,
                        type_params: None,
                        return_type: None,
                      }))),
                      args: vec![ExprOrSpread {
                        spread: None,
                        expr: Box::new(expression), // replace with your expression
                      }],
                      type_args: None,
                    })
                  } else {
                    Expr::Cond(CondExpr {
                      span: DUMMY_SP,
                      test: Box::new(Expr::Bin(BinExpr {
                        span: DUMMY_SP,
                        op: BinaryOp::NotEq,
                        left: Box::new(expression.clone()), // replace with your expression
                        right: Box::new(Expr::Lit(Lit::Null(Null { span: DUMMY_SP }))),
                      })),
                      cons: Box::new(expression), // replace with your expression
                      alt: Box::new(Expr::Lit(Lit::Str(Str {
                        span: DUMMY_SP,
                        value: "initial".into(),
                        raw: Option::None,
                      }))),
                    })
                  };

                  inline_styles.insert(var_name, Box::new(result_expression));
                } else {
                  let new_prop = prop_or_spread_expression_creator(
                    key.as_str(),
                    Box::new(
                      result
                        .value
                        .expect("Value not found")
                        .as_expr()
                        .expect("Value not an expression")
                        .clone(),
                    ),
                  );

                  obj.push(new_prop);
                }
              }
            }
          }
          Prop::Method(_) => {
            return Box::new(EvaluateResult {
              confident: false,
              deopt: Option::None,
              value: Option::None,
              inline_styles: Option::None,
              fns: Option::None,
            });
          }
          _ => {}
        }
      }
    }
  }

  Box::new(EvaluateResult {
    confident: true,
    deopt: Option::None,
    value: Option::Some(Box::new(EvaluateResultValue::Expr(Box::new(Expr::Object(
      ObjectLit {
        span: DUMMY_SP,
        props: obj,
      },
    ))))),
    inline_styles: Option::Some(inline_styles),
    fns: None,
  })
}
