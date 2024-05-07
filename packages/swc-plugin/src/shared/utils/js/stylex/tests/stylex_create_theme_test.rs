#[cfg(test)]
mod stylex_create_theme {
  use indexmap::IndexMap;
  use swc_core::ecma::ast::PropOrSpread;

  use crate::shared::{
    structures::{
      evaluate_result::EvaluateResultValue, injectable_style::InjectableStyle,
      state_manager::StateManager,
    },
    utils::{
      common::{
        prop_or_spread_expr_creator, prop_or_spread_expression_creator,
        prop_or_spread_string_creator, string_to_expression,
      },
      css::factories::object_expression_factory,
      js::stylex::stylex_create_theme::stylex_create_theme,
    },
  };

  fn default_vars_factory(args: &[(&str, &str)]) -> EvaluateResultValue {
    let props = args
      .iter()
      .map(|(key, value)| prop_or_spread_string_creator(key, value))
      .collect::<Vec<PropOrSpread>>();

    EvaluateResultValue::Expr(object_expression_factory(props).unwrap())
  }

  fn exprected_result_factory(
    injected_styles: &[(&str, (&str, f32))],
  ) -> IndexMap<String, InjectableStyle> {
    let mut expected_injected_styles = IndexMap::new();

    for injected_style in injected_styles {
      let (key, value) = injected_style;
      expected_injected_styles.insert(
        key.to_string(),
        InjectableStyle {
          ltr: value.0.to_string(),
          rtl: Option::None,
          priority: Option::Some(value.1),
        },
      );
    }
    expected_injected_styles
  }

  fn style_object_factory(
    args: &[(&str, &[(&str, &str)], &[(&str, &[(&str, &str)])])],
    str_args: &[(&str, &str)],
  ) -> EvaluateResultValue {
    let mut props = args
      .iter()
      .map(|(key, values, nested_values)| {
        let mut props = values
          .iter()
          .map(|val| prop_or_spread_expression_creator(val.0, string_to_expression(val.1).unwrap()))
          .collect::<Vec<PropOrSpread>>();

        let nested_props = nested_values
          .iter()
          .map(|val| {
            let props = val
              .1
              .iter()
              .map(|val| {
                prop_or_spread_expression_creator(val.0, string_to_expression(val.1).unwrap())
              })
              .collect::<Vec<PropOrSpread>>();

            prop_or_spread_expression_creator(val.0, object_expression_factory(props).unwrap())
          })
          .collect::<Vec<PropOrSpread>>();

        props.extend(nested_props);

        prop_or_spread_expr_creator(key, props)
      })
      .collect::<Vec<PropOrSpread>>();

    for (key, value) in str_args.iter() {
      props.push(prop_or_spread_string_creator(key, value));
    }

    EvaluateResultValue::Expr(object_expression_factory(props).unwrap())
  }

  #[test]
  fn overrides_set_of_vars_with_css_class() {
    let theme_name = "TestTheme.stylex.js//buttonTheme";

    let default_vars = default_vars_factory(&[
      ("__themeName__", theme_name),
      ("bgColor", "var(--xgck17p)"),
      ("bgColorDisabled", "var(--xpegid5)"),
      ("cornerRadius", "var(--xrqfjmn)"),
      ("fgColor", "var(--x4y59db)"),
    ]);

    let created_theme = style_object_factory(
      &[
        (
          "bgColor",
          &[
            ("default", "green"),
            ("@media (prefers-color-scheme: dark)", "lightgreen"),
            ("@media print", "transparent"),
          ],
          &[],
        ),
        (
          "bgColorDisabled",
          &[
            ("default", "antiquewhite"),
            ("@media (prefers-color-scheme: dark)", "floralwhite"),
          ],
          &[],
        ),
        ("cornerRadius", &[("default", "6px")], &[]),
      ],
      &[("fgColor", "coral")],
    );

    let (class_name_output, css_output) = stylex_create_theme(
      &default_vars,
      &created_theme,
      &mut StateManager::default(),
      &mut IndexMap::default(),
    );

    let key = class_name_output
      .get(theme_name)
      .unwrap()
      .as_string()
      .unwrap();

    let injectable_rule = css_output.get(key).unwrap();

    assert_eq!(
      injectable_rule,
      exprected_result_factory(&[(
        theme_name,
        (
          ".xtrlmmh{--xgck17p:green;--xpegid5:antiquewhite;--xrqfjmn:6px;--x4y59db:coral;}",
          0.5
        )
      )])
      .get(theme_name)
      .unwrap()
    )
  }

