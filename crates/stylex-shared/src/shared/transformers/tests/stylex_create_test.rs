#[cfg(test)]
mod stylex_create {
  use std::rc::Rc;

  use indexmap::IndexMap;
  use swc_core::ecma::ast::{Expr, ExprOrSpread, KeyValueProp};

  use crate::shared::{
    constants::common::COMPILED_KEY,
    enums::data_structures::{
      evaluate_result_value::EvaluateResultValue,
      flat_compiled_styles_value::FlatCompiledStylesValue, injectable_style::InjectableStyleKind,
    },
    structures::{
      functions::FunctionMap,
      injectable_style::InjectableStyle,
      state::EvaluationState,
      state_manager::StateManager,
      stylex_state_options::StyleXStateOptions,
      types::{ClassPathsInNamespace, FlatCompiledStyles},
    },
    transformers::stylex_create::stylex_create_set,
    utils::ast::{
      convertors::string_to_expression,
      factories::{
        array_expression_factory, key_value_ident_factory, lit_null_factory,
        object_expression_factory, prop_or_spread_expr_factory, prop_or_spread_expression_factory,
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
          .map(|(key, value)| key_value_ident_factory(key, string_to_expression(value)))
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
            key_value_ident_factory(
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
            key_value_ident_factory(
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

            key_value_ident_factory(key, array_expression_factory(elems))
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
    IndexMap<String, Rc<InjectableStyleKind>>,
    IndexMap<String, Rc<ClassPathsInNamespace>>,
  ) {
    let mut expected_resolved_namespaces = IndexMap::new();
    let mut expected_injected_styles = IndexMap::new();
    let mut expected_class_paths_in_namespace = IndexMap::new();

    for (resolved_namespace, namespace) in resolved_namespaces {
      let mut default_val = IndexMap::new();

      default_val.insert(
        COMPILED_KEY.to_string(),
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
          Rc::new(InjectableStyleKind::Regular(InjectableStyle {
            ltr: value.to_string(),
            rtl: None,
            priority: Some(*priority),
          })),
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
    IndexMap<String, Rc<InjectableStyleKind>>,
    IndexMap<String, Rc<ClassPathsInNamespace>>,
  ) {
    stylex_create_set(
      &EvaluateResultValue::Map(style_object),
      &mut EvaluationState::default(),
      &mut StateManager {
        options: StyleXStateOptions {
          debug: true,
          ..Default::default()
        },
        ..Default::default()
      },
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
          &[
            ("backgroundColor-kWkggS", "backgroundColor-xrkmrrc"),
            ("color-kMwMTN", "color-xju2f9n"),
          ],
        )],
        &[(
          "default",
          &[
            (
              "backgroundColor-xrkmrrc",
              (".backgroundColor-xrkmrrc{background-color:red}", 3000.0),
            ),
            ("color-xju2f9n", (".color-xju2f9n{color:blue}", 3000.0)),
          ],
        )],
        &[(
          "default",
          &[
            ("backgroundColor-xrkmrrc", &["backgroundColor"]),
            ("color-xju2f9n", &["color"]),
          ],
        )],
      );

    assert_eq!(resolved_namespaces, expected_resolved_namespaces);
    assert_eq!(injected_styles, expected_injected_styles);
    assert_eq!(class_paths_in_namespace, expected_class_paths_in_namespace);
  }

  #[test]
  fn webkit() {
    let object = style_object_factory(&[(
      "default",
      &[("WebkitBoxOrient", "vertical"), ("WebkitLineClamp", "2")],
    )]);

    let (resolved_namespaces, injected_styles, class_paths_in_namespace) = stylex_create(object);

    let (expected_resolved_namespaces, expected_injected_styles, expected_class_paths_in_namespace) =
      exprected_result_factory(
        &[(
          "default",
          &[
            ("WebkitBoxOrient-kgKLqz", "WebkitBoxOrient-x1ua5tub"),
            ("WebkitLineClamp-kJFfOR", "WebkitLineClamp-x1h7i4cw"),
          ],
        )],
        &[(
          "default",
          &[
            (
              "WebkitBoxOrient-x1ua5tub",
              (
                ".WebkitBoxOrient-x1ua5tub{-webkit-box-orient:vertical}",
                3000.0,
              ),
            ),
            (
              "WebkitLineClamp-x1h7i4cw",
              (".WebkitLineClamp-x1h7i4cw{-webkit-line-clamp:2}", 3000.0),
            ),
          ],
        )],
        &[(
          "default",
          &[
            ("WebkitBoxOrient-x1ua5tub", &["WebkitBoxOrient"]),
            ("WebkitLineClamp-x1h7i4cw", &["WebkitLineClamp"]),
          ],
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
        &[(
          "default",
          &[("transitionProperty-k1ekBW", "transitionProperty-x1cfch2b")],
        )],
        &[(
          "default",
          &[(
            "transitionProperty-x1cfch2b",
            (
              ".transitionProperty-x1cfch2b{transition-property:margin-top}",
              3000.0,
            ),
          )],
        )],
        &[(
          "default",
          &[("transitionProperty-x1cfch2b", &["transitionProperty"])],
        )],
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
        &[("default", &[("willChange-k6sLGO", "willChange-x1a6dnx1")])],
        &[(
          "default",
          &[(
            "willChange-x1a6dnx1",
            (".willChange-x1a6dnx1{will-change:margin-top}", 3000.0),
          )],
        )],
        &[("default", &[("willChange-x1a6dnx1", &["willChange"])])],
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
        &[(
          "default",
          &[("transitionProperty-k1ekBW", "transitionProperty-x17389it")],
        )],
        &[(
          "default",
          &[(
            "transitionProperty-x17389it",
            (
              ".transitionProperty-x17389it{transition-property:--foo}",
              3000.0,
            ),
          )],
        )],
        &[(
          "default",
          &[("transitionProperty-x17389it", &["transitionProperty"])],
        )],
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
        &[("default", &[("willChange-k6sLGO", "willChange-x1lxaxzv")])],
        &[(
          "default",
          &[(
            "willChange-x1lxaxzv",
            (".willChange-x1lxaxzv{will-change:--foo}", 3000.0),
          )],
        )],
        &[("default", &[("willChange-x1lxaxzv", &["willChange"])])],
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
        &[(
          "default",
          &[("transitionProperty-k1ekBW", "transitionProperty-x95ccmk")],
        )],
        &[(
          "default",
          &[(
            "transitionProperty-x95ccmk",
            (
              ".transitionProperty-x95ccmk{transition-property:opacity,margin-top}",
              3000.0,
            ),
          )],
        )],
        &[(
          "default",
          &[("transitionProperty-x95ccmk", &["transitionProperty"])],
        )],
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
            ("padding-kmVPX3", "padding-x1lmef92"),
            ("paddingTop-kLKAdn", "paddingTop-xexx8yu"),
          ],
        )],
        &[(
          "default",
          &[
            (
              "padding-x1lmef92",
              (
                ".padding-x1lmef92{padding:calc((100% - 50px) * .5) var(--rightpadding,20px)}",
                1000.0,
              ),
            ),
            (
              "paddingTop-xexx8yu",
              (".paddingTop-xexx8yu{padding-top:0}", 4000.0),
            ),
          ],
        )],
        &[(
          "short",
          &[
            ("padding-x1lmef92", &["padding"]),
            ("paddingTop-xexx8yu", &["paddingTop"]),
          ],
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
        &[(
          "default",
          &[("--background-color", "--background-color-xgau0yw")],
        )],
        &[(
          "default",
          &[(
            "--background-color-xgau0yw",
            (".--background-color-xgau0yw{--background-color:red}", 1.0),
          )],
        )],
        &[(
          "default",
          &[("--background-color-xgau0yw", &["--background-color"])],
        )],
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
        &[("default", &[("--final-color", "--final-color-x13tgbkp")])],
        &[(
          "default",
          &[(
            "--final-color-x13tgbkp",
            (
              ".--final-color-x13tgbkp{--final-color:var(--background-color)}",
              1.0,
            ),
          )],
        )],
        &[("default", &[("--final-color-x13tgbkp", &["--final-color"])])],
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
          (
            "default",
            &[("backgroundColor-kWkggS", "backgroundColor-xrkmrrc")],
          ),
          ("default2", &[("color-kMwMTN", "color-xju2f9n")]),
        ],
        &[
          (
            "default",
            &[("color-xju2f9n", (".color-xju2f9n{color:blue}", 3000.0))],
          ),
          (
            "backgroundColor-xrkmrrc",
            &[(
              "backgroundColor-xrkmrrc",
              (".backgroundColor-xrkmrrc{background-color:red}", 3000.0),
            )],
          ),
        ],
        &[
          (
            "default",
            &[("backgroundColor-xrkmrrc", &["backgroundColor"])],
          ),
          ("default2", &[("color-xju2f9n", &["color"])]),
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
        &[("default", &[("content-kah6P1", "content-xd71okc")])],
        &[(
          "default",
          &[(
            "content-xd71okc",
            (".content-xd71okc{content:attr(some-attribute)}", 3000.0),
          )],
        )],
        &[("default", &[("content-xd71okc", &["content"])])],
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
        &[("default", &[("--foo", "--foo-xwzgxvi")])],
        &[(
          "default",
          &[("--foo-xwzgxvi", (".--foo-xwzgxvi{--foo:500}", 1.0))],
        )],
        &[("default", &[("--foo-xwzgxvi", &["--foo"])])],
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
            (":hover_backgroundColor-kGzVvX", "backgroundColor-x1gykpug"),
            (":hover_color-kDPRdz", "color-x17z2mba"),
          ],
        )],
        &[(
          "default",
          &[
            (
              "backgroundColor-x1gykpug",
              (
                ".backgroundColor-x1gykpug:hover{background-color:red}",
                3130.0,
              ),
            ),
            (
              "color-x17z2mba",
              (".color-x17z2mba:hover{color:blue}", 3130.0),
            ),
          ],
        )],
        &[(
          "default",
          &[
            ("backgroundColor-x1gykpug", &[":hover", "backgroundColor"]),
            ("color-x17z2mba", &[":hover", "color"]),
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
        &[(
          "default",
          &[("::before_color-kxBb7d", "color-x16oeupf color-xeb2lg0")],
        )],
        &[(
          "default",
          &[
            (
              "color-x16oeupf",
              (".color-x16oeupf::before{color:red}", 8000.0),
            ),
            (
              "color-xeb2lg0",
              (".color-xeb2lg0::before:hover{color:blue}", 8130.0),
            ),
          ],
        )],
        &[(
          "default",
          &[
            ("color-x16oeupf", &["::before", "default", "color"]),
            ("color-xeb2lg0", &["::before", ":hover", "color"]),
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
            ("::before_:hover_color-kkC3X7", "color-xeb2lg0"),
            ("::before_color-kxBb7d", "color-x16oeupf"),
          ],
        )],
        &[(
          "default",
          &[
            (
              "color-x16oeupf",
              (".color-x16oeupf::before{color:red}", 8000.0),
            ),
            (
              "color-xeb2lg0",
              (".color-xeb2lg0::before:hover{color:blue}", 8130.0),
            ),
          ],
        )],
        &[(
          "default",
          &[
            ("color-x16oeupf", &["::before", "color"]),
            ("color-xeb2lg0", &["::before", ":hover", "color"]),
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
            ("::before_color-kxBb7d", "color-x16oeupf"),
            (
              ":hover_::before_color-kFlxxK",
              "color-xzzpreb color-x1gobd9t color-x1lvqgcc",
            ),
          ],
        )],
        &[(
          "default",
          &[
            (
              "color-x16oeupf",
              (".color-x16oeupf::before{color:red}", 8000.0),
            ),
            (
              "color-x1gobd9t",
              (".color-x1gobd9t:hover::before:hover{color:green}", 8260.0),
            ),
            (
              "color-x1lvqgcc",
              (".color-x1lvqgcc:hover::before:active{color:yellow}", 8300.0),
            ),
            (
              "color-xzzpreb",
              (".color-xzzpreb:hover::before{color:blue}", 8130.0),
            ),
          ],
        )],
        &[(
          "default",
          &[
            ("color-x16oeupf", &["::before", "color"]),
            ("color-x1gobd9t", &[":hover", "::before", ":hover", "color"]),
            (
              "color-x1lvqgcc",
              &[":hover", "::before", ":active", "color"],
            ),
            ("color-xzzpreb", &[":hover", "::before", "default", "color"]),
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
      &Rc::new(FlatCompiledStylesValue::String("color-xeb2lg0".to_string()))
    );

    assert_eq!(
      hover_before_class,
      &Rc::new(FlatCompiledStylesValue::String("color-xeb2lg0".to_string()))
    );

    assert_ne!(before_hover_class, hover_before_class)
  }

  #[test]
  fn transforms_array_values_as_fallbacks() {
    let object = style_array_object_factory(&[("default", &[("position", &["sticky", "fixed"])])]);

    let (resolved_namespaces, injected_styles, class_paths_in_namespace) = stylex_create(object);

    let (expected_resolved_namespaces, expected_injected_styles, expected_class_paths_in_namespace) =
      exprected_result_factory(
        &[("default", &[("position-kVAEAm", "position-x1ruww2u")])],
        &[(
          "default",
          &[(
            "position-x1ruww2u",
            (".position-x1ruww2u{position:sticky;position:fixed}", 3000.0),
          )],
        )],
        &[("default", &[("position-x1ruww2u", &["position"])])],
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
            ("borderStyle-ksu8eU", "borderStyle-xbsl7fq"),
            ("borderWidth-kMzoRj", "borderWidth-xmkeg23"),
            ("overflow-kVQacm", "overflow-xb3r6kr"),
          ],
        )],
        &[(
          "default",
          &[
            (
              "overflow-xb3r6kr",
              (".overflow-xb3r6kr{overflow:hidden}", 2000.0),
            ),
            (
              "borderStyle-xbsl7fq",
              (".borderStyle-xbsl7fq{border-style:dashed}", 2000.0),
            ),
            (
              "borderWidth-xmkeg23",
              (".borderWidth-xmkeg23{border-width:1px}", 2000.0),
            ),
          ],
        )],
        &[(
          "default",
          &[
            ("overflow-xb3r6kr", &["overflow"]),
            ("borderStyle-xbsl7fq", &["borderStyle"]),
            ("borderWidth-xmkeg23", &["borderWidth"]),
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

    def.push(key_value_ident_factory(
      "backgroundColor",
      string_to_expression("red"),
    ));

    let (resolved_namespaces, injected_styles, class_paths_in_namespace) = stylex_create(object);

    let (expected_resolved_namespaces, expected_injected_styles, expected_class_paths_in_namespace) =
      exprected_result_factory(
        &[(
          "default",
          &[
            (
              "@media (min-width: 1000px)_backgroundColor-ksQ81T",
              "backgroundColor-xc445zv",
            ),
            (
              "@media (min-width: 2000px)_backgroundColor-kkpvmn",
              "backgroundColor-x1ssfqz5",
            ),
            ("backgroundColor-kWkggS", "backgroundColor-xrkmrrc"),
          ],
        )],
        &[(
          "default",
          &[
            (
              "backgroundColor-x1ssfqz5",
              (
                "@media (min-width: 2000px){.backgroundColor-x1ssfqz5.backgroundColor-x1ssfqz5{background-color:purple}}",
                3200.0,
              ),
            ),
            (
              "backgroundColor-xc445zv",
              (
                "@media (min-width: 1000px){.backgroundColor-xc445zv.backgroundColor-xc445zv{background-color:blue}}",
                3200.0,
              ),
            ),
            (
              "backgroundColor-xrkmrrc",
              (".backgroundColor-xrkmrrc{background-color:red}", 3000.0),
            ),
          ],
        )],
        &[(
          "default",
          &[
            (
              "backgroundColor-xc445zv",
              &["@media (min-width: 1000px)", "backgroundColor"],
            ),
            (
              "backgroundColor-x1ssfqz5",
              &["@media (min-width: 2000px)", "backgroundColor"],
            ),
            ("backgroundColor-xrkmrrc", &["backgroundColor"]),
          ],
        )],
      );

    assert_eq!(resolved_namespaces, expected_resolved_namespaces);
    assert_eq!(injected_styles, expected_injected_styles);
    assert_eq!(class_paths_in_namespace, expected_class_paths_in_namespace)
  }
}
