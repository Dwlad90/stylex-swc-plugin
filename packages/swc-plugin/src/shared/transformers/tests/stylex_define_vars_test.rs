#[cfg(test)]
mod stylex_define_vars {
  use std::rc::Rc;

  use indexmap::IndexMap;
  use swc_core::ecma::ast::{Expr, PropOrSpread};

  use crate::shared::{
    enums::data_structures::{
      evaluate_result_value::EvaluateResultValue,
      flat_compiled_styles_value::FlatCompiledStylesValue, value_with_default::ValueWithDefault,
    },
    structures::{
      base_css_type::BaseCSSType, functions::FunctionType, injectable_style::InjectableStyle,
      state_manager::StateManager, stylex_state_options::StyleXStateOptions,
    },
    transformers::{stylex_define_vars::stylex_define_vars, stylex_types::get_types_fn},
    utils::{
      ast::{
        convertors::string_to_expression,
        factories::{
          object_expression_factory, prop_or_spread_expr_factory,
          prop_or_spread_expression_factory, prop_or_spread_string_factory,
        },
      },
      common::create_hash,
    },
  };

  type DefaultVarsFactoryArgs<'a> = [(
    &'a str,
    &'a [(&'a str, &'a str)],
    &'a [(&'a str, &'a [(&'a str, &'a str)])],
    &'a [Expr],
  )];
  fn default_vars_factory(
    args: &DefaultVarsFactoryArgs,
    str_args: &[(&str, &str)],
  ) -> EvaluateResultValue {
    let mut props = args
      .iter()
      .map(|(key, values, nested_values, types_values)| {
        let mut props = values
          .iter()
          .map(|(key, value)| prop_or_spread_expression_factory(key, string_to_expression(value)))
          .collect::<Vec<PropOrSpread>>();

        let nested_props = nested_values
          .iter()
          .map(|val| {
            let props = val
              .1
              .iter()
              .map(|(key, value)| {
                prop_or_spread_expression_factory(key, string_to_expression(value))
              })
              .collect::<Vec<PropOrSpread>>();

            prop_or_spread_expression_factory(val.0, object_expression_factory(props))
          })
          .collect::<Vec<PropOrSpread>>();

        props.extend(nested_props);

        let types_props = types_values
          .iter()
          .flat_map(|expr| expr.as_object().unwrap().props.clone())
          .collect::<Vec<PropOrSpread>>();

        props.extend(types_props);

        prop_or_spread_expr_factory(key, props)
      })
      .collect::<Vec<PropOrSpread>>();

    for (key, value) in str_args.iter() {
      props.push(prop_or_spread_string_factory(key, value));
    }

    EvaluateResultValue::Expr(Box::new(object_expression_factory(props)))
  }

  fn exprected_css_result_factory(
    injected_styles: &[(&str, (&str, f64))],
  ) -> IndexMap<String, Box<InjectableStyle>> {
    let mut expected_injected_styles = IndexMap::new();

    for injected_style in injected_styles {
      let (key, value) = injected_style;
      expected_injected_styles.insert(
        key.to_string(),
        Box::new(InjectableStyle {
          ltr: value.0.to_string(),
          rtl: None,
          priority: Some(value.1),
        }),
      );
    }
    expected_injected_styles
  }

  fn exprected_js_result_factory(
    js_output: &[(&str, &str)],
  ) -> IndexMap<String, Box<FlatCompiledStylesValue>> {
    let mut expected_injected_styles = IndexMap::new();

    for (key, value) in js_output {
      expected_injected_styles.insert(
        key.to_string(),
        Box::new(FlatCompiledStylesValue::String(value.to_string())),
      );
    }

    expected_injected_styles
  }

  #[test]
  fn converts_set_of_vars_to_css() {
    let theme_name = "TestTheme.stylex.js//buttonTheme";
    let class_name_prefix = 'x';

    let default_vars = default_vars_factory(
      &[
        (
          "bgColor",
          &[
            ("default", "blue"),
            ("@media (prefers-color-scheme: dark)", "lightblue"),
            ("@media print", "white"),
          ],
          &[],
          &[],
        ),
        (
          "bgColorDisabled",
          &[
            ("default", "grey"),
            ("@media (prefers-color-scheme: dark)", "rgba(0, 0, 0, 0.8)"),
          ],
          &[],
          &[],
        ),
        ("fgColor", &[("default", "pink")], &[], &[]),
      ],
      &[("cornerRadius", "10px")],
    );

    let mut state = Box::new(StateManager {
      theme_name: Some(theme_name.to_string()),
      ..StateManager::default()
    });

    let (js_output, css_output) = stylex_define_vars(&default_vars, &mut state);

    assert_eq!(
      js_output,
      exprected_js_result_factory(&[
        (
          "__themeName__",
          format!("{}{}", class_name_prefix, create_hash(theme_name)).as_str()
        ),
        (
          "bgColor",
          format!(
            "var(--{}{})",
            class_name_prefix,
            create_hash(format!("{}.bgColor", theme_name).as_str())
          )
          .as_str()
        ),
        (
          "bgColorDisabled",
          format!(
            "var(--{}{})",
            class_name_prefix,
            create_hash(format!("{}.bgColorDisabled", theme_name).as_str())
          )
          .as_str()
        ),
        (
          "cornerRadius",
          format!(
            "var(--{}{})",
            class_name_prefix,
            create_hash(format!("{}.cornerRadius", theme_name).as_str())
          )
          .as_str()
        ),
        (
          "fgColor",
          format!(
            "var(--{}{})",
            class_name_prefix,
            create_hash(format!("{}.fgColor", theme_name).as_str())
          )
          .as_str()
        ),
      ])
    );

    assert_eq!(
      css_output,
      exprected_css_result_factory(&[(
        "x568ih9",
        (
          ":root{--xgck17p:blue;--xpegid5:grey;--x4y59db:pink;--xrqfjmn:10px;}",
          0.0
        )
      ),
      (
        "x568ih9-1lveb7",
        (
          "@media (prefers-color-scheme: dark){:root{--xgck17p:lightblue;--xpegid5:rgba(0, 0, 0, 0.8);}}",
          0.1
        )
      ),
      (
        "x568ih9-bdddrq",
        (
          "@media print{:root{--xgck17p:white;}}",
          0.1
        )
      )])
    )
  }

  #[test]
  fn maintains_literal_var_names_in_css() {
    let theme_name = "TestTheme.stylex.js//buttonTheme";
    let class_name_prefix = 'x';

    let default_vars = default_vars_factory(
      &[
        (
          "--bgColor",
          &[
            ("default", "blue"),
            ("@media (prefers-color-scheme: dark)", "lightblue"),
            ("@media print", "white"),
          ],
          &[],
          &[],
        ),
        (
          "--bgColorDisabled",
          &[
            ("default", "grey"),
            ("@media (prefers-color-scheme: dark)", "rgba(0, 0, 0, 0.8)"),
          ],
          &[],
          &[],
        ),
        ("--fgColor", &[("default", "pink")], &[], &[]),
      ],
      &[("--cornerRadius", "10px")],
    );

    let mut state = Box::new(StateManager {
      theme_name: Some(theme_name.to_string()),
      ..StateManager::default()
    });

    let (js_output, css_output) = stylex_define_vars(&default_vars, &mut state);

    assert_eq!(
      js_output,
      exprected_js_result_factory(&[
        (
          "__themeName__",
          format!("{}{}", class_name_prefix, create_hash(theme_name)).as_str()
        ),
        ("--bgColor", "var(--bgColor)"),
        ("--bgColorDisabled", "var(--bgColorDisabled)"),
        ("--cornerRadius", "var(--cornerRadius)"),
        ("--fgColor", "var(--fgColor)"),
      ])
    );

    assert_eq!(
      css_output,
      exprected_css_result_factory(&[(
        "x568ih9",
        (
          ":root{--bgColor:blue;--bgColorDisabled:grey;--fgColor:pink;--cornerRadius:10px;}",
          0.0
        )
      ),
      (
        "x568ih9-1lveb7",
        (
          "@media (prefers-color-scheme: dark){:root{--bgColor:lightblue;--bgColorDisabled:rgba(0, 0, 0, 0.8);}}",
          0.1
        )
      ),
      (
        "x568ih9-bdddrq",
        (
          "@media print{:root{--bgColor:white;}}",
          0.1
        )
      )])
    )
  }

  #[test]
  fn converts_set_of_vars_with_nested_at_rules_to_css() {
    let theme_name = "TestTheme.stylex.js//buttonTheme";
    let class_name_prefix = 'x';

    let default_vars = default_vars_factory(
      &[
        (
          "bgColor",
          &[("default", "blue"), ("@media print", "white")],
          &[(
            "@media (prefers-color-scheme: dark)",
            &[
              ("default", "lightblue"),
              ("@supports (color: oklab(0 0 0))", "oklab(0.7 -0.3 -0.4)"),
            ],
          )],
          &[],
        ),
        (
          "bgColorDisabled",
          &[],
          &[
            (
              "default",
              &[
                ("default", "grey"),
                ("@supports (color: oklab(0 0 0))", "oklab(0.7 -0.3 -0.4)"),
              ],
            ),
            (
              "@media (prefers-color-scheme: dark)",
              &[
                ("default", "rgba(0, 0, 0, 0.8)"),
                ("@supports (color: oklab(0 0 0))", "oklab(0.7 -0.3 -0.4)"),
              ],
            ),
          ],
          &[],
        ),
        ("fgColor", &[("default", "pink")], &[], &[]),
      ],
      &[("cornerRadius", "10px")],
    );

    let mut state = Box::new(StateManager {
      theme_name: Some(theme_name.to_string()),
      ..StateManager::default()
    });

    let (js_output, css_output) = stylex_define_vars(&default_vars, &mut state);

    assert_eq!(
      js_output,
      exprected_js_result_factory(&[
        (
          "__themeName__",
          format!("{}{}", class_name_prefix, create_hash(theme_name)).as_str()
        ),
        (
          "bgColor",
          format!(
            "var(--{}{})",
            class_name_prefix,
            create_hash(format!("{}.bgColor", theme_name).as_str())
          )
          .as_str()
        ),
        (
          "bgColorDisabled",
          format!(
            "var(--{}{})",
            class_name_prefix,
            create_hash(format!("{}.bgColorDisabled", theme_name).as_str())
          )
          .as_str()
        ),
        (
          "cornerRadius",
          format!(
            "var(--{}{})",
            class_name_prefix,
            create_hash(format!("{}.cornerRadius", theme_name).as_str())
          )
          .as_str()
        ),
        (
          "fgColor",
          format!(
            "var(--{}{})",
            class_name_prefix,
            create_hash(format!("{}.fgColor", theme_name).as_str())
          )
          .as_str()
        ),
      ])
    );

    assert_eq!(
      css_output,
      exprected_css_result_factory(&[(
        "x568ih9",
        (
          ":root{--xgck17p:blue;--xpegid5:grey;--x4y59db:pink;--xrqfjmn:10px;}",
          0.0
        )
      ),
      (
        "x568ih9-1e6ryz3",
        (
          "@supports (color: oklab(0 0 0)){@media (prefers-color-scheme: dark){:root{--xgck17p:oklab(0.7 -0.3 -0.4);--xpegid5:oklab(0.7 -0.3 -0.4);}}}",
          0.2
        )
      ),
      (
        "x568ih9-1lveb7",
        (
          "@media (prefers-color-scheme: dark){:root{--xgck17p:lightblue;--xpegid5:rgba(0, 0, 0, 0.8);}}",
          0.1
        )
      ),
      (
        "x568ih9-bdddrq",
        (
          "@media print{:root{--xgck17p:white;}}",
          0.1
        )
      ),
      (
        "x568ih9-kpd015",
        (
          "@supports (color: oklab(0 0 0)){:root{--xpegid5:oklab(0.7 -0.3 -0.4);}}",
          0.1
        )
      )
      ])
    )
  }

  #[test]
  fn converts_set_of_typed_vars_with_nested_at_rules_to_css() {
    let theme_name = "TestTheme.stylex.js//buttonTheme";
    let class_name_prefix = 'x';

    let types_fn = match get_types_fn().fn_ptr {
      FunctionType::StylexFnsFactory(func) => func,
      _ => unreachable!(),
    };

    let color_fn = types_fn("color".into());
    let length_fn = types_fn("length".into());

    // #region bgColor
    let mut bg_color_map = IndexMap::new();

    bg_color_map.insert(
      "default".to_string(),
      ValueWithDefault::String("blue".to_string()),
    );

    let mut bg_color_mq_map = IndexMap::new();

    bg_color_mq_map.insert(
      "default".to_string(),
      ValueWithDefault::String("lightblue".to_string()),
    );
    bg_color_mq_map.insert(
      "@supports (color: oklab(0 0 0))".to_string(),
      ValueWithDefault::String("oklab(0.7 -0.3 -0.4)".to_string()),
    );

    bg_color_map.insert(
      "@media (prefers-color-scheme: dark)".to_string(),
      ValueWithDefault::Map(bg_color_mq_map),
    );

    bg_color_map.insert(
      "@media print".to_string(),
      ValueWithDefault::String("white".to_string()),
    );
    // #endregion bgColor

    // #region bgColorDisabled
    let mut bg_color_disabled_map = IndexMap::new();

    let mut bg_color_disabled_default_map = IndexMap::new();

    bg_color_disabled_default_map.insert(
      "default".to_string(),
      ValueWithDefault::String("grey".to_string()),
    );

    bg_color_disabled_default_map.insert(
      "@supports (color: oklab(0 0 0))".to_string(),
      ValueWithDefault::String("oklab(0.7 -0.3 -0.4)".to_string()),
    );

    bg_color_disabled_map.insert(
      "default".to_string(),
      ValueWithDefault::Map(bg_color_disabled_default_map),
    );

    let mut bg_color_disabled_mq_map = IndexMap::new();

    bg_color_disabled_mq_map.insert(
      "default".to_string(),
      ValueWithDefault::String("rgba(0, 0, 0, 0.8)".to_string()),
    );

    bg_color_disabled_mq_map.insert(
      "@supports (color: oklab(0 0 0))".to_string(),
      ValueWithDefault::String("oklab(0.7 -0.3 -0.4)".to_string()),
    );

    bg_color_disabled_map.insert(
      "@media (prefers-color-scheme: dark)".to_string(),
      ValueWithDefault::Map(bg_color_disabled_mq_map),
    );
    // #endregion bgColorDisabled

    // #region fgColor
    let mut fg_color_map = IndexMap::new();

    fg_color_map.insert(
      "default".to_string(),
      ValueWithDefault::String("pink".to_string()),
    );

    // #endregion fgColor

    let bg_color = type_fabric(&color_fn, ValueWithDefault::Map(bg_color_map));
    let bg_color_disabled = type_fabric(&color_fn, ValueWithDefault::Map(bg_color_disabled_map));

    let corner_radius = type_fabric(&length_fn, ValueWithDefault::String("10px".to_string()));

    let fg_color = type_fabric(&color_fn, ValueWithDefault::Map(fg_color_map));

    let default_vars = default_vars_factory(
      &[
        ("bgColor", &[], &[], &[bg_color.into()]),
        ("bgColorDisabled", &[], &[], &[bg_color_disabled.into()]),
        ("cornerRadius", &[], &[], &[corner_radius.into()]),
        ("fgColor", &[], &[], &[fg_color.into()]),
      ],
      &[],
    );

    let state = Box::<StateManager>::default();
    let mut state = Box::new(StateManager {
      theme_name: Some(theme_name.to_string()),
      options: Box::new(StyleXStateOptions {
        class_name_prefix: class_name_prefix.to_string(),
        ..*state.options
      }),
      ..*state
    });

    let (_, css_output) = stylex_define_vars(&default_vars, &mut state);

    assert_eq!(
      css_output,
      exprected_css_result_factory(&[(
        "x4y59db",
        (
          r#"@property --x4y59db { syntax: "<color>"; inherits: true; initial-value: pink }"#,
          0.0
        )
      ),
      (
        "x568ih9",
        (
          ":root{--xgck17p:blue;--xpegid5:grey;--xrqfjmn:10px;--x4y59db:pink;}",
          0.0
        )
      ),
      (
        "x568ih9-1e6ryz3",
        (
          "@supports (color: oklab(0 0 0)){@media (prefers-color-scheme: dark){:root{--xgck17p:oklab(0.7 -0.3 -0.4);--xpegid5:oklab(0.7 -0.3 -0.4);}}}",
          0.2
        )
      ),
      (
        "x568ih9-1lveb7",
        (
          "@media (prefers-color-scheme: dark){:root{--xgck17p:lightblue;--xpegid5:rgba(0, 0, 0, 0.8);}}",
          0.1
        )
      ),
      (
        "x568ih9-bdddrq",
        (
          "@media print{:root{--xgck17p:white;}}",
          0.1
        )
      ),
      (
        "x568ih9-kpd015",
        (
          "@supports (color: oklab(0 0 0)){:root{--xpegid5:oklab(0.7 -0.3 -0.4);}}",
          0.1
        )
      ),
      (
        "xgck17p",
        (
          r#"@property --xgck17p { syntax: "<color>"; inherits: true; initial-value: blue }"#,
          0.0
        )
      ),
      (
        "xpegid5",
        (
          r#"@property --xpegid5 { syntax: "<color>"; inherits: true; initial-value: grey }"#,
          0.0
        )
      ),

      (
        "xrqfjmn",
        (
          r#"@property --xrqfjmn { syntax: "<length>"; inherits: true; initial-value: 10px }"#,
          0.0
        )
      ),
      ])
    )
  }

  #[test]
  fn preserves_names_of_literals_with_double_dash_prefix() {
    let theme_name = "TestTheme.stylex.js//buttonTheme";
    let class_name_prefix = 'x';

    let types_fn = match get_types_fn().fn_ptr {
      FunctionType::StylexFnsFactory(func) => func,
      _ => unreachable!(),
    };

    let color_fn = types_fn("color".into());
    let length_fn = types_fn("length".into());

    // #region bgColor
    let mut bg_color_map = IndexMap::new();

    bg_color_map.insert(
      "default".to_string(),
      ValueWithDefault::String("blue".to_string()),
    );

    let mut bg_color_mq_map = IndexMap::new();

    bg_color_mq_map.insert(
      "default".to_string(),
      ValueWithDefault::String("lightblue".to_string()),
    );
    bg_color_mq_map.insert(
      "@supports (color: oklab(0 0 0))".to_string(),
      ValueWithDefault::String("oklab(0.7 -0.3 -0.4)".to_string()),
    );

    bg_color_map.insert(
      "@media (prefers-color-scheme: dark)".to_string(),
      ValueWithDefault::Map(bg_color_mq_map),
    );

    bg_color_map.insert(
      "@media print".to_string(),
      ValueWithDefault::String("white".to_string()),
    );
    // #endregion bgColor

    // #region bgColorDisabled
    let mut bg_color_disabled_map = IndexMap::new();

    let mut bg_color_disabled_default_map = IndexMap::new();

    bg_color_disabled_default_map.insert(
      "default".to_string(),
      ValueWithDefault::String("grey".to_string()),
    );

    bg_color_disabled_default_map.insert(
      "@supports (color: oklab(0 0 0))".to_string(),
      ValueWithDefault::String("oklab(0.7 -0.3 -0.4)".to_string()),
    );

    bg_color_disabled_map.insert(
      "default".to_string(),
      ValueWithDefault::Map(bg_color_disabled_default_map),
    );

    let mut bg_color_disabled_mq_map = IndexMap::new();

    bg_color_disabled_mq_map.insert(
      "default".to_string(),
      ValueWithDefault::String("rgba(0, 0, 0, 0.8)".to_string()),
    );

    bg_color_disabled_mq_map.insert(
      "@supports (color: oklab(0 0 0))".to_string(),
      ValueWithDefault::String("oklab(0.7 -0.3 -0.4)".to_string()),
    );

    bg_color_disabled_map.insert(
      "@media (prefers-color-scheme: dark)".to_string(),
      ValueWithDefault::Map(bg_color_disabled_mq_map),
    );
    // #endregion bgColorDisabled

    // #region fgColor
    let mut fg_color_map = IndexMap::new();

    fg_color_map.insert(
      "default".to_string(),
      ValueWithDefault::String("pink".to_string()),
    );

    // #endregion fgColor

    let bg_color = type_fabric(&color_fn, ValueWithDefault::Map(bg_color_map));
    let bg_color_disabled = type_fabric(&color_fn, ValueWithDefault::Map(bg_color_disabled_map));

    let corner_radius = type_fabric(&length_fn, ValueWithDefault::String("10px".to_string()));

    let fg_color = type_fabric(&color_fn, ValueWithDefault::Map(fg_color_map));

    let default_vars = default_vars_factory(
      &[
        ("--bgColor", &[], &[], &[bg_color.into()]),
        ("--bgColorDisabled", &[], &[], &[bg_color_disabled.into()]),
        ("--cornerRadius", &[], &[], &[corner_radius.into()]),
        ("--fgColor", &[], &[], &[fg_color.into()]),
      ],
      &[],
    );

    let state = Box::<StateManager>::default();
    let mut state = Box::new(StateManager {
      theme_name: Some(theme_name.to_string()),
      options: Box::new(StyleXStateOptions {
        class_name_prefix: class_name_prefix.to_string(),
        ..*state.options
      }),
      ..*state
    });

    let (_, css_output) = stylex_define_vars(&default_vars, &mut state);

    assert_eq!(
      css_output,
      exprected_css_result_factory(&[(
        "bgColor",
        (
          r#"@property --bgColor { syntax: "<color>"; inherits: true; initial-value: blue }"#,
          0.0
        )
      ),
      (
        "bgColorDisabled",
        (
          r#"@property --bgColorDisabled { syntax: "<color>"; inherits: true; initial-value: grey }"#,
          0.0
        )
      ),
      (
        "cornerRadius",
        (
          r#"@property --cornerRadius { syntax: "<length>"; inherits: true; initial-value: 10px }"#,
          0.0
        )
      ),
      (
        "fgColor",
        (
          r#"@property --fgColor { syntax: "<color>"; inherits: true; initial-value: pink }"#,
          0.0
        )
      ),
      (
        "x568ih9",
        (
          ":root{--bgColor:blue;--bgColorDisabled:grey;--cornerRadius:10px;--fgColor:pink;}",
          0.0
        )
      ),
      (
        "x568ih9-1e6ryz3",
        (
          "@supports (color: oklab(0 0 0)){@media (prefers-color-scheme: dark){:root{--bgColor:oklab(0.7 -0.3 -0.4);--bgColorDisabled:oklab(0.7 -0.3 -0.4);}}}",
          0.2
        )
      ),
      (
        "x568ih9-1lveb7",
        (
          "@media (prefers-color-scheme: dark){:root{--bgColor:lightblue;--bgColorDisabled:rgba(0, 0, 0, 0.8);}}",
          0.1
        )
      ),
      (
        "x568ih9-bdddrq",
        (
          "@media print{:root{--bgColor:white;}}",
          0.1
        )
      ),
      (
        "x568ih9-kpd015",
        (
          "@supports (color: oklab(0 0 0)){:root{--bgColorDisabled:oklab(0.7 -0.3 -0.4);}}",
          0.1
        )
      ),
      ])
    )
  }

  fn type_fabric(
    func: &Rc<dyn Fn(ValueWithDefault) -> Expr>,
    types: ValueWithDefault,
  ) -> BaseCSSType {
    let result = func(types);
    let result_object = result.as_object();
    let css_type: BaseCSSType = result_object.unwrap().clone().into();

    css_type
  }
}
