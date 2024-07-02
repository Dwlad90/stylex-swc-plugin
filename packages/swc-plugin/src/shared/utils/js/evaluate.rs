use core::panic;
use std::{
  collections::{HashMap, HashSet},
  rc::Rc,
};

use indexmap::IndexMap;
use swc_core::{
  common::{EqIgnoreSpan, DUMMY_SP},
  ecma::{
    ast::{
      ArrayLit, BlockStmtOrExpr, Callee, ComputedPropName, Expr, ExprOrSpread, Id, Ident,
      KeyValueProp, Lit, MemberProp, ModuleExportName, Number, ObjectLit, Prop, PropName,
      PropOrSpread, TplElement, VarDeclarator,
    },
    utils::{drop_span, ident::IdentLike, ExprExt},
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
    misc::VarDeclAction,
  },
  structures::{
    evaluate_result::EvaluateResult,
    functions::{CallbackType, FunctionConfig, FunctionConfigType, FunctionMap, FunctionType},
    named_import_source::ImportSources,
    seen_value::SeenValue,
    state::EvaluationState,
    state_manager::{add_import_expression, StateManager},
    theme_ref::ThemeRef,
    types::{FunctionMapIdentifiers, FunctionMapMemberExpression},
  },
  utils::{
    ast::{
      convertors::{
        big_int_to_expression, binary_expr_to_num, expr_to_num, expr_to_str, number_to_expression,
        string_to_expression, transform_shorthand_to_key_values,
      },
      factories::{
        array_expression_factory, ident_factory, lit_str_factory, object_expression_factory,
      },
    },
    common::{
      char_code_at, deep_merge_props, get_import_by_ident, get_key_str, get_string_val_from_lit,
      get_var_decl_by_ident, get_var_decl_from, normalize_expr, remove_duplicates,
      sort_numbers_factory,
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

  let key: Expr;

  match key_path {
    PropName::Ident(ident) => {
      key = string_to_expression(ident.sym.to_string().as_str());
    }
    PropName::Computed(computed) => {
      let computed_path = &computed.expr;
      let computed_result = evaluate(computed_path, state, functions);
      if computed_result.confident {
        key = match computed_result.value.as_ref() {
          Some(eval_result) => {
            if let EvaluateResultValue::Expr(value) = eval_result.as_ref() {
              *value.clone()
            } else {
              panic!("Expected expression value");
            }
          }
          _ => panic!("Expected string value"),
        };
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
    PropName::Str(str) => key = string_to_expression(&str.value),
    PropName::Num(num) => key = number_to_expression(num.value),
    PropName::BigInt(big_int) => {
      key = big_int_to_expression(big_int.clone());
    }
  }

  let key_expr = string_to_expression(expr_to_str(&key, state, functions).as_str());

  EvaluateResult {
    confident: true,
    deopt: None,
    value: Some(Box::new(EvaluateResultValue::Expr(Box::new(key_expr)))),
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
    added_imports: HashSet::new(),
    functions: fns.clone(),
    traversal_state: traversal_state.clone(),
  });

  let mut value = evaluate_cached(path, &mut state, fns);

  if !state.confident {
    value = None;
  }

  *traversal_state = state.traversal_state;

  Box::new(EvaluateResult {
    confident: state.confident,
    value,
    deopt: state.deopt_path,
    inline_styles: None,
    fns: None,
  })
}

fn deopt(path: &Expr, state: &mut EvaluationState) -> Option<Box<EvaluateResultValue>> {
  if state.confident {
    state.confident = false;
    state.deopt_path = Some(Box::new(path.clone()));
  }

  None
}

fn _evaluate(
  path: &mut Expr,
  state: &mut EvaluationState,
  fns: &FunctionMap,
) -> Option<Box<EvaluateResultValue>> {
  if !state.confident {
    return None;
  }

  let path = normalize_expr(path);

  let result: Option<Box<EvaluateResultValue>> = match path {
    Expr::Arrow(arrow) => {
      let body = arrow.body.clone();
      let params = arrow.params.clone();

      let ident_params = params
        .clone()
        .into_iter()
        .filter_map(|param| {
          if param.is_ident() {
            Some(param.as_ident().unwrap().to_id())
          } else {
            None
          }
        })
        .collect::<Vec<Id>>();

      match body.as_ref() {
        BlockStmtOrExpr::Expr(body_expr) => {
          if ident_params.len() == params.len() {
            let arrow_closure_fabric =
              |functions: FunctionMapIdentifiers,
               ident_params: Vec<Id>,
               body_expr: Box<Expr>,
               traversal_state: StateManager| {
                move |cb_args: Vec<Option<EvaluateResultValue>>| {
                  let mut functions = functions.clone();

                  let mut member_expressions: FunctionMapMemberExpression = HashMap::new();

                  ident_params.iter().enumerate().for_each(|(index, ident)| {
                    if let Some(arg) = cb_args.get(index) {
                      let expr = arg
                        .clone()
                        .and_then(|arg| arg.as_expr().cloned())
                        .expect("Argument is not an expression");

                      let cl = |arg: Expr| move || arg.clone();

                      let result = (cl)(expr);
                      let function = FunctionConfig {
                        fn_ptr: FunctionType::Mapper(Rc::new(result)),
                        takes_path: false,
                      };
                      functions.insert(
                        Box::new(ident.clone()),
                        Box::new(FunctionConfigType::Regular(function.clone())),
                      );

                      member_expressions.insert(
                        Box::new(ImportSources::Regular("entry".to_string())),
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
              state.traversal_state.clone(),
            ));

            return Some(Box::new(EvaluateResultValue::Callback(arrow_closure)));
          }

          None
        }
        BlockStmtOrExpr::BlockStmt(_) => None,
      }
    }
    Expr::Ident(ident) => {
      let ident_id = ident.to_id();

      if state.functions.identifiers.contains_key(&ident_id) {
        let func = state.functions.identifiers.get(&ident_id)?;

        match func.as_ref() {
          FunctionConfigType::Regular(func) => {
            let FunctionType::Mapper(func) = func.fn_ptr.clone() else {
              panic!("Function not found");
            };

            return Some(Box::new(EvaluateResultValue::Expr(Box::new(func()))));
          }
          FunctionConfigType::Map(func_map) => {
            return Some(Box::new(EvaluateResultValue::FunctionConfigMap(
              func_map.clone(),
            )));
          }
        }
      }

      None
    }
    Expr::TsAs(_) => unimplemented!("TsAs"),
    Expr::TsSatisfies(_) => unimplemented!("TsSatisfies"),
    Expr::Seq(_) => unimplemented!("Seq"),
    Expr::Lit(lit_path) => Some(Box::new(EvaluateResultValue::Expr(Box::new(Expr::Lit(
      lit_path.clone(),
    ))))),
    Expr::Tpl(tpl) => evaluate_quasis(&Expr::Tpl(tpl.clone()), &tpl.quasis, false, state, fns),
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
    Expr::Cond(_) => unimplemented!("Cond"),
    Expr::Paren(_) => {
      panic!("Paren must be normalized before evaluation")
    }
    Expr::Member(member) => {
      let parent_is_call_expr = state
        .traversal_state
        .all_call_expressions
        .clone()
        .into_iter()
        .any(|call_expr| {
          if let Some(callee) = call_expr.callee.as_expr() {
            callee
              .as_ref()
              .eq_ignore_span(&Expr::Member(member.clone()))
          } else {
            false
          }
        });

      let evaluated_value = if parent_is_call_expr {
        None
      } else {
        evaluate_cached(&member.obj, state, fns)
      };

      if let Some(object) = evaluated_value {
        if !state.confident {
          return None;
        };

        let prop_path = &member.prop;

        let propery = match prop_path {
          MemberProp::Ident(ident) => Some(Box::new(EvaluateResultValue::Expr(Box::new(
            Expr::from(ident.clone()),
          )))),
          MemberProp::Computed(ComputedPropName { expr, .. }) => {
            let result = evaluate_cached(&expr.clone(), state, fns);

            if !state.confident {
              return None;
            }

            result
          }
          MemberProp::PrivateName(_) => {
            return deopt(path, state);
          }
        };

        match object.as_ref() {
          EvaluateResultValue::Expr(expr) => match expr.as_ref() {
            Expr::Array(ArrayLit { elems, .. }) => {
              let Some(eval_res) = propery else {
                panic!("Property not found");
              };

              let EvaluateResultValue::Expr(expr) = eval_res.as_ref() else {
                panic!("Property not found");
              };

              let Expr::Lit(Lit::Num(Number { value, .. })) = *expr.as_expr() else {
                panic!("Member not found");
              };

              let property = elems.get(value as usize)?;

              let Some(ExprOrSpread { expr, .. }) = property else {
                panic!("Member not found");
              };

              Some(Box::new(EvaluateResultValue::Expr(expr.clone())))
            }
            Expr::Object(ObjectLit { props, .. }) => {
              let Some(eval_res) = propery else {
                panic!("Property not found");
              };

              let EvaluateResultValue::Expr(ident) = eval_res.as_ref() else {
                panic!("Property not found");
              };

              let Expr::Ident(ident) = ident.as_expr().clone() else {
                panic!("Member not found");
              };

              let property = props.iter().find(|prop| match prop {
                PropOrSpread::Spread(_) => {
                  unimplemented!("Spread");
                }
                PropOrSpread::Prop(prop) => {
                  let mut prop = prop.clone();

                  transform_shorthand_to_key_values(&mut prop);

                  match prop.as_ref() {
                    Prop::KeyValue(key_value) => {
                      let key = get_key_str(key_value);

                      ident.sym == key
                    }
                    _ => unimplemented!("Prop"),
                  }
                }
              })?;

              if let PropOrSpread::Prop(prop) = property {
                return Some(Box::new(EvaluateResultValue::Expr(Box::new(
                  *prop
                    .as_key_value()
                    .expect("Expression is not a key value")
                    .clone()
                    .value,
                ))));
              } else {
                panic!("Member not found");
              }
            }
            _ => unimplemented!("Expression"),
          },
          EvaluateResultValue::FunctionConfigMap(fc_map) => {
            let key = match propery {
              Some(propery) => match propery.as_ref() {
                EvaluateResultValue::Expr(expr) => match expr.as_ref() {
                  Expr::Ident(ident) => Box::new(ident.clone()),
                  _ => panic!("Member not found"),
                },
                _ => unimplemented!(),
              },
              None => panic!("Member not found"),
            };

            let fc = fc_map.get(&key.into_id()).unwrap();

            return Some(Box::new(EvaluateResultValue::FunctionConfig(fc.clone())));
          }
          EvaluateResultValue::ThemeRef(theme_ref) => {
            let key = match propery {
              Some(propery) => match propery.as_ref() {
                EvaluateResultValue::Expr(expr) => match expr.as_ref() {
                  Expr::Ident(Ident { sym, .. }) => sym.to_string(),
                  Expr::Lit(lit) => {
                    get_string_val_from_lit(lit).expect("Property must be a string")
                  }
                  _ => {
                    panic!("Member not found")
                  }
                },
                _ => unimplemented!(),
              },
              None => panic!("Member not found"),
            };

            let mut cloned_theme_ref = theme_ref.clone();

            let (value, updated_state) = &cloned_theme_ref.get(&key);

            state.traversal_state.combine(updated_state);

            return Some(Box::new(EvaluateResultValue::Expr(Box::new(
              string_to_expression(value.as_str()),
            ))));
          }
          _ => unimplemented!("EvaluateResultValue"),
        }
      } else {
        None
      }
    }
    Expr::Unary(_) => unimplemented!("Unary"),
    Expr::Array(arr_path) => {
      let mut arr: Vec<Option<EvaluateResultValue>> = vec![];

      for elem in arr_path.elems.iter().flatten() {
        let elem_value = evaluate(&elem.expr, &mut state.traversal_state, &state.functions);

        if elem_value.confident {
          arr.push(elem_value.value.map(|value| *value));
        } else {
          return None;
        }
      }

      Some(Box::new(EvaluateResultValue::Vec(arr)))
    }
    Expr::Object(obj_path) => {
      let mut props = vec![];

      for prop in &obj_path.props {
        match prop {
          PropOrSpread::Spread(prop) => {
            let spread_expression = evaluate_cached(&prop.expr, state, fns);

            if !state.confident {
              return deopt(path, state);
            }

            let new_props = spread_expression
              .and_then(|spread| spread.as_expr().cloned())
              .and_then(|expr| expr.object())
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
                  PropName::Str(str) => Some(str.value.to_string()),
                  PropName::Num(num) => Some(num.value.to_string()),
                  PropName::Computed(computed) => {
                    let evaluated_result =
                      evaluate(&computed.expr, &mut state.traversal_state, &state.functions);

                    if !evaluated_result.confident {
                      if evaluated_result.deopt.is_some() {
                        deopt(&evaluated_result.deopt.unwrap(), state);
                      };

                      return None;
                    }

                    Some(expr_to_str(
                      &evaluated_result
                        .value
                        .and_then(|value| value.as_expr().cloned())
                        .expect("Property must be an expression"),
                      &mut state.traversal_state,
                      &state.functions,
                    ))
                  }
                  PropName::BigInt(big_int) => Some(big_int.value.to_string()),
                };

                let value = evaluate(
                  &path_key_value.value,
                  &mut state.traversal_state,
                  &state.functions,
                );

                if !value.confident {
                  if value.deopt.is_some() {
                    deopt(&value.deopt.unwrap(), state);
                  };

                  return None;
                }

                let value = value.value.unwrap();

                let value = match value.as_ref() {
                  EvaluateResultValue::Expr(expr) => expr.clone(),
                  EvaluateResultValue::Vec(items) => {
                    let mut elems: Vec<Option<ExprOrSpread>> = vec![];

                    for entry in items.clone() {
                      let expr = entry
                        .and_then(|entry| {
                          entry
                            .as_vec()
                            .map(|vec| {
                              let mut elems = vec![];

                              for item in vec.iter().flatten() {
                                let item = item.as_expr().unwrap();
                                elems.push(Some(ExprOrSpread {
                                  spread: None,
                                  expr: Box::new(item.clone()),
                                }));
                              }

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
                        _ => panic!("{}", ILLEGAL_PROP_ARRAY_VALUE,),
                      };

                      elems.push(Some(ExprOrSpread {
                        spread: None,
                        expr: Box::new(expr),
                      }));
                    }

                    let array = ArrayLit {
                      span: DUMMY_SP,
                      elems,
                    };

                    Box::new(Expr::Array(array))
                  }
                  EvaluateResultValue::Callback(_cb) => {
                    unimplemented!("EvaluateResultValue::Callback");
                  }
                  _ => {
                    panic!("Property value must be an expression")
                  }
                };

                props.push(PropOrSpread::Prop(Box::new(Prop::from(KeyValueProp {
                  key: PropName::Ident(Ident {
                    sym: key.unwrap().into(),
                    span: DUMMY_SP,
                    optional: false,
                  }),
                  value: value.clone(),
                }))));
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

      return Some(Box::new(EvaluateResultValue::Expr(Box::new(Expr::Object(
        obj,
      )))));
    }
    Expr::Bin(bin) => {
      if let Some(result) = binary_expr_to_num(bin, state, fns) {
        let result = number_to_expression(result);

        return Some(Box::new(EvaluateResultValue::Expr(Box::new(result))));
      } else {
        None
      }
    }
    Expr::Call(call) => {
      let mut context: Option<Box<Vec<Option<EvaluateResultValue>>>> = None;
      let mut func: Option<Box<FunctionConfig>> = None;

      if let Callee::Expr(callee_expr) = &call.callee {
        if get_binding(callee_expr, &mut state.traversal_state).is_none()
          && is_valid_callee(callee_expr)
        {
          panic!("{}", BUILT_IN_FUNCTION)
        } else if let Expr::Ident(ident) = callee_expr.as_ref() {
          let ident_id = ident.to_id();

          if state.functions.identifiers.contains_key(&ident_id) {
            match state.functions.identifiers.get(&ident_id).unwrap().as_ref() {
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

                match callee_name.as_str() {
                  "Math" => {
                    let Some(first_arg) = &call.args.first() else {
                      panic!("Math.{} requires an argument", method_name)
                    };

                    if first_arg.spread.is_some() {
                      unimplemented!("Spread")
                    }

                    match method_name.as_ref() {
                      "pow" => {
                        func = Some(Box::new(FunctionConfig {
                          fn_ptr: FunctionType::Callback(Box::new(CallbackType::Math(MathJS::Pow))),
                          takes_path: false,
                        }));

                        let Some(second_arg) = call.args.get(1) else {
                          panic!("Math.pow requires an second argument")
                        };

                        if second_arg.spread.is_some() {
                          unimplemented!("Spread")
                        }
                        let cached_first_arg = evaluate_cached(&first_arg.expr, state, fns);
                        let cached_second_arg = evaluate_cached(&second_arg.expr, state, fns);

                        context = Some(Box::new(vec![Some(EvaluateResultValue::Vec(vec![
                          cached_first_arg.map(|arg| *arg),
                          cached_second_arg.map(|arg| *arg),
                        ]))]));
                      }
                      "round" | "ceil" | "floor" => {
                        func = Some(Box::new(FunctionConfig {
                          fn_ptr: FunctionType::Callback(Box::new(CallbackType::Math(
                            match method_name.as_ref() {
                              "round" => MathJS::Round,
                              "ceil" => MathJS::Ceil,
                              "floor" => MathJS::Floor,
                              _ => unreachable!("Invalid method: {}", method_name),
                            },
                          ))),
                          takes_path: false,
                        }));

                        let cached_first_arg = evaluate_cached(&first_arg.expr, state, fns);

                        context = Some(Box::new(vec![Some(EvaluateResultValue::Expr(Box::new(
                          cached_first_arg
                            .and_then(|arg| arg.as_expr().cloned())
                            .expect("First argument should be an expression"),
                        )))]));
                      }

                      "min" | "max" => {
                        func = Some(Box::new(FunctionConfig {
                          fn_ptr: FunctionType::Callback(Box::new(CallbackType::Math(
                            match method_name.as_ref() {
                              "min" => MathJS::Min,
                              "max" => MathJS::Max,
                              _ => unreachable!("Invalid method: {}", method_name),
                            },
                          ))),
                          takes_path: false,
                        }));

                        let cached_first_arg = evaluate_cached(&first_arg.expr, state, fns);

                        let mut result = vec![cached_first_arg];

                        result.extend(
                          call
                            .args
                            .iter()
                            .skip(1)
                            .map(|arg| evaluate_cached(&arg.expr, state, fns))
                            .collect::<Vec<Option<Box<EvaluateResultValue>>>>(),
                        );

                        context = Some(Box::new(vec![Some(EvaluateResultValue::Vec(
                          result
                            .into_iter()
                            .map(|arg| arg.map(|boxed_arg| *boxed_arg))
                            .collect(),
                        ))]));
                      }
                      _ => {
                        panic!("{} - {}:{}", BUILT_IN_FUNCTION, callee_name, method_name)
                      }
                    }
                  }
                  "Object" => {
                    let args = &call.args;

                    let Some(arg) = args.first() else {
                      panic!("Object.{} requires an argument", method_name)
                    };

                    if arg.spread.is_some() {
                      unimplemented!("Spread")
                    }

                    let cached_arg = evaluate_cached(&arg.expr, state, fns);

                    match method_name.as_ref() {
                      "fromEntries" => {
                        func = Some(Box::new(FunctionConfig {
                          fn_ptr: FunctionType::Callback(Box::new(CallbackType::Object(
                            ObjectJS::FromEntries,
                          ))),
                          takes_path: false,
                        }));

                        let mut entries_result = IndexMap::new();

                        match cached_arg
                          .expect("Object.entries requires an argument")
                          .as_ref()
                        {
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

                              entries_result.insert(Box::new(key.clone()), Box::new(value.clone()));
                            }
                          }
                          EvaluateResultValue::Vec(vec) => {
                            for entry in vec.clone() {
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

                              entries_result.insert(
                                Box::new(key.clone().clone()),
                                Box::new(value.clone().clone()),
                              );
                            }
                          }
                          _ => {
                            panic!("Object.entries requires an object")
                          }
                        };

                        context = Some(Box::new(vec![Some(EvaluateResultValue::Entries(
                          entries_result,
                        ))]));
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

                        context = Some(Box::new(vec![Some(EvaluateResultValue::Expr(Box::new(
                          Expr::Array(ArrayLit {
                            span: DUMMY_SP,
                            elems: keys,
                          }),
                        )))]));
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

                        context = Some(Box::new(vec![Some(EvaluateResultValue::Expr(Box::new(
                          Expr::Array(ArrayLit {
                            span: DUMMY_SP,
                            elems: values,
                          }),
                        )))]));
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

                        let mut entries: IndexMap<Box<Lit>, Box<Lit>> = IndexMap::new();

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

                          entries.insert(
                            Box::new(lit_str_factory(key.as_str())),
                            Box::new(value.clone()),
                          );
                        }

                        context = Some(Box::new(vec![Some(EvaluateResultValue::Entries(entries))]));
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
                let prop_id = prop_ident.to_id();

                let member_expressions = state
                  .functions
                  .member_expressions
                  .get(&ImportSources::Regular(obj_name));

                if let Some(member_expr) = member_expressions {
                  if let Some(member_expr_fn) = member_expr.get(&prop_id) {
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

              if state
                .functions
                .member_expressions
                .contains_key(&ImportSources::Regular(obj_name.clone()))
              {
                let member_expr = state
                  .functions
                  .member_expressions
                  .get(&ImportSources::Regular(obj_name))
                  .unwrap();

                if member_expr.contains_key(&prop_id) {
                  unimplemented!("Check what's happening here");

                  // context = Some(member_expr.clone());

                  // TODO: uncomment this for implementation of member expressions
                  // match member_expr.get(&prop_id).unwrap().as_ref() {
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
            let parsed_obj = evaluate(object, &mut state.traversal_state, &state.functions);

            if parsed_obj.confident {
              if property.is_ident() {
                let prop_ident = property.as_ident().expect("Property is not an identifier");
                let prop_name = prop_ident.sym.to_string();

                let value = parsed_obj.value.expect("Parsed object has no value");

                match value.as_ref() {
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

                    context = Some(Box::new(expr.clone()))
                  }
                  EvaluateResultValue::Expr(expr) => match expr.as_ref() {
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
                        .map(|elem| {
                          Some(EvaluateResultValue::Expr(Box::new(
                            *elem.clone().unwrap().expr,
                          )))
                        })
                        .collect::<Vec<Option<EvaluateResultValue>>>();

                      context = Some(Box::new(vec![Some(EvaluateResultValue::Vec(expr))]));
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

                      context = Some(Box::new(vec![Some(EvaluateResultValue::Expr(
                        expr.clone(),
                      ))]));
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

                      context = Some(Box::new(vec![Some(EvaluateResultValue::Entries(
                        IndexMap::default(),
                      ))]));
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

                let result_fn = map.get(&string_to_expression(prop_id.clone().0.as_str()));

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
          let args = call
            .args
            .iter()
            .map(|arg| *arg.expr.clone())
            .collect::<Vec<Expr>>();

          match func.fn_ptr {
            FunctionType::ArrayArgs(func) => {
              let func_result = (func)(args);

              return Some(Box::new(EvaluateResultValue::Expr(Box::new(func_result))));
            }
            FunctionType::StylexExprFn(func) => {
              let func_result = (func)(args.first().unwrap().clone(), &mut state.traversal_state);

              return Some(Box::new(EvaluateResultValue::Expr(Box::new(func_result))));
            }
            FunctionType::StylexTypeFn(_) => {
              panic!("StylexTypeFn");
            }
            FunctionType::StylexFnsFactory(_) => {
              panic!("StylexFnsFactory");
            }
            FunctionType::Callback(_) => {
              panic!("Arrow function");
            }
            FunctionType::Mapper(_) => {
              panic!("Mapper");
            }
          }
        } else {
          let args: Vec<Box<EvaluateResultValue>> = call
            .args
            .iter()
            .filter_map(|arg| evaluate_cached(&arg.expr, state, fns))
            .collect();

          if !state.confident {
            return None;
          }

          match func.fn_ptr {
            FunctionType::ArrayArgs(func) => {
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
              return Some(Box::new(EvaluateResultValue::Expr(Box::new(func_result))));
            }
            FunctionType::StylexExprFn(func) => {
              let func_result = (func)(
                args.first().and_then(|arg| arg.as_expr().cloned()).unwrap(),
                &mut state.traversal_state,
              );

              return Some(Box::new(EvaluateResultValue::Expr(Box::new(func_result))));
            }
            FunctionType::StylexTypeFn(func) => {
              let mut fn_args = IndexMap::default();

              let expr = args
                .first()
                .and_then(|expr| expr.as_expr())
                .expect("Argument is not an expression");

              match expr {
                Expr::Object(obj) => {
                  for prop in obj.props.iter() {
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

              let css_type = func_result;

              return Some(Box::new(EvaluateResultValue::Expr(Box::new(css_type))));
            }
            FunctionType::Callback(func) => {
              let context = context.expect("Object.entries requires a context");

              match func.as_ref() {
                CallbackType::Array(ArrayJS::Map) => {
                  return evaluate_map(&args, &context);
                }
                CallbackType::Array(ArrayJS::Filter) => {
                  return evaluate_filter(&args, &context);
                }
                CallbackType::Array(ArrayJS::Join) => {
                  return evaluate_join(
                    &args,
                    &context,
                    &mut state.traversal_state,
                    &state.functions,
                  );
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
                      expr: Box::new(Expr::from(*key.clone())),
                    };

                    let value: ExprOrSpread = ExprOrSpread {
                      spread: None,
                      expr: Box::new(Expr::from(*value.clone())),
                    };

                    entry_elems.push(Some(ExprOrSpread {
                      spread: None,
                      expr: Box::new(array_expression_factory(vec![Some(key), Some(value)])),
                    }));
                  }

                  return Some(Box::new(EvaluateResultValue::Expr(Box::new(
                    array_expression_factory(entry_elems),
                  ))));
                }
                CallbackType::Object(ObjectJS::Keys) => {
                  let Some(Some(EvaluateResultValue::Expr(keys))) = context.first() else {
                    panic!("Object.keys requires an argument")
                  };

                  return Some(Box::new(EvaluateResultValue::Expr(keys.clone())));
                }
                CallbackType::Object(ObjectJS::Values) => {
                  let Some(Some(EvaluateResultValue::Expr(values))) = context.first() else {
                    panic!("Object.keys requires an argument")
                  };

                  return Some(Box::new(EvaluateResultValue::Expr(values.clone())));
                }
                CallbackType::Object(ObjectJS::FromEntries) => {
                  let Some(Some(EvaluateResultValue::Entries(entries))) = context.first() else {
                    panic!("Object.fromEntries requires an argument")
                  };

                  let mut entry_elems = vec![];

                  for (key, value) in entries {
                    let ident = if let Lit::Str(lit_str) = key.as_ref() {
                      ident_factory(lit_str.value.as_str())
                    } else {
                      panic!("Expected a string literal")
                    };

                    let prop = PropOrSpread::Prop(Box::new(Prop::from(KeyValueProp {
                      key: PropName::Ident(ident),
                      value: Box::new(Expr::from(*value.clone())),
                    })));

                    entry_elems.push(prop);
                  }

                  return Some(Box::new(EvaluateResultValue::Expr(Box::new(
                    object_expression_factory(entry_elems),
                  ))));
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
                        .map(|expr| expr_to_num(expr, &mut state.traversal_state, fns))
                        .expect("All arguments must be a number")
                    })
                    .collect::<Vec<f64>>();

                  let result = num_args.first().unwrap().powf(*num_args.get(1).unwrap());

                  // let trancated_num = trancate_f64(result);

                  return Some(Box::new(EvaluateResultValue::Expr(Box::new(
                    number_to_expression(result),
                  ))));
                }
                CallbackType::Math(MathJS::Round | MathJS::Floor | MathJS::Ceil) => {
                  let Some(Some(EvaluateResultValue::Expr(expr))) = context.first() else {
                    panic!("Math.(round | ceil | floor) requires an argument")
                  };

                  let num = expr_to_num(expr.as_ref(), &mut state.traversal_state, fns);

                  let result = match func.as_ref() {
                    CallbackType::Math(MathJS::Round) => num.round(),
                    CallbackType::Math(MathJS::Ceil) => num.ceil(),
                    CallbackType::Math(MathJS::Floor) => num.floor(),
                    _ => unreachable!("Invalid function type"),
                  };

                  return Some(Box::new(EvaluateResultValue::Expr(Box::new(
                    number_to_expression(result),
                  ))));
                }
                CallbackType::Math(MathJS::Min | MathJS::Max) => {
                  let Some(Some(EvaluateResultValue::Vec(args))) = context.first() else {
                    panic!("Math.pow requires an argument")
                  };

                  let num_args = args_to_numbers(args, state, fns);

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

                  return Some(Box::new(EvaluateResultValue::Expr(Box::new(
                    number_to_expression(result),
                  ))));
                }
                CallbackType::String(StringJS::Concat) => {
                  let Some(Some(EvaluateResultValue::Expr(base_str))) = context.first() else {
                    panic!("String concat requires an argument")
                  };

                  let str_args = args
                    .iter()
                    .map(|arg| {
                      arg
                        .as_expr()
                        .map(|expr| expr_to_str(expr, &mut state.traversal_state, fns))
                        .expect("All arguments must be a string")
                    })
                    .collect::<Vec<String>>()
                    .join("");

                  let base_str = expr_to_str(base_str, &mut state.traversal_state, fns);

                  return Some(Box::new(EvaluateResultValue::Expr(Box::new(
                    string_to_expression(format!("{}{}", base_str, str_args).as_str()),
                  ))));
                }
                CallbackType::String(StringJS::CharCodeAt) => {
                  let Some(Some(EvaluateResultValue::Expr(base_str))) = context.first() else {
                    panic!("String concat requires an argument")
                  };

                  let base_str = expr_to_str(base_str, &mut state.traversal_state, fns);

                  let num_args = args
                    .iter()
                    .map(|arg| {
                      arg
                        .as_expr()
                        .map(|expr| expr_to_num(expr, &mut state.traversal_state, fns))
                        .expect("First argument must be a number")
                    })
                    .collect::<Vec<f64>>();

                  let char_index = num_args
                    .first()
                    .expect("First argument of 'charCodeAt' method must be a number");

                  let char_code = char_code_at(&base_str, *char_index as usize)
                    .expect("Char code not found for index");

                  return Some(Box::new(EvaluateResultValue::Expr(Box::new(
                    number_to_expression(char_code as f64),
                  ))));
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
      panic!("_evaluate not implemented this type of expression");
    }
  };

  if result.is_none() && path.is_ident() {
    let ident = path.as_ident().expect("Identifier not found");

    let binding = get_var_decl_by_ident(
      ident,
      &mut state.traversal_state,
      &state.functions,
      VarDeclAction::Reduce,
    );

    match binding {
      Some(binding) => {
        if path.eq(&&Expr::Ident(binding.name.as_ident().unwrap().id.clone())) {
          unimplemented!("Binding")
        }

        let result = evaluate_cached(
          &Box::new(*binding.init.expect("Binding not found")),
          state,
          fns,
        );
        return result;
      }
      None => {
        let name = ident.sym.to_string();

        if name == "undefined" || name == "infinity" || name == "NaN" {
          return Some(Box::new(EvaluateResultValue::Expr(Box::new(Expr::from(
            ident.clone(),
          )))));
        }

        let binding =
          get_import_by_ident(ident, &mut state.traversal_state).and_then(|import_decl| {
            if import_decl
              .specifiers
              .iter()
              .any(|import| import.is_named())
            {
              Some(import_decl)
            } else {
              None
            }
          });

        if let Some(import_path) = binding {
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
            .unwrap_or(ModuleExportName::Ident(import_specifier.local.clone()));

          let abs_path = &state
            .traversal_state
            .import_path_resolver(&import_path.src.value);

          let imported_name = match imported {
            ModuleExportName::Ident(ident) => ident.sym.to_string(),
            ModuleExportName::Str(str) => str.value.to_string(),
          };

          let return_value = match abs_path {
            ImportPathResolution::Tuple(ImportPathResolutionType::ThemeNameRef, value) => {
              evaluate_theme_ref(value, imported_name, &state.traversal_state)
            }
            _ => {
              return deopt(path, state);
            }
          };

          if state.confident {
            let import_path_src = import_path.src.value.to_string();

            if !state.added_imports.contains(&import_path_src)
              && state.traversal_state.get_treeshake_compensation()
            {
              state
                .traversal_state
                .prepend_import_module_items
                .push(add_import_expression(&import_path_src));

              state.added_imports.insert(import_path_src);
            }

            return Some(Box::new(EvaluateResultValue::ThemeRef(return_value)));
          }
        }
      }
    }
  }

  if result.is_none() {
    return deopt(path, state);
  }

  result
}

fn args_to_numbers(
  args: &[Option<EvaluateResultValue>],
  state: &mut EvaluationState,
  fns: &FunctionMap,
) -> Vec<f64> {
  args
    .iter()
    .flat_map(|arg| match arg {
      Some(arg) => match arg {
        EvaluateResultValue::Expr(expr) => {
          vec![expr_to_num(expr, &mut state.traversal_state, fns)]
        }
        EvaluateResultValue::Vec(vec) => args_to_numbers(vec, state, fns),
        _ => unreachable!("Math.min/max requires a number"),
      },
      None => vec![],
    })
    .collect::<Vec<f64>>()
}

fn get_binding(callee: &Expr, state: &mut StateManager) -> Option<VarDeclarator> {
  match callee {
    Expr::Ident(ident) => get_var_decl_from(state, ident).cloned(),
    _ => None,
  }
}

fn is_valid_callee(callee: &Expr) -> bool {
  match callee {
    Expr::Ident(ident) => {
      let name = ident.sym.to_string();
      VALID_CALLEES.contains(name.as_str())
    }
    _ => false,
  }
}

fn get_callee_name(callee: &Expr) -> String {
  match callee {
    Expr::Ident(ident) => ident.sym.to_string(),
    _ => panic!("Callee is not an identifier"),
  }
}

fn is_invalid_method(prop: &MemberProp) -> bool {
  match prop {
    MemberProp::Ident(ident_prop) => INVALID_METHODS.contains(ident_prop.sym.to_string().as_str()),
    _ => false,
  }
}

fn get_method_name(prop: &MemberProp) -> String {
  match prop {
    MemberProp::Ident(ident_prop) => ident_prop.sym.to_string(),
    _ => panic!("Method is not an identifier"),
  }
}

fn is_id_prop(prop: &MemberProp) -> Option<Id> {
  match prop {
    MemberProp::Computed(comp_prop) => match comp_prop.expr.as_ref() {
      Expr::Lit(Lit::Str(str)) => Some(str.value.to_id()),
      _ => None,
    },
    _ => None,
  }
}

pub(crate) fn evaluate_quasis(
  tpl_expr: &Expr,
  quasis: &[TplElement],
  raw: bool,
  state: &mut EvaluationState,
  fns: &FunctionMap,
) -> Option<Box<EvaluateResultValue>> {
  let mut str = String::default();

  let exprs = match tpl_expr {
    Expr::Tpl(tpl) => tpl.exprs.clone(),
    Expr::TaggedTpl(tagged_tpl) => tagged_tpl.tpl.exprs.clone(),
    _ => panic!("The expression is not a template"),
  };

  for (i, elem) in quasis.iter().enumerate() {
    if !state.confident {
      return None;
    };

    str += (if raw {
      elem.raw.to_string()
    } else {
      elem
        .cooked
        .clone()
        .expect("Cooked should be some")
        .to_string()
    })
    .as_str();

    let expr = exprs.get(i);

    if let Some(expr) = expr {
      let evaluated_expr = evaluate_cached(expr, state, fns);

      if let Some(expr) = evaluated_expr {
        let expr = expr.as_expr().expect("Expression not found");

        let lit = expr.as_lit().expect("Literal not found");

        let lit_str = get_string_val_from_lit(lit);

        if let Some(lit_str) = lit_str {
          str += &lit_str;
        }
      }
    }
  }

  if !state.confident {
    return None;
  };

  Some(Box::new(EvaluateResultValue::Expr(Box::new(
    string_to_expression(str.as_str()),
  ))))
}

pub(crate) fn evaluate_cached(
  path: &Expr,
  state: &mut EvaluationState,
  fns: &FunctionMap,
) -> Option<Box<EvaluateResultValue>> {
  let mut cleaned_path = drop_span(path.clone());
  let existing = state.traversal_state.seen.get(&cleaned_path);

  match existing {
    Some(evaluated_value) => {
      if evaluated_value.resolved {
        let resolved = evaluated_value.value.clone();

        return resolved;
      }
      deopt(path, state)
    }
    None => {
      let val = _evaluate(&mut cleaned_path, state, fns);

      if state.confident {
        state.traversal_state.seen.insert(
          Box::new(path.clone()),
          Box::new(SeenValue {
            value: val.clone(),
            resolved: true,
          }),
        );
      } else {
        let item = SeenValue {
          value: None,
          resolved: false,
        };

        state
          .traversal_state
          .seen
          .insert(Box::new(cleaned_path.clone()), Box::new(item));
      }

      val
    }
  }
}

fn evaluate_theme_ref(file_name: &str, export_name: String, state: &StateManager) -> ThemeRef {
  ThemeRef::new(file_name.to_string(), export_name, state.clone())
}
