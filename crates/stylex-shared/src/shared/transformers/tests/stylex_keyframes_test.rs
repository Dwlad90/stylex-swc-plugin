#[cfg(test)]
mod stylex_keyframes {
  use indexmap::IndexMap;
  use swc_core::ecma::ast::PropOrSpread;

  use crate::shared::{
    enums::data_structures::{
      evaluate_result_value::EvaluateResultValue, injectable_style::InjectableStyleKind,
    },
    structures::{injectable_style::InjectableStyle, state_manager::StateManager},
    transformers::stylex_keyframes::stylex_keyframes,
    utils::ast::{
      convertors::string_to_expression,
      factories::{
        object_expression_factory, prop_or_spread_expr_factory, prop_or_spread_expression_factory,
      },
    },
  };

  fn default_vars_factory(args: &[(&str, &[(&str, &str)])]) -> EvaluateResultValue {
    let props = args
      .iter()
      .map(|(key, values)| {
        let props = values
          .iter()
          .map(|(key, value)| prop_or_spread_expression_factory(key, string_to_expression(value)))
          .collect::<Vec<PropOrSpread>>();

        prop_or_spread_expr_factory(key, props)
      })
      .collect::<Vec<PropOrSpread>>();

    EvaluateResultValue::Expr(object_expression_factory(props))
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
        1.0,
      ),
    )]);

    assert_eq!(result, *expected_result.get(key.as_str()).unwrap())
  }

  #[test]
  fn generates_rtl_specific_keyframes() {
    let keyframes =
      default_vars_factory(&[("from", &[("start", "0")]), ("to", &[("start", "500")])]);

    let (key, result) = stylex_keyframes(&keyframes, &mut StateManager::default());

    let expected_result = expected_css_result_factory(&[(
      "x1jkcf39-B",
      (
        "@keyframes x1jkcf39-B{from{inset-inline-start:0;}to{inset-inline-start:500px;}}",
        1.0,
      ),
    )]);

    assert_eq!(result, *expected_result.get(key.as_str()).unwrap())
  }
}
