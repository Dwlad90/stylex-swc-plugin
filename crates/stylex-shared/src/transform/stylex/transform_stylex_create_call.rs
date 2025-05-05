use std::rc::Rc;

use indexmap::IndexMap;
use rustc_hash::FxHashMap;
use swc_core::{
  common::DUMMY_SP,
  ecma::{
    ast::{BinExpr, ParenExpr},
    utils::drop_span,
  },
};
use swc_core::{
  common::SyntaxContext,
  ecma::ast::{ArrowExpr, BinaryOp, BlockStmtOrExpr, CondExpr, ExprOrSpread, Pat, Prop, PropName},
};
use swc_core::{
  common::comments::Comments,
  ecma::ast::{CallExpr, Expr, PropOrSpread},
};

use crate::shared::{
  constants::messages::NON_STATIC_VALUE,
  structures::injectable_style::InjectableStyle,
  utils::{
    ast::convertors::{key_value_to_str, lit_to_string},
    core::{
      add_source_map_data::add_source_map_data,
      dev_class_name::{convert_to_test_styles, inject_dev_class_names},
    },
    log::build_code_frame_error::build_code_frame_error,
  },
};
use crate::shared::{
  structures::functions::{FunctionConfig, FunctionMap, FunctionType},
  transformers::{
    stylex_create::stylex_create_set, stylex_first_that_works::stylex_first_that_works,
    stylex_keyframes::get_keyframes_fn,
  },
};
use crate::shared::{
  structures::state::EvaluationState,
  utils::validators::{is_create_call, validate_stylex_create},
};
use crate::shared::{
  structures::types::{FlatCompiledStyles, FunctionMapMemberExpression},
  utils::core::evaluate_stylex_create_arg::evaluate_stylex_create_arg,
};
use crate::shared::{
  structures::{dynamic_style::DynamicStyle, stylex_options::StyleResolution},
  utils::{
    ast::{convertors::null_to_expression, factories::array_expression_factory},
    core::js_to_expr::{NestedStringObject, convert_object_to_ast, remove_objects_with_spreads},
  },
};
use crate::shared::{
  structures::{functions::FunctionConfigType, types::FunctionMapIdentifiers},
  utils::ast::factories::prop_or_spread_expression_factory,
};
use crate::shared::{
  structures::{
    order_pair::OrderPair, pre_rule::PreRuleValue, stylex_state_options::StyleXStateOptions,
  },
  utils::{
    ast::{convertors::string_to_expression, factories::object_expression_factory},
    common::get_key_values_from_object,
    core::flat_map_expanded_shorthands::flat_map_expanded_shorthands,
  },
};
use crate::{
  StyleXTransform, shared::utils::log::build_code_frame_error::build_code_frame_error_and_panic,
};

