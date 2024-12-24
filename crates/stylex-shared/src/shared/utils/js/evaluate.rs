use core::panic;
use std::{borrow::Borrow, rc::Rc};

use indexmap::IndexMap;
use log::warn;
use rustc_hash::{FxHashMap, FxHashSet};
use swc_core::{
  atoms::Atom,
  common::{EqIgnoreSpan, SyntaxContext, DUMMY_SP},
  ecma::{
    ast::{
      ArrayLit, BlockStmtOrExpr, CallExpr, Callee, ComputedPropName, Expr, ExprOrSpread, Ident,
      KeyValueProp, Lit, MemberProp, ModuleExportName, Number, ObjectLit, Pat, Prop, PropName,
      PropOrSpread, TplElement, UnaryOp, VarDeclarator,
    },
    utils::{drop_span, ident::IdentLike, quote_ident, ExprExt},
  },
};

use crate::shared::{
  constants::{
    common::{INVALID_METHODS, VALID_CALLEES},
    messages::{BUILT_IN_FUNCTION, ILLEGAL_PROP_ARRAY_VALUE},
  },
  enums::{
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
    state_manager::{add_import_expression, SeenValueWithVarDeclCount, StateManager},
    theme_ref::ThemeRef,
    types::{FunctionMapIdentifiers, FunctionMapMemberExpression},
  },
  utils::{
    ast::{
      convertors::{
        big_int_to_expression, binary_expr_to_num, bool_to_expression, expr_to_bool, expr_to_num,
        expr_to_str, number_to_expression, string_to_expression, transform_shorthand_to_key_values,
      },
      factories::{array_expression_factory, lit_str_factory, object_expression_factory},
    },
    common::{
      char_code_at, deep_merge_props, get_hash_map_difference, get_hash_map_value_difference,
      get_import_by_ident, get_key_str, get_string_val_from_lit, get_var_decl_by_ident,
      get_var_decl_from, normalize_expr, reduce_ident_count, reduce_member_expression_count,
      remove_duplicates, sort_numbers_factory, stable_hash, sum_hash_map_values,
    },
    js::native_functions::{evaluate_filter, evaluate_join, evaluate_map},
  },
};

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
          value: None,
          inline_styles: None,
          fns: None,
        };
      }
    }
    PropName::Str(strng) => string_to_expression(&strng.value),
    PropName::Num(num) => number_to_expression(num.value),
    PropName::BigInt(big_int) => big_int_to_expression(big_int.clone()),
  };

  let key_expr = string_to_expression(&expr_to_str(&key, state, functions));

  EvaluateResult {
    confident: true,
    deopt: None,
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
    inline_styles: None,
    fns: None,
  })
}

