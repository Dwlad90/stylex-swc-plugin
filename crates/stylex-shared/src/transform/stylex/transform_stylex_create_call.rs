use once_cell::sync::Lazy;
use std::{rc::Rc, sync::Arc};
use stylex_path_resolver::package_json::PackageJsonExtended;

use indexmap::IndexMap;
use rustc_hash::FxHashMap;
use swc_core::{
  common::DUMMY_SP,
  ecma::{
    ast::{
      BinExpr, Bool, Decl, KeyValueProp, Lit, ModuleItem, ParenExpr, Stmt, UnaryOp, VarDecl,
      VarDeclKind, VarDeclarator,
    },
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
  constants::{common::COMPILED_KEY, messages::non_static_value},
  enums::data_structures::top_level_expression::TopLevelExpression,
  structures::{
    functions::StylexExprFn,
    injectable_style::InjectableStyle,
    state_manager::StateManager,
    types::InjectableStylesMap,
    uid_generator::{CounterMode, UidGenerator},
  },
  transformers::{stylex_default_maker, stylex_position_try::get_position_try_fn},
  utils::{
    ast::{
      convertors::{atom_to_string, expr_to_str, key_value_to_str, lit_to_string},
      factories::binding_ident_factory,
    },
    common::normalize_expr,
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
  utils::when as stylex_when,
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

/// Lazily-initialized Arc-wrapped map of stylex.when helper functions.
///
/// Thread-safety: Arc ensures safe sharing across threads; Lazy guarantees one-time initialization.
/// Lifecycle: Initialized on first access, immutable thereafter.
/// Contains pure, stateless transformation functions (ancestor, descendant, etc.)
/// that convert expressions to CSS selectors for relational styling.
static STYLEX_WHEN_MAP: Lazy<Arc<IndexMap<String, StylexExprFn>>> = Lazy::new(|| {
  let mut map: IndexMap<String, StylexExprFn> = IndexMap::default();

  map.insert(
    "ancestor".to_string(),
    |expr: Expr, state: &mut StateManager| {
      let result = match stylex_when::ancestor(
        &expr_to_str(&expr, state, &FunctionMap::default()).expect("Expression is not a string"),
        Some(&state.options),
      ) {
        Ok(v) => v,
        Err(e) => panic!("stylex_when::ancestor error: {}", e),
      };
      string_to_expression(&result)
    },
  );

  map.insert(
    "descendant".to_string(),
    |expr: Expr, state: &mut StateManager| {
      let result = match stylex_when::descendant(
        &expr_to_str(&expr, state, &FunctionMap::default()).expect("Expression is not a string"),
        Some(&state.options),
      ) {
        Ok(v) => v,
        Err(e) => panic!("stylex_when::descendant error: {}", e),
      };
      string_to_expression(&result)
    },
  );

  map.insert(
    "siblingBefore".to_string(),
    |expr: Expr, state: &mut StateManager| {
      let result = match stylex_when::sibling_before(
        &expr_to_str(&expr, state, &FunctionMap::default()).expect("Expression is not a string"),
        Some(&state.options),
      ) {
        Ok(v) => v,
        Err(e) => panic!("stylex_when::sibling_before error: {}", e),
      };
      string_to_expression(&result)
    },
  );

  map.insert(
    "siblingAfter".to_string(),
    |expr: Expr, state: &mut StateManager| {
      let result = match stylex_when::sibling_after(
        &expr_to_str(&expr, state, &FunctionMap::default()).expect("Expression is not a string"),
        Some(&state.options),
      ) {
        Ok(v) => v,
        Err(e) => panic!("stylex_when::sibling_after error: {}", e),
      };
      string_to_expression(&result)
    },
  );

  map.insert(
    "anySibling".to_string(),
    |expr: Expr, state: &mut StateManager| {
      let result = match stylex_when::any_sibling(
        &expr_to_str(&expr, state, &FunctionMap::default()).expect("Expression is not a string"),
        Some(&state.options),
      ) {
        Ok(v) => v,
        Err(e) => panic!("stylex_when::any_sibling error: {}", e),
      };
      string_to_expression(&result)
    },
  );

  Arc::new(map)
});

impl<C> StyleXTransform<C>
where
  C: Comments,
{
  pub(crate) fn transform_stylex_create(&mut self, call: &CallExpr) -> Option<Expr> {
    self.state.in_stylex_create = true;
    let mut package_json_seen: FxHashMap<String, PackageJsonExtended> = FxHashMap::default();

    let is_create_call = is_create_call(call, &self.state);

    let result = if is_create_call {
      validate_stylex_create(call, &mut self.state);

      let is_program_level = self
        .state
        .find_top_level_expr(
          call,
          |tpe: &TopLevelExpression| matches!(tpe.1, Expr::Array(_)),
          None,
        )
        .is_some();

      let mut first_arg = call.args.first()?.expr.clone();

      let mut resolved_namespaces: IndexMap<String, Box<FlatCompiledStyles>> = IndexMap::new();
      let mut identifiers: FunctionMapIdentifiers = FxHashMap::default();
      let mut member_expressions: FunctionMapMemberExpression = FxHashMap::default();

      let first_that_works_fn = FunctionConfig {
        fn_ptr: FunctionType::ArrayArgs(stylex_first_that_works),
        takes_path: false,
      };

      let keyframes_fn = get_keyframes_fn();
      let position_try_fn = get_position_try_fn();

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

      for name in &self.state.stylex_position_try_import {
        identifiers.insert(
          name.clone(),
          Box::new(FunctionConfigType::Regular(position_try_fn.clone())),
        );
      }

      for name in &self.state.stylex_default_marker_import {
        identifiers.insert(
          name.clone(),
          Box::new(FunctionConfigType::IndexMap(
            stylex_default_maker::stylex_default_marker(&self.state.options)
              .as_values()
              .expect("Expected FlatCompiledStylesValues")
              .clone(),
          )),
        );
      }

      for name in &self.state.stylex_when_import {
        identifiers.insert(
          name.clone(),
          Box::new(FunctionConfigType::Regular(FunctionConfig {
            fn_ptr: FunctionType::DefaultMarker(Arc::clone(Lazy::force(&STYLEX_WHEN_MAP))),
            takes_path: false,
          })),
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

        member_expression.insert(
          "positionTry".into(),
          Box::new(FunctionConfigType::Regular(position_try_fn.clone())),
        );

        member_expression.insert(
          "defaultMarker".into(),
          Box::new(FunctionConfigType::IndexMap(
            stylex_default_maker::stylex_default_marker(&self.state.options)
              .as_values()
              .expect("Expected FlatCompiledStylesValues")
              .clone(),
          )),
        );

        identifiers
          .entry(name.get_import_str().into())
          .and_modify(|func_type| {
            if let Some(map) = func_type.as_map_mut() {
              map.insert(
                "when".into(),
                FunctionConfig {
                  fn_ptr: FunctionType::DefaultMarker(Arc::clone(Lazy::force(&STYLEX_WHEN_MAP))),
                  takes_path: false,
                },
              );
            }
          })
          .or_insert_with(|| {
            let mut map = FxHashMap::default();
            map.insert(
              "when".into(),
              FunctionConfig {
                fn_ptr: FunctionType::DefaultMarker(Arc::clone(Lazy::force(&STYLEX_WHEN_MAP))),
                takes_path: false,
              },
            );
            Box::new(FunctionConfigType::Map(map))
          });
      }

      let function_map: Box<FunctionMap> = Box::new(FunctionMap {
        identifiers,
        member_expressions,
        disable_imports: false,
      });

      let evaluated_arg =
        evaluate_stylex_create_arg(&mut first_arg, &mut self.state, &function_map);

      assert!(
        evaluated_arg.confident,
        "{}",
        build_code_frame_error(
          &Expr::Call(call.clone()),
          &evaluated_arg.deopt.unwrap_or_else(|| *first_arg.to_owned()),
          evaluated_arg
            .reason
            .as_deref()
            .unwrap_or(&non_static_value("create")),
          &mut self.state,
        )
      );

      let value = evaluated_arg
        .value
        .unwrap_or_else(|| panic!("{}", non_static_value("create")));

      assert!(
        evaluated_arg.confident,
        "{}",
        build_code_frame_error(
          &Expr::Call(call.clone()),
          &evaluated_arg.deopt.unwrap_or_else(|| *first_arg.to_owned()),
          evaluated_arg
            .reason
            .as_deref()
            .unwrap_or(&non_static_value("create")),
          &mut self.state,
        )
      );

      let mut injected_inherit_styles: InjectableStylesMap = IndexMap::default();

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
            InjectableStyle::regular(
              format!(
                "@property {} {{ syntax: \"*\"; {}}}",
                variable_name,
                if is_pseudo_element {
                  ""
                } else {
                  "inherits: false; "
                },
              ),
              Some(0f64),
            ),
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

      let mut injected_styles = self.state.other_injected_css_rules.clone();

      injected_styles.extend(injected_styles_sans_keyframes);

      injected_styles.extend(injected_inherit_styles);

      let (var_name, parent_var_decl) = self.get_call_var_name(call);

      if self.state.is_debug() && self.state.options.enable_debug_data_prop {
        compiled_styles = add_source_map_data(
          &compiled_styles,
          call,
          &mut self.state,
          &mut package_json_seen,
        );
      }

      if self.state.is_dev() && self.state.options.enable_dev_class_names {
        compiled_styles = inject_dev_class_names(&compiled_styles, &var_name, &self.state);
      }

      if self.state.is_test() {
        compiled_styles = convert_to_test_styles(&compiled_styles, &var_name, &self.state);
      }

      if is_program_level && let Some(var_name) = var_name.as_ref() {
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

      let styles_ast =
        convert_object_to_ast(&NestedStringObject::FlatCompiledStyles(compiled_styles));

      let mut result_ast =
        path_replace_hoisted(styles_ast.clone(), is_program_level, &mut self.state);

      if let Some(fns) = evaluated_arg.fns
        && let Some(object) = result_ast.as_object()
      {
        let key_values = get_key_values_from_object(object);

        let props = key_values
            .iter()
            .map(|key_value| {
              let orig_key = key_value_to_str(key_value);
              let mut value = key_value.value.clone();

              let key = match &key_value.key {
                PropName::Ident(ident) => Some(ident.sym.to_string()),
                PropName::Str(strng) => Some(atom_to_string(&strng.value)),
                _ => None,
              };

              let mut prop: Option<PropOrSpread> = None;

              if let Some(key) = key
                && let Some((params, inline_styles)) = fns.get(&key) {
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
                    let mut css_tag_value:Box<Expr> = Box::new(Expr::Lit(Lit::Bool(Bool {
                      span: DUMMY_SP,
                      value: true,
                    })));

                    let mut static_props = vec![];
                    let mut conditional_props = vec![];

                    for prop in value.props.iter_mut() {
                      if let PropOrSpread::Prop(prop) = prop {
                        if let Some(obj_prop) = prop.as_mut_key_value() {
                          let prop_key = match &obj_prop.key {
                            PropName::Ident(ident) => Some(ident.sym.to_string()),
                            PropName::Str(strng) => Some(atom_to_string(&strng.value)),
                            _ => None,
                          };

                          if let Some(prop_key) = prop_key {
                            if prop_key == COMPILED_KEY {
                              css_tag_value = obj_prop.value.clone();
                              continue;
                            }

                            let class_list = obj_prop
                              .value
                              .as_lit()
                              .and_then(lit_to_string)
                              .map(|s| {
                                s.split_whitespace()
                                  .map(str::to_owned)
                                  .collect::<Vec<String>>()
                              })
                              .unwrap_or_default();

                            if !class_list.is_empty() {
                              let mut is_static = true;
                              let mut expr_list = vec![];

                              // Pre-calculate class strings with spaces to avoid repeated allocations
                              let class_strings: Vec<String> = class_list
                                .iter()
                                .enumerate()
                                .map(|(index, cls)| {
                                  if index == class_list.len() - 1 {
                                    cls.clone()
                                  } else {
                                    format!("{} ", cls)
                                  }
                                })
                                .collect();

                              for (index, cls) in class_list.iter().enumerate() {
                                let expr = dynamic_styles
                                  .iter()
                                  .find(|dynamic_style| {
                                    orig_class_paths.get(cls) == Some(&dynamic_style.path)
                                  })
                                  .map(|dynamic_style| dynamic_style.expression.clone());

                                let cls_with_space = &class_strings[index];

                                if let Some(expr) = expr.and_then(|mut e| {
                                  if is_safe_to_skip_null_check(&mut e) {
                                    None
                                  } else {
                                    Some(e)
                                  }
                                }) {
                                  is_static = false;
                                  expr_list.push(Expr::Cond(CondExpr {
                                    span: DUMMY_SP,
                                    test: Box::new(Expr::Bin(BinExpr {
                                      span: DUMMY_SP,
                                      op: BinaryOp::NotEq,
                                      left: Box::new(expr.clone()),
                                      right: Box::new(null_to_expression()),
                                    })),
                                    cons: Box::new(string_to_expression(cls_with_space)),
                                    alt: Box::new(expr),
                                  }));
                                } else {
                                  expr_list.push(string_to_expression(cls_with_space));
                                }
                              }

                              let joined = if expr_list.is_empty() {
                                string_to_expression("")
                              } else {
                                expr_list
                                  .into_iter()
                                  .reduce(|acc, curr| {
                                    Expr::Bin(BinExpr {
                                      span: DUMMY_SP,
                                      op: BinaryOp::Add,
                                      left: Box::new(acc),
                                      right: Box::new(curr),
                                    })
                                  })
                                  .unwrap()
                              };

                              if is_static {
                                static_props.push(PropOrSpread::Prop(Box::new(Prop::KeyValue(
                                  KeyValueProp {
                                    key: obj_prop.key.clone(),
                                    value: Box::new(joined),
                                  },
                                ))));
                              } else {
                                conditional_props.push(PropOrSpread::Prop(Box::new(Prop::KeyValue(
                                  KeyValueProp {
                                    key: obj_prop.key.clone(),
                                    value: Box::new(joined),
                                  },
                                ))));
                              }
                            }
                          } else {
                            static_props.push(PropOrSpread::Prop(Box::new(Prop::from(
                                obj_prop.to_owned(),
                              ))));
                              continue;
                          }
                        } else {
                          let expr = Expr::from(call.clone());

                          build_code_frame_error_and_panic(
                            &expr,
                            &expr,
                            "Unsupported prop type encountered in stylex.create. Only object properties are allowed.",
                            &mut self.state,
                          );
                        }
                      } else {
                        let expr = Expr::from(call.clone());

                        build_code_frame_error_and_panic(
                          &expr,
                          &expr,
                          "Unsupported prop type encountered in stylex.create. Only object properties are allowed.",
                          &mut self.state,
                        );
                      }
                    }

                    let mut static_obj = None;
                    let mut conditional_obj = None;

                    if !static_props.is_empty(){
                      static_props.push(prop_or_spread_expression_factory(
                        COMPILED_KEY,
                        *css_tag_value.clone(),
                      ));

                      static_obj = Some(object_expression_factory(static_props));
                    }

                    if !conditional_props.is_empty(){
                      conditional_props.push(prop_or_spread_expression_factory(
                        COMPILED_KEY,
                        *css_tag_value.clone(),
                      ));

                      conditional_obj = Some(object_expression_factory(conditional_props.clone()));
                    }

                    let mut final_fn_value = object_expression_factory(
                      inline_styles
                        .iter()
                        .map(|(key, val)| {
                          prop_or_spread_expression_factory(
                            key.as_str(),
                            val.expression.clone(),
                          )
                        })
                        .collect(),
                    );

                    if static_obj.is_some() || conditional_obj.is_some() {
                      let mut array_elements = Vec::new();

                      if let Some(static_obj) = static_obj {
                        array_elements.push(Some(ExprOrSpread {
                          spread: None,
                          expr: Box::new(hoist_expression(static_obj, &mut self.state)),
                        }));
                      }

                      if let Some(conditional_obj) = conditional_obj {
                        array_elements.push(Some(ExprOrSpread {
                          spread: None,
                          expr: Box::new(conditional_obj),
                        }));
                      }

                      array_elements.push(Some(ExprOrSpread {
                        spread: None,
                        expr: Box::new(final_fn_value),
                      }));

                      final_fn_value = array_expression_factory(array_elements);
                    }

                    value.props = conditional_props;

                    let value = Expr::from(ArrowExpr {
                      span: DUMMY_SP,
                      params: params.iter().map(|arg| Pat::Ident(arg.clone())).collect(),
                      body: Box::new(BlockStmtOrExpr::from(Box::new(final_fn_value))),
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

        result_ast = path_replace_hoisted(
          object_expression_factory(props),
          is_program_level,
          &mut self.state,
        );
      };

      self.state.register_styles(
        call,
        &injected_styles,
        &result_ast,
        (!result_ast.eq(&styles_ast)).then_some(&styles_ast),
      );

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

fn is_safe_to_skip_null_check(expr: &mut Expr) -> bool {
  let expr = normalize_expr(expr);

  match expr {
    Expr::Tpl(_) => true,
    Expr::Lit(lit) => matches!(lit, Lit::Str(_) | Lit::Num(_) | Lit::Bool(_)),
    Expr::Bin(bin_expr) => match bin_expr.op {
      BinaryOp::Add
      | BinaryOp::Sub
      | BinaryOp::Mul
      | BinaryOp::Div
      | BinaryOp::Mod
      | BinaryOp::Exp => true,
      BinaryOp::NullishCoalescing | BinaryOp::LogicalOr => {
        is_safe_to_skip_null_check(&mut bin_expr.left)
          || is_safe_to_skip_null_check(&mut bin_expr.right)
      }
      BinaryOp::LogicalAnd => {
        is_safe_to_skip_null_check(&mut bin_expr.left)
          && is_safe_to_skip_null_check(&mut bin_expr.right)
      }
      _ => false,
    },
    Expr::Unary(unary_expr) => matches!(unary_expr.op, UnaryOp::Minus | UnaryOp::Plus),
    Expr::Cond(cond_expr) => {
      is_safe_to_skip_null_check(&mut cond_expr.cons)
        && is_safe_to_skip_null_check(&mut cond_expr.alt)
    }
    _ => false,
  }
}

/// Hoists an expression to the program level by creating a const variable declaration.
/// This is the Rust equivalent of the JavaScript `hoistExpression` function.
///
/// # Arguments
/// * `ast_expression` - The expression to hoist
/// * `state` - The state manager to add the hoisted declaration to
///
/// # Returns
/// An identifier referencing the hoisted variable
pub(crate) fn hoist_expression(
  ast_expression: Expr,
  state: &mut crate::shared::structures::state_manager::StateManager,
) -> Expr {
  let uid_generator = UidGenerator::new("temp", CounterMode::ThreadLocal);
  let hoisted_ident = uid_generator.generate_ident();

  let var_decl = VarDecl {
    span: DUMMY_SP,
    kind: VarDeclKind::Const,
    declare: false,
    decls: vec![VarDeclarator {
      span: DUMMY_SP,
      name: Pat::Ident(binding_ident_factory(hoisted_ident.clone())),
      init: Some(Box::new(ast_expression)),
      definite: false,
    }],
    ctxt: swc_core::common::SyntaxContext::empty(),
  };

  let module_item = ModuleItem::Stmt(Stmt::Decl(Decl::Var(Box::new(var_decl))));
  state.hoisted_module_items.push(module_item);

  Expr::Ident(hoisted_ident)
}

pub(crate) fn path_replace_hoisted(
  ast_expression: Expr,
  is_program_level: bool,
  state: &mut crate::shared::structures::state_manager::StateManager,
) -> Expr {
  if is_program_level {
    return ast_expression;
  }

  let uid_generator = UidGenerator::new("styles", CounterMode::ThreadLocal);
  let name_ident = uid_generator.generate_ident();

  let var_decl = VarDecl {
    span: DUMMY_SP,
    kind: VarDeclKind::Const,
    declare: false,
    decls: vec![VarDeclarator {
      span: DUMMY_SP,
      name: Pat::Ident(binding_ident_factory(name_ident.clone())),
      init: Some(Box::new(ast_expression)),
      definite: false,
    }],
    ctxt: swc_core::common::SyntaxContext::empty(),
  };

  let module_item = ModuleItem::Stmt(Stmt::Decl(Decl::Var(Box::new(var_decl))));
  state.hoisted_module_items.push(module_item);

  Expr::Ident(name_ident)
}
