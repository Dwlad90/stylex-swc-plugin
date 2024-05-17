use core::panic;
// use sha2::{Digest, Sha256};
use std::{
  collections::{HashMap, HashSet},
  env, fs,
  ops::Deref,
  rc::Rc,
};

use colored::Colorize;
use indexmap::IndexMap;
use swc_core::{
  common::{EqIgnoreSpan, Span, Spanned, DUMMY_SP},
  ecma::{
    ast::{
      ArrayLit, BlockStmtOrExpr, Callee, ComputedPropName, Expr, ExprOrSpread, Id, Ident,
      ImportDecl, ImportNamedSpecifier, KeyValueProp, Lit, MemberProp, ModuleExportName, Number,
      ObjectLit, Pat, Prop, PropName, PropOrSpread, Str, TplElement, VarDeclarator,
    },
    utils::{drop_span, ident::IdentLike, ExprExt},
    visit::{Fold, FoldWith},
  },
};

struct SpanReplacer;

impl Fold for SpanReplacer {
  fn fold_span(&mut self, n: Span) -> Span {
    DUMMY_SP
  }
}

fn replace_spans(expr: &mut Expr) -> Expr {
  expr.clone().fold_children_with(&mut SpanReplacer)
}

use crate::shared::{
  constants::{
    self,
    common::{INVALID_METHODS, VALID_CALLEES},
  },
  enums::{ImportPathResolution, ImportPathResolutionType, VarDeclAction},
  structures::{
    evaluate_result::{EvaluateResult, EvaluateResultValue},
    functions::{CallbackType, FunctionConfig, FunctionConfigType, FunctionMap, FunctionType},
    injectable_style::InjectableStyle,
    named_import_source::ImportSources,
    state_manager::{add_import_expression, StateManager},
    stylex_options::StyleXOptions,
    stylex_state_options::StyleXStateOptions,
    theme_ref::{self, ThemeRef},
  },
  utils::{
    common::{
      binary_expr_to_num, create_hash, deep_merge_props, expr_to_num, expr_to_str,
      gen_file_based_identifier, get_import_by_ident, get_key_str, get_string_val_from_lit,
      get_var_decl_by_ident, get_var_decl_from, hash_f32, lit_to_num, normalize_expr,
      number_to_expression, reduce_member_expression_count, remove_duplicates,
      string_to_expression, transform_shorthand_to_key_values,
    },
    css::factories::object_expression_factory,
    js::{
      enums::{ArrayJS, MathJS, ObjectJS},
      native_functions::{evaluate_filter, evaluate_map},
      stylex::stylex_types::ValueWithDefault,
    },
    object,
  },
};

#[derive(Clone, Debug)]
pub(crate) struct SeenValue {
  pub(crate) value: Option<Box<EvaluateResultValue>>,
  pub(crate) resolved: bool,
}

#[derive(Debug)]
pub struct State {
  pub(crate) confident: bool,
  pub(crate) deopt_path: Option<Box<Expr>>, // Assuming this is a string identifier
  pub(crate) added_imports: HashSet<String>,
  pub(crate) functions: FunctionMap,
  pub(crate) traversal_state: StateManager,
}

impl Default for State {
  fn default() -> Self {
    State {
      confident: true,
      deopt_path: Option::None,
      added_imports: HashSet::new(),
      functions: FunctionMap {
        identifiers: HashMap::new(),
        member_expressions: HashMap::new(),
      },
      traversal_state: StateManager::default(),
    }
  }
}

impl State {
  pub(crate) fn new(traversal_state: &StateManager) -> Self {
    State {
      confident: true,
      deopt_path: Option::None,
      added_imports: HashSet::new(),
      functions: FunctionMap {
        identifiers: HashMap::new(),
        member_expressions: HashMap::new(),
      },
      traversal_state: traversal_state.clone(),
    }
  }
}

// enum KeyResult {
//     ConfidentTrue {
//         confident: bool,
//         value: String,
//     },
//     ConfidentFalse {
//         confident: bool,
//         deopt: Option<Expr>,
//     },
// }

pub(crate) fn evaluate_obj_key(
  prop_kv: &KeyValueProp,
  state: &mut StateManager,
  functions: &FunctionMap,
) -> EvaluateResult {
  let key_path = &prop_kv.key;

  let key: Expr;

  match key_path {
    PropName::Ident(ident) => {
      key = Expr::Lit(Lit::Str(Str {
        value: ident.sym.to_string().into(),
        raw: Option::None,
        span: DUMMY_SP,
      }));
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
          inline_styles: Option::None,
          fns: Option::None,
        };
      }
    }
    PropName::Str(str) => {
      key = Expr::Lit(Lit::Str(str.clone()));
    }
    PropName::Num(num) => {
      key = Expr::Lit(Lit::Num(num.clone()));
    }
    PropName::BigInt(big_int) => {
      key = Expr::Lit(Lit::BigInt(big_int.clone()));
    }
  }

  let key_expr = string_to_expression(expr_to_str(&key, state, functions).as_str())
    .expect("Expected string value");

  EvaluateResult {
    confident: true,
    deopt: Option::None,
    value: Option::Some(Box::new(EvaluateResultValue::Expr(Box::new(key_expr)))),
    inline_styles: Option::None,
    fns: Option::None,
  }
}

pub fn evaluate(
  path: &Expr,
  traversal_state: &mut StateManager,
  fns: &FunctionMap,
) -> Box<EvaluateResult> {
  let mut state = Box::new(State {
    confident: true,
    deopt_path: None,
    added_imports: HashSet::new(),
    functions: fns.clone(),
    traversal_state: traversal_state.clone(),
  });

  let mut value = evaluate_cached(path, &mut state);

  if !state.confident {
    value = Option::None;
  }

  *traversal_state = state.traversal_state;

  Box::new(EvaluateResult {
    confident: state.confident,
    value,
    deopt: state.deopt_path,
    inline_styles: Option::None,
    fns: Option::None,
  })
}

fn deopt(path: &Expr, state: &mut State) -> Option<Box<EvaluateResultValue>> {
  if state.confident {
    state.confident = false;
    state.deopt_path = Some(Box::new(path.clone()));
  }

  Option::None
}

// fn evaluate_arror_function(
//     arrow: ArrowExpr,
//     state: &mut State,
//     declarations: &Vec<VarDeclarator>,
//     var_dec_count_map: &mut HashMap<Id, i8>,
// ) -> Option<FunctionConfig> {

