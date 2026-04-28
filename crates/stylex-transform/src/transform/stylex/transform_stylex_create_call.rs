use std::{
  fmt::Write,
  rc::Rc,
  sync::{Arc, LazyLock},
};
use stylex_macros::stylex_panic;
use stylex_path_resolver::package_json::PackageJsonExtended;

use indexmap::IndexMap;
use rustc_hash::FxHashMap;
use swc_core::{
  common::{DUMMY_SP, SyntaxContext, comments::Comments},
  ecma::{
    ast::{
      ArrowExpr, BinaryOp, BlockStmtOrExpr, Bool, CallExpr, Decl, Expr, Lit, ModuleItem, ParenExpr,
      Pat, Prop, PropName, PropOrSpread, Stmt, UnaryOp, VarDecl, VarDeclKind,
    },
    utils::drop_span,
  },
};

use crate::{
  shared::{
    structures::{
      functions::{FunctionConfig, FunctionConfigType, FunctionMap, FunctionType, StylexExprFn},
      pre_rule::PreRuleValue,
      state::EvaluationState,
      state_manager::ImportKind,
      types::{
        FlatCompiledStyles, FunctionMapIdentifiers, FunctionMapMemberExpression,
        InjectableStylesMap,
      },
    },
    transformers::{
      stylex_create::stylex_create_set, stylex_default_marker,
      stylex_first_that_works::stylex_first_that_works, stylex_keyframes::get_keyframes_fn,
      stylex_position_try::get_position_try_fn,
    },
    utils::{
      ast::convertors::{
        convert_atom_to_string, convert_expr_to_str, convert_key_value_to_str,
        convert_lit_to_string, create_null_expr, create_string_expr,
      },
      common::{
        downcast_style_options_to_state_manager, get_key_values_from_object, normalize_expr,
      },
      core::{
        add_source_map_data::add_source_map_data,
        dev_class_name::{convert_to_test_styles, inject_dev_class_names},
        evaluate_stylex_create_arg::evaluate_stylex_create_arg,
        flat_map_expanded_shorthands::flat_map_expanded_shorthands,
        js_to_ast::{NestedStringObject, convert_object_to_ast, remove_objects_with_spreads},
      },
      log::build_code_frame_error::{build_code_frame_error, build_code_frame_error_and_panic},
      validators::{is_create_call, validate_stylex_create},
    },
  },
  transform::StyleXTransform,
};
use stylex_ast::ast::factories::{
  create_array_expression, create_bin_expr, create_cond_expr, create_expr_or_spread,
  create_key_value_prop, create_object_expression, create_prop_from_name,
  create_string_var_declarator, create_var_declarator,
};
use stylex_constants::constants::{
  api_names::{
    STYLEX_CREATE, STYLEX_DEFAULT_MARKER, STYLEX_FIRST_THAT_WORKS, STYLEX_KEYFRAMES,
    STYLEX_POSITION_TRY, STYLEX_WHEN,
  },
  common::COMPILED_KEY,
  messages::{EXPECTED_COMPILED_STYLES, non_static_value},
};
use stylex_css::utils::when as stylex_when;
use stylex_enums::{counter_mode::CounterMode, style_resolution::StyleResolution};
use stylex_regex::regex::VAR_EXTRACTION_REGEX;
use stylex_structures::{
  dynamic_style::DynamicStyle, order_pair::OrderPair, stylex_state_options::StyleXStateOptions,
  top_level_expression::TopLevelExpression, uid_generator::UidGenerator,
};
use stylex_types::{
  enums::data_structures::injectable_style::InjectableStyleKind,
  structures::injectable_style::InjectableStyle,
};

