#[cfg(test)]
mod stylex_create {
  use std::rc::Rc;

  use indexmap::IndexMap;
  use swc_core::ecma::ast::{Expr, ExprOrSpread, KeyValueProp};

  use crate::shared::{
    enums::data_structures::{
      evaluate_result_value::EvaluateResultValue,
      flat_compiled_styles_value::FlatCompiledStylesValue,
    },
    structures::{
      functions::FunctionMap,
      injectable_style::InjectableStyle,
      state::EvaluationState,
      state_manager::StateManager,
      types::{ClassPathsInNamespace, FlatCompiledStyles},
    },
    transformers::stylex_create::stylex_create_set,
    utils::ast::{
      convertors::string_to_expression,
      factories::{
        array_expression_factory, key_value_factory, lit_null_factory, object_expression_factory,
        prop_or_spread_expr_factory, prop_or_spread_expression_factory,
        prop_or_spread_string_factory,
      },
    },
  };

  fn style_object_factory(args: &[(&str, &[(&str, &str)])]) -> IndexMap<Expr, Vec<KeyValueProp>> {
    let mut object = IndexMap::new();

    for (key, value) in args {
      object.insert(
        string_to_expression(key),
        value
          .iter()
          .map(|(key, value)| key_value_factory(key, string_to_expression(value)))
          .collect(),
      );
    }

    object
  }

  type StyleNestedObjectFactoryArgs<'a> = [(&'a str, &'a [(&'a str, &'a [(&'a str, &'a str)])])];

  fn style_nested_object_factory(
    args: &StyleNestedObjectFactoryArgs,
  ) -> IndexMap<Expr, Vec<KeyValueProp>> {
    let mut object = IndexMap::new();

    for (key, value) in args {
      object.insert(
        string_to_expression(key),
        value
          .iter()
          .map(|(key, value)| {
            key_value_factory(
              key,
              object_expression_factory(
                value
                  .iter()
                  .map(|(key, value)| prop_or_spread_string_factory(key, value))
                  .collect(),
              ),
            )
          })
          .collect(),
      );
    }

    object
  }

  enum StringOrNull<'a> {
    String(&'a str),
    Null,
  }

