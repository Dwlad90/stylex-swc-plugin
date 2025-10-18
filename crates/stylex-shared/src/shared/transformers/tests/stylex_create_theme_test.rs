#[cfg(test)]
mod stylex_create_theme {
  use std::rc::Rc;

  use indexmap::IndexMap;
  use swc_core::ecma::ast::PropOrSpread;

  use crate::shared::{
    enums::data_structures::{
      evaluate_result_value::EvaluateResultValue, injectable_style::InjectableStyleKind,
    },
    structures::{
      injectable_style::InjectableStyle, state_manager::StateManager, types::InjectableStylesMap,
    },
    transformers::stylex_create_theme::stylex_create_theme,
    utils::ast::{
      convertors::string_to_expression,
      factories::{
        object_expression_factory, prop_or_spread_expr_factory, prop_or_spread_expression_factory,
        prop_or_spread_string_factory,
      },
    },
  };

  fn default_vars_factory(args: &[(&str, &str)]) -> EvaluateResultValue {
    let props = args
      .iter()
      .map(|(key, value)| prop_or_spread_string_factory(key, value))
      .collect::<Vec<PropOrSpread>>();

    EvaluateResultValue::Expr(object_expression_factory(props))
  }

  fn exprected_result_factory(injected_styles: &[(&str, (&str, f64))]) -> InjectableStylesMap {
    let mut expected_injected_styles = IndexMap::new();

    for injected_style in injected_styles {
      let (key, value) = injected_style;
      expected_injected_styles.insert(
        key.to_string(),
        Rc::new(InjectableStyleKind::Regular(InjectableStyle {
          ltr: value.0.to_string(),
          rtl: None,
          priority: Some(value.1),
        })),
      );
    }
    expected_injected_styles
  }

