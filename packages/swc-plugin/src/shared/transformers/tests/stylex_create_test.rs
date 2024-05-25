#[cfg(test)]
mod stylex_create {
  use indexmap::IndexMap;
  use swc_core::ecma::ast::{Expr, ExprOrSpread, KeyValueProp};

  use crate::shared::{
    enums::data_structures::{
      evaluate_result_value::EvaluateResultValue,
      flat_compiled_styles_value::FlatCompiledStylesValue,
    },
    structures::{
      functions::FunctionMap, injectable_style::InjectableStyle, state_manager::StateManager,
      types::FlatCompiledStyles,
    },
    transformers::stylex_create::stylex_create_set,
    utils::ast::{
      convertors::string_to_expression,
      factories::{
        array_expression_factory, key_value_factory, object_expression_factory,
        prop_or_spread_string_factory,
      },
    },
  };

  fn style_object_factory(
    args: &[(&str, &[(&str, &str)])],
  ) -> IndexMap<Box<Expr>, Vec<KeyValueProp>> {
    let mut object = IndexMap::new();

    for (key, value) in args {
      object.insert(
        Box::new(string_to_expression(key).unwrap()),
        value
          .iter()
          .map(|(key, value)| key_value_factory(key, string_to_expression(value).unwrap()))
          .collect(),
      );
    }

    object
  }

  type StyleNestedObjectFactoryArgs<'a> = [(&'a str, &'a [(&'a str, &'a [(&'a str, &'a str)])])];