impl<C> StyleXTransform<C>
where
  C: Comments,
{
  pub(crate) fn transform_stylex_create(&mut self, call: &CallExpr) -> Option<Expr> {
    self.state.in_stylex_create = true;
    let is_create_call = is_create_call(call, &self.state);

    let result = if is_create_call {
      validate_stylex_create(call, &mut self.state);

      let mut first_arg = call.args.first()?.expr.clone();

      let mut resolved_namespaces: IndexMap<String, Box<FlatCompiledStyles>> = IndexMap::new();
      let mut identifiers: FunctionMapIdentifiers = FxHashMap::default();
      let mut member_expressions: FunctionMapMemberExpression = FxHashMap::default();

      let first_that_works_fn = FunctionConfig {
        fn_ptr: FunctionType::ArrayArgs(stylex_first_that_works),
        takes_path: false,
      };

      let keyframes_fn = get_keyframes_fn();

      for name in &self.state.stylex_first_that_works_import {
        identifiers.insert(
          name.clone(),
          Box::new(FunctionConfigType::Regular(first_that_works_fn.clone())),
        );
      }

      for name in &self.state.stylex_keyframes_import {
        identifiers.insert(
          name.clone(),
          Box::new(FunctionConfigType::Regular(keyframes_fn.clone())),
        );
      }

      for name in &self.state.stylex_import {
        member_expressions.entry(name.clone()).or_default();

        let member_expression = member_expressions.get_mut(name).unwrap();

        member_expression.insert(
          "firstThatWorks".into(),
          Box::new(FunctionConfigType::Regular(first_that_works_fn.clone())),
        );

        member_expression.insert(
          "keyframes".into(),
          Box::new(FunctionConfigType::Regular(keyframes_fn.clone())),
        );
      }

      let function_map: Box<FunctionMap> = Box::new(FunctionMap {
        identifiers,
        member_expressions,
      });

      let evaluated_arg =
        evaluate_stylex_create_arg(&mut first_arg, &mut self.state, &function_map);

      assert!(
        evaluated_arg.confident,
        "{}",
        build_code_frame_error(
          &Expr::Call(call.clone()),
          &evaluated_arg.deopt.unwrap_or_else(|| *first_arg.to_owned()),
          evaluated_arg.reason.as_deref().unwrap_or(NON_STATIC_VALUE),
          &mut self.state,
        )
      );

      let value = evaluated_arg.value.expect(NON_STATIC_VALUE);

      assert!(
        evaluated_arg.confident,
        "{}",
        build_code_frame_error(
          &Expr::Call(call.clone()),
          &evaluated_arg.deopt.unwrap_or_else(|| *first_arg.to_owned()),
          evaluated_arg.reason.as_deref().unwrap_or(NON_STATIC_VALUE),
          &mut self.state,
        )
      );

      let mut injected_inherit_styles: IndexMap<String, Rc<InjectableStyle>> = IndexMap::default();

      if let Some(fns) = &evaluated_arg.fns {
        let dynamic_fns_names = fns
          .values()
          .flat_map(|(_, map)| {
            map.keys().map(|k| {
              let path = map.get(k).map(|p| p.path.clone()).unwrap_or_default();

              (k.clone(), path)
            })
          })
          .collect::<Vec<(String, Vec<String>)>>();

        for (variable_name, paths) in dynamic_fns_names {
          // Pseudo elements can only access css vars via inheritance
          let is_pseudo_element = paths.iter().any(|path| path.starts_with(':'));

          injected_inherit_styles.insert(
            variable_name.clone(),
            Rc::new(InjectableStyle {
              priority: Some(0f64),
              ltr: format!(
                "@property {} {{ syntax: \"*\"; {}}}",
                variable_name,
                if is_pseudo_element {
                  ""
                } else {
                  "inherits: false; "
                },
              ),
              rtl: None,
            }),
          );
        }
      }

      let (mut compiled_styles, injected_styles_sans_keyframes, class_paths_per_namespace) =
        stylex_create_set(
          &value,
          &mut EvaluationState::new(),
          &mut self.state,
          &function_map,
        );

      for (namespace, properties) in compiled_styles.iter() {
        resolved_namespaces
          .entry(namespace.clone())
          .or_default()
          .extend(properties.iter().map(|(k, v)| (k.clone(), v.clone())));
      }

      let mut injected_styles = self.state.injected_keyframes.clone();

      injected_styles.extend(injected_styles_sans_keyframes);

      injected_styles.extend(injected_inherit_styles);

      let (var_name, parent_var_decl) = self.get_call_var_name(call);

      if self.state.is_debug() && self.state.options.enable_debug_data_prop {
        compiled_styles = add_source_map_data(&compiled_styles, call, &mut self.state);
      }

      if self.state.is_dev() && self.state.options.enable_dev_class_names {
        compiled_styles = inject_dev_class_names(&compiled_styles, &var_name, &self.state);
      }

      if self.state.is_test() {
        compiled_styles = convert_to_test_styles(&compiled_styles, &var_name, &self.state);
      }

      if let Some(var_name) = var_name.as_ref() {
        let styles_to_remember = remove_objects_with_spreads(&compiled_styles);

        self
          .state
          .style_map
          .insert(var_name.clone(), Rc::new(styles_to_remember));

        if let Some(parent_var_decl) = parent_var_decl {
          self
            .state
            .style_vars
            .insert(var_name.clone(), drop_span(parent_var_decl.clone()));
        } else {
          let call_expr = Expr::Call(call.clone());

          build_code_frame_error_and_panic(
            &Expr::Paren(ParenExpr {
              span: DUMMY_SP,
              expr: Box::new(call_expr.clone()),
            }),
            &call_expr,
            "Function type",
            &mut self.state,
          )
        }
      }

      let mut result_ast =
        convert_object_to_ast(&NestedStringObject::FlatCompiledStyles(compiled_styles));

      if let Some(fns) = evaluated_arg.fns {
        if let Some(object) = result_ast.as_object() {
          let key_values = get_key_values_from_object(object);

          let props = key_values
            .iter()
            .map(|key_value| {
              let orig_key = key_value_to_str(key_value);
              let mut value = key_value.value.clone();

              let key = match &key_value.key {
                PropName::Ident(ident) => Some(ident.sym.to_string()),
                PropName::Str(strng) => Some(strng.value.to_string()),
                _ => None,
              };

              let mut prop: Option<PropOrSpread> = None;

              if let Some(key) = key {
                if let Some((params, inline_styles)) = fns.get(&key) {
                  let mut orig_class_paths = IndexMap::new();

                  if let Some(namespace) = class_paths_per_namespace.get(&key) {
                    for (class_name, class_paths) in namespace.iter() {
                      orig_class_paths.insert(class_name.clone(), class_paths.join("_"));
                    }
                  }

                  let mut dynamic_styles: Vec<DynamicStyle> = inline_styles
                    .iter()
                    .map(|(_key, v)| {
                      let key = v
                        .path
                        .iter()
                        .take(
                          v.path
                            .iter()
                            .position(|p| !p.starts_with(':') && !p.starts_with('@'))
                            .map_or(0, |index| index + 1),
                        )
                        .cloned()
                        .collect::<Vec<String>>()
                        .join("_");

                      DynamicStyle {
                        expression: v.original_expression.clone(),
                        key,
                        path: v.path.join("_"),
                      }
                    })
                    .collect();

                  if self.state.options.style_resolution == StyleResolution::LegacyExpandShorthands
                  {
                    dynamic_styles = legacy_expand_shorthands(dynamic_styles);
                  }

                  if let Some(value) = value.as_mut_object() {
                    value.props = value
                      .props
                      .iter_mut()
                      .map(|prop| {
                        if let PropOrSpread::Prop(prop) = prop {
                          if let Some(obj_prop) = prop.as_mut_key_value() {
                            let prop_key = match &obj_prop.key {
                              PropName::Ident(ident) => Some(ident.sym.to_string()),
                              PropName::Str(strng) => Some(strng.value.to_string()),
                              _ => None,
                            };

                            if let Some(prop_key) = prop_key {
                              let dynamic_match = dynamic_styles
                                .iter()
                                .filter(|dynamic_style| dynamic_style.key == prop_key)
                                .cloned()
                                .collect::<Vec<DynamicStyle>>();

                              if !dynamic_match.is_empty() {
                                let value = &obj_prop.value;

                                if let Expr::Lit(lit) = value.as_ref() {
                                  let class_names_string =
                                    lit_to_string(lit).expect("Lit cannot be stringified");

                                  let class_list =
                                    class_names_string.split(' ').collect::<Vec<&str>>();

                                  if class_list.len() == 1 {
                                    let cls = class_list.first().unwrap();

                                    let expr = dynamic_match
                                      .iter()
                                      .find(|dynamic_style| {
                                        orig_class_paths.get(&cls.to_string())
                                          == Some(&dynamic_style.path)
                                      })
                                      .map(|dynamic_style| {
                                        let expression = &dynamic_style.expression;

                                        expression.clone()
                                      });

                                    if let Some(expr) = expr {
                                      obj_prop.value = Box::new(Expr::Cond(CondExpr {
                                        span: DUMMY_SP,
                                        test: Box::new(Expr::Bin(BinExpr {
                                          span: DUMMY_SP,
                                          op: BinaryOp::EqEq,
                                          left: Box::new(expr.clone()),
                                          right: Box::new(null_to_expression()),
                                        })),
                                        cons: Box::new(null_to_expression()),
                                        alt: value.clone(),
                                      }));
                                    }
                                  } else if class_list.iter().any(|cls| {
                                    dynamic_match.iter().any(|dynamic_style| {
                                      orig_class_paths.get(&cls.to_string())
                                        == Some(&dynamic_style.path)
                                    })
                                  }) {
                                    let expr_array: Vec<Expr> = class_list
                                      .iter()
                                      .enumerate()
                                      .map(|(index, cls)| {
                                        let expr = dynamic_match
                                          .iter()
                                          .find(|dynamic_style| {
                                            orig_class_paths.get(&cls.to_string())
                                              == Some(&dynamic_style.path)
                                          })
                                          .map(|dynamic_style| dynamic_style.expression.clone());

                                        let suffix = if index == class_list.len() - 1 {
                                          ""
                                        } else {
                                          " "
                                        };

                                        match expr {
                                          Some(expr) => Expr::Cond(CondExpr {
                                            span: DUMMY_SP,
                                            test: Box::new(Expr::Bin(BinExpr {
                                              span: DUMMY_SP,
                                              op: BinaryOp::EqEq,
                                              left: Box::new(expr.clone()),
                                              right: Box::new(null_to_expression()),
                                            })),
                                            cons: Box::new(
                                              string_to_expression(Default::default()),
                                            ),
                                            alt: Box::new(string_to_expression(
                                              format!("{}{}", cls, suffix).as_str(),
                                            )),
                                          }),
                                          _ => string_to_expression(
                                            format!("{}{}", cls, suffix).as_str(),
                                          ),
                                        }
                                      })
                                      .collect();

                                    let (first, rest) =
                                      expr_array.split_first().expect("Expression array is empty");

                                    let reduced_expr =
                                      rest.iter().fold(first.clone(), |acc, curr| {
                                        Expr::Bin(BinExpr {
                                          span: DUMMY_SP,
                                          op: BinaryOp::Add,
                                          left: Box::new(acc),
                                          right: Box::new(curr.clone()),
                                        })
                                      });
                                    obj_prop.value = Box::new(reduced_expr);
                                  }
                                }
                              }
                            }

                            PropOrSpread::Prop(Box::new(Prop::from(obj_prop.to_owned())))
                          } else {
                            PropOrSpread::Prop(prop.to_owned())
                          }
                        } else {
                          prop.to_owned()
                        }
                      })
                      .collect::<Vec<PropOrSpread>>()
                  }

                  let value = Expr::from(ArrowExpr {
                    span: DUMMY_SP,
                    params: params.iter().map(|arg| Pat::Ident(arg.clone())).collect(),
                    body: Box::new(BlockStmtOrExpr::from(Box::new(array_expression_factory(
                      vec![
                        Some(ExprOrSpread {
                          spread: None,
                          expr: Box::new(*value.clone()),
                        }),
                        Some(ExprOrSpread {
                          spread: None,
                          expr: Box::new(object_expression_factory(
                            inline_styles
                              .iter()
                              .map(|(key, value)| {
                                prop_or_spread_expression_factory(
                                  key.as_str(),
                                  value.expression.clone(),
                                )
                              })
                              .collect(),
                          )),
                        }),
                      ],
                    )))),
                    is_async: false,
                    is_generator: false,
                    type_params: None,
                    return_type: None,
                    ctxt: SyntaxContext::empty(),
                  });

                  prop = Some(prop_or_spread_expression_factory(orig_key.as_str(), value));
                }
              }

              prop.unwrap_or_else(|| {
                prop_or_spread_expression_factory(orig_key.as_str(), *value.clone())
              })
            })
            .collect::<Vec<PropOrSpread>>();

          result_ast = object_expression_factory(props);
        }
      };
      self
        .state
        .register_styles(call, &injected_styles, &result_ast);

      Some(result_ast)
    } else {
      None
    };

    self.state.in_stylex_create = false;

    result
  }
}