// }

fn _evaluate(path: &Expr, state: &mut State) -> Option<Box<EvaluateResultValue>> {
  if !state.confident {
    panic!("Should not be here");
    // return Option::None;
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
            Option::Some(param.as_ident().unwrap().to_id())
          } else {
            Option::None
          }
        })
        .collect::<Vec<Id>>();

      match body.as_ref() {
        BlockStmtOrExpr::Expr(body_expr) => {
          if ident_params.len() == params.len() {
            let orig_args: Vec<Pat> = params.clone();

            let arrow_closure_fabric =
              |orig_args: Vec<Pat>,
               functions: HashMap<Box<Id>, Box<FunctionConfigType>>,
               ident_params: Vec<Id>,
               body_expr: Box<Expr>,
               traversal_state: StateManager| {
                move |cb_args: Vec<Option<EvaluateResultValue>>| {
                  let mut functions = functions.clone();

                  let mut member_expressions: HashMap<
                    Box<ImportSources>,
                    Box<HashMap<Box<Id>, Box<FunctionConfigType>>>,
                  > = HashMap::new();
                  // println!(
                  //   "!!!!__ orig_args: {:#?}, functions: {:#?}, cb_args: {:#?}",
                  //   orig_args, functions, cb_args
                  // );

                  ident_params.iter().enumerate().for_each(|(index, ident)| {
                    if let Some(arg) = cb_args.get(index) {
                      let arg = arg.clone();
                      let expr = arg.unwrap().as_expr().unwrap().clone();

                      let cl = |arg: Expr| move || arg.clone();

                      // panic!("Check what's happening here, expr: {:#?}", expr);
                      let result = (cl)(expr.clone());
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

                  // panic!("Check what's happening here, body_expr: {:#?} ident_params: {:#?}", body_expr, ident_params);
                  let result = evaluate(
                    &body_expr,
                    &mut traversal_state.clone(),
                    &FunctionMap {
                      identifiers: functions,
                      member_expressions,
                    },
                  );

                  let value = result.value;

                  match value {
                    Some(res) => res.as_expr().unwrap().clone(),
                    None => todo!(),
                  }
                }
              };

            let functions = state.functions.identifiers.clone();

            let arrow_closure = Rc::new(arrow_closure_fabric(
              orig_args,
              functions,
              ident_params,
              Box::new(*body_expr.clone()),
              state.traversal_state.clone(),
            ));

            return Option::Some(Box::new(EvaluateResultValue::Callback(
              arrow_closure,
              // Expr::Arrow(arrow.clone()),
            )));
          }

          Option::None
        }
        BlockStmtOrExpr::BlockStmt(_) => Option::None,
      }
    }
    Expr::Ident(ident) => {
      let ident_id = ident.to_id();

      // dbg!(&ident_id, &state.functions.identifiers);

      if state.functions.identifiers.contains_key(&ident_id) {
        let func = state.functions.identifiers.get(&ident_id)?;
        // dbg!(&ident_id, &func);

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

      // let ident_binding =
      //     get_binding(&Expr::Ident(ident.clone()), &mut state.traversal_state);

      Option::None
    }
    Expr::TsAs(_) => todo!("TsAs not implemented yet"),
    Expr::TsSatisfies(_) => todo!("TsSatisfies not implemented yet"),
    Expr::Seq(_) => todo!("Seq not implemented yet"),
    Expr::Lit(lit_path) => Option::Some(Box::new(EvaluateResultValue::Expr(Box::new(Expr::Lit(
      lit_path.clone(),
    ))))),
    Expr::Tpl(tpl) => evaluate_quasis(&Expr::Tpl(tpl.clone()), &tpl.quasis, false, state),
    Expr::TaggedTpl(tagged_tpl) => {
      todo!("TaggedTpl");
      evaluate_quasis(
        &Expr::TaggedTpl(tagged_tpl.clone()),
        &tagged_tpl.tpl.quasis,
        false,
        state,
      )
    }
    Expr::Cond(_) => todo!("Cond not implemented yet"),
    Expr::Paren(_) => {
      panic!("Paren must be normalized before evaluation")
      // let result = evaluate_cached(&paren.expr, state);

      // result
    }
    Expr::Member(member) => {
      // dbg!(&member);

      let parent_is_call_expr = state
        .traversal_state
        .all_call_expressions
        .clone()
        .into_iter()
        .any(|call_expr| {
          // dbg!(&call_expr.callee, &member.obj);
          if let Some(callee) = call_expr.callee.as_expr() {
            callee
              .as_ref()
              .eq_ignore_span(&Expr::Member(member.clone()))
          } else {
            false
          }
        });
      // dbg!(&parent_is_call_expr, &member.obj);

      let evaluated_value = if parent_is_call_expr {
        Option::None
      } else {
        evaluate_cached(&member.obj, state)
      };

      // dbg!(&evaluated_value);

      if let Some(object) = evaluated_value {
        if !state.confident {
          return Option::None;
        };

        // let Some(ArrayLit { elems, .. }) = expr.as_array() else {
        //     if let Some(object) = expr.as_object() {
        //         return Some(EvaluateResultValue::Expr(Expr::Object(object.clone())));
        //     } else {
        //        // dbg!(&expr);
        //         todo!("Expression");
        //     }
        // };

        let prop_path = &member.prop;
        // dbg!(&prop_path);

        let propery = match prop_path {
          MemberProp::Ident(ident) => Option::Some(Box::new(EvaluateResultValue::Expr(Box::new(
            Expr::Ident(ident.clone()),
          )))),
          MemberProp::Computed(ComputedPropName { expr, .. }) => {
            let result = evaluate_cached(&expr.clone(), state);

            if !state.confident {
              return Option::None;
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

              let property = elems.get(value as usize)?.clone();

              let Some(ExprOrSpread { expr, .. }) = property else {
                panic!("Member not found");
              };

              Some(Box::new(EvaluateResultValue::Expr(Box::new(*expr))))
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

              let property = props
                .iter()
                .find(|prop| match prop {
                  PropOrSpread::Spread(_) => {
                    todo!("Spread not implemented yet");
                  }
                  PropOrSpread::Prop(prop) => {
                    let mut prop = prop.clone();

                    transform_shorthand_to_key_values(&mut prop);

                    match prop.as_ref() {
                      Prop::KeyValue(key_value) => {
                        let key = get_key_str(key_value);

                        ident.sym == key
                      }
                      _ => todo!("Prop not implemented yet"),
                    }
                  }
                })?
                .clone();

              if let PropOrSpread::Prop(prop) = property {
                // dbg!(&prop, ident);
                return Some(Box::new(EvaluateResultValue::Expr(Box::new(
                  *prop
                    .key_value()
                    .expect("Expression is not a key value")
                    .value,
                ))));
              } else {
                panic!("Member not found");
              }
            }
            _ => todo!("Expression"),
          },
          EvaluateResultValue::Vec(_) => todo!("EvaluateResultValue::Vec"),
          EvaluateResultValue::Map(_) => todo!("EvaluateResultValue::Map"),
          EvaluateResultValue::Entries(_) => todo!("EvaluateResultValue::Entries"),
          EvaluateResultValue::Callback(_) => todo!("EvaluateResultValue::Callback"),
          EvaluateResultValue::FunctionConfig(_) => todo!("EvaluateResultValue::FunctionConfig"),
          EvaluateResultValue::FunctionConfigMap(fc_map) => {
            // dbg!(&fc_map, &propery);

            let key = match propery {
              Some(propery) => match propery.as_ref() {
                EvaluateResultValue::Expr(expr) => match expr.as_ref() {
                  Expr::Ident(ident) => Box::new(ident.clone()),
                  _ => panic!("Member not found"),
                },
                EvaluateResultValue::Vec(_) => todo!(),
                EvaluateResultValue::Map(_) => todo!(),
                EvaluateResultValue::Entries(_) => todo!(),
                EvaluateResultValue::Callback(_) => todo!(),
                EvaluateResultValue::FunctionConfig(_) => todo!(),
                EvaluateResultValue::FunctionConfigMap(_) => todo!(),
                EvaluateResultValue::ThemeRef(_) => todo!(),
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
                  Expr::Ident(Ident { sym, .. }) => sym.to_string().clone(),
                  _ => panic!("Member not found"),
                },
                EvaluateResultValue::Vec(_) => todo!(),
                EvaluateResultValue::Map(_) => todo!(),
                EvaluateResultValue::Entries(_) => todo!(),
                EvaluateResultValue::Callback(_) => todo!(),
                EvaluateResultValue::FunctionConfig(_) => todo!(),
                EvaluateResultValue::FunctionConfigMap(_) => todo!(),
                EvaluateResultValue::ThemeRef(_) => todo!(),
              },
              None => panic!("Member not found"),
            };

            let (value, updated_state) = &theme_ref.clone().get(&key);

            state.traversal_state = state.traversal_state.clone().combine(updated_state.clone());

            // dbg!(
            //   &value,
            //   &state.traversal_state.prepend_include_module_items,
            //   &updated_state.prepend_include_module_items
            // );

            return Some(Box::new(EvaluateResultValue::Expr(Box::new(
              string_to_expression(value.as_str()).unwrap(),
            ))));
          }
        }
      } else {
        Option::None
      }
    }
    Expr::Unary(_) => todo!("Unary not implemented yet"),
    Expr::Array(arr_path) => {
      let elems = arr_path.elems.clone();

      let mut arr: Vec<Option<EvaluateResultValue>> = vec![];

      for elem in elems.iter().filter_map(|elem| elem.clone()) {
        let elem_value = evaluate(&elem.expr, &mut state.traversal_state, &state.functions);

        if elem_value.confident {
          arr.push(elem_value.value.map(|value| *value));
        } else {
          // elem_value.deopt.is_some() && deopt(&elem_value.deopt.unwrap(), state);
          return Option::None;
        }
      }

      Option::Some(Box::new(EvaluateResultValue::Vec(arr)))
    }
    Expr::Object(obj_path) => {
      let mut props = vec![];

      for prop in &obj_path.props {
        match prop {
          PropOrSpread::Spread(prop) => {
            let spread_expression = evaluate_cached(&prop.expr, state);

            if !state.confident {
              return deopt(path, state);
            }

            let new_props = &spread_expression.unwrap();
            let new_props = new_props.as_expr().unwrap();
            let new_props = new_props.as_object().unwrap();

            let merged_object = deep_merge_props(props, new_props.props.clone());

            props = merged_object;

            continue;
          }
          PropOrSpread::Prop(prop) => {
            if prop.is_method() {
              return deopt(path, state);
            }

            let mut prop = prop.clone();

            transform_shorthand_to_key_values(&mut prop);

            // dbg!(&prop);

            match prop.as_ref() {
              Prop::KeyValue(path_key_value) => {
                let key_path = path_key_value.key.clone();

                let key = match &key_path {
                  PropName::Ident(ident) => Option::Some(ident.clone().sym.to_string()),
                  PropName::Str(str) => Option::Some(str.value.clone().to_string()),
                  PropName::Num(num) => Option::Some(num.value.to_string()),
                  PropName::Computed(computed) => {
                    let evaluated_result =
                      evaluate(&computed.expr, &mut state.traversal_state, &state.functions);

                    if !evaluated_result.confident {
                      if evaluated_result.deopt.is_some() {
                        deopt(&evaluated_result.deopt.unwrap(), state);
                      };

                      return Option::None;
                    }

                    // panic!("Check what's happening here");
                    Option::Some(expr_to_str(
                      evaluated_result.value.unwrap().as_expr().unwrap(),
                      &mut state.traversal_state,
                      &state.functions,
                    ))
                  }
                  PropName::BigInt(big_int) => Option::Some(big_int.value.to_string()),
                };

                let value_path = path_key_value.value.clone();

                // dbg!(&value_path);

                let value = evaluate(&value_path, &mut state.traversal_state, &state.functions);

                if !value.confident {
                  if value.deopt.is_some() {
                    deopt(&value.deopt.unwrap(), state);
                  };

                  return Option::None;
                }

                let value = value.value.unwrap();
                // props = deep_merge_props(
                //     props,
                //     vec![PropOrSpread::Prop(Box::new(Prop::KeyValue(
                //         KeyValueProp {
                //             key: PropName::Ident(Ident {
                //                 sym: key.unwrap().into(),
                //                 span: DUMMY_SP,
                //                 optional: false,
                //             }),
                //             value: Box::new(value.as_expr().unwrap().clone()),
                //         },
                //     )))],
                // );

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

                              for item in vec {
                                if let Some(item) = item {
                                  let item = item.as_expr().unwrap();
                                  elems.push(Some(ExprOrSpread {
                                    spread: Option::None,
                                    expr: Box::new(item.clone()),
                                  }));
                                }
                              }

                              Expr::Array(ArrayLit {
                                span: DUMMY_SP,
                                elems,
                              })
                            })
                            .or_else(|| entry.as_expr().cloned())
                        })
                        .expect(constants::messages::ILLEGAL_PROP_ARRAY_VALUE);

                      let expr = match expr {
                        Expr::Array(array) => Expr::Array(array),
                        Expr::Lit(lit) => Expr::Lit(lit),
                        _ => panic!("{}", constants::messages::ILLEGAL_PROP_ARRAY_VALUE,),
                      };

                      elems.push(Option::Some(ExprOrSpread {
                        spread: Option::None,
                        expr: Box::new(expr),
                      }));
                    }

                    let array = ArrayLit {
                      span: DUMMY_SP,
                      elems,
                    };

                    Box::new(Expr::Array(array))
                  }
                  EvaluateResultValue::Callback(cb) => {
                    let a = cb(vec![]);

                    // dbg!(&a);

                    panic!();
                  }
                  _ => {
                    // dbg!(&value);
                    panic!("Property value must be an expression")
                  }
                };

                props.push(PropOrSpread::Prop(Box::new(Prop::KeyValue(KeyValueProp {
                  key: PropName::Ident(Ident {
                    sym: key.unwrap().into(),
                    span: DUMMY_SP,
                    optional: false,
                  }),
                  value: value.clone(),
                }))));
              }

              _ => todo!(),
            }
          }
        }
      }

      let obj = ObjectLit {
        props: remove_duplicates(props.clone()),
        span: DUMMY_SP,
      };

      return Option::Some(Box::new(EvaluateResultValue::Expr(Box::new(Expr::Object(
        obj,
      )))));
    }
    Expr::Bin(bin) => {
      // dbg!(&bin);
      if let Some(result) = binary_expr_to_num(bin, state) {
        let result = number_to_expression(result as f64).unwrap();

        return Option::Some(Box::new(EvaluateResultValue::Expr(Box::new(result))));
      } else {
        Option::None
      }
    }
    Expr::Call(call) => {
      let callee = call.callee.clone();

      let mut context: Option<Box<Vec<Option<EvaluateResultValue>>>> = Option::None;
      let mut func: Option<Box<FunctionConfig>> = Option::None;

      if let Callee::Expr(callee_expr) = callee {
        let callee_expr = callee_expr;

        if get_binding(&callee_expr, &mut state.traversal_state).is_none()
          && is_valid_callee(&callee_expr)
        {
          panic!("{}", constants::messages::BUILT_IN_FUNCTION)
        } else if let Expr::Ident(ident) = callee_expr.as_ref() {
          let ident_id = ident.to_id();

          if state.functions.identifiers.contains_key(&ident_id) {
            match state.functions.identifiers.get(&ident_id).unwrap().as_ref() {
              FunctionConfigType::Map(_) => todo!("FunctionConfigType::Map"),
              FunctionConfigType::Regular(fc) => func = Option::Some(Box::new(fc.clone())),
            }
          }
        }

        if let Expr::Member(member) = callee_expr.as_ref() {
          let object = member.obj.clone();
          let property = &member.prop;

          if object.is_ident() {
            let obj_ident = object.as_ident().unwrap();

            if property.is_ident() {
              if is_valid_callee(&object) && !is_invalid_method(property) {
                let callee_name = get_callee_name(&object);

                let method_name = get_method_name(property);

                match callee_name.as_str() {
                  "Math" => {
                    let args = call.args.clone();

                    let Option::Some(first_arg) = args.first() else {
                      panic!("Math.{} requires an argument", method_name)
                    };

                    if first_arg.spread.is_some() {
                      panic!("Spread not implemented yet")
                    }

                    // if method_name == "ceil"{
                    //   panic!("Math.round not implemented yet")
                    // }

                    match method_name.as_ref() {
                      "pow" => {
                        func = Option::Some(Box::new(FunctionConfig {
                          fn_ptr: FunctionType::Callback(Box::new(CallbackType::Math(MathJS::Pow))),
                          takes_path: false,
                        }));

                        let Option::Some(second_arg) = args.get(1) else {
                          panic!("Math.pow requires an second argument")
                        };

                        if second_arg.spread.is_some() {
                          panic!("Spread not implemented yet")
                        }
                        let cached_first_arg = evaluate_cached(&first_arg.expr, state);
                        let cached_second_arg = evaluate_cached(&second_arg.expr, state);

                        context = Option::Some(Box::new(vec![Option::Some(
                          EvaluateResultValue::Vec(vec![
                            cached_first_arg.map(|arg| *arg),
                            cached_second_arg.map(|arg| *arg),
                          ]),
                        )]));
                      }
                      "round" | "ceil" | "floor" => {
                        func = Option::Some(Box::new(FunctionConfig {
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
                        // let a = cached_first_arg.unwrap();

                        let cached_first_arg = evaluate_cached(&first_arg.expr, state);
                      // dbg!(&cached_first_arg, &first_arg.expr);

                        context = Option::Some(Box::new(vec![Option::Some(
                          EvaluateResultValue::Expr(Box::new(
                            cached_first_arg
                              .unwrap()
                              .as_expr()
                              .expect("First argument should be an expression")
                              .clone(),
                          )),
                        )]));
                      }

                      "min" | "max" => {
                        func = Option::Some(Box::new(FunctionConfig {
                          fn_ptr: FunctionType::Callback(Box::new(CallbackType::Math(
                            match method_name.as_ref() {
                              "min" => MathJS::Min,
                              "max" => MathJS::Max,
                              _ => unreachable!("Invalid method: {}", method_name),
                            },
                          ))),
                          takes_path: false,
                        }));

                        let cached_first_arg = evaluate_cached(&first_arg.expr, state);

                        let mut result = vec![cached_first_arg];

                        result.extend(
                          args
                            .iter()
                            .skip(1)
                            .map(|arg| {
                              if arg.spread.is_some() {
                                panic!("Spread not implemented yet")
                              }

                              evaluate_cached(&arg.expr, state)
                            })
                            .collect::<Vec<Option<Box<EvaluateResultValue>>>>(),
                        );

                        context =
                          Option::Some(Box::new(vec![Option::Some(EvaluateResultValue::Vec(
                            result
                              .into_iter()
                              .map(|arg| arg.map(|boxed_arg| *boxed_arg))
                              .collect(),
                          ))]));
                      }
                      _ => {
                        panic!(
                          "{} - {}:{}",
                          constants::messages::BUILT_IN_FUNCTION,
                          callee_name,
                          method_name
                        )
                      }
                    }
                  }
                  "Object" => {
                    let args = call.args.clone();

                    let Option::Some(arg) = args.first() else {
                      panic!("Object.{} requires an argument", method_name)
                    };

                    if arg.spread.is_some() {
                      panic!("Spread not implemented yet")
                    }

                    let cached_arg = evaluate_cached(&arg.expr, state);

                    match method_name.as_ref() {
                      "fromEntries" => {
                        func = Option::Some(Box::new(FunctionConfig {
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
                              // .and_then(|items| items.flatten())
                              .collect::<Vec<ExprOrSpread>>();

                            for entry in entries {
                              assert!(entry.spread.is_none(), "Spread not implemented yet");

                              let array = entry.expr.as_array().expect("Entry must be an array");

                              let elems = array
                                .elems
                                .clone()
                                .into_iter()
                                .filter_map(|item| item)
                                .collect::<Vec<ExprOrSpread>>();

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

                        context = Option::Some(Box::new(vec![Option::Some(
                          EvaluateResultValue::Entries(entries_result),
                        )]));
                      }
                      "keys" => {
                        func = Option::Some(Box::new(FunctionConfig {
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
                          let expr = prop
                            .as_prop()
                            .map(|prop| *prop.clone())
                            .expect("Spread not implemented yet");

                          let key_values = expr
                            .as_key_value()
                            .expect("Object.entries requires an object");

                          let key = get_key_str(key_values);

                          keys.push(Option::Some(ExprOrSpread {
                            spread: Option::None,
                            expr: Box::new(Expr::Lit(Lit::Str(Str {
                              span: DUMMY_SP,
                              value: key.into(),
                              raw: Option::None,
                            }))),
                          }));
                        }

                        context = Option::Some(Box::new(vec![Option::Some(
                          EvaluateResultValue::Expr(Box::new(Expr::Array(ArrayLit {
                            span: DUMMY_SP,
                            elems: keys,
                          }))),
                        )]));
                      }
                      "values" => {
                        func = Option::Some(Box::new(FunctionConfig {
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
                          let expr = prop
                            .as_prop()
                            .map(|prop| *prop.clone())
                            .expect("Spread not implemented yet");

                          let key_values = expr
                            .as_key_value()
                            .expect("Object.entries requires an object");

                          let value = key_values
                            .value
                            .as_lit()
                            .expect("Object value should be a literal");

                          values.push(Option::Some(ExprOrSpread {
                            spread: Option::None,
                            expr: Box::new(Expr::Lit(value.clone())),
                          }));
                        }

                        context = Option::Some(Box::new(vec![Option::Some(
                          EvaluateResultValue::Expr(Box::new(Expr::Array(ArrayLit {
                            span: DUMMY_SP,
                            elems: values,
                          }))),
                        )]));
                      }
                      "entries" => {
                        func = Option::Some(Box::new(FunctionConfig {
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
                          let expr = prop
                            .as_prop()
                            .map(|prop| *prop.clone())
                            .expect("Spread not implemented yet");

                          let key_values = expr
                            .as_key_value()
                            .expect("Object.entries requires an object");

                          let value = key_values
                            .value
                            .as_lit()
                            .expect("Object value should be a literal");

                          let key = get_key_str(key_values);

                          entries.insert(
                            Box::new(Lit::Str(Str {
                              span: DUMMY_SP,
                              value: key.into(),
                              raw: Option::None,
                            })),
                            Box::new(value.clone()),
                          );
                        }

                        context = Option::Some(Box::new(vec![Option::Some(
                          EvaluateResultValue::Entries(entries),
                        )]));
                      }
                      _ => {
                        panic!(
                          "{} - {}:{}",
                          constants::messages::BUILT_IN_FUNCTION,
                          callee_name,
                          method_name
                        )
                      }
                    }
                  }
                  _ => panic!(
                    "{} - {}",
                    constants::messages::BUILT_IN_FUNCTION,
                    callee_name
                  ),
                }
              } else {
                // let memberExpressionFn = state
                //     .functions
                //     .member_expressions
                //     .get(&ImportSources::Regular(obj_ident.sym.to_string()));

                // if let Some(memberExpressionFn) = memberExpressionFn {

                // }

                //// dbg!(&state.functions.member_expressions);
                // panic!("{}", constants::messages::BUILT_IN_FUNCTION);
                let prop_ident = property.as_ident().unwrap();

                let obj_name = obj_ident.sym.to_string();
                let prop_id = prop_ident.to_id();

                let member_expressions = state
                  .functions
                  .member_expressions
                  .get(&ImportSources::Regular(obj_name.clone()));

                if let Some(member_expr) = member_expressions {
                  let member_expr = member_expr.clone();
                  if let Some(member_expr_fn) = member_expr.get(&prop_id) {
                    // dbg!(&member_expr, &member_expr_fn,);

                    // panic!();
                    // context = Option::Some(vec![Option::Some(EvaluateResultValue::Expr(
                    //     member_expr_fn.clone(),
                    // ))]);

                    match member_expr_fn.as_ref() {
                      FunctionConfigType::Regular(fc) => {
                        func = Option::Some(Box::new(fc.clone()));
                      }
                      FunctionConfigType::Map(_) => todo!("FunctionConfigType::Map"),
                    }
                  }
                }
              }
            }

            if let Option::Some(prop_id) = is_id_prop(property) {
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
                let member_expr = member_expr.clone();

                if member_expr.contains_key(&prop_id) {
                  todo!("Check what's happening here");

                  // context = Option::Some(member_expr.clone());
                  match member_expr.get(&prop_id).unwrap().as_ref() {
                    FunctionConfigType::Regular(fc) => {
                      func = Option::Some(Box::new(fc.clone()));
                    }
                    FunctionConfigType::Map(_) => todo!("FunctionConfigType::Map"),
                  }
                }
              }
            }
          }

          if object.is_lit() {
            let obj_lit = object.as_lit().unwrap();

            if property.is_ident() {
              let prop_ident = property.as_ident().unwrap();
              let prop_name = prop_ident.sym.to_string();

              match obj_lit {
                Lit::Str(_) => todo!("{}", constants::messages::BUILT_IN_FUNCTION),
                Lit::Bool(_) => todo!("{}", constants::messages::BUILT_IN_FUNCTION),
                _ => {}
              }
            }
          }

          if func.is_none() {
            let parsed_obj = evaluate(&object, &mut state.traversal_state, &state.functions);

            // println!("!!!!__ obj: {:#?}, parsed_obj: {:#?}", object, parsed_obj);

            // panic!("Member not implemented yet");

            if parsed_obj.confident {
              if property.is_ident() {
                let prop_ident = property.as_ident().unwrap().clone();
                let prop_name = prop_ident.sym.to_string();

                let value = parsed_obj.value.unwrap();

                match value.clone().as_ref() {
                  EvaluateResultValue::Map(map) => {
                    let result_fn = map.get(&Expr::Ident(prop_ident.clone()));

                    func = match result_fn {
                      Some(_) => panic!("Not implemented yet"),
                      None => Option::None,
                    };
                  }
                  EvaluateResultValue::Vec(expr) => {
                    func = Option::Some(Box::new(FunctionConfig {
                      fn_ptr: FunctionType::Callback(
                        Box::new(match prop_name.as_str() {
                          "map" => CallbackType::Array(ArrayJS::Map),
                          "filter" => CallbackType::Array(ArrayJS::Filter),
                          "entries" => CallbackType::Object(ObjectJS::Entries),
                          _ => todo!("Array method '{}' implemented yet", prop_name),
                        }),
                        // obj.clone(),
                      ),
                      takes_path: false,
                    }));

                    // panic!("Array method not implemented yet, {:#?}",expr);

                    context = Option::Some(Box::new(expr.clone()))
                  }
                  EvaluateResultValue::Expr(expr) => {
                    if let Some(ArrayLit { elems, .. }) = expr.as_array() {
                      func = Option::Some(Box::new(FunctionConfig {
                        fn_ptr: FunctionType::Callback(
                          Box::new(match prop_name.as_str() {
                            "map" => CallbackType::Array(ArrayJS::Map),
                            "filter" => CallbackType::Array(ArrayJS::Filter),
                            "entries" => CallbackType::Object(ObjectJS::Entries),
                            _ => todo!("Method '{}' implemented yet", prop_name),
                          }),
                          // obj.clone(),
                        ),
                        takes_path: false,
                      }));

                      let expr = elems
                        .iter()
                        .map(|elem| {
                          Option::Some(EvaluateResultValue::Expr(Box::new(
                            *elem.clone().unwrap().expr.clone(),
                          )))
                        })
                        .collect::<Vec<Option<EvaluateResultValue>>>();
                      // panic!("Array method not implemented yet, {:#?}",expr);

                      context =
                        Option::Some(Box::new(vec![Option::Some(EvaluateResultValue::Vec(expr))]));
                    }
                  }
                  EvaluateResultValue::FunctionConfig(fc) => {
                    match fc.fn_ptr {
                      FunctionType::ArrayArgs(_) => todo!(),
                      FunctionType::StylexFnsFactory(sfns) => {
                        // dbg!(&sfns);
                        let fc = sfns(prop_name);

                        func = Option::Some(Box::new(FunctionConfig {
                          fn_ptr: FunctionType::StylexTypeFn(fc),
                          takes_path: false,
                        }));

                        context = Option::Some(Box::new(vec![Option::Some(
                          EvaluateResultValue::Entries(IndexMap::default()),
                        )]));
                      }
                      FunctionType::StylexExprFn(_) => todo!(),
                      FunctionType::StylexTypeFn(_) => todo!(),
                      FunctionType::Mapper(_) => todo!(),
                      FunctionType::Callback(_) => todo!(),
                    }
                    // func = Option::Some(fc);
                  }
                  _ => {
                    // println!("!!!!__ Evaluation result value: {:#?}", value);
                    panic!("Evaluation result not implemented yet")
                  }
                }
              } else if let Option::Some(prop_id) = is_id_prop(property) {
                let prop_id = prop_id.clone();
                let value = parsed_obj.value.unwrap();
                let map = value.as_map().unwrap();

                let result_fn = map.get(&Expr::Lit(Lit::Str(Str {
                  value: prop_id.clone().0,
                  raw: Option::None,
                  span: DUMMY_SP,
                })));

                func = match result_fn {
                  Some(_) => panic!("Not implemented yet"),
                  None => Option::None,
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
            .clone()
            .into_iter()
            .map(|arg| *arg.expr)
            .collect::<Vec<Expr>>();

          match func.fn_ptr {
            FunctionType::ArrayArgs(func) => {
              let func_result = (func)(args.clone());

              // dbg!(&func_result, &args);
              return Option::Some(Box::new(EvaluateResultValue::Expr(Box::new(func_result))));
            }
            FunctionType::StylexExprFn(func) => {
              let func_result =
                (func)(args.first().unwrap().clone(), state.traversal_state.clone());
              state.traversal_state = state.traversal_state.clone().combine(func_result.1);
              return Option::Some(Box::new(EvaluateResultValue::Expr(Box::new(func_result.0))));
            }
            FunctionType::StylexTypeFn(_) => {
              panic!("StylexTypeFn not implemented yet");
            }
            FunctionType::StylexFnsFactory(_) => {
              panic!("StylexFnsFactory not implemented yet");
            }
            FunctionType::Callback(_) => {
              panic!("Arrow function not implemented yet");
              // let func_result = (cb)(args.get(0).unwrap().clone());
              // return Option::Some(EvaluateResultValue::Expr(func_result));
            }
            FunctionType::Mapper(_) => {
              panic!("Mapper not implemented yet");
              // let func_result = (cb)(args.get(0).unwrap().clone());
              // return Option::Some(EvaluateResultValue::Expr(func_result));
            }
          }
        } else {
          let args: Vec<Box<EvaluateResultValue>> = call
            .args
            .clone()
            .into_iter()
            .filter_map(|arg| {
              let cached_arg = evaluate_cached(&arg.expr, state);

              // println!("!!!!__ cached_arg: {:#?}, arg: {:#?}", cached_arg, arg);
              cached_arg
            })
            .collect();

          if !state.confident {
            return Option::None;
          }

          match func.fn_ptr {
            FunctionType::ArrayArgs(func) => {
              let func_result = (func)(
                args
                  .into_iter()
                  .map(|arg| arg.as_expr().unwrap().clone())
                  .collect(),
              );
              return Option::Some(Box::new(EvaluateResultValue::Expr(Box::new(func_result))));
            }
            FunctionType::StylexExprFn(func) => {
              let func_result = (func)(
                args.first().unwrap().clone().as_expr().unwrap().clone(),
                state.traversal_state.clone(),
              );

              state.traversal_state = state.traversal_state.clone().combine(func_result.1);

              return Option::Some(Box::new(EvaluateResultValue::Expr(Box::new(func_result.0))));
            }
            FunctionType::StylexTypeFn(func) => {
              // dbg!(&args);

              let mut fn_args = IndexMap::default();

              let expr = args
                .first()
                .and_then(|expr| expr.as_expr())
                .expect("Argument is not an expression");

              match expr {
                Expr::Object(obj) => {
                  for prop in obj.props.clone() {
                    let prop = prop.as_prop().unwrap();
                    let key_value = prop.as_key_value().unwrap();

                    let key = key_value.key.as_ident().unwrap().sym.to_string();
                    let value = key_value.value.as_lit().unwrap();

                    // dbg!(&key, &value);

                    fn_args.insert(
                      key,
                      ValueWithDefault::String(get_string_val_from_lit(value).unwrap()),
                    );
                  }
                }
                Expr::Lit(lit) => {
                  // dbg!(&lit);
                  fn_args.insert(
                    "default".to_string(),
                    ValueWithDefault::String(get_string_val_from_lit(lit).unwrap()),
                  );
                }
                _ => {}
              }

              // dbg!(&fn_args);

              let func_result = (func)(ValueWithDefault::Map(fn_args));

              let css_type = func_result;
              // dbg!(&css_type);

              return Option::Some(Box::new(EvaluateResultValue::Expr(Box::new(css_type))));
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
                CallbackType::Object(ObjectJS::Entries) => {
                  let Some(Some(eval_result)) = context.first() else {
                    panic!("Object.entries requires an argument")
                  };

                  let EvaluateResultValue::Entries(entries) = eval_result.clone() else {
                    panic!("Object.entries requires an argument")
                  };

                  let mut entry_elems: Vec<Option<ExprOrSpread>> = vec![];

                  for (key, value) in entries {
                    let key: ExprOrSpread = ExprOrSpread {
                      spread: Option::None,
                      expr: Box::new(Expr::Lit(*key.clone())),
                    };

                    let value: ExprOrSpread = ExprOrSpread {
                      spread: Option::None,
                      expr: Box::new(Expr::Lit(*value.clone())),
                    };

                    entry_elems.push(Option::Some(ExprOrSpread {
                      spread: Option::None,
                      expr: Box::new(Expr::Array(ArrayLit {
                        span: DUMMY_SP,
                        elems: vec![Option::Some(key), Option::Some(value)],
                      })),
                    }));
                  }

                  return Option::Some(Box::new(EvaluateResultValue::Expr(Box::new(Expr::Array(
                    ArrayLit {
                      span: DUMMY_SP,
                      elems: entry_elems,
                    },
                  )))));
                }
                CallbackType::Object(ObjectJS::Keys) => {
                  let Some(Some(EvaluateResultValue::Expr(keys))) = context.first() else {
                    panic!("Object.keys requires an argument")
                  };

                  return Option::Some(Box::new(EvaluateResultValue::Expr(keys.clone())));
                }
                CallbackType::Object(ObjectJS::Values) => {
                  let Some(Some(EvaluateResultValue::Expr(values))) = context.first() else {
                    panic!("Object.keys requires an argument")
                  };

                  return Option::Some(Box::new(EvaluateResultValue::Expr(values.clone())));
                }
                CallbackType::Object(ObjectJS::FromEntries) => {
                  let Some(Some(EvaluateResultValue::Entries(entries))) = context.first() else {
                    panic!("Object.fromEntries requires an argument")
                  };

                  let mut entry_elems = vec![];

                  for (key, value) in entries {
                    let ident = if let Lit::Str(lit_str) = key.as_ref() {
                      Ident::new(lit_str.value.clone(), DUMMY_SP)
                    } else {
                      panic!("Expected a string literal")
                    };

                    let prop = PropOrSpread::Prop(Box::new(Prop::KeyValue(KeyValueProp {
                      key: PropName::Ident(ident),
                      value: Box::new(Expr::Lit(*value.clone())),
                    })));

                    entry_elems.push(prop);
                  }

                  return Option::Some(Box::new(EvaluateResultValue::Expr(Box::new(
                    object_expression_factory(entry_elems).expect("Object creation failed"),
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
                        .map(|expr| expr_to_num(expr, &mut state.traversal_state))
                        .expect("All arguments must be a number")
                    })
                    .collect::<Vec<f32>>();

                  let result = num_args.first().unwrap().powf(*num_args.get(1).unwrap());

                  return Option::Some(Box::new(EvaluateResultValue::Expr(Box::new(
                    number_to_expression(result as f64).unwrap(),
                  ))));
                }
                CallbackType::Math(MathJS::Round | MathJS::Floor | MathJS::Ceil) => {
                  let Some(Some(EvaluateResultValue::Expr(expr))) = context.first() else {
                    panic!("Math.(round | ceil | floor) requires an argument")
                  };

                  let num = expr_to_num(expr.as_ref(), &mut state.traversal_state);

                  let result = match func.as_ref() {
                    CallbackType::Math(MathJS::Round) => num.round(),
                    CallbackType::Math(MathJS::Ceil) => num.ceil(),
                    CallbackType::Math(MathJS::Floor) => num.floor(),
                    _ => unreachable!("Invalid function type"),
                  };

                  return Option::Some(Box::new(EvaluateResultValue::Expr(Box::new(
                    number_to_expression(result as f64).unwrap(),
                  ))));
                }
                CallbackType::Math(MathJS::Min | MathJS::Max) => {
                  let Some(Some(EvaluateResultValue::Vec(args))) = context.first() else {
                    panic!("Math.pow requires an argument")
                  };

                  let num_args = args
                    .iter()
                    .flatten()
                    .map(|arg| {
                      arg
                        .as_expr()
                        .map(|expr| expr_to_num(expr, &mut state.traversal_state))
                        .expect("All arguments must be a number")
                    })
                    .collect::<Vec<f32>>();

                  let result = match func.as_ref() {
                    CallbackType::Math(MathJS::Min) => num_args
                      .iter()
                      .cloned()
                      .min_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal)),
                    CallbackType::Math(MathJS::Max) => num_args
                      .iter()
                      .cloned()
                      .max_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal)),
                    _ => unreachable!("Invalid function type"),
                  }
                  .unwrap();

                  return Option::Some(Box::new(EvaluateResultValue::Expr(Box::new(
                    number_to_expression(result as f64).unwrap(),
                  ))));
                }
              }
            }
            _ => panic!("Function type not implemented yet"),
          }
        }
      }

      // let args = call
      //     .args
      //     .clone()
      //     .into_iter()
      //     .map(|a| a.expr.as_ref().clone())
      //     .collect();
      // let elems = (state.functions.identifiers.get("makeArray").unwrap().fn_ptr)(args);
      return deopt(path, state);
    }
    _ => {
      // println!("!!!!__ path_not_implemented: {:#?}", path);
      panic!("Not implemented yet, return something");
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

    // dbg!(&binding);

    match binding {
      Some(binding) => {
        if path.eq(&Expr::Ident(binding.name.as_ident().unwrap().id.clone())) {
          todo!("Check what's happening here")
        }
        eprintln!("{}", Colorize::yellow("!!!! binding: {:#?} !!!!"));
        // dbg!(&binding.init);

        let result = evaluate_cached(
          &Box::new(*binding.init.expect("Binding nof found").clone()),
          state,
        );
        // dbg!(&result);
        return result;
      }
      None => {
        // dbg!(&state.traversal_state.top_imports);

        let name = ident.sym.to_string();

        if name == "undefined" || name == "infinity" || name == "NaN" {
          return Option::Some(Box::new(EvaluateResultValue::Expr(Box::new(Expr::Ident(
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
              Option::Some(import_decl.clone())
            } else {
              Option::None
            }
          });

        match binding {
          Some(import_path) => {
            let import_specifier = import_path
              .specifiers
              .iter()
              .find_map(|import| {
                // dbg!(&import);
                if let Some(name_import) = import.as_named() {
                  if name_import.local.sym.to_string() == ident.sym.to_string() {
                    return Option::Some(name_import.clone());
                  }

                  // match name_import
                  //     .imported
                  //     .clone()
                  //     .unwrap_or(ModuleExportName::Ident(ident.clone()))
                  // {
                  //     ModuleExportName::Ident(import_ident) => {
                  //        // dbg!(&import_ident.sym.to_string(), &ident.sym.to_string());
                  //         if import_ident.sym.to_string() == ident.sym.to_string()
                  //         {
                  //             return Option::Some(name_import.clone());
                  //         }
                  //     }
                  //     ModuleExportName::Str(str) => {
                  //        // dbg!(&str.value, &ident.sym.to_string());
                  //         if str.value == ident.sym.to_string() {
                  //             return Option::Some(name_import.clone());
                  //         }
                  //     }
                  // }
                }
                Option::None
              })
              .expect("Import specifier not found");
            let imported = import_specifier
              .imported
              .clone()
              .unwrap_or(ModuleExportName::Ident(import_specifier.local.clone()));

            let abs_path = &state
              .traversal_state
              .import_path_resolver(&import_path.src.value);

            // dbg!(&import_path.src.value, &abs_path);

            let imported_name = match imported {
              ModuleExportName::Ident(ident) => ident.sym.to_string(),
              ModuleExportName::Str(str) => str.value.clone().to_string(),
            };

            let return_value = match abs_path {
              ImportPathResolution::Tuple(ImportPathResolutionType::ThemeNameRef, value) => {
                evaluate_theme_ref(value, imported_name, &state.traversal_state)
              }
              _ => {
                return deopt(path, state);
              }
            };

            // dbg!(&abs_path, &return_value);

            if state.confident {
              let import_path_src = import_path.src.value.to_string();

              if !state.added_imports.contains(&import_path_src)
                && state.traversal_state.get_treeshake_compensation()
              {
                state
                  .traversal_state
                  .prepend_import_module_items
                  .push(add_import_expression(&import_path_src));

                // dbg!(&state.traversal_state.prepend_include_module_items);
                state.added_imports.insert(import_path_src);
              }

              return Option::Some(Box::new(EvaluateResultValue::ThemeRef(return_value)));
            }
          }
          None => (),
        }
      }
    }
  }

  if result.is_none() {
    return deopt(path, state);
  }

  result
}

fn get_binding(callee: &Box<Expr>, state: &mut StateManager) -> Option<VarDeclarator> {
  match callee.as_ref() {
    Expr::Ident(ident) => get_var_decl_from(state, ident).cloned(),
    _ => Option::None,
  }
}

fn is_valid_callee(callee: &Box<Expr>) -> bool {
  match callee.as_ref() {
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
      Expr::Lit(Lit::Str(str)) => Option::Some(str.value.to_id()),
      _ => Option::None,
    },
    _ => Option::None,
  }
}

pub(crate) fn evaluate_quasis(
  tpl_expr: &Expr,
  quasis: &Vec<TplElement>,
  raw: bool,
  state: &mut State,
) -> Option<Box<EvaluateResultValue>> {
  let mut str = "".to_string();

  let exprs = match tpl_expr {
    Expr::Tpl(tpl) => tpl.exprs.clone(),
    Expr::TaggedTpl(tagged_tpl) => tagged_tpl.tpl.exprs.clone(),
    _ => panic!("The expression is not a template"),
  };

  for (i, elem) in quasis.iter().enumerate() {
    if !state.confident {
      return Option::None;
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
      let evaluated_expr = evaluate_cached(expr, state);

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
    return Option::None;
  };

  Option::Some(Box::new(EvaluateResultValue::Expr(Box::new(
    string_to_expression(str.as_str())?,
  ))))
}

pub(crate) fn evaluate_cached(path: &Expr, state: &mut State) -> Option<Box<EvaluateResultValue>> {
  // dbg!(&state.seen);
  let cleaned_path = drop_span(path.clone());
  let existing = state.traversal_state.seen.get(&cleaned_path);

  match existing {
    Some(value) => {
    // dbg!(&cleaned_path, value);
      if value.resolved {
        let resolved = value.value.clone();
      // dbg!(&resolved);
        return resolved;
      }
      deopt(path, state)
      // value.value.unwrap().clone()
    }
    None => {
      // if cleaned_path.is_bin() {
      // // dbg!(&cleaned_path);
      // }

      let val = _evaluate(&cleaned_path, state);

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
          value: Option::None,
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