  fn style_nested_object_factory(
    args: &StyleNestedObjectFactoryArgs,
  ) -> IndexMap<Box<Expr>, Vec<KeyValueProp>> {
    let mut object = IndexMap::new();

    for (key, value) in args {
      object.insert(
        Box::new(string_to_expression(key).unwrap()),
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
              )
              .unwrap(),
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
  ) -> IndexMap<Box<Expr>, Vec<KeyValueProp>> {
    let mut object = IndexMap::new();

    for (key, value) in args {
      object.insert(
        Box::new(string_to_expression(key).unwrap()),
        value
          .iter()
          .map(|(key, value)| {
            let elems = value
              .iter()
              .map(|arg| {
                Option::Some(ExprOrSpread {
                  spread: Option::None,
                  expr: Box::new(string_to_expression(arg).unwrap()),
                })
              })
              .collect::<Vec<Option<ExprOrSpread>>>();

            key_value_factory(key, array_expression_factory(elems).unwrap())
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
  ) -> (
    IndexMap<String, Box<FlatCompiledStyles>>,
    IndexMap<String, Box<InjectableStyle>>,
  ) {
    let mut expected_resolved_namespaces = IndexMap::new();
    let mut expected_injected_styles = IndexMap::new();

    for (resolved_namespace, namespace) in resolved_namespaces {
      let mut default_val = IndexMap::new();

      default_val.insert(
        "$$css".to_string(),
        Box::new(FlatCompiledStylesValue::Bool(true)),
      );

      for (key, value) in namespace.iter() {
        default_val.insert(
          key.to_string(),
          Box::new(if value.eq(&"null") {
            FlatCompiledStylesValue::Null
          } else {
            FlatCompiledStylesValue::String(value.to_string())
          }),
        );
      }

      expected_resolved_namespaces.insert(resolved_namespace.to_string(), Box::new(default_val));
    }

    for injected_style in injected_styles {
      for (key, inj) in injected_style.1 {
        let (value, priority) = inj;
        expected_injected_styles.insert(
          key.to_string(),
          Box::new(InjectableStyle {
            ltr: value.to_string(),
            rtl: Option::None,
            priority: Option::Some(*priority),
          }),
        );
      }
    }

    (expected_resolved_namespaces, expected_injected_styles)
  }

  fn stylex_create(
    style_object: IndexMap<Box<Expr>, Vec<KeyValueProp>>,
  ) -> (
    IndexMap<String, Box<FlatCompiledStyles>>,
    IndexMap<String, Box<InjectableStyle>>,
  ) {
    stylex_create_set(
      &EvaluateResultValue::Map(style_object),
      &mut StateManager::default(),
      &FunctionMap::default(),
    )
  }

  #[test]
  fn color_red() {
    let object =
      style_object_factory(&[("default", &[("backgroundColor", "red"), ("color", "blue")])]);

    let (resolved_namespaces, injected_styles) = stylex_create(object);

    let (expected_resolved_namespaces, expected_injected_styles) = exprected_result_factory(
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
    );

    assert_eq!(resolved_namespaces, expected_resolved_namespaces);
    assert_eq!(injected_styles, expected_injected_styles);
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

    let (resolved_namespaces, injected_styles) = camel_case;

    let (expected_resolved_namespaces, expected_injected_styles) = exprected_result_factory(
      &[("default", &[("transitionProperty", "x1cfch2b")])],
      &[(
        "default",
        &[(
          "x1cfch2b",
          (".x1cfch2b{transition-property:margin-top}", 3000.0),
        )],
      )],
    );

    assert_eq!(resolved_namespaces, expected_resolved_namespaces);
    assert_eq!(injected_styles, expected_injected_styles);
  }

  #[test]
  fn will_change_margin_top() {
    let camel_case_object = style_object_factory(&[("default", &[("willChange", "marginTop")])]);
    let kebab_case_object = style_object_factory(&[("default", &[("willChange", "margin-top")])]);

    let camel_case = stylex_create(camel_case_object);
    let kebab_case = stylex_create(kebab_case_object);

    assert_eq!(camel_case, kebab_case);

    let (resolved_namespaces, injected_styles) = camel_case;

    let (expected_resolved_namespaces, expected_injected_styles) = exprected_result_factory(
      &[("default", &[("willChange", "x1a6dnx1")])],
      &[(
        "default",
        &[("x1a6dnx1", (".x1a6dnx1{will-change:margin-top}", 3000.0))],
      )],
    );

    assert_eq!(resolved_namespaces, expected_resolved_namespaces);
    assert_eq!(injected_styles, expected_injected_styles);
  }

  #[test]
  fn transition_property_dash_dash_foo() {
    let object = style_object_factory(&[("default", &[("transitionProperty", "--foo")])]);

    let (resolved_namespaces, injected_styles) = stylex_create(object);

    let (expected_resolved_namespaces, expected_injected_styles) = exprected_result_factory(
      &[("default", &[("transitionProperty", "x17389it")])],
      &[(
        "default",
        &[("x17389it", (".x17389it{transition-property:--foo}", 3000.0))],
      )],
    );

    assert_eq!(resolved_namespaces, expected_resolved_namespaces);
    assert_eq!(injected_styles, expected_injected_styles);
  }

  #[test]
  fn will_change_dash_dash_foo() {
    let object = style_object_factory(&[("default", &[("willChange", "--foo")])]);

    let (resolved_namespaces, injected_styles) = stylex_create(object);

    let (expected_resolved_namespaces, expected_injected_styles) = exprected_result_factory(
      &[("default", &[("willChange", "x1lxaxzv")])],
      &[(
        "default",
        &[("x1lxaxzv", (".x1lxaxzv{will-change:--foo}", 3000.0))],
      )],
    );

    assert_eq!(resolved_namespaces, expected_resolved_namespaces);
    assert_eq!(injected_styles, expected_injected_styles);
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

    let (resolved_namespaces, injected_styles) = camel_case;

    let (expected_resolved_namespaces, expected_injected_styles) = exprected_result_factory(
      &[("default", &[("transitionProperty", "x95ccmk")])],
      &[(
        "default",
        &[(
          "x95ccmk",
          (".x95ccmk{transition-property:opacity,margin-top}", 3000.0),
        )],
      )],
    );

    assert_eq!(resolved_namespaces, expected_resolved_namespaces);
    assert_eq!(injected_styles, expected_injected_styles);
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

    let (resolved_namespaces, injected_styles) = stylex_create(object);

    let (expected_resolved_namespaces, expected_injected_styles) = exprected_result_factory(
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
    );

    assert_eq!(resolved_namespaces, expected_resolved_namespaces);
    assert_eq!(injected_styles, expected_injected_styles);
  }

  #[test]
  fn transforms_style_object_with_custom_property() {
    let object = style_object_factory(&[("default", &[("--background-color", "red")])]);

    let (resolved_namespaces, injected_styles) = stylex_create(object);

    let (expected_resolved_namespaces, expected_injected_styles) = exprected_result_factory(
      &[("default", &[("--background-color", "xgau0yw")])],
      &[(
        "default",
        &[("xgau0yw", (".xgau0yw{--background-color:red}", 1.0))],
      )],
    );

    assert_eq!(resolved_namespaces, expected_resolved_namespaces);
    assert_eq!(injected_styles, expected_injected_styles);
  }

  #[test]
  fn transforms_style_object_with_custom_property_as_value() {
    let object =
      style_object_factory(&[("default", &[("--final-color", "var(--background-color)")])]);

    let (resolved_namespaces, injected_styles) = stylex_create(object);

    let (expected_resolved_namespaces, expected_injected_styles) = exprected_result_factory(
      &[("default", &[("--final-color", "x13tgbkp")])],
      &[(
        "default",
        &[(
          "x13tgbkp",
          (".x13tgbkp{--final-color:var(--background-color)}", 1.0),
        )],
      )],
    );

    assert_eq!(resolved_namespaces, expected_resolved_namespaces);
    assert_eq!(injected_styles, expected_injected_styles);
  }

  #[test]
  fn transforms_multiple_namespaces() {
    let object = style_object_factory(&[
      ("default", &[("backgroundColor", "red")]),
      ("default2", &[("color", "blue")]),
    ]);

    let (resolved_namespaces, injected_styles) = stylex_create(object);

    let (expected_resolved_namespaces, expected_injected_styles) = exprected_result_factory(
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
    );

    assert_eq!(resolved_namespaces, expected_resolved_namespaces);
    assert_eq!(injected_styles, expected_injected_styles);
  }

  #[test]
  fn does_not_transform_attr_value() {
    let object = style_object_factory(&[("default", &[("content", "attr(some-attribute)")])]);

    let (resolved_namespaces, injected_styles) = stylex_create(object);

    let (expected_resolved_namespaces, expected_injected_styles) = exprected_result_factory(
      &[("default", &[("content", "xd71okc")])],
      &[(
        "default",
        &[(
          "xd71okc",
          (".xd71okc{content:attr(some-attribute)}", 3000.0),
        )],
      )],
    );

    assert_eq!(resolved_namespaces, expected_resolved_namespaces);
    assert_eq!(injected_styles, expected_injected_styles);
  }

  #[test]
  fn transforms_nested_pseudo_class_to_css() {
    let object = style_nested_object_factory(&[(
      "default",
      &[(":hover", &[("backgroundColor", "red"), ("color", "blue")])],
    )]);

    let (resolved_namespaces, injected_styles) = stylex_create(object);

    let (expected_resolved_namespaces, expected_injected_styles) = exprected_result_factory(
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
    );

    assert_eq!(resolved_namespaces, expected_resolved_namespaces);
    assert_eq!(injected_styles, expected_injected_styles);
  }

  #[test]
  fn transforms_array_values_as_fallbacks() {
    let object = style_array_object_factory(&[("default", &[("position", &["sticky", "fixed"])])]);

    let (resolved_namespaces, injected_styles) = stylex_create(object);

    let (expected_resolved_namespaces, expected_injected_styles) = exprected_result_factory(
      &[("default", &[("position", "x1ruww2u")])],
      &[(
        "default",
        &[(
          "x1ruww2u",
          (".x1ruww2u{position:sticky;position:fixed}", 3000.0),
        )],
      )],
    );

    assert_eq!(resolved_namespaces, expected_resolved_namespaces);
    assert_eq!(injected_styles, expected_injected_styles);
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

    let (resolved_namespaces, injected_styles) = stylex_create(object);

    let (expected_resolved_namespaces, expected_injected_styles) = exprected_result_factory(
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
    );

    assert_eq!(resolved_namespaces, expected_resolved_namespaces);
    assert_eq!(injected_styles, expected_injected_styles);
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

    let def = object
      .get_mut(&string_to_expression("default").unwrap())
      .unwrap();

    def.push(key_value_factory(
      "backgroundColor",
      string_to_expression("red").unwrap(),
    ));

    let (resolved_namespaces, injected_styles) = stylex_create(object);

    let (expected_resolved_namespaces, expected_injected_styles) = exprected_result_factory(
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
    );

    assert_eq!(resolved_namespaces, expected_resolved_namespaces);
    assert_eq!(injected_styles, expected_injected_styles);
  }
}