/// Lazily-initialized Arc-wrapped map of stylex.when helper functions.
///
/// Thread-safety: Arc ensures safe sharing across threads; Lazy guarantees
/// one-time initialization. Lifecycle: Initialized on first access, immutable
/// thereafter. Contains pure, stateless transformation functions (ancestor,
/// descendant, etc.) that convert expressions to CSS selectors for relational
/// styling.
static STYLEX_WHEN_MAP: LazyLock<Arc<IndexMap<String, StylexExprFn>>> = LazyLock::new(|| {
  let mut map: IndexMap<String, StylexExprFn> = IndexMap::default();

  map.insert(
    "ancestor".to_string(),
    |expr: Expr, state: &mut dyn stylex_types::traits::StyleOptions| {
      let state = downcast_style_options_to_state_manager(state);
      let expr_str = match convert_expr_to_str(&expr, state, &FunctionMap::default()) {
        Some(s) => s,
        #[cfg_attr(coverage_nightly, coverage(off))]
        None => stylex_panic!("stylex.when ancestor: expression is not a string"),
      };
      let result = match stylex_when::ancestor(&expr_str, Some(&state.options)) {
        Ok(v) => v,
        #[cfg_attr(coverage_nightly, coverage(off))]
        Err(e) => stylex_panic!("stylex.when ancestor error: {}", e),
      };
      create_string_expr(&result)
    },
  );

  map.insert(
    "descendant".to_string(),
    |expr: Expr, state: &mut dyn stylex_types::traits::StyleOptions| {
      let state = downcast_style_options_to_state_manager(state);
      let expr_str = match convert_expr_to_str(&expr, state, &FunctionMap::default()) {
        Some(s) => s,
        #[cfg_attr(coverage_nightly, coverage(off))]
        None => stylex_panic!("stylex.when descendant: expression is not a string"),
      };
      let result = match stylex_when::descendant(&expr_str, Some(&state.options)) {
        Ok(v) => v,
        #[cfg_attr(coverage_nightly, coverage(off))]
        Err(e) => stylex_panic!("stylex.when descendant error: {}", e),
      };
      create_string_expr(&result)
    },
  );

  map.insert(
    "siblingBefore".to_string(),
    |expr: Expr, state: &mut dyn stylex_types::traits::StyleOptions| {
      let state = downcast_style_options_to_state_manager(state);
      let expr_str = match convert_expr_to_str(&expr, state, &FunctionMap::default()) {
        Some(s) => s,
        #[cfg_attr(coverage_nightly, coverage(off))]
        None => stylex_panic!("stylex.when siblingBefore: expression is not a string"),
      };
      let result = match stylex_when::sibling_before(&expr_str, Some(&state.options)) {
        Ok(v) => v,
        #[cfg_attr(coverage_nightly, coverage(off))]
        Err(e) => stylex_panic!("stylex.when siblingBefore error: {}", e),
      };
      create_string_expr(&result)
    },
  );

  map.insert(
    "siblingAfter".to_string(),
    |expr: Expr, state: &mut dyn stylex_types::traits::StyleOptions| {
      let state = downcast_style_options_to_state_manager(state);
      let expr_str = match convert_expr_to_str(&expr, state, &FunctionMap::default()) {
        Some(s) => s,
        #[cfg_attr(coverage_nightly, coverage(off))]
        None => stylex_panic!("stylex.when siblingAfter: expression is not a string"),
      };
      let result = match stylex_when::sibling_after(&expr_str, Some(&state.options)) {
        Ok(v) => v,
        #[cfg_attr(coverage_nightly, coverage(off))]
        Err(e) => stylex_panic!("stylex.when siblingAfter error: {}", e),
      };
      create_string_expr(&result)
    },
  );

  map.insert(
    "anySibling".to_string(),
    |expr: Expr, state: &mut dyn stylex_types::traits::StyleOptions| {
      let state = downcast_style_options_to_state_manager(state);
      let expr_str = match convert_expr_to_str(&expr, state, &FunctionMap::default()) {
        Some(s) => s,
        #[cfg_attr(coverage_nightly, coverage(off))]
        None => stylex_panic!("stylex.when anySibling: expression is not a string"),
      };
      let result = match stylex_when::any_sibling(&expr_str, Some(&state.options)) {
        Ok(v) => v,
        #[cfg_attr(coverage_nightly, coverage(off))]
        Err(e) => stylex_panic!("stylex.when anySibling error: {}", e),
      };
      create_string_expr(&result)
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

      if let Some(set) = self.state.get_stylex_api_import(ImportKind::FirstThatWorks) {
        for name in set {
          identifiers.insert(
            name.clone(),
            Box::new(FunctionConfigType::Regular(first_that_works_fn.clone())),
          );
        }
      }

      if let Some(set) = self.state.get_stylex_api_import(ImportKind::Keyframes) {
        for name in set {
          identifiers.insert(
            name.clone(),
            Box::new(FunctionConfigType::Regular(keyframes_fn.clone())),
          );
        }
      }

      if let Some(set) = self.state.get_stylex_api_import(ImportKind::PositionTry) {
        for name in set {
          identifiers.insert(
            name.clone(),
            Box::new(FunctionConfigType::Regular(position_try_fn.clone())),
          );
        }
      }

      if let Some(set) = self.state.get_stylex_api_import(ImportKind::DefaultMarker) {
        for name in set {
          identifiers.insert(
            name.clone(),
            Box::new(FunctionConfigType::IndexMap(
              stylex_default_marker::stylex_default_marker(&self.state.options)
                .as_values()
                .unwrap_or_else(|| stylex_panic!("{}", EXPECTED_COMPILED_STYLES))
                .clone(),
            )),
          );
        }
      }

      if let Some(set) = self.state.get_stylex_api_import(ImportKind::When) {
        for name in set {
          identifiers.insert(
            name.clone(),
            Box::new(FunctionConfigType::Regular(FunctionConfig {
              fn_ptr: FunctionType::DefaultMarker(Arc::clone(LazyLock::force(&STYLEX_WHEN_MAP))),
              takes_path: false,
            })),
          );
        }
      }

      for name in &self.state.stylex_import {
        member_expressions.entry(name.clone()).or_default();

        let member_expression = match member_expressions.get_mut(name) {
          Some(me) => me,
          #[cfg_attr(coverage_nightly, coverage(off))]
          None => stylex_panic!("Could not resolve the member expression for the StyleX import."),
        };

        member_expression.insert(
          STYLEX_FIRST_THAT_WORKS.into(),
          Box::new(FunctionConfigType::Regular(first_that_works_fn.clone())),
        );

        member_expression.insert(
          STYLEX_KEYFRAMES.into(),
          Box::new(FunctionConfigType::Regular(keyframes_fn.clone())),
        );

        member_expression.insert(
          STYLEX_POSITION_TRY.into(),
          Box::new(FunctionConfigType::Regular(position_try_fn.clone())),
        );

        member_expression.insert(
          STYLEX_DEFAULT_MARKER.into(),
          Box::new(FunctionConfigType::IndexMap(
            stylex_default_marker::stylex_default_marker(&self.state.options)
              .as_values()
              .unwrap_or_else(|| {
                #[cfg_attr(coverage_nightly, coverage(off))]
                {
                  stylex_panic!("{}", EXPECTED_COMPILED_STYLES)
                }
              })
              .clone(),
          )),
        );

        identifiers
          .entry(name.get_import_str().into())
          .and_modify(|func_type| {
            if let Some(map) = func_type.as_map_mut() {
              map.insert(
                STYLEX_WHEN.into(),
                FunctionConfig {
                  fn_ptr: FunctionType::DefaultMarker(Arc::clone(LazyLock::force(
                    &STYLEX_WHEN_MAP,
                  ))),
                  takes_path: false,
                },
              );
            }
          })
          .or_insert_with(|| {
            let mut map = FxHashMap::default();
            map.insert(
              STYLEX_WHEN.into(),
              FunctionConfig {
                fn_ptr: FunctionType::DefaultMarker(Arc::clone(LazyLock::force(&STYLEX_WHEN_MAP))),
                takes_path: false,
              },
            );
            Box::new(FunctionConfigType::Map(map))
          });
      }

      self
        .state
        .apply_stylex_env(&mut identifiers, &mut member_expressions);

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
            .unwrap_or(&non_static_value(STYLEX_CREATE)),
          &mut self.state,
        )
      );

      let value = match evaluated_arg.value {
        Some(v) => v,
        #[cfg_attr(coverage_nightly, coverage(off))]
        None => stylex_panic!("{}", non_static_value(STYLEX_CREATE)),
      };

      assert!(
        evaluated_arg.confident,
        "{}",
        build_code_frame_error(
          &Expr::Call(call.clone()),
          &evaluated_arg.deopt.unwrap_or_else(|| *first_arg.to_owned()),
          evaluated_arg
            .reason
            .as_deref()
            .unwrap_or(&non_static_value(STYLEX_CREATE)),
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
            variable_name.clone().into(),
            InjectableStyle::regular(
              create_property_rule(&variable_name, is_pseudo_element),
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
          &function_map,
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

        let props: Vec<PropOrSpread> = key_values
            .iter()
            .map(|key_value| {
              let orig_key = convert_key_value_to_str(key_value);
              let mut value = key_value.value.clone();

              let key = match &key_value.key {
                PropName::Ident(ident) => Some(ident.sym.to_string()),
                PropName::Str(strng) => Some(convert_atom_to_string(&strng.value)),
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
                    .map(|(var_name, v)| {
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
                        var_name: var_name.clone(),
                      }
                    })
                    .collect();

                  if self.state.options.style_resolution == StyleResolution::LegacyExpandShorthands
                  {
                    dynamic_styles = legacy_expand_shorthands(dynamic_styles);
                  }

                  let mut nullish_var_expressions: FxHashMap<String, Expr> = FxHashMap::default();
                  for dynamic_style in dynamic_styles.iter() {
                    if has_explicit_nullish_fallback(&mut dynamic_style.expression.clone()) {
                      nullish_var_expressions
                        .insert(dynamic_style.var_name.clone(), dynamic_style.expression.clone());
                    }
                  }

                  if let Some(value) = value.as_mut_object() {
                    let mut css_tag_value:Box<Expr> = Box::new(Expr::Lit(Lit::Bool(Bool {
                      span: DUMMY_SP,
                      value: true,
                    })));

                    let mut static_props = Vec::with_capacity(value.props.len());
                    let mut conditional_props = Vec::with_capacity(value.props.len());

                    for prop in value.props.iter_mut() {
                      if let PropOrSpread::Prop(prop) = prop {
                        if let Some(obj_prop) = prop.as_mut_key_value() {
                          let prop_key = match &obj_prop.key {
                            PropName::Ident(ident) => Some(ident.sym.to_string()),
                            PropName::Str(strng) => Some(convert_atom_to_string(&strng.value)),
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
                              .and_then(convert_lit_to_string)
                              .map(|s| {
                                s.split_whitespace()
                                  .map(str::to_owned)
                                  .collect::<Vec<String>>()
                              })
                              .unwrap_or_default();

                            if !class_list.is_empty() {
                              let mut is_static = true;
                              let mut expr_list = Vec::with_capacity(class_list.len());

                              // Pre-calculate class strings with spaces to avoid repeated allocations
                              let class_strings: Vec<String> = class_list
                                .iter()
                                .enumerate()
                                 .map(|(index, cls)| {
                                   if index == class_list.len() - 1 {
                                     cls.clone()
                                   } else {
                                     let mut spaced = String::with_capacity(cls.len() + 1);
                                     spaced.push_str(cls);
                                     spaced.push(' ');
                                     spaced
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

                                let expr = if expr.is_none() && !nullish_var_expressions.is_empty()
                                {
                                  injected_styles.get(cls.as_str()).and_then(|style| {
                                    let rule = match style.as_ref() {
                                      InjectableStyleKind::Regular(s) => {
                                        let ltr = s.ltr.as_str();
                                        let rtl = s.rtl.as_deref().unwrap_or_default();

                                        if ltr.is_empty() {
                                          rtl
                                        } else {
                                          ltr
                                        }
                                      },
                                      InjectableStyleKind::Const(s) => {
                                        let ltr = s.ltr.as_str();
                                        let rtl = s.rtl.as_deref().unwrap_or_default();

                                        if ltr.is_empty() {
                                          rtl
                                        } else {
                                          ltr
                                        }
                                      },
                                    };
                                    extract_expr_from_rule(rule, &nullish_var_expressions)
                                  })
                                } else {
                                  expr
                                };

                                let cls_with_space = &class_strings[index];

                                if let Some(expr) = expr.and_then(|mut e| {
                                  if is_safe_to_skip_null_check(&mut e) {
                                    None
                                  } else {
                                    Some(e)
                                  }
                                }) {
                                  is_static = false;
                                  expr_list.push(create_cond_expr(
                                    create_bin_expr(
                                      BinaryOp::NotEq,
                                      expr.clone(),
                                      create_null_expr(),
                                    ),
                                    create_string_expr(cls_with_space),
                                    expr,
                                  ));
                                } else {
                                  expr_list.push(create_string_expr(cls_with_space));
                                }
                              }

                              let joined = if expr_list.is_empty() {
                                create_string_expr("")
                              } else {
                                expr_list
                                  .into_iter()
                                  .reduce(|acc, curr| create_bin_expr(BinaryOp::Add, acc, curr))
                                  .unwrap_or_else(|| {
                                    #[cfg_attr(coverage_nightly, coverage(off))]
                                    {
                                      stylex_panic!(
                                        "Expected at least one expression to reduce in class name concatenation."
                                      )
                                    }
                                  })
                              };

                              if is_static {
                                static_props.push(create_prop_from_name(
                                  obj_prop.key.clone(),
                                  joined,
                                ));
                              } else {
                                conditional_props.push(create_prop_from_name(
                                  obj_prop.key.clone(),
                                  joined,
                                ));
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
                      static_props.push(create_key_value_prop(
                        COMPILED_KEY,
                        *css_tag_value.clone(),
                      ));

                      static_obj = Some(create_object_expression(static_props));
                    }

                    if !conditional_props.is_empty(){
                      conditional_props.push(create_key_value_prop(
                        COMPILED_KEY,
                        *css_tag_value.clone(),
                      ));

                      conditional_obj = Some(create_object_expression(conditional_props.clone()));
                    }

                    let mut final_fn_value = create_object_expression(
                      inline_styles
                        .iter()
                        .map(|(key, val)| {
                          create_key_value_prop(
                            key.as_str(),
                            val.expression.clone(),
                          )
                        })
                        .collect(),
                    );

                    if static_obj.is_some() || conditional_obj.is_some() {
                      let mut array_elements = Vec::with_capacity(2);

                      if let Some(static_obj) = static_obj {
                        let hoist_ident = create_expr_or_spread(hoist_expression(
                          static_obj,
                          &mut self.state,
                        ));

                        let hoist_ident_expr = match hoist_ident.expr.as_ident() {
                          Some(ident) => ident.clone(),
                          #[cfg_attr(coverage_nightly, coverage(off))]
                          None => stylex_panic!("Expected an identifier for the hoisted style variable."),
                        };
                        self.state.declarations.push(
                          create_string_var_declarator(hoist_ident_expr, "hoisted variable"),
                        );

                        array_elements.push(Some(hoist_ident));
                      }

                      if let Some(conditional_obj) = conditional_obj {
                        array_elements.push(Some(create_expr_or_spread(conditional_obj)));
                      }

                      array_elements.push(Some(create_expr_or_spread(final_fn_value)));

                      final_fn_value = create_array_expression(array_elements);
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

                    prop = Some(create_key_value_prop(orig_key.as_str(), value));
                  }
                }

              prop.unwrap_or_else(|| {
                create_key_value_prop(orig_key.as_str(), *value.clone())
              })
            })
            .collect();

        result_ast = path_replace_hoisted(
          create_object_expression(props),
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
        PreRuleValue::String(create_shorthand_key(i)),
      );

      let options = StyleXStateOptions::default()
        .with_style_resolution(StyleResolution::LegacyExpandShorthands);

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

fn create_property_rule(variable_name: &str, is_pseudo_element: bool) -> String {
  let inherits = if is_pseudo_element { "true" } else { "false" };
  let mut rule = String::with_capacity(variable_name.len() + inherits.len() + 40);
  rule.push_str("@property ");
  rule.push_str(variable_name);
  rule.push_str(" { syntax: \"*\"; inherits: ");
  rule.push_str(inherits);
  rule.push_str(";}");
  rule
}

fn create_shorthand_key(index: usize) -> String {
  let digit_count = if index == 0 {
    1
  } else {
    index.ilog10() as usize + 1
  };
  let mut key = String::with_capacity(digit_count + 1);
  key.push('p');
  let _ = write!(key, "{index}");
  key
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
      },
      BinaryOp::LogicalAnd => {
        is_safe_to_skip_null_check(&mut bin_expr.left)
          && is_safe_to_skip_null_check(&mut bin_expr.right)
      },
      _ => false,
    },
    Expr::Unary(unary_expr) => matches!(unary_expr.op, UnaryOp::Minus | UnaryOp::Plus),
    Expr::Cond(cond_expr) => {
      is_safe_to_skip_null_check(&mut cond_expr.cons)
        && is_safe_to_skip_null_check(&mut cond_expr.alt)
    },
    _ => false,
  }
}

fn has_explicit_nullish_fallback(expr: &mut Expr) -> bool {
  let expr = normalize_expr(expr);
  match expr {
    Expr::Lit(Lit::Null(_)) => true,
    Expr::Ident(ident) if ident.sym == "undefined" => true,
    Expr::Unary(unary) if matches!(unary.op, UnaryOp::Void) => true,
    Expr::Cond(cond) => {
      has_explicit_nullish_fallback(&mut cond.cons) || has_explicit_nullish_fallback(&mut cond.alt)
    },
    Expr::Bin(bin) => match bin.op {
      BinaryOp::LogicalOr | BinaryOp::NullishCoalescing | BinaryOp::LogicalAnd => {
        has_explicit_nullish_fallback(&mut bin.left)
          || has_explicit_nullish_fallback(&mut bin.right)
      },
      _ => false,
    },
    _ => false,
  }
}

fn extract_expr_from_rule(
  rule: &str,
  nullish_var_expressions: &FxHashMap<String, Expr>,
) -> Option<Expr> {
  for cap in VAR_EXTRACTION_REGEX.captures_iter(rule).flatten() {
    if let Some(var_match) = cap.get(1) {
      let var_name = var_match.as_str();
      if let Some(expr) = nullish_var_expressions.get(var_name) {
        return Some(expr.clone());
      }
    }
  }
  None
}

/// Hoists an expression to the program level by creating a const variable
/// declaration. This is the Rust equivalent of the JavaScript `hoistExpression`
/// function.
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
    decls: vec![create_var_declarator(hoisted_ident.clone(), ast_expression)],
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
    decls: vec![create_var_declarator(name_ident.clone(), ast_expression)],
    ctxt: swc_core::common::SyntaxContext::empty(),
  };

  let module_item = ModuleItem::Stmt(Stmt::Decl(Decl::Var(Box::new(var_decl))));
  state.hoisted_module_items.push(module_item);

  Expr::Ident(name_ident)
}