  #[test]
  fn variables_order_does_not_change_the_hash() {
    let theme_name = "TestTheme.stylex.js//buttonTheme";

    let default_vars = default_vars_factory(&[
      ("__themeName__", theme_name),
      ("bgColor", "var(--xgck17p)"),
      ("bgColorDisabled", "var(--xpegid5)"),
      ("cornerRadius", "var(--xrqfjmn)"),
      ("fgColor", "var(--x4y59db)"),
    ]);

    let created_theme = style_object_factory(
      &[
        (
          "bgColor",
          &[
            ("default", "green"),
            ("@media (prefers-color-scheme: dark)", "lightgreen"),
            ("@media print", "transparent"),
          ],
          &[],
        ),
        (
          "bgColorDisabled",
          &[
            ("default", "antiquewhite"),
            ("@media (prefers-color-scheme: dark)", "floralwhite"),
          ],
          &[],
        ),
        ("cornerRadius", &[("default", "6px")], &[]),
      ],
      &[("fgColor", "coral")],
    );

    let created_theme_2 = style_object_factory(
      &[
        ("cornerRadius", &[("default", "6px")], &[]),
        (
          "bgColorDisabled",
          &[
            ("default", "antiquewhite"),
            ("@media (prefers-color-scheme: dark)", "floralwhite"),
          ],
          &[],
        ),
        (
          "bgColor",
          &[
            ("default", "green"),
            ("@media (prefers-color-scheme: dark)", "lightgreen"),
            ("@media print", "transparent"),
          ],
          &[],
        ),
      ],
      &[("fgColor", "coral")],
    );

    let (class_name_output, css_output) = stylex_create_theme(
      &default_vars,
      &created_theme,
      &mut StateManager::default(),
      &mut IndexMap::default(),
    );

    let (class_name_output_2, css_output_2) = stylex_create_theme(
      &default_vars,
      &created_theme_2,
      &mut StateManager::default(),
      &mut IndexMap::default(),
    );

    assert_eq!(class_name_output, class_name_output_2);

    let key = class_name_output
      .get(theme_name)
      .unwrap()
      .as_string()
      .unwrap();

    let key_2 = class_name_output_2
      .get(theme_name)
      .unwrap()
      .as_string()
      .unwrap();

    let injectable_rule = css_output.get(key).unwrap();
    let injectable_rule_2 = css_output_2.get(key_2).unwrap();

    assert_eq!(injectable_rule, injectable_rule_2);
  }

  #[test]
  fn adding_an_at_rule_changes_the_hash() {
    let theme_name = "TestTheme.stylex.js//buttonTheme";

    let default_vars =
      default_vars_factory(&[("__themeName__", theme_name), ("bgColor", "var(--xgck17p)")]);

    let created_theme = style_object_factory(&[], &[("bgColor", "green")]);

    let created_theme_2 = style_object_factory(
      &[(
        "bgColor",
        &[
          ("default", "green"),
          ("@media (prefers-color-scheme: dark)", "lightgreen"),
        ],
        &[],
      )],
      &[],
    );

    let (class_name_output, css_output) = stylex_create_theme(
      &default_vars,
      &created_theme,
      &mut StateManager::default(),
      &mut IndexMap::default(),
    );

    let (class_name_output_2, css_output_2) = stylex_create_theme(
      &default_vars,
      &created_theme_2,
      &mut StateManager::default(),
      &mut IndexMap::default(),
    );

    assert_ne!(class_name_output, class_name_output_2);

    let key = class_name_output
      .get(theme_name)
      .unwrap()
      .as_string()
      .unwrap();

    let key_2 = class_name_output_2
      .get(theme_name)
      .unwrap()
      .as_string()
      .unwrap();

    let injectable_rule = css_output.get(key).unwrap();
    let injectable_rule_2 = css_output_2.get(key_2).unwrap();

    assert_ne!(injectable_rule, injectable_rule_2);
  }

