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
      base_css_type::BaseCSSType,
      functions::FunctionType,
      injectable_style::InjectableStyle,
      state_manager::StateManager,
      stylex_state_options::StyleXStateOptions,
      types::{FlatCompiledStyles, InjectableStylesMap},
    },
    transformers::{stylex_define_vars::stylex_define_vars, stylex_types::get_types_fn},
    utils::{
      ast::{
        convertors::string_to_expression,
        factories::{
          object_expression_factory, prop_or_spread_expr_factory, prop_or_spread_expression_factory,
        },
      },
      common::create_hash,
    },
  };

  enum DefaultVarsFactoryValue<'a> {
    Simple(&'a str),
    Nested(&'a [(&'a str, &'a str)]),
  }

  type DefaultVarsFactoryArgs<'a> = [(
    &'a str,
    DefaultVarsFactoryValue<'a>,
    &'a [(&'a str, &'a [(&'a str, &'a str)])],
    &'a [Expr],
  )];
  fn default_vars_factory(args: &DefaultVarsFactoryArgs) -> EvaluateResultValue {
    let props = args
      .iter()
      .map(|(key, values, nested_values, types_values)| match values {
        DefaultVarsFactoryValue::Simple(value) => {
          prop_or_spread_expression_factory(key, string_to_expression(value))
        }
        DefaultVarsFactoryValue::Nested(values) => {
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
        }
      })
      .collect::<Vec<PropOrSpread>>();

    EvaluateResultValue::Expr(object_expression_factory(props))
  }

  fn expected_css_result_factory(injected_styles: &[(&str, (&str, f64))]) -> InjectableStylesMap {
    let mut expected_injected_styles = IndexMap::new();

    for injected_style in injected_styles {
      let (key, value) = injected_style;
      expected_injected_styles.insert(
        key.to_string(),
        InjectableStyle::regular(value.0.to_string(), Some(value.1)),
      );
    }
    expected_injected_styles
  }

  fn expected_js_result_factory(js_output: &[(&str, &str)]) -> FlatCompiledStyles {
    let mut expected_injected_styles = IndexMap::new();

    for (key, value) in js_output {
      expected_injected_styles.insert(
        key.to_string(),
        Rc::new(FlatCompiledStylesValue::String(value.to_string())),
      );
    }

    expected_injected_styles
  }

  #[test]
  fn converts_set_of_vars_to_css() {
    let export_id = "TestTheme.stylex.js//buttonTheme";
    let class_name_prefix = 'x';

    let default_vars = default_vars_factory(&[
      (
        "bgColor",
        DefaultVarsFactoryValue::Nested(&[
          ("default", "blue"),
          ("@media (prefers-color-scheme: dark)", "lightblue"),
          ("@media print", "white"),
        ]),
        &[],
        &[],
      ),
      (
        "bgColorDisabled",
        DefaultVarsFactoryValue::Nested(&[
          ("default", "grey"),
          ("@media (prefers-color-scheme: dark)", "rgba(0, 0, 0, 0.8)"),
        ]),
        &[],
        &[],
      ),
      (
        "cornerRadius",
        DefaultVarsFactoryValue::Simple("10px"),
        &[],
        &[],
      ),
      (
        "fgColor",
        DefaultVarsFactoryValue::Nested(&[("default", "pink")]),
        &[],
        &[],
      ),
    ]);

    let mut state = Box::new(StateManager {
      export_id: Some(export_id.to_string()),
      ..StateManager::default()
    });

    let (js_output, css_output) = stylex_define_vars(&default_vars, &mut state);

    assert_eq!(
      js_output,
      expected_js_result_factory(&[
        (
          "__varGroupHash__",
          format!("{}{}", class_name_prefix, create_hash(export_id)).as_str()
        ),
        (
          "bgColor",
          format!(
            "var(--{}{})",
            class_name_prefix,
            create_hash(format!("{}.bgColor", export_id).as_str())
          )
          .as_str()
        ),
        (
          "bgColorDisabled",
          format!(
            "var(--{}{})",
            class_name_prefix,
            create_hash(format!("{}.bgColorDisabled", export_id).as_str())
          )
          .as_str()
        ),
        (
          "cornerRadius",
          format!(
            "var(--{}{})",
            class_name_prefix,
            create_hash(format!("{}.cornerRadius", export_id).as_str())
          )
          .as_str()
        ),
        (
          "fgColor",
          format!(
            "var(--{}{})",
            class_name_prefix,
            create_hash(format!("{}.fgColor", export_id).as_str())
          )
          .as_str()
        ),
      ])
    );

    assert_eq!(
      css_output,
      expected_css_result_factory(&[
        (
          "x568ih9",
          (
            ":root, .x568ih9{--xgck17p:blue;--xpegid5:grey;--xrqfjmn:10px;--x4y59db:pink;}",
            0.1
          )
        ),
        (
          "x568ih9-1lveb7",
          (
            "@media (prefers-color-scheme: dark){:root, .x568ih9{--xgck17p:lightblue;--xpegid5:rgba(0, 0, 0, 0.8);}}",
            0.2
          )
        ),
        (
          "x568ih9-bdddrq",
          ("@media print{:root, .x568ih9{--xgck17p:white;}}", 0.2)
        )
      ])
    )
  }

  #[test]
  fn maintains_literal_var_names_in_css() {
    let export_id = "TestTheme.stylex.js//buttonTheme";
    let class_name_prefix = 'x';

    let default_vars = default_vars_factory(&[
      (
        "--bgColor",
        DefaultVarsFactoryValue::Nested(&[
          ("default", "blue"),
          ("@media (prefers-color-scheme: dark)", "lightblue"),
          ("@media print", "white"),
        ]),
        &[],
        &[],
      ),
      (
        "--bgColorDisabled",
        DefaultVarsFactoryValue::Nested(&[
          ("default", "grey"),
          ("@media (prefers-color-scheme: dark)", "rgba(0, 0, 0, 0.8)"),
        ]),
        &[],
        &[],
      ),
      (
        "--cornerRadius",
        DefaultVarsFactoryValue::Simple("10px"),
        &[],
        &[],
      ),
      (
        "--fgColor",
        DefaultVarsFactoryValue::Nested(&[("default", "pink")]),
        &[],
        &[],
      ),
    ]);

    let mut state = Box::new(StateManager {
      export_id: Some(export_id.to_string()),
      ..StateManager::default()
    });

    let (js_output, css_output) = stylex_define_vars(&default_vars, &mut state);

    assert_eq!(
      js_output,
      expected_js_result_factory(&[
        (
          "__varGroupHash__",
          format!("{}{}", class_name_prefix, create_hash(export_id)).as_str()
        ),
        ("--bgColor", "var(--bgColor)"),
        ("--bgColorDisabled", "var(--bgColorDisabled)"),
        ("--cornerRadius", "var(--cornerRadius)"),
        ("--fgColor", "var(--fgColor)"),
      ])
    );

    assert_eq!(
      css_output,
      expected_css_result_factory(&[
        (
          "x568ih9",
          (
            ":root, .x568ih9{--bgColor:blue;--bgColorDisabled:grey;--cornerRadius:10px;--fgColor:pink;}",
            0.1
          )
        ),
        (
          "x568ih9-1lveb7",
          (
            "@media (prefers-color-scheme: dark){:root, .x568ih9{--bgColor:lightblue;--bgColorDisabled:rgba(0, 0, 0, 0.8);}}",
            0.2
          )
        ),
        (
          "x568ih9-bdddrq",
          ("@media print{:root, .x568ih9{--bgColor:white;}}", 0.2)
        )
      ])
    )
  }

  #[test]
  fn converts_set_of_vars_with_nested_at_rules_to_css() {
    let export_id = "TestTheme.stylex.js//buttonTheme";
    let class_name_prefix = 'x';

    let default_vars = default_vars_factory(&[
      (
        "bgColor",
        DefaultVarsFactoryValue::Nested(&[("default", "blue"), ("@media print", "white")]),
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
        DefaultVarsFactoryValue::Nested(&[]),
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
      (
        "cornerRadius",
        DefaultVarsFactoryValue::Simple("10px"),
        &[],
        &[],
      ),
      (
        "fgColor",
        DefaultVarsFactoryValue::Nested(&[("default", "pink")]),
        &[],
        &[],
      ),
    ]);

    let mut state = Box::new(StateManager {
      export_id: Some(export_id.to_string()),
      ..StateManager::default()
    });

    let (js_output, css_output) = stylex_define_vars(&default_vars, &mut state);

    assert_eq!(
      js_output,
      expected_js_result_factory(&[
        (
          "__varGroupHash__",
          format!("{}{}", class_name_prefix, create_hash(export_id)).as_str()
        ),
        (
          "bgColor",
          format!(
            "var(--{}{})",
            class_name_prefix,
            create_hash(format!("{}.bgColor", export_id).as_str())
          )
          .as_str()
        ),
        (
          "bgColorDisabled",
          format!(
            "var(--{}{})",
            class_name_prefix,
            create_hash(format!("{}.bgColorDisabled", export_id).as_str())
          )
          .as_str()
        ),
        (
          "cornerRadius",
          format!(
            "var(--{}{})",
            class_name_prefix,
            create_hash(format!("{}.cornerRadius", export_id).as_str())
          )
          .as_str()
        ),
        (
          "fgColor",
          format!(
            "var(--{}{})",
            class_name_prefix,
            create_hash(format!("{}.fgColor", export_id).as_str())
          )
          .as_str()
        ),
      ])
    );

    assert_eq!(
      css_output,
      expected_css_result_factory(&[
        (
          "x568ih9",
          (
            ":root, .x568ih9{--xgck17p:blue;--xpegid5:grey;--xrqfjmn:10px;--x4y59db:pink;}",
            0.1
          )
        ),
        (
          "x568ih9-1e6ryz3",
          (
            "@supports (color: oklab(0 0 0)){@media (prefers-color-scheme: dark){:root, .x568ih9{--xgck17p:oklab(0.7 -0.3 -0.4);--xpegid5:oklab(0.7 -0.3 -0.4);}}}",
            0.3
          )
        ),
        (
          "x568ih9-1lveb7",
          (
            "@media (prefers-color-scheme: dark){:root, .x568ih9{--xgck17p:lightblue;--xpegid5:rgba(0, 0, 0, 0.8);}}",
            0.2
          )
        ),
        (
          "x568ih9-bdddrq",
          ("@media print{:root, .x568ih9{--xgck17p:white;}}", 0.2)
        ),
        (
          "x568ih9-kpd015",
          (
            "@supports (color: oklab(0 0 0)){:root, .x568ih9{--xpegid5:oklab(0.7 -0.3 -0.4);}}",
            0.2
          )
        )
      ])
    )
  }

  #[test]
  fn converts_set_of_vars_with_nested_at_rules_to_css_and_includes_key_in_variable_name_as_prefix_in_debug_mode()
   {
    let export_id = "TestTheme.stylex.js//buttonTheme";
    let class_name_prefix = 'x';

    let default_vars = default_vars_factory(&[
      (
        "bgColor",
        DefaultVarsFactoryValue::Nested(&[("default", "blue"), ("@media print", "white")]),
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
        DefaultVarsFactoryValue::Nested(&[]),
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
      (
        "cornerRadius",
        DefaultVarsFactoryValue::Simple("10px"),
        &[],
        &[],
      ),
      (
        "fgColor",
        DefaultVarsFactoryValue::Nested(&[("default", "pink")]),
        &[],
        &[],
      ),
    ]);

    let mut state = Box::new(StateManager {
      export_id: Some(export_id.to_string()),
      options: StyleXStateOptions {
        debug: true,
        ..StateManager::default().options
      },
      ..StateManager::default()
    });

    let (js_output, css_output) = stylex_define_vars(&default_vars, &mut state);

    assert_eq!(
      js_output,
      expected_js_result_factory(&[
        (
          "__varGroupHash__",
          format!("{}{}", class_name_prefix, create_hash(export_id)).as_str()
        ),
        (
          "bgColor",
          format!(
            "var(--bgColor-{}{})",
            class_name_prefix,
            create_hash(format!("{}.bgColor", export_id).as_str())
          )
          .as_str()
        ),
        (
          "bgColorDisabled",
          format!(
            "var(--bgColorDisabled-{}{})",
            class_name_prefix,
            create_hash(format!("{}.bgColorDisabled", export_id).as_str())
          )
          .as_str()
        ),
        (
          "cornerRadius",
          format!(
            "var(--cornerRadius-{}{})",
            class_name_prefix,
            create_hash(format!("{}.cornerRadius", export_id).as_str())
          )
          .as_str()
        ),
        (
          "fgColor",
          format!(
            "var(--fgColor-{}{})",
            class_name_prefix,
            create_hash(format!("{}.fgColor", export_id).as_str())
          )
          .as_str()
        ),
      ])
    );

    assert_eq!(
      css_output,
      expected_css_result_factory(&[
        (
          "x568ih9",
          (
            ":root, .x568ih9{--bgColor-xgck17p:blue;--bgColorDisabled-xpegid5:grey;--cornerRadius-xrqfjmn:10px;--fgColor-x4y59db:pink;}",
            0.1
          )
        ),
        (
          "x568ih9-1e6ryz3",
          (
            "@supports (color: oklab(0 0 0)){@media (prefers-color-scheme: dark){:root, .x568ih9{--bgColor-xgck17p:oklab(0.7 -0.3 -0.4);--bgColorDisabled-xpegid5:oklab(0.7 -0.3 -0.4);}}}",
            0.3
          )
        ),
        (
          "x568ih9-1lveb7",
          (
            "@media (prefers-color-scheme: dark){:root, .x568ih9{--bgColor-xgck17p:lightblue;--bgColorDisabled-xpegid5:rgba(0, 0, 0, 0.8);}}",
            0.2
          )
        ),
        (
          "x568ih9-bdddrq",
          (
            "@media print{:root, .x568ih9{--bgColor-xgck17p:white;}}",
            0.2
          )
        ),
        (
          "x568ih9-kpd015",
          (
            "@supports (color: oklab(0 0 0)){:root, .x568ih9{--bgColorDisabled-xpegid5:oklab(0.7 -0.3 -0.4);}}",
            0.2
          )
        )
      ])
    )
  }

  #[test]
  fn converts_set_of_vars_with_nested_at_rules_to_css_and_does_not_include_key_prefix_in_debug_mode_with_debug_classnames_off()
   {
    let export_id = "TestTheme.stylex.js//buttonTheme";
    let class_name_prefix = 'x';

    let default_vars = default_vars_factory(&[
      (
        "bgColor",
        DefaultVarsFactoryValue::Nested(&[("default", "blue"), ("@media print", "white")]),
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
        DefaultVarsFactoryValue::Nested(&[]),
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
      (
        "cornerRadius",
        DefaultVarsFactoryValue::Simple("10px"),
        &[],
        &[],
      ),
      (
        "fgColor",
        DefaultVarsFactoryValue::Nested(&[("default", "pink")]),
        &[],
        &[],
      ),
    ]);

    let mut state = Box::new(StateManager {
      export_id: Some(export_id.to_string()),
      options: StyleXStateOptions {
        debug: false,
        enable_debug_class_names: false,
        ..StateManager::default().options
      },
      ..StateManager::default()
    });

    let (js_output, css_output) = stylex_define_vars(&default_vars, &mut state);

    assert_eq!(
      js_output,
      expected_js_result_factory(&[
        (
          "__varGroupHash__",
          format!("{}{}", class_name_prefix, create_hash(export_id)).as_str()
        ),
        (
          "bgColor",
          format!(
            "var(--{}{})",
            class_name_prefix,
            create_hash(format!("{}.bgColor", export_id).as_str())
          )
          .as_str()
        ),
        (
          "bgColorDisabled",
          format!(
            "var(--{}{})",
            class_name_prefix,
            create_hash(format!("{}.bgColorDisabled", export_id).as_str())
          )
          .as_str()
        ),
        (
          "cornerRadius",
          format!(
            "var(--{}{})",
            class_name_prefix,
            create_hash(format!("{}.cornerRadius", export_id).as_str())
          )
          .as_str()
        ),
        (
          "fgColor",
          format!(
            "var(--{}{})",
            class_name_prefix,
            create_hash(format!("{}.fgColor", export_id).as_str())
          )
          .as_str()
        ),
      ])
    );

    assert_eq!(
      css_output,
      expected_css_result_factory(&[
        (
          "x568ih9",
          (
            ":root, .x568ih9{--xgck17p:blue;--xpegid5:grey;--xrqfjmn:10px;--x4y59db:pink;}",
            0.1
          )
        ),
        (
          "x568ih9-1e6ryz3",
          (
            "@supports (color: oklab(0 0 0)){@media (prefers-color-scheme: dark){:root, .x568ih9{--xgck17p:oklab(0.7 -0.3 -0.4);--xpegid5:oklab(0.7 -0.3 -0.4);}}}",
            0.3
          )
        ),
        (
          "x568ih9-1lveb7",
          (
            "@media (prefers-color-scheme: dark){:root, .x568ih9{--xgck17p:lightblue;--xpegid5:rgba(0, 0, 0, 0.8);}}",
            0.2
          )
        ),
        (
          "x568ih9-bdddrq",
          ("@media print{:root, .x568ih9{--xgck17p:white;}}", 0.2)
        ),
        (
          "x568ih9-kpd015",
          (
            "@supports (color: oklab(0 0 0)){:root, .x568ih9{--xpegid5:oklab(0.7 -0.3 -0.4);}}",
            0.2
          )
        )
      ])
    )
  }

  #[test]
  fn converts_set_of_typed_vars_with_nested_at_rules_to_css() {
    let export_id = "TestTheme.stylex.js//buttonTheme";
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

    let default_vars = default_vars_factory(&[
      (
        "bgColor",
        DefaultVarsFactoryValue::Nested(&[]),
        &[],
        &[bg_color.into()],
      ),
      (
        "bgColorDisabled",
        DefaultVarsFactoryValue::Nested(&[]),
        &[],
        &[bg_color_disabled.into()],
      ),
      (
        "cornerRadius",
        DefaultVarsFactoryValue::Nested(&[]),
        &[],
        &[corner_radius.into()],
      ),
      (
        "fgColor",
        DefaultVarsFactoryValue::Nested(&[]),
        &[],
        &[fg_color.into()],
      ),
    ]);

    let state = Box::<StateManager>::default();
    let mut state = Box::new(StateManager {
      export_id: Some(export_id.to_string()),
      options: StyleXStateOptions {
        class_name_prefix: class_name_prefix.to_string(),
        ..state.options.clone()
      },
      ..*state
    });

    let (_, css_output) = stylex_define_vars(&default_vars, &mut state);

    assert_eq!(
      css_output,
      expected_css_result_factory(&[
        (
          "x4y59db",
          (
            r#"@property --x4y59db { syntax: "<color>"; inherits: true; initial-value: pink }"#,
            0.0
          )
        ),
        (
          "x568ih9",
          (
            ":root, .x568ih9{--xgck17p:blue;--xpegid5:grey;--xrqfjmn:10px;--x4y59db:pink;}",
            0.0
          )
        ),
        (
          "x568ih9-1e6ryz3",
          (
            "@supports (color: oklab(0 0 0)){@media (prefers-color-scheme: dark){:root, .x568ih9{--xgck17p:oklab(0.7 -0.3 -0.4);--xpegid5:oklab(0.7 -0.3 -0.4);}}}",
            0.3
          )
        ),
        (
          "x568ih9-1lveb7",
          (
            "@media (prefers-color-scheme: dark){:root, .x568ih9{--xgck17p:lightblue;--xpegid5:rgba(0, 0, 0, 0.8);}}",
            0.0
          )
        ),
        (
          "x568ih9-bdddrq",
          ("@media print{:root, .x568ih9{--xgck17p:white;}}", 0.2)
        ),
        (
          "x568ih9-kpd015",
          (
            "@supports (color: oklab(0 0 0)){:root, .x568ih9{--xpegid5:oklab(0.7 -0.3 -0.4);}}",
            0.0
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
        (
          "x4y59db",
          (
            r#"@property --x4y59db { syntax: "<color>"; inherits: true; initial-value: pink }"#,
            0.0
          )
        ),
        (
          "x568ih9",
          (
            ":root, .x568ih9{--xgck17p:blue;--xpegid5:grey;--xrqfjmn:10px;--x4y59db:pink;}",
            0.1
          )
        ),
        (
          "x568ih9-1lveb7",
          (
            "@media (prefers-color-scheme: dark){:root, .x568ih9{--xgck17p:lightblue;--xpegid5:rgba(0, 0, 0, 0.8);}}",
            0.2
          )
        ),
        (
          "x568ih9-1e6ryz3",
          (
            "@supports (color: oklab(0 0 0)){@media (prefers-color-scheme: dark){:root, .x568ih9{--xgck17p:oklab(0.7 -0.3 -0.4);--xpegid5:oklab(0.7 -0.3 -0.4);}}}",
            0.3
          )
        ),
        (
          "x568ih9-bdddrq",
          ("@media print{:root, .x568ih9{--xgck17p:white;}}", 0.2)
        ),
        (
          "x568ih9-kpd015",
          (
            "@supports (color: oklab(0 0 0)){:root, .x568ih9{--xpegid5:oklab(0.7 -0.3 -0.4);}}",
            0.2
          )
        ),
      ])
    )
  }

  #[test]
  fn preserves_names_of_literals_with_double_dash_prefix() {
    let export_id = "TestTheme.stylex.js//buttonTheme";
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

    let default_vars = default_vars_factory(&[
      (
        "--bgColor",
        DefaultVarsFactoryValue::Nested(&[]),
        &[],
        &[bg_color.into()],
      ),
      (
        "--bgColorDisabled",
        DefaultVarsFactoryValue::Nested(&[]),
        &[],
        &[bg_color_disabled.into()],
      ),
      (
        "--cornerRadius",
        DefaultVarsFactoryValue::Nested(&[]),
        &[],
        &[corner_radius.into()],
      ),
      (
        "--fgColor",
        DefaultVarsFactoryValue::Nested(&[]),
        &[],
        &[fg_color.into()],
      ),
    ]);

    let state = Box::<StateManager>::default();
    let mut state = Box::new(StateManager {
      export_id: Some(export_id.to_string()),
      options: StyleXStateOptions {
        class_name_prefix: class_name_prefix.to_string(),
        ..state.options.clone()
      },
      ..*state
    });

    let (_, css_output) = stylex_define_vars(&default_vars, &mut state);

    assert_eq!(
      css_output,
      expected_css_result_factory(&[
        (
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
            ":root, .x568ih9{--bgColor:blue;--bgColorDisabled:grey;--cornerRadius:10px;--fgColor:pink;}",
            0.1
          )
        ),
        (
          "x568ih9-1e6ryz3",
          (
            "@supports (color: oklab(0 0 0)){@media (prefers-color-scheme: dark){:root, .x568ih9{--bgColor:oklab(0.7 -0.3 -0.4);--bgColorDisabled:oklab(0.7 -0.3 -0.4);}}}",
            0.3
          )
        ),
        (
          "x568ih9-1lveb7",
          (
            "@media (prefers-color-scheme: dark){:root, .x568ih9{--bgColor:lightblue;--bgColorDisabled:rgba(0, 0, 0, 0.8);}}",
            0.2
          )
        ),
        (
          "x568ih9-bdddrq",
          ("@media print{:root, .x568ih9{--bgColor:white;}}", 0.2)
        ),
        (
          "x568ih9-kpd015",
          (
            "@supports (color: oklab(0 0 0)){:root, .x568ih9{--bgColorDisabled:oklab(0.7 -0.3 -0.4);}}",
            0.2
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