  type StyleObjectFactoryArgs<'a> = [(
    &'a str,
    &'a [(&'a str, &'a str)],
    &'a [(&'a str, &'a [(&'a str, &'a str)])],
  )];

  fn style_object_factory(
    args: &StyleObjectFactoryArgs,
    str_args: &[(&str, &str)],
  ) -> EvaluateResultValue {
    let mut props = args
      .iter()
      .map(|(key, values, nested_values)| {
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

        prop_or_spread_expr_factory(key, props)
      })
      .collect::<Vec<PropOrSpread>>();

    for (key, value) in str_args.iter() {
      props.push(prop_or_spread_string_factory(key, value));
    }

    EvaluateResultValue::Expr(object_expression_factory(props))
  }

  #[test]
  fn overrides_set_of_vars_with_css_class() {
    let export_id = "TestTheme.stylex.js//buttonTheme";

    let mut default_vars = default_vars_factory(&[
      ("__varGroupHash__", export_id),
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
      &mut default_vars,
      &created_theme,
      &mut StateManager::default(),
      &mut IndexMap::default(),
    );

    let key = class_name_output
      .get(export_id)
      .unwrap()
      .as_string()
      .unwrap()
      .split(' ')
      .next()
      .unwrap();

    let injectable_rule = css_output.get(key).unwrap();

    assert_eq!(
      injectable_rule,
      exprected_result_factory(&[(
        export_id,
        (
          ".xtrlmmh, .xtrlmmh:root{--xgck17p:green;--xpegid5:antiquewhite;--xrqfjmn:6px;--x4y59db:coral;}",
          0.5
        )
      )])
      .get(export_id)
      .unwrap()
    )
  }

  #[test]
  fn overrides_set_of_literal_vars_with_css_class() {
    let export_id = "TestTheme.stylex.js//buttonTheme";

    let mut default_vars = default_vars_factory(&[
      ("__varGroupHash__", export_id),
      ("--bgColor", "var(--bgColor)"),
      ("--bgColorDisabled", "var(--bgColorDisabled)"),
      ("--cornerRadius", "var(--cornerRadius)"),
      ("--fgColor", "var(--fgColor)"),
    ]);

    let created_theme = style_object_factory(
      &[
        (
          "--bgColor",
          &[
            ("default", "green"),
            ("@media (prefers-color-scheme: dark)", "lightgreen"),
            ("@media print", "transparent"),
          ],
          &[],
        ),
        (
          "--bgColorDisabled",
          &[
            ("default", "antiquewhite"),
            ("@media (prefers-color-scheme: dark)", "floralwhite"),
          ],
          &[],
        ),
        ("--cornerRadius", &[("default", "6px")], &[]),
      ],
      &[("--fgColor", "coral")],
    );

    let (class_name_output, css_output) = stylex_create_theme(
      &mut default_vars,
      &created_theme,
      &mut StateManager::default(),
      &mut IndexMap::default(),
    );

    let key = class_name_output
      .get(export_id)
      .unwrap()
      .as_string()
      .unwrap()
      .split(' ')
      .next()
      .unwrap();

    let injectable_rule = css_output.get(key).unwrap();

    assert_eq!(
      injectable_rule,
      exprected_result_factory(&[(
        export_id,
        (
          ".x4znj40, .x4znj40:root{--bgColor:green;--bgColorDisabled:antiquewhite;--cornerRadius:6px;--fgColor:coral;}",
          0.5
        )
      )])
      .get(export_id)
      .unwrap()
    )
  }

  #[test]
  fn variables_order_does_not_change_the_hash() {
    let export_id = "TestTheme.stylex.js//buttonTheme";

    let mut default_vars = default_vars_factory(&[
      ("__varGroupHash__", export_id),
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
      &mut default_vars,
      &created_theme,
      &mut StateManager::default(),
      &mut IndexMap::default(),
    );

    let (class_name_output_2, css_output_2) = stylex_create_theme(
      &mut default_vars,
      &created_theme_2,
      &mut StateManager::default(),
      &mut IndexMap::default(),
    );

    assert_eq!(class_name_output, class_name_output_2);

    let key = class_name_output
      .get(export_id)
      .unwrap()
      .as_string()
      .unwrap()
      .split(' ')
      .next()
      .unwrap();

    let key_2 = class_name_output_2
      .get(export_id)
      .unwrap()
      .as_string()
      .unwrap()
      .split(' ')
      .next()
      .unwrap();

    let injectable_rule = css_output.get(key).unwrap();
    let injectable_rule_2 = css_output_2.get(key_2).unwrap();

    assert_eq!(injectable_rule, injectable_rule_2);
  }

  #[test]
  fn adding_an_at_rule_changes_the_hash() {
    let export_id = "TestTheme.stylex.js//buttonTheme";

    let mut default_vars = default_vars_factory(&[
      ("__varGroupHash__", export_id),
      ("bgColor", "var(--xgck17p)"),
    ]);

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
      &mut default_vars,
      &created_theme,
      &mut StateManager::default(),
      &mut IndexMap::default(),
    );

    let (class_name_output_2, css_output_2) = stylex_create_theme(
      &mut default_vars,
      &created_theme_2,
      &mut StateManager::default(),
      &mut IndexMap::default(),
    );

    assert_ne!(class_name_output, class_name_output_2);

    let key = class_name_output
      .get(export_id)
      .unwrap()
      .as_string()
      .unwrap()
      .split(' ')
      .next()
      .unwrap();

    let key_2 = class_name_output_2
      .get(export_id)
      .unwrap()
      .as_string()
      .unwrap()
      .split(' ')
      .next()
      .unwrap();

    let injectable_rule = css_output.get(key).unwrap();
    let injectable_rule_2 = css_output_2.get(key_2).unwrap();

    assert_ne!(injectable_rule, injectable_rule_2);
  }

  #[test]
  fn generates_styles_for_nested_at_rules() {
    let export_id = "TestTheme.stylex.js//buttonTheme";

    let mut default_vars = default_vars_factory(&[
      ("__varGroupHash__", export_id),
      ("bgColor", "var(--xgck17p)"),
    ]);

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
      &mut default_vars,
      &created_theme,
      &mut StateManager::default(),
      &mut IndexMap::default(),
    );

    assert_eq!(
      css_output,
      exprected_result_factory(&[
        (
          "x2y918k",
          (".x2y918k, .x2y918k:root{--xgck17p:green;}", 0.5)
        ),
        (
          "x2y918k-1lveb7",
          (".x2y918k, .x2y918k:root{--xgck17p:lightgreen;}", 0.6)
        ),
        (
          "x2y918k-1e6ryz3",
          (
            ".x2y918k, .x2y918k:root{--xgck17p:oklab(0.7 -0.2 -0.4);}",
            0.7
          )
        ),
        (
          "x2y918k-kpd015",
          (
            ".x2y918k, .x2y918k:root{--xgck17p:oklab(0.7 -0.3 -0.4);}",
            0.6
          )
        )
      ])
    )
  }

  #[test]
  fn generates_styles_for_typed_nested_at_rules() {
    let export_id = "TestTheme.stylex.js//buttonTheme";

    let mut default_vars = default_vars_factory(&[
      ("__varGroupHash__", export_id),
      ("bgColor", "var(--xgck17p)"),
    ]);

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
      &mut default_vars,
      &created_theme,
      &mut StateManager::default(),
      &mut IndexMap::default(),
    );

    assert_eq!(
      css_output,
      exprected_result_factory(&[
        (
          "x2y918k",
          (".x2y918k, .x2y918k:root{--xgck17p:green;}", 0.5)
        ),
        (
          "x2y918k-1lveb7",
          (".x2y918k, .x2y918k:root{--xgck17p:lightgreen;}", 0.6)
        ),
        (
          "x2y918k-1e6ryz3",
          (
            ".x2y918k, .x2y918k:root{--xgck17p:oklab(0.7 -0.2 -0.4);}",
            0.7
          )
        ),
        (
          "x2y918k-kpd015",
          (
            ".x2y918k, .x2y918k:root{--xgck17p:oklab(0.7 -0.3 -0.4);}",
            0.6
          )
        )
      ])
    )
  }
}
