#[cfg(test)]
mod stylex_keyframes {
  use indexmap::IndexMap;
  use stylex_ast::ast::convertors::create_string_expr;
  use swc_core::ecma::ast::PropOrSpread;

  use crate::shared::transformers::stylex_keyframes::stylex_keyframes;
  use crate::tests::support::expr;
  use stylex_ast::ast::factories::{
    create_key_value_prop, create_nested_object_prop, create_object_expression,
  };
  use stylex_enums::style_resolution::StyleResolution;
  use stylex_state::{evaluate_result_value::EvaluateResultValue, state_manager::StateManager};
  use stylex_types::{
    enums::data_structures::injectable_style::InjectableStyleKind,
    structures::injectable_style::InjectableStyle,
  };

  fn default_vars_factory(args: &[(&str, &[(&str, &str)])]) -> EvaluateResultValue {
    let props = args
      .iter()
      .map(|(key, values)| {
        let props = values
          .iter()
          .map(|(key, value)| create_key_value_prop(key, create_string_expr(value)))
          .collect::<Vec<PropOrSpread>>();

        create_nested_object_prop(key, props)
      })
      .collect::<Vec<PropOrSpread>>();

    EvaluateResultValue::Expr(create_object_expression(props))
  }

  fn expected_css_result_factory(
    injected_styles: &[(&str, (&str, f64))],
  ) -> IndexMap<String, InjectableStyleKind> {
    let mut expected_injected_styles = IndexMap::new();

    for injected_style in injected_styles {
      let (key, value) = injected_style;
      expected_injected_styles.insert(
        key.to_string(),
        InjectableStyleKind::Regular(InjectableStyle {
          ltr: value.0.to_string(),
          rtl: None,
          priority: Some(value.1),
        }),
      );
    }
    expected_injected_styles
  }

  #[test]
  fn converts_keyframes_to_css() {
    let keyframes = default_vars_factory(&[
      ("from", &[("backgroundColor", "red")]),
      ("to", &[("backgroundColor", "blue")]),
    ]);

    let (key, result) = stylex_keyframes(&keyframes, &mut StateManager::default());

    let expected_result = expected_css_result_factory(&[(
      "xbopttm-B",
      (
        "@keyframes xbopttm-B{from{background-color:red;}to{background-color:blue;}}",
        0.0,
      ),
    )]);

    assert_eq!(result, *expected_result.get(key.as_str()).unwrap())
  }

  #[test]
  fn generates_rtl_specific_keyframes() {
    let keyframes =
      default_vars_factory(&[("from", &[("left", "0")]), ("to", &[("left", "500px")])]);

    let (key, result) = stylex_keyframes(&keyframes, &mut StateManager::default());

    let mut expected_injected_styles = IndexMap::new();
    expected_injected_styles.insert(
      "x1lvx8r0-B".to_string(),
      InjectableStyleKind::Regular(InjectableStyle {
        ltr: "@keyframes x1lvx8r0-B{from{left:0;}to{left:500px;}}".to_string(),
        rtl: None,
        priority: Some(0.0),
      }),
    );

    assert_eq!(result, *expected_injected_styles.get(key.as_str()).unwrap())
  }

  /// A logical value reads as a different physical value in each direction, so
  /// the animation carries a rule for each.
  #[test]
  fn carries_a_right_to_left_rule_for_a_logical_value() {
    use crate::tests::support::expr;

    let keyframes = EvaluateResultValue::Expr(expr("{ from: { float: 'start' } }"));

    let (name, result) = stylex_keyframes(&keyframes, &mut StateManager::default());

    assert_eq!(
      result,
      InjectableStyleKind::Regular(InjectableStyle {
        ltr: format!("@keyframes {name}{{from{{float:left;}}}}"),
        rtl: Some(format!("@keyframes {name}{{from{{float:right;}}}}")),
        priority: Some(0.0),
      })
    );
  }

  /// An animation that names no step is still a named rule, with an empty body.
  #[test]
  fn writes_an_empty_body_for_an_animation_that_names_no_step() {
    use crate::tests::support::expr;

    let (name, result) = stylex_keyframes(
      &EvaluateResultValue::Expr(expr("{}")),
      &mut StateManager::default(),
    );

    assert_eq!(
      result,
      InjectableStyleKind::Regular(InjectableStyle {
        ltr: format!("@keyframes {name}{{}}"),
        rtl: None,
        priority: Some(0.0),
      })
    );
  }

  /// The name and the rule a source compiles to with the logical properties
  /// resolved to physical ones.
  fn compile_with_legacy_polyfill(code: &str) -> (String, InjectableStyle) {
    let mut state = StateManager::default();
    state.options.style_resolution = StyleResolution::LegacyExpandShorthands;
    state.options.enable_logical_styles_polyfill = true;

    match stylex_keyframes(&EvaluateResultValue::Expr(expr(code)), &mut state) {
      (name, InjectableStyleKind::Regular(style)) => (name, style),
      (_, other) => panic!("an animation is a regular rule: {other:?}"),
    }
  }

  /// When two declarations of a step resolve to one property name, the later
  /// value wins and keeps the place of the first, as for any object key.
  #[test]
  fn keeps_one_declaration_per_property_name_in_a_step() {
    let cases = [
      (
        "{ '0%': { insetInlineStart: 0, left: 2 } }",
        "xbdbhb5-B",
        "0%{left:2px;}",
        "0%{right:0;left:2px;}",
      ),
      (
        "{ '0%': { insetInlineStart: 0, top: 1, left: 2 } }",
        "x1e4n2n5-B",
        "0%{left:2px;top:1px;}",
        "0%{right:0;top:1px;left:2px;}",
      ),
      (
        "{ '0%': { left: 2, top: 1, insetInlineStart: 0 } }",
        "x1rf296t-B",
        "0%{left:0;top:1px;}",
        "0%{left:2px;top:1px;right:0;}",
      ),
      (
        "{ from: { marginInlineStart: 1, marginLeft: 3 }, to: { left: 5 } }",
        "xmlwlte-B",
        "from{margin-left:3px;}to{left:5px;}",
        "from{margin-right:1px;margin-left:3px;}to{left:5px;}",
      ),
    ];

    for (code, expected_name, ltr, rtl) in cases {
      let (name, style) = compile_with_legacy_polyfill(code);

      assert_eq!(name, expected_name, "{code}");
      assert_eq!(style.ltr, format!("@keyframes {name}{{{ltr}}}"), "{code}");
      assert_eq!(
        style.rtl,
        Some(format!("@keyframes {name}{{{rtl}}}")),
        "{code}"
      );
    }
  }
}
