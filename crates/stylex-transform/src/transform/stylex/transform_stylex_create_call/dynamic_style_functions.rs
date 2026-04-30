use super::*;
use crate::shared::structures::types::{ClassPathsMap, DynamicFns};

pub(super) fn apply_dynamic_style_functions<C>(
  transform: &mut StyleXTransform<C>,
  call: &CallExpr,
  mut result_ast: Expr,
  fns: Option<DynamicFns>,
  class_paths_per_namespace: &ClassPathsMap,
  injected_styles: &InjectableStylesMap,
  is_program_level: bool,
) -> Expr
where
  C: Comments,
{
  if let Some(fns) = fns
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

                  if transform.state.options.style_resolution == StyleResolution::LegacyExpandShorthands
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
                            &mut transform.state,
                          );
                        }
                      } else {
                        let expr = Expr::from(call.clone());

                        build_code_frame_error_and_panic(
                          &expr,
                          &expr,
                          "Unsupported prop type encountered in stylex.create. Only object properties are allowed.",
                          &mut transform.state,
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
                        *css_tag_value,
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
                          &mut transform.state,
                        ));

                        let hoist_ident_expr = match hoist_ident.expr.as_ident() {
                          Some(ident) => ident.clone(),
                          #[cfg_attr(coverage_nightly, coverage(off))]
                          None => stylex_panic!("Expected an identifier for the hoisted style variable."),
                        };
                        transform.state.declarations.push(
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
      &mut transform.state,
    );
  };

  result_ast
}