fn deopt(path: &Expr, state: &mut EvaluationState) -> Option<EvaluateResultValue> {
  if state.confident {
    state.confident = false;
    state.deopt_path = Some(path.clone());
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

  let path = normalize_expr(path);

  let result: Option<EvaluateResultValue> = match path {
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
              |functions: FunctionMapIdentifiers,
               ident_params: Vec<Atom>,
               body_expr: Box<Expr>,
               traversal_state: StateManager| {
                move |cb_args: Vec<Option<EvaluateResultValue>>| {
                  let mut functions = functions.clone();

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
                      functions.insert(
                        ident.clone(),
                        Box::new(FunctionConfigType::Regular(function.clone())),
                      );

                      member_expressions.insert(
                        ImportSources::Regular("entry".to_string()),
                        Box::new(functions.clone()),
                      );
                    }
                  });

                  let mut local_state = traversal_state.clone();

                  let result = evaluate(
                    &body_expr,
                    &mut local_state,
                    &FunctionMap {
                      identifiers: functions,
                      member_expressions,
                    },
                  );

                  let value = result.value;

                  let expr = match value {
                    Some(res) => res
                      .as_expr()
                      .expect("Evaluation result must be an expression")
                      .clone(),
                    None => unreachable!("Evaluation result must be non optional"),
                  };

                  expr
                }
              };

            let functions = state.functions.identifiers.clone();

            let arrow_closure = Rc::new(arrow_closure_fabric(
              functions,
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
          FunctionConfigType::Regular(func) => {
            if let FunctionType::Mapper(func) = &func.fn_ptr {
              return Some(EvaluateResultValue::Expr(func()));
            } else {
              panic!("Function not found");
            }
          }
          FunctionConfigType::Map(func_map) => {
            return Some(EvaluateResultValue::FunctionConfigMap(func_map.clone()));
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
    #[allow(dead_code)]
    Expr::TaggedTpl(_tagged_tpl) => {
      unimplemented!("TaggedTpl");
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
        _ => panic!("Test of condition must be an expression"),
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
    Expr::Paren(_) => {
      panic!("Paren must be normalized before evaluation")
    }
    Expr::Member(member) => {
      let parent_is_call_expr = traversal_state
        .all_call_expressions
        .iter()
        .any(|call_expr| {
          if let Some(callee) = call_expr.callee.as_expr() {
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

      if let Some(object) = evaluated_value {
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
            return deopt(path, state);
          }
        };

        match object {
          EvaluateResultValue::Expr(expr) => match &expr {
            Expr::Array(ArrayLit { elems, .. }) => {
              let eval_res = property.expect("Property not found");

              let expr = match eval_res {
                EvaluateResultValue::Expr(expr) => expr,
                _ => panic!("Property not found: {:?}", expr.get_type()),
              };

              let value = match expr {
                Expr::Lit(Lit::Num(Number { value, .. })) => value as usize,
                _ => panic!("Member not found: {:?}", expr.get_type()),
              };

              let property = elems.get(value)?;

              let expr = property.as_ref().expect("Member not found").expr.clone();

              Some(EvaluateResultValue::Expr(*expr))
            }
            Expr::Object(ObjectLit { props, .. }) => {
              let eval_res = property.expect("Property not found");

              let ident = match eval_res {
                EvaluateResultValue::Expr(ident) => ident,
                _ => panic!("Property not found: {:?}", expr.get_type()),
              };

              let ident = &mut ident.to_owned();
              let normalized_ident = normalize_expr(ident);

              let ident_string_name = match normalized_ident {
                Expr::Ident(ident) => ident.sym.to_string(),
                Expr::Lit(lit) => get_string_val_from_lit(lit).unwrap_or_else(|| {
                  panic!(
                    "Property must be convertable to string: {:?}",
                    normalized_ident.get_type()
                  )
                }),
                _ => unimplemented!("Member property: {:?}", normalized_ident.get_type()),
              };

              let property = props.iter().find(|prop| match prop {
                PropOrSpread::Spread(_) => unimplemented!("Spread"),
                PropOrSpread::Prop(prop) => {
                  let mut prop = prop.clone();

                  transform_shorthand_to_key_values(&mut prop);

                  match prop.as_ref() {
                    Prop::KeyValue(key_value) => {
                      let key = get_key_str(key_value);

                      ident_string_name == key
                    }
                    _ => unimplemented!("Prop"),
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
                panic!("Member not found: {:?}", expr.get_type());
              }
            }
            _ => unimplemented!("Expression: {:?}", expr.get_type()),
          },
          EvaluateResultValue::FunctionConfigMap(fc_map) => {
            let key = match property {
              Some(property) => match property {
                EvaluateResultValue::Expr(expr) => match expr {
                  Expr::Ident(ident) => Box::new(ident.clone()),
                  _ => panic!("Member not found: {:?}", expr.get_type()),
                },
                _ => unimplemented!(),
              },
              None => panic!("Member not found"),
            };

            let fc = fc_map.get(&key.sym).unwrap();

            return Some(EvaluateResultValue::FunctionConfig(fc.clone()));
          }
          EvaluateResultValue::ThemeRef(mut theme_ref) => {
            let key = match property {
              Some(property) => match property {
                EvaluateResultValue::Expr(expr) => match expr {
                  Expr::Ident(Ident { sym, .. }) => sym.to_string(),
                  Expr::Lit(lit) => {
                    get_string_val_from_lit(&lit).expect("Property must be a string")
                  }
                  _ => panic!("Member not found: {:?}", expr.get_type()),
                },
                _ => unimplemented!(),
              },
              None => panic!("Member not found"),
            };

            let value = theme_ref.get(&key, &traversal_state);

            return Some(EvaluateResultValue::Expr(string_to_expression(
              value.as_str(),
            )));
          }
          _ => unimplemented!("EvaluateResultValue"),
        }
      } else {
        None
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
        _ => panic!("Unary argument is not an expression"),
      };

      match unary.op {
        UnaryOp::Bang => {
          let value = expr_to_bool(&arg, traversal_state, fns);

          Some(EvaluateResultValue::Expr(bool_to_expression(!value)))
        }
        UnaryOp::Plus => {
          let value = expr_to_num(&arg, state, traversal_state, fns)
            .unwrap_or_else(|error| panic!("{}", error));

          Some(EvaluateResultValue::Expr(number_to_expression(value)))
        }
        UnaryOp::Minus => {
          let value = expr_to_num(&arg, state, traversal_state, fns)
            .unwrap_or_else(|error| panic!("{}", error));

          Some(EvaluateResultValue::Expr(number_to_expression(
            value * -1.0,
          )))
        }
        UnaryOp::Tilde => {
          let value = expr_to_num(&arg, state, traversal_state, fns)
            .unwrap_or_else(|error| panic!("{}", error));

          Some(EvaluateResultValue::Expr(number_to_expression(
            (!(value as i64)) as f64,
          )))
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
            _ => unimplemented!("Unary TypeOf: {:?}", arg.get_type()),
          };

          Some(EvaluateResultValue::Expr(string_to_expression(arg_type)))
        }
        UnaryOp::Void => Some(EvaluateResultValue::Expr(Expr::Ident(quote_ident!(
          SyntaxContext::empty(),
          "undefined"
        )))),
        _ => deopt(&Expr::from(unary.clone()), state),
      }
    }
    Expr::Array(arr_path) => {
      let mut arr: Vec<Option<EvaluateResultValue>> = Vec::with_capacity(arr_path.elems.len());

      for elem in arr_path.elems.iter().flatten() {
        let elem_value = evaluate(&elem.expr, traversal_state, &state.functions);

        if elem_value.confident {
          arr.push(elem_value.value);
        } else {
          return None;
        }
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
              return deopt(path, state);
            }

            let new_props = spread_expression
              .and_then(|spread| spread.as_expr().cloned())
              .and_then(|expr| expr.as_object().cloned())
              .expect("Spread must be an object");

            let merged_object = deep_merge_props(props, new_props.props);

            props = merged_object;

            continue;
          }
          PropOrSpread::Prop(prop) => {
            if prop.is_method() {
              return deopt(path, state);
            }

            let mut prop = prop.clone();

            transform_shorthand_to_key_values(&mut prop);

            match prop.as_ref() {
              Prop::KeyValue(path_key_value) => {
                let key = match &path_key_value.key {
                  PropName::Ident(ident) => Some(ident.sym.to_string()),
                  PropName::Str(strng) => Some(strng.value.to_string()),
                  PropName::Num(num) => Some(num.value.to_string()),
                  PropName::Computed(computed) => {
                    let evaluated_result =
                      evaluate(&computed.expr, traversal_state, &state.functions);

                    if !evaluated_result.confident {
                      if let Some(deopt_val) = evaluated_result.deopt {
                        deopt(&deopt_val, state);
                      }

                      return None;
                    }

                    Some(expr_to_str(
                      evaluated_result
                        .value
                        .as_ref()
                        .and_then(|value| value.as_expr())
                        .expect("Property must be an expression"),
                      traversal_state,
                      &state.functions,
                    ))
                  }
                  PropName::BigInt(big_int) => Some(big_int.value.to_string()),
                };

                let value = evaluate(&path_key_value.value, traversal_state, &state.functions);

                if !value.confident {
                  if let Some(deopt_val) = value.deopt {
                    deopt(&deopt_val, state);
                  }

                  return None;
                }

                let value = value.value.unwrap_or_else(|| {
                  panic!(
                    "Value of key '{}' must be present, but got {:?}",
                    key.clone().unwrap_or_else(|| "Unknown".to_string()),
                    path_key_value.value.get_type()
                  )
                });

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
                          Expr::Lit(lit) => Expr::Lit(lit),
                          _ => panic!("{}", ILLEGAL_PROP_ARRAY_VALUE),
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
                  EvaluateResultValue::Callback(_) => None,
                  _ => panic!("Property value must be an expression"),
                };

                if let Some(value) = value {
                  props.push(PropOrSpread::Prop(Box::new(Prop::from(KeyValueProp {
                    key: PropName::Ident(quote_ident!(key.unwrap())),
                    value: Box::new(value),
                  }))));
                }
              }
              _ => unimplemented!(),
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
    Expr::Bin(bin) => {
      match binary_expr_to_num(bin, state, traversal_state, fns)
        .unwrap_or_else(|error| panic!("{}", error))
      {
        BinaryExprType::Number(result) => {
          return Some(EvaluateResultValue::Expr(number_to_expression(result)))
        }
        BinaryExprType::String(strng) => {
          return Some(EvaluateResultValue::Expr(string_to_expression(&strng)))
        }
        BinaryExprType::Null => None,
      }
    }
    Expr::Call(call) => {
      let mut context: Option<Vec<Option<EvaluateResultValue>>> = None;
      let mut func: Option<Box<FunctionConfig>> = None;

      if let Callee::Expr(callee_expr) = &call.callee {
        if get_binding(callee_expr, traversal_state).is_none() && is_valid_callee(callee_expr) {
          panic!("{}", BUILT_IN_FUNCTION)
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
              FunctionConfigType::Map(_) => unimplemented!("FunctionConfigType::Map"),
              FunctionConfigType::Regular(fc) => func = Some(Box::new(fc.clone())),
            }
          }
        }

        if let Expr::Member(member) = callee_expr.as_ref() {
          let object = &member.obj;
          let property = &member.prop;

          if object.is_ident() {
            let obj_ident = object.as_ident().unwrap();

            if property.is_ident() {
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
                      unimplemented!("Spread")
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
                          unimplemented!("Spread")
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
                      unimplemented!("Spread")
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

                        let mut entries_result = IndexMap::new();

                        match cached_arg.expect("Object.entries requires an argument") {
                          EvaluateResultValue::Expr(expr) => {
                            let array = expr
                              .as_array()
                              .cloned()
                              .expect("Object.entries requires an object");

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
                                .and_then(|e| e.expr.as_lit())
                                .expect("Value must be a literal");

                              entries_result.insert(key.clone(), value.clone());
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
                                .and_then(|expr| expr.as_lit().cloned())
                                .expect("Value must be a literal");

                              entries_result.insert(key.clone(), value.clone());
                            }
                          }
                          _ => {
                            panic!("Object.entries requires an object")
                          }
                        };

                        context = Some(vec![Some(EvaluateResultValue::Entries(entries_result))]);
                      }
                      "keys" => {
                        func = Some(Box::new(FunctionConfig {
                          fn_ptr: FunctionType::Callback(Box::new(CallbackType::Object(
                            ObjectJS::Keys,
                          ))),
                          takes_path: false,
                        }));

                        let object = cached_arg
                          .and_then(|arg| arg.as_expr().cloned())
                          .and_then(|expr| expr.as_object().cloned())
                          .expect("Object.entries requires an object");

                        let mut keys = vec![];

                        for prop in &object.props {
                          let expr = prop.as_prop().cloned().expect("Spread");

                          let key_values = expr
                            .as_key_value()
                            .expect("Object.entries requires an object");

                          let key = get_key_str(key_values);

                          keys.push(Some(ExprOrSpread {
                            spread: None,
                            expr: Box::new(string_to_expression(key.as_str())),
                          }));
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

                        let object = cached_arg
                          .and_then(|arg| arg.as_expr().cloned())
                          .and_then(|expr| expr.as_object().cloned())
                          .expect("Object.entries requires an object");

                        let mut values = vec![];

                        for prop in &object.props {
                          let expr = prop.as_prop().cloned().expect("Spread");

                          let key_values = expr
                            .as_key_value()
                            .expect("Object.entries requires an object");

                          let value = key_values
                            .value
                            .as_lit()
                            .expect("Object value should be a literal");

                          values.push(Some(ExprOrSpread {
                            spread: None,
                            expr: Box::new(Expr::from(value.clone())),
                          }));
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

                        let object = cached_arg
                          .and_then(|arg| arg.as_expr().cloned())
                          .and_then(|expr| expr.as_object().cloned())
                          .expect("Object.entries requires an object");

                        let mut entries: IndexMap<Lit, Lit> = IndexMap::new();

                        for prop in &object.props {
                          let expr = prop.as_prop().map(|prop| *prop.clone()).expect("Spread");

                          let key_values = expr
                            .as_key_value()
                            .expect("Object.entries requires an object");

                          let value = key_values
                            .value
                            .as_lit()
                            .expect("Object value should be a literal");

                          let key = get_key_str(key_values);

                          entries.insert(lit_str_factory(key.as_str()), value.clone());
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
                {
                  if let Some(member_expr_fn) = member_expr.get(&prop_id.0) {
                    match member_expr_fn.as_ref() {
                      FunctionConfigType::Regular(fc) => {
                        func = Some(Box::new(fc.clone()));
                      }
                      FunctionConfigType::Map(_) => unimplemented!("FunctionConfigType::Map"),
                    }
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
              {
                if member_expr.contains_key(prop_id) {
                  unimplemented!("Check what's happening here");

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
          }

          if object.is_lit() {
            let obj_lit = object.as_lit().unwrap();

            if property.is_ident() {
              if let Lit::Bool(_) = obj_lit {
                unimplemented!("{}", BUILT_IN_FUNCTION)
              }
            }
          }

          if func.is_none() {
            let parsed_obj = evaluate(object, traversal_state, &state.functions);

            if parsed_obj.confident {
              if property.is_ident() {
                let prop_ident = property.as_ident().expect("Property is not an identifier");
                let prop_name = prop_ident.sym.to_string();

                let value = parsed_obj.value.expect("Parsed object has no value");

                match value {
                  EvaluateResultValue::Map(map) => {
                    let result_fn = map.get(&Expr::from(prop_ident.clone()));

                    func = match result_fn {
                      Some(_) => unimplemented!("EvaluateResultValue::Map"),
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
                        _ => unimplemented!("Array method '{}' implemented yet", prop_name),
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
                          _ => unimplemented!("Method '{}' implemented yet", prop_name),
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
                          _ => unimplemented!("Method '{}' implemented yet", prop_name),
                        })),
                        takes_path: false,
                      }));

                      context = Some(vec![Some(EvaluateResultValue::Expr(expr.clone()))]);
                    }
                    _ => unimplemented!("Expression evaluation not implemented"),
                  },
                  EvaluateResultValue::FunctionConfig(fc) => match fc.fn_ptr {
                    FunctionType::StylexFnsFactory(sxfns) => {
                      let fc = sxfns(prop_name);

                      func = Some(Box::new(FunctionConfig {
                        fn_ptr: FunctionType::StylexTypeFn(fc),
                        takes_path: false,
                      }));

                      context = Some(vec![Some(
                        EvaluateResultValue::Entries(IndexMap::default()),
                      )]);
                    }
                    _ => unimplemented!(),
                  },
                  _ => {
                    panic!("Evaluation result")
                  }
                }
              } else if let Some(prop_id) = is_id_prop(property) {
                let value = parsed_obj.value.unwrap();
                let map = value.as_map().unwrap();

                let result_fn = map.get(&string_to_expression(prop_id.as_str()));

                func = match result_fn {
                  Some(_) => unimplemented!(),
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
              let func_result = (func)(args.iter().map(|arg| (*arg).clone()).collect());
              return Some(EvaluateResultValue::Expr(func_result));
            }
            FunctionType::StylexExprFn(func) => {
              let func_result = (func)((**args.first().unwrap()).clone(), traversal_state);

              return Some(EvaluateResultValue::Expr(func_result));
            }
            FunctionType::StylexTypeFn(_) => panic!("StylexTypeFn"),
            FunctionType::StylexFnsFactory(_) => panic!("StylexFnsFactory"),
            FunctionType::Callback(_) => panic!("Arrow function"),
            FunctionType::Mapper(_) => panic!("Mapper"),
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

                    fn_args.insert(
                      key,
                      ValueWithDefault::String(get_string_val_from_lit(value).unwrap()),
                    );
                  }
                }
                Expr::Lit(lit) => {
                  fn_args.insert(
                    "default".to_string(),
                    ValueWithDefault::String(get_string_val_from_lit(lit).unwrap()),
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
                    panic!("Object.entries requires an argument")
                  };

                  let EvaluateResultValue::Entries(entries) = eval_result else {
                    panic!("Object.entries requires an argument")
                  };

                  let mut entry_elems: Vec<Option<ExprOrSpread>> = vec![];

                  for (key, value) in entries {
                    let key: ExprOrSpread = ExprOrSpread {
                      spread: None,
                      expr: Box::new(Expr::from(key.clone())),
                    };

                    let value: ExprOrSpread = ExprOrSpread {
                      spread: None,
                      expr: Box::new(Expr::from(value.clone())),
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
                    panic!("Object.keys requires an argument")
                  };

                  return Some(EvaluateResultValue::Expr(keys.clone()));
                }
                CallbackType::Object(ObjectJS::Values) => {
                  let Some(Some(EvaluateResultValue::Expr(values))) = context.first() else {
                    panic!("Object.keys requires an argument")
                  };

                  return Some(EvaluateResultValue::Expr(values.clone()));
                }
                CallbackType::Object(ObjectJS::FromEntries) => {
                  let Some(Some(EvaluateResultValue::Entries(entries))) = context.first() else {
                    panic!("Object.fromEntries requires an argument")
                  };

                  let mut entry_elems = vec![];

                  for (key, value) in entries {
                    let ident_name = if let Lit::Str(lit_str) = key {
                      quote_ident!(lit_str.value.as_ref())
                    } else {
                      panic!(
                        "Expected a string literal: {:?}",
                        Expr::from(key.clone()).get_type()
                      )
                    };

                    let prop = PropOrSpread::Prop(Box::new(Prop::from(KeyValueProp {
                      key: PropName::Ident(ident_name),
                      value: Box::new(Expr::from(value.clone())),
                    })));

                    entry_elems.push(prop);
                  }

                  return Some(EvaluateResultValue::Expr(object_expression_factory(
                    entry_elems,
                  )));
                }
                CallbackType::Math(MathJS::Pow) => {
                  let Some(Some(EvaluateResultValue::Vec(args))) = context.first() else {
                    panic!("Math.pow requires an argument")
                  };

                  let num_args = args
                    .iter()
                    .flatten()
                    .map(|arg| {
                      arg
                        .as_expr()
                        .map(|expr| {
                          expr_to_num(expr, state, traversal_state, fns)
                            .unwrap_or_else(|error| panic!("{}", error))
                        })
                        .expect("All arguments must be a number")
                    })
                    .collect::<Vec<f64>>();

                  let result = num_args.first().unwrap().powf(*num_args.get(1).unwrap());

                  return Some(EvaluateResultValue::Expr(number_to_expression(result)));
                }
                CallbackType::Math(MathJS::Round | MathJS::Floor | MathJS::Ceil) => {
                  let Some(Some(EvaluateResultValue::Expr(expr))) = context.first() else {
                    panic!("Math.(round | ceil | floor) requires an argument")
                  };

                  let num = expr_to_num(expr, state, traversal_state, fns)
                    .unwrap_or_else(|error| panic!("{}", error));

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
                    panic!("Math.(min | max) requires an argument")
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
                CallbackType::String(StringJS::Concat) => {
                  let Some(Some(EvaluateResultValue::Expr(base_str))) = context.first() else {
                    panic!("String concat requires an argument")
                  };

                  let args = evaluate_func_call_args(call, state, traversal_state, fns);

                  let str_args = args
                    .iter()
                    .map(|arg| {
                      arg
                        .as_expr()
                        .map(|expr| expr_to_str(expr, traversal_state, fns))
                        .expect("All arguments must be a string")
                    })
                    .collect::<Vec<String>>()
                    .join("");

                  let base_str = expr_to_str(base_str, traversal_state, fns);

                  return Some(EvaluateResultValue::Expr(string_to_expression(
                    format!("{}{}", base_str, str_args).as_str(),
                  )));
                }
                CallbackType::String(StringJS::CharCodeAt) => {
                  let Some(Some(EvaluateResultValue::Expr(base_str))) = context.first() else {
                    panic!("String concat requires an argument")
                  };

                  let base_str = expr_to_str(base_str, traversal_state, fns);

                  let args = evaluate_func_call_args(call, state, traversal_state, fns);

                  let num_args = args
                    .iter()
                    .map(|arg| {
                      arg
                        .as_expr()
                        .map(|expr| {
                          expr_to_num(expr, state, traversal_state, fns)
                            .unwrap_or_else(|error| panic!("{}", error))
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
              }
            }
            _ => panic!("Function type"),
          }
        }
      }

      return deopt(path, state);
    }
    _ => {
      warn!("Unsupported type of expression: {:?}", path.get_type());

      return deopt(path, state);
    }
  };

  if result.is_none() && path.is_ident() {
    let ident = path.as_ident().expect("Identifier not found");

    if let Some(binding) = get_var_decl_by_ident(
      ident,
      traversal_state,
      &state.functions,
      VarDeclAction::Reduce,
    ) {
      if (*path).eq(&Expr::Ident(binding.name.as_ident().unwrap().id.clone())) {
        unimplemented!("Binding")
      }

      if let Some(init) = binding.init.as_ref() {
        return evaluate_cached(&init.clone(), state, traversal_state, fns);
      }
    }

    let name = ident.sym.to_string();

    if name == "undefined" || name == "infinity" || name == "NaN" {
      return Some(EvaluateResultValue::Expr(Expr::from(ident.clone())));
    }

    if let Some(import_path) = get_import_by_ident(ident, traversal_state).and_then(|import_decl| {
      if import_decl
        .specifiers
        .iter()
        .any(|import| import.is_named())
      {
        Some(import_decl)
      } else {
        None
      }
    }) {
      let import_specifier = import_path
        .specifiers
        .iter()
        .find_map(|import| {
          if let Some(name_import) = import.as_named() {
            if ident.sym == name_import.local.sym {
              return Some(name_import);
            }
          }
          None
        })
        .expect("Import specifier not found");

      let imported = import_specifier
        .imported
        .clone()
        .unwrap_or_else(|| ModuleExportName::Ident(import_specifier.local.clone()));

      let abs_path =
        traversal_state.import_path_resolver(&import_path.src.value, &mut FxHashMap::default());

      let imported_name = match imported {
        ModuleExportName::Ident(ident) => ident.sym.to_string(),
        ModuleExportName::Str(strng) => strng.value.to_string(),
      };

      let return_value = match abs_path {
        ImportPathResolution::Tuple(ImportPathResolutionType::ThemeNameRef, value) => {
          evaluate_theme_ref(&value, imported_name, traversal_state)
        }
        _ => return deopt(path, state),
      };

      if state.confident {
        let import_path_src = import_path.src.value.to_string();

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
  }

  if result.is_none() {
    return deopt(path, state);
  }

  result
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
          vec![expr_to_num(expr, state, traversal_state, fns)
            .unwrap_or_else(|error| panic!("{}", error))]
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

fn get_method_name(prop: &MemberProp) -> &str {
  match prop {
    MemberProp::Ident(ident_prop) => &ident_prop.sym,
    _ => panic!("Method is not an identifier"),
  }
}

fn is_id_prop(prop: &MemberProp) -> Option<&Atom> {
  if let MemberProp::Computed(comp_prop) = prop {
    if let Expr::Lit(Lit::Str(strng)) = comp_prop.expr.as_ref() {
      return Some(&strng.value);
    }
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
    _ => panic!("The expression is not a template"),
  };

  for (i, elem) in quasis.iter().enumerate() {
    if !state.confident {
      return None;
    }

    strng.push_str(if raw {
      &elem.raw
    } else {
      elem.cooked.as_ref().expect("Cooked should be some")
    });

    if let Some(expr) = exprs.get(i) {
      if let Some(evaluated_expr) = evaluate_cached(expr, state, traversal_state, fns) {
        if let Some(lit_str) = evaluated_expr
          .as_expr()
          .and_then(|expr| expr.as_lit())
          .and_then(get_string_val_from_lit)
        {
          strng.push_str(&lit_str);
        }
      }
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
      deopt(path, state)
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