fn legacy_expand_shorthands(dynamic_styles: Vec<DynamicStyle>) -> Vec<DynamicStyle> {
  let expanded_keys_to_key_paths: Vec<DynamicStyle> = dynamic_styles
    .iter()
    .enumerate()
    .flat_map(|(i, dynamic_style)| {
      let obj_entry = (
        dynamic_style.key.clone(),
        PreRuleValue::String(format!("p{}", i)),
      );

      let options = StyleXStateOptions {
        style_resolution: StyleResolution::LegacyExpandShorthands,
        ..Default::default()
      };

      flat_map_expanded_shorthands(obj_entry, &options)
    })
    .filter_map(|OrderPair(key, value)| {
      let value = value?;

      let index = value[1..].parse::<usize>().ok()?;
      let that_dyn_style = dynamic_styles.get(index)?;

      Some(DynamicStyle {
        key: key.clone(),
        path: if that_dyn_style.path == that_dyn_style.key {
          key.clone()
        } else if that_dyn_style
          .path
          .contains(&(that_dyn_style.key.clone() + "_"))
        {
          that_dyn_style
            .path
            .replace(&(that_dyn_style.key.clone() + "_"), &(key.clone() + "_"))
        } else {
          that_dyn_style.path.replace(
            &("_".to_string() + that_dyn_style.key.as_str()),
            &("_".to_string() + key.as_str()),
          )
        },
        ..that_dyn_style.clone()
      })
    })
    .collect();

  expanded_keys_to_key_paths
}