  #[test]
  fn generates_styles_for_nested_at_rules() {
    let theme_name = "TestTheme.stylex.js//buttonTheme";

    let default_vars =
      default_vars_factory(&[("__themeName__", theme_name), ("bgColor", "var(--xgck17p)")]);

    let created_theme = style_object_factory(
      &[(
        "bgColor",
        &[
          ("default", "green"),
          ("@supports (color: oklab(0 0 0))", "oklab(0.7 -0.3 -0.4)"),
        ],
        &[(
          "@media (prefers-color-scheme: dark)",
          &[
            ("default", "lightgreen"),
            ("@supports (color: oklab(0 0 0))", "oklab(0.7 -0.2 -0.4)"),
          ],
        )],
      )],
      &[],
    );

    let (_class_name_output, css_output) = stylex_create_theme(
      &default_vars,
      &created_theme,
      &mut StateManager::default(),
      &mut IndexMap::default(),
    );

    assert_eq!(
      css_output,
      exprected_result_factory(&[("x2y918k", (".x2y918k{--xgck17p:green;}", 0.5)),
      ("x2y918k-1e6ryz3", ("@supports (color: oklab(0 0 0)){@media (prefers-color-scheme: dark){.x2y918k{--xgck17p:oklab(0.7 -0.2 -0.4);}}}", 0.7)),
      ("x2y918k-1lveb7", ("@media (prefers-color-scheme: dark){.x2y918k{--xgck17p:lightgreen;}}", 0.6)),
      ("x2y918k-kpd015", ("@supports (color: oklab(0 0 0)){.x2y918k{--xgck17p:oklab(0.7 -0.3 -0.4);}}", 0.6))
      ])
    )
  }

  #[test]
  fn generates_styles_for_typed_nested_at_rules() {
    let theme_name = "TestTheme.stylex.js//buttonTheme";

    let default_vars =
      default_vars_factory(&[("__themeName__", theme_name), ("bgColor", "var(--xgck17p)")]);

    let created_theme = style_object_factory(
      &[(
        "bgColor",
        &[],
        &[
          (
            "default",
            &[
              ("default", "green"),
              ("@supports (color: oklab(0 0 0))", "oklab(0.7 -0.3 -0.4)"),
            ],
          ),
          (
            "@media (prefers-color-scheme: dark)",
            &[
              ("default", "lightgreen"),
              ("@supports (color: oklab(0 0 0))", "oklab(0.7 -0.2 -0.4)"),
            ],
          ),
        ],
      )],
      &[],
    );

    let (_class_name_output, css_output) = stylex_create_theme(
      &default_vars,
      &created_theme,
      &mut StateManager::default(),
      &mut IndexMap::default(),
    );

    assert_eq!(
      css_output,
      exprected_result_factory(&[("x2y918k", (".x2y918k{--xgck17p:green;}", 0.5)),
      ("x2y918k-1e6ryz3", ("@supports (color: oklab(0 0 0)){@media (prefers-color-scheme: dark){.x2y918k{--xgck17p:oklab(0.7 -0.2 -0.4);}}}", 0.7)),
      ("x2y918k-1lveb7", ("@media (prefers-color-scheme: dark){.x2y918k{--xgck17p:lightgreen;}}", 0.6)),
      ("x2y918k-kpd015", ("@supports (color: oklab(0 0 0)){.x2y918k{--xgck17p:oklab(0.7 -0.3 -0.4);}}", 0.6))
      ])
    )
  }
}