  enum DepthProps<'a> {
    One(&'a str),
    Two(&'a [(&'a str, &'a StringOrNull<'a>)]),
    Three(&'a [(&'a str, &'a [(&'a str, &'a str)])]),
    // Four(&'a [(&'a str, &'a [(&'a str, &'a [(&'a str, &'a str)])])]),
  }

  type StyleMultipleDepthNestedObjectFactoryArgs<'a> = [(
    &'a str,
    &'a [(&'a str, &'a [(&'a str, &'a DepthProps<'a>)])],
  )];

  fn style_multiple_depth_nested_object_factory(
    args: &StyleMultipleDepthNestedObjectFactoryArgs,
  ) -> IndexMap<Expr, Vec<KeyValueProp>> {
    let mut object = IndexMap::new();

    for (key, value) in args {
      object.insert(
        string_to_expression(key),
        value
          .iter()
          .map(|(key, value)| {
            key_value_factory(
              key,
              object_expression_factory(
                value
                  .iter()
                  .map(|(key, values)| match values {
                    DepthProps::One(strng) => prop_or_spread_string_factory(key, strng),
                    DepthProps::Two(arr) => prop_or_spread_expr_factory(
                      key,
                      arr
                        .iter()
                        .map(|(key, value)| match value {
                          StringOrNull::String(strng) => prop_or_spread_string_factory(key, strng),
                          StringOrNull::Null => {
                            prop_or_spread_expression_factory(key, Expr::Lit(lit_null_factory()))
                          }
                        })
                        .collect(),
                    ),
                    DepthProps::Three(arr) => prop_or_spread_expr_factory(
                      key,
                      arr
                        .iter()
                        .map(|(key, values)| {
                          prop_or_spread_expr_factory(
                            key,
                            values
                              .iter()
                              .map(|(key, value)| prop_or_spread_string_factory(key, value))
                              .collect(),
                          )
                        })
                        .collect(),
                    ),
                    // DepthProps::Four(arr) => prop_or_spread_expr_factory(
                    //   key,
                    //   arr
                    //     .iter()
                    //     .map(|(key, values)| {
                    //       prop_or_spread_expr_factory(
                    //         key,
                    //         values
                    //           .iter()
                    //           .map(|(key, values)| {
                    //             prop_or_spread_expr_factory(
                    //               key,
                    //               values
                    //                 .iter()
                    //                 .map(|(key, value)| prop_or_spread_string_factory(key, value))
                    //                 .collect(),
                    //             )
                    //           })
                    //           .collect(),
                    //       )
                    //     })
                    //     .collect(),
                    // ),
                  })
                  .collect(),
              ),
            )
          })
          .collect(),
      );
    }

    object
  }

  type StyleArrayObjectFactoryArgs<'a> = [(&'a str, &'a [(&'a str, &'a [&'a str])])];
  fn style_array_object_factory(
    args: &StyleArrayObjectFactoryArgs,
  ) -> IndexMap<Expr, Vec<KeyValueProp>> {
    let mut object = IndexMap::new();

    for (key, value) in args {
      object.insert(
        string_to_expression(key),
        value
          .iter()
          .map(|(key, value)| {
            let elems = value
              .iter()
              .map(|arg| {
                Some(ExprOrSpread {
                  spread: None,
                  expr: Box::new(string_to_expression(arg)),
                })
              })
              .collect::<Vec<Option<ExprOrSpread>>>();

            key_value_factory(key, array_expression_factory(elems))
          })
          .collect(),
      );
    }

    object
  }

  type InjectedStylesArg<'a> = [(&'a str, &'a [(&'a str, (&'a str, f64))])];

  fn exprected_result_factory(
    resolved_namespaces: &[(&str, &[(&str, &str)])],
    injected_styles: &InjectedStylesArg,
    class_paths_in_namespace: &[(&str, &[(&str, &[&str])])],
  ) -> (
    IndexMap<String, Rc<FlatCompiledStyles>>,
    IndexMap<String, Rc<InjectableStyle>>,
    IndexMap<String, Rc<ClassPathsInNamespace>>,
  ) {
    let mut expected_resolved_namespaces = IndexMap::new();
    let mut expected_injected_styles = IndexMap::new();
    let mut expected_class_paths_in_namespace = IndexMap::new();

    for (resolved_namespace, namespace) in resolved_namespaces {
      let mut default_val = IndexMap::new();

      default_val.insert(
        "$$css".to_string(),
        Rc::new(FlatCompiledStylesValue::Bool(true)),
      );

      for (key, value) in namespace.iter() {
        default_val.insert(
          key.to_string(),
          Rc::new(if value.eq(&"null") {
            FlatCompiledStylesValue::Null
          } else {
            FlatCompiledStylesValue::String(value.to_string())
          }),
        );
      }

      expected_resolved_namespaces.insert(resolved_namespace.to_string(), Rc::new(default_val));
    }

    for injected_style in injected_styles {
      for (key, inj) in injected_style.1 {
        let (value, priority) = inj;
        expected_injected_styles.insert(
          key.to_string(),
          Rc::new(InjectableStyle {
            ltr: value.to_string(),
            rtl: None,
            priority: Some(*priority),
          }),
        );
      }
    }

    for (namespace, class_paths) in class_paths_in_namespace {
      let mut default_val = IndexMap::new();

      for (key, value) in class_paths.iter() {
        default_val.insert(
          key.to_string(),
          value.iter().map(|v| v.to_string()).collect(),
        );
      }

      expected_class_paths_in_namespace.insert(namespace.to_string(), Rc::new(default_val));
    }

    (
      expected_resolved_namespaces,
      expected_injected_styles,
      expected_class_paths_in_namespace,
    )
  }

  fn stylex_create(
    style_object: IndexMap<Expr, Vec<KeyValueProp>>,
  ) -> (
    IndexMap<String, Rc<FlatCompiledStyles>>,
    IndexMap<String, Rc<InjectableStyle>>,
    IndexMap<String, Rc<ClassPathsInNamespace>>,
  ) {
    stylex_create_set(
      &EvaluateResultValue::Map(style_object),
      &mut EvaluationState::default(),
      &mut StateManager::default(),
      &FunctionMap::default(),
    )
  }

  #[test]
  fn color_red() {
    let object =
      style_object_factory(&[("default", &[("backgroundColor", "red"), ("color", "blue")])]);

    let (resolved_namespaces, injected_styles, class_paths_in_namespace) = stylex_create(object);

    let (expected_resolved_namespaces, expected_injected_styles, expected_class_paths_in_namespace) =
      exprected_result_factory(
        &[(
          "default",
          &[("backgroundColor", "xrkmrrc"), ("color", "xju2f9n")],
        )],
        &[(
          "default",
          &[
            ("xrkmrrc", (".xrkmrrc{background-color:red}", 3000.0)),
            ("xju2f9n", (".xju2f9n{color:blue}", 3000.0)),
          ],
        )],
        &[(
          "default",
          &[("xrkmrrc", &["backgroundColor"]), ("xju2f9n", &["color"])],
        )],
      );

    assert_eq!(resolved_namespaces, expected_resolved_namespaces);
    assert_eq!(injected_styles, expected_injected_styles);
    assert_eq!(class_paths_in_namespace, expected_class_paths_in_namespace);
  }

  #[test]
  fn transition_property_margin_top() {
    let camel_case_object =
      style_object_factory(&[("default", &[("transitionProperty", "marginTop")])]);
    let kebab_case_object =
      style_object_factory(&[("default", &[("transitionProperty", "margin-top")])]);

    let camel_case = stylex_create(camel_case_object);
    let kebab_case = stylex_create(kebab_case_object);

    assert_eq!(camel_case, kebab_case);

    let (resolved_namespaces, injected_styles, class_paths_in_namespace) = camel_case;

    let (expected_resolved_namespaces, expected_injected_styles, expected_class_paths_in_namespace) =
      exprected_result_factory(
        &[("default", &[("transitionProperty", "x1cfch2b")])],
        &[(
          "default",
          &[(
            "x1cfch2b",
            (".x1cfch2b{transition-property:margin-top}", 3000.0),
          )],
        )],
        &[("default", &[("x1cfch2b", &["transitionProperty"])])],
      );

    assert_eq!(resolved_namespaces, expected_resolved_namespaces);
    assert_eq!(injected_styles, expected_injected_styles);
    assert_eq!(class_paths_in_namespace, expected_class_paths_in_namespace);
  }

  #[test]
  fn will_change_margin_top() {
    let camel_case_object = style_object_factory(&[("default", &[("willChange", "marginTop")])]);
    let kebab_case_object = style_object_factory(&[("default", &[("willChange", "margin-top")])]);

    let camel_case = stylex_create(camel_case_object);
    let kebab_case = stylex_create(kebab_case_object);

    assert_eq!(camel_case, kebab_case);

    let (resolved_namespaces, injected_styles, class_paths_in_namespace) = camel_case;

    let (expected_resolved_namespaces, expected_injected_styles, expected_class_paths_in_namespace) =
      exprected_result_factory(
        &[("default", &[("willChange", "x1a6dnx1")])],
        &[(
          "default",
          &[("x1a6dnx1", (".x1a6dnx1{will-change:margin-top}", 3000.0))],
        )],
        &[("default", &[("x1a6dnx1", &["willChange"])])],
      );

    assert_eq!(resolved_namespaces, expected_resolved_namespaces);
    assert_eq!(injected_styles, expected_injected_styles);
    assert_eq!(class_paths_in_namespace, expected_class_paths_in_namespace)
  }

  #[test]
  fn transition_property_dash_dash_foo() {
    let object = style_object_factory(&[("default", &[("transitionProperty", "--foo")])]);

    let (resolved_namespaces, injected_styles, class_paths_in_namespace) = stylex_create(object);

    let (expected_resolved_namespaces, expected_injected_styles, expected_class_paths_in_namespace) =
      exprected_result_factory(
        &[("default", &[("transitionProperty", "x17389it")])],
        &[(
          "default",
          &[("x17389it", (".x17389it{transition-property:--foo}", 3000.0))],
        )],
        &[("default", &[("x17389it", &["transitionProperty"])])],
      );

    assert_eq!(resolved_namespaces, expected_resolved_namespaces);
    assert_eq!(injected_styles, expected_injected_styles);
    assert_eq!(class_paths_in_namespace, expected_class_paths_in_namespace)
  }

  #[test]
  fn will_change_dash_dash_foo() {
    let object = style_object_factory(&[("default", &[("willChange", "--foo")])]);

    let (resolved_namespaces, injected_styles, class_paths_in_namespace) = stylex_create(object);

    let (expected_resolved_namespaces, expected_injected_styles, expected_class_paths_in_namespace) =
      exprected_result_factory(
        &[("default", &[("willChange", "x1lxaxzv")])],
        &[(
          "default",
          &[("x1lxaxzv", (".x1lxaxzv{will-change:--foo}", 3000.0))],
        )],
        &[("default", &[("x1lxaxzv", &["willChange"])])],
      );

    assert_eq!(resolved_namespaces, expected_resolved_namespaces);
    assert_eq!(injected_styles, expected_injected_styles);
    assert_eq!(class_paths_in_namespace, expected_class_paths_in_namespace)
  }

  #[test]
  fn transition_property_opacity_margin_top() {
    let camel_case_object =
      style_object_factory(&[("default", &[("transitionProperty", "opacity, marginTop")])]);
    let kebab_case_object =
      style_object_factory(&[("default", &[("transitionProperty", "opacity, margin-top")])]);

    let camel_case = stylex_create(camel_case_object);
    let kebab_case = stylex_create(kebab_case_object);

    assert_eq!(camel_case, kebab_case);

    let (resolved_namespaces, injected_styles, class_paths_in_namespace) = camel_case;

    let (expected_resolved_namespaces, expected_injected_styles, expected_class_paths_in_namespace) =
      exprected_result_factory(
        &[("default", &[("transitionProperty", "x95ccmk")])],
        &[(
          "default",
          &[(
            "x95ccmk",
            (".x95ccmk{transition-property:opacity,margin-top}", 3000.0),
          )],
        )],
        &[("default", &[("x95ccmk", &["transitionProperty"])])],
      );

    assert_eq!(resolved_namespaces, expected_resolved_namespaces);
    assert_eq!(injected_styles, expected_injected_styles);
    assert_eq!(class_paths_in_namespace, expected_class_paths_in_namespace)
  }

  #[test]
  fn padding_shorthand() {
    let object = style_object_factory(&[(
      "short",
      &[
        (
          "padding",
          "calc((100% - 50px) * 0.5) var(--rightpadding, 20px)",
        ),
        ("paddingTop", "0"),
      ],
    )]);

    let (resolved_namespaces, injected_styles, class_paths_in_namespace) = stylex_create(object);

    let (expected_resolved_namespaces, expected_injected_styles, expected_class_paths_in_namespace) =
      exprected_result_factory(
        &[(
          "short",
          &[
            ("padding", "x1lmef92"),
            ("paddingBlock", "null"),
            ("paddingBottom", "null"),
            ("paddingEnd", "null"),
            ("paddingInline", "null"),
            ("paddingLeft", "null"),
            ("paddingRight", "null"),
            ("paddingStart", "null"),
            ("paddingTop", "xexx8yu"),
          ],
        )],
        &[(
          "default",
          &[
            (
              "x1lmef92",
              (
                ".x1lmef92{padding:calc((100% - 50px) * .5) var(--rightpadding,20px)}",
                1000.0,
              ),
            ),
            ("xexx8yu", (".xexx8yu{padding-top:0}", 4000.0)),
          ],
        )],
        &[(
          "short",
          &[("x1lmef92", &["padding"]), ("xexx8yu", &["paddingTop"])],
        )],
      );

    assert_eq!(resolved_namespaces, expected_resolved_namespaces);
    assert_eq!(injected_styles, expected_injected_styles);
    assert_eq!(class_paths_in_namespace, expected_class_paths_in_namespace)
  }

  #[test]
  fn transforms_style_object_with_custom_property() {
    let object = style_object_factory(&[("default", &[("--background-color", "red")])]);

    let (resolved_namespaces, injected_styles, class_paths_in_namespace) = stylex_create(object);

    let (expected_resolved_namespaces, expected_injected_styles, expected_class_paths_in_namespace) =
      exprected_result_factory(
        &[("default", &[("--background-color", "xgau0yw")])],
        &[(
          "default",
          &[("xgau0yw", (".xgau0yw{--background-color:red}", 1.0))],
        )],
        &[("default", &[("xgau0yw", &["--background-color"])])],
      );

    assert_eq!(resolved_namespaces, expected_resolved_namespaces);
    assert_eq!(injected_styles, expected_injected_styles);
    assert_eq!(class_paths_in_namespace, expected_class_paths_in_namespace)
  }

  #[test]
  fn transforms_style_object_with_custom_property_as_value() {
    let object =
      style_object_factory(&[("default", &[("--final-color", "var(--background-color)")])]);

    let (resolved_namespaces, injected_styles, class_paths_in_namespace) = stylex_create(object);

    let (expected_resolved_namespaces, expected_injected_styles, expected_class_paths_in_namespace) =
      exprected_result_factory(
        &[("default", &[("--final-color", "x13tgbkp")])],
        &[(
          "default",
          &[(
            "x13tgbkp",
            (".x13tgbkp{--final-color:var(--background-color)}", 1.0),
          )],
        )],
        &[("default", &[("x13tgbkp", &["--final-color"])])],
      );

    assert_eq!(resolved_namespaces, expected_resolved_namespaces);
    assert_eq!(injected_styles, expected_injected_styles);
    assert_eq!(class_paths_in_namespace, expected_class_paths_in_namespace)
  }

  #[test]
  fn transforms_multiple_namespaces() {
    let object = style_object_factory(&[
      ("default", &[("backgroundColor", "red")]),
      ("default2", &[("color", "blue")]),
    ]);

    let (resolved_namespaces, injected_styles, class_paths_in_namespace) = stylex_create(object);

    let (expected_resolved_namespaces, expected_injected_styles, expected_class_paths_in_namespace) =
      exprected_result_factory(
        &[
          ("default", &[("backgroundColor", "xrkmrrc")]),
          ("default2", &[("color", "xju2f9n")]),
        ],
        &[
          ("default", &[("xju2f9n", (".xju2f9n{color:blue}", 3000.0))]),
          (
            "xrkmrrc",
            &[("xrkmrrc", (".xrkmrrc{background-color:red}", 3000.0))],
          ),
        ],
        &[
          ("default", &[("xrkmrrc", &["backgroundColor"])]),
          ("default2", &[("xju2f9n", &["color"])]),
        ],
      );

    assert_eq!(resolved_namespaces, expected_resolved_namespaces);
    assert_eq!(injected_styles, expected_injected_styles);
    assert_eq!(class_paths_in_namespace, expected_class_paths_in_namespace)
  }

  #[test]
  fn does_not_transform_attr_value() {
    let object = style_object_factory(&[("default", &[("content", "attr(some-attribute)")])]);

    let (resolved_namespaces, injected_styles, class_paths_in_namespace) = stylex_create(object);

    let (expected_resolved_namespaces, expected_injected_styles, expected_class_paths_in_namespace) =
      exprected_result_factory(
        &[("default", &[("content", "xd71okc")])],
        &[(
          "default",
          &[(
            "xd71okc",
            (".xd71okc{content:attr(some-attribute)}", 3000.0),
          )],
        )],
        &[("default", &[("xd71okc", &["content"])])],
      );

    assert_eq!(resolved_namespaces, expected_resolved_namespaces);
    assert_eq!(injected_styles, expected_injected_styles);
    assert_eq!(class_paths_in_namespace, expected_class_paths_in_namespace)
  }

  #[test]
  fn does_not_add_units_to_variable_value() {
    let object = style_object_factory(&[("default", &[("--foo", "500")])]);

    let (resolved_namespaces, injected_styles, class_paths_in_namespace) = stylex_create(object);

    let (expected_resolved_namespaces, expected_injected_styles, expected_class_paths_in_namespace) =
      exprected_result_factory(
        &[("default", &[("--foo", "xwzgxvi")])],
        &[("default", &[("xwzgxvi", (".xwzgxvi{--foo:500}", 1.0))])],
        &[("default", &[("xwzgxvi", &["--foo"])])],
      );

    assert_eq!(resolved_namespaces, expected_resolved_namespaces);
    assert_eq!(injected_styles, expected_injected_styles);
    assert_eq!(class_paths_in_namespace, expected_class_paths_in_namespace)
  }

  #[test]
  fn transforms_nested_pseudo_class_to_css() {
    let object = style_nested_object_factory(&[(
      "default",
      &[(":hover", &[("backgroundColor", "red"), ("color", "blue")])],
    )]);

    let (resolved_namespaces, injected_styles, class_paths_in_namespace) = stylex_create(object);

    let (expected_resolved_namespaces, expected_injected_styles, expected_class_paths_in_namespace) =
      exprected_result_factory(
        &[(
          "default",
          &[
            (":hover_backgroundColor", "x1gykpug"),
            (":hover_color", "x17z2mba"),
          ],
        )],
        &[(
          "default",
          &[
            (
              "x1gykpug",
              (".x1gykpug:hover{background-color:red}", 3130.0),
            ),
            ("x17z2mba", (".x17z2mba:hover{color:blue}", 3130.0)),
          ],
        )],
        &[(
          "default",
          &[
            ("x1gykpug", &[":hover", "backgroundColor"]),
            ("x17z2mba", &[":hover", "color"]),
          ],
        )],
      );

    assert_eq!(resolved_namespaces, expected_resolved_namespaces);
    assert_eq!(injected_styles, expected_injected_styles);
    assert_eq!(class_paths_in_namespace, expected_class_paths_in_namespace)
  }

  #[test]
  fn transforms_nested_pseudo_classes_within_pseudo_elements() {
    let object = style_multiple_depth_nested_object_factory(&[(
      "default",
      &[(
        "::before",
        &[(
          "color",
          &DepthProps::Two(&[
            ("default", &StringOrNull::String("red")),
            (":hover", &StringOrNull::String("blue")),
          ]),
        )],
      )],
    )]);

    let (resolved_namespaces, injected_styles, class_paths_in_namespace) = stylex_create(object);

    let (expected_resolved_namespaces, expected_injected_styles, expected_class_paths_in_namespace) =
      exprected_result_factory(
        &[("default", &[("::before_color", "x16oeupf xeb2lg0")])],
        &[(
          "default",
          &[
            ("x16oeupf", (".x16oeupf::before{color:red}", 8000.0)),
            ("xeb2lg0", (".xeb2lg0::before:hover{color:blue}", 8130.0)),
          ],
        )],
        &[(
          "default",
          &[
            ("x16oeupf", &["::before", "default", "color"]),
            ("xeb2lg0", &["::before", ":hover", "color"]),
          ],
        )],
      );

    assert_eq!(resolved_namespaces, expected_resolved_namespaces);
    assert_eq!(injected_styles, expected_injected_styles);
    assert_eq!(class_paths_in_namespace, expected_class_paths_in_namespace)
  }

  #[test]
  fn transforms_nested_legacy_pseudo_classes_within_pseudo_elements() {
    let object = style_multiple_depth_nested_object_factory(&[(
      "default",
      &[(
        "::before",
        &[
          ("color", &DepthProps::One("red")),
          (
            ":hover",
            &DepthProps::Two(&[("color", &StringOrNull::String("blue"))]),
          ),
        ],
      )],
    )]);

    let (resolved_namespaces, injected_styles, class_paths_in_namespace) = stylex_create(object);

    let (expected_resolved_namespaces, expected_injected_styles, expected_class_paths_in_namespace) =
      exprected_result_factory(
        &[(
          "default",
          &[
            ("::before_:hover_color", "xeb2lg0"),
            ("::before_color", "x16oeupf"),
          ],
        )],
        &[(
          "default",
          &[
            ("x16oeupf", (".x16oeupf::before{color:red}", 8000.0)),
            ("xeb2lg0", (".xeb2lg0::before:hover{color:blue}", 8130.0)),
          ],
        )],
        &[(
          "default",
          &[
            ("x16oeupf", &["::before", "color"]),
            ("xeb2lg0", &["::before", ":hover", "color"]),
          ],
        )],
      );

    assert_eq!(resolved_namespaces, expected_resolved_namespaces);
    assert_eq!(injected_styles, expected_injected_styles);
    assert_eq!(class_paths_in_namespace, expected_class_paths_in_namespace)
  }

  #[test]
  fn transforms_nested_pseudo_element_within_legacy_pseudo_class() {
    let object = style_multiple_depth_nested_object_factory(&[(
      "default",
      &[
        ("::before", &[("color", &DepthProps::One("red"))]),
        (
          ":hover",
          &[(
            "::before",
            &DepthProps::Three(&[(
              "color",
              &[
                ("default", "blue"),
                (":hover", "green"),
                (":active", "yellow"),
              ],
            )]),
          )],
        ),
      ],
    )]);

    let (resolved_namespaces, injected_styles, class_paths_in_namespace) = stylex_create(object);

    let (expected_resolved_namespaces, expected_injected_styles, expected_class_paths_in_namespace) =
      exprected_result_factory(
        &[(
          "default",
          &[
            ("::before_color", "x16oeupf"),
            (":hover_::before_color", "xzzpreb x1gobd9t x1lvqgcc"),
          ],
        )],
        &[(
          "default",
          &[
            ("x16oeupf", (".x16oeupf::before{color:red}", 8000.0)),
            (
              "x1gobd9t",
              (".x1gobd9t:hover::before:hover{color:green}", 8260.0),
            ),
            (
              "x1lvqgcc",
              (".x1lvqgcc:hover::before:active{color:yellow}", 8300.0),
            ),
            ("xzzpreb", (".xzzpreb:hover::before{color:blue}", 8130.0)),
          ],
        )],
        &[(
          "default",
          &[
            ("x16oeupf", &["::before", "color"]),
            ("x1gobd9t", &[":hover", "::before", ":hover", "color"]),
            ("x1lvqgcc", &[":hover", "::before", ":active", "color"]),
            ("xzzpreb", &[":hover", "::before", "default", "color"]),
          ],
        )],
      );

    assert_eq!(resolved_namespaces, expected_resolved_namespaces);
    assert_eq!(injected_styles, expected_injected_styles);
    assert_eq!(class_paths_in_namespace, expected_class_paths_in_namespace)
  }

  #[test]
  #[ignore]

  fn transforms_nested_pseudo_classes_within_pseudo_elements_v2() {
    let before_hover_object = style_multiple_depth_nested_object_factory(&[(
      "default",
      &[(
        "::before",
        &[(
          "color",
          &DepthProps::Two(&[
            ("default", &StringOrNull::Null),
            (":hover", &StringOrNull::String("blue")),
          ]),
        )],
      )],
    )]);

    let hover_before_object = style_multiple_depth_nested_object_factory(&[(
      "default",
      &[(
        ":hover",
        &[(
          "::before",
          &DepthProps::Two(&[("color", &StringOrNull::String("blue"))]),
        )],
      )],
    )]);

    let (before_hover, _, _) = stylex_create(before_hover_object);
    let (hover_before, _, _) = stylex_create(hover_before_object);

    let before_hover_class = before_hover
      .get("default")
      .and_then(|a| a.get("::before_color"))
      .unwrap();

    let hover_before_class = hover_before
      .get("default")
      .and_then(|a| a.get(":hover_::before_color"))
      .unwrap();

    assert_eq!(
      before_hover_class,
      &Rc::new(FlatCompiledStylesValue::String("xeb2lg0".to_string()))
    );

    assert_eq!(
      hover_before_class,
      &Rc::new(FlatCompiledStylesValue::String("xeb2lg0".to_string()))
    );

    assert_ne!(before_hover_class, hover_before_class)
  }

  #[test]
  fn transforms_array_values_as_fallbacks() {
    let object = style_array_object_factory(&[("default", &[("position", &["sticky", "fixed"])])]);

    let (resolved_namespaces, injected_styles, class_paths_in_namespace) = stylex_create(object);

    let (expected_resolved_namespaces, expected_injected_styles, expected_class_paths_in_namespace) =
      exprected_result_factory(
        &[("default", &[("position", "x1ruww2u")])],
        &[(
          "default",
          &[(
            "x1ruww2u",
            (".x1ruww2u{position:sticky;position:fixed}", 3000.0),
          )],
        )],
        &[("default", &[("x1ruww2u", &["position"])])],
      );

    assert_eq!(resolved_namespaces, expected_resolved_namespaces);
    assert_eq!(injected_styles, expected_injected_styles);
    assert_eq!(class_paths_in_namespace, expected_class_paths_in_namespace)
  }

  #[test]
  fn transforms_valid_shorthands() {
    let object = style_object_factory(&[(
      "default",
      &[
        ("overflow", "hidden"),
        ("borderStyle", "dashed"),
        ("borderWidth", "1"),
      ],
    )]);

    let (resolved_namespaces, injected_styles, class_paths_in_namespace) = stylex_create(object);

    let (expected_resolved_namespaces, expected_injected_styles, expected_class_paths_in_namespace) =
      exprected_result_factory(
        &[(
          "default",
          &[
            ("borderBlockStyle", "null"),
            ("borderBlockWidth", "null"),
            ("borderBottomStyle", "null"),
            ("borderBottomWidth", "null"),
            ("borderInlineEndStyle", "null"),
            ("borderInlineEndWidth", "null"),
            ("borderInlineStartStyle", "null"),
            ("borderInlineStartWidth", "null"),
            ("borderInlineStyle", "null"),
            ("borderInlineWidth", "null"),
            ("borderLeftStyle", "null"),
            ("borderLeftWidth", "null"),
            ("borderRightStyle", "null"),
            ("borderRightWidth", "null"),
            ("borderStyle", "xbsl7fq"),
            ("borderTopStyle", "null"),
            ("borderTopWidth", "null"),
            ("borderWidth", "xmkeg23"),
            ("overflow", "xb3r6kr"),
            ("overflowX", "null"),
            ("overflowY", "null"),
          ],
        )],
        &[(
          "default",
          &[
            ("xb3r6kr", (".xb3r6kr{overflow:hidden}", 2000.0)),
            ("xbsl7fq", (".xbsl7fq{border-style:dashed}", 2000.0)),
            ("xmkeg23", (".xmkeg23{border-width:1px}", 2000.0)),
          ],
        )],
        &[(
          "default",
          &[
            ("xb3r6kr", &["overflow"]),
            ("xbsl7fq", &["borderStyle"]),
            ("xmkeg23", &["borderWidth"]),
          ],
        )],
      );

    assert_eq!(resolved_namespaces, expected_resolved_namespaces);
    assert_eq!(injected_styles, expected_injected_styles);
    assert_eq!(class_paths_in_namespace, expected_class_paths_in_namespace)
  }

  #[test]
  fn transforms_media_queries() {
    let mut object = style_nested_object_factory(&[(
      "default",
      &[
        ("@media (min-width: 1000px)", &[("backgroundColor", "blue")]),
        (
          "@media (min-width: 2000px)",
          &[("backgroundColor", "purple")],
        ),
      ],
    )]);

    let def = object.get_mut(&string_to_expression("default")).unwrap();

    def.push(key_value_factory(
      "backgroundColor",
      string_to_expression("red"),
    ));

    let (resolved_namespaces, injected_styles, class_paths_in_namespace) = stylex_create(object);

    let (expected_resolved_namespaces, expected_injected_styles, expected_class_paths_in_namespace) =
      exprected_result_factory(
        &[(
          "default",
          &[
            ("@media (min-width: 1000px)_backgroundColor", "xc445zv"),
            ("@media (min-width: 2000px)_backgroundColor", "x1ssfqz5"),
            ("backgroundColor", "xrkmrrc"),
          ],
        )],
        &[(
          "default",
          &[
            (
              "x1ssfqz5",
              (
                "@media (min-width: 2000px){.x1ssfqz5.x1ssfqz5{background-color:purple}}",
                3200.0,
              ),
            ),
            (
              "xc445zv",
              (
                "@media (min-width: 1000px){.xc445zv.xc445zv{background-color:blue}}",
                3200.0,
              ),
            ),
            ("xrkmrrc", (".xrkmrrc{background-color:red}", 3000.0)),
          ],
        )],
        &[(
          "default",
          &[
            (
              "xc445zv",
              &["@media (min-width: 1000px)", "backgroundColor"],
            ),
            (
              "x1ssfqz5",
              &["@media (min-width: 2000px)", "backgroundColor"],
            ),
            ("xrkmrrc", &["backgroundColor"]),
          ],
        )],
      );

    assert_eq!(resolved_namespaces, expected_resolved_namespaces);
    assert_eq!(injected_styles, expected_injected_styles);
    assert_eq!(class_paths_in_namespace, expected_class_paths_in_namespace)
  }
}
