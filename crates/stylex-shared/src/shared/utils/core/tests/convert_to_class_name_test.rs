#[cfg(test)]
mod convert_style_to_class_name {
  use crate::shared::{
    structures::{
      pre_rule::PreRuleValue, state_manager::StateManager, stylex_options::StyleResolution,
      stylex_state_options::StyleXStateOptions,
    },
    utils::core::convert_style_to_class_name::convert_style_to_class_name,
  };
  fn convert(styles: (&str, &PreRuleValue)) -> String {
    let result = convert_style_to_class_name(
      styles,
      &mut [],
      &mut [],
      &mut [],
      &mut StateManager::default(),
    );

    extract_body(result.2.ltr)
  }

  fn extract_body(s: String) -> String {
    let start = s.find('{').unwrap_or(0) + 1;
    let end = s.len() - 1;
    s[start..end].to_string()
  }

  #[test]
  fn converts_style_to_class_name() {
    let result = convert(("margin", &PreRuleValue::String("10".to_string())));

    assert_eq!(result, "margin:10px")
  }

  #[test]
  fn prefixes_classname_with_property_name_when_options_debug_is_true() {
    let (_, class_name, _) = convert_style_to_class_name(
      ("margin", &PreRuleValue::String("10".to_string())),
      &mut [],
      &mut [],
      &mut [],
      &mut StateManager {
        options: StyleXStateOptions {
          class_name_prefix: 'x'.to_string(),
          style_resolution: StyleResolution::ApplicationOrder,
          dev: false,
          test: false,
          debug: true,
          ..Default::default()
        },
        ..Default::default()
      },
    );
    assert!(class_name.starts_with("margin-"))
  }

  #[test]
  fn prefixes_classname_with_prefix_only_when_options_enable_debug_class_names_is_false() {
    let (_, class_name, _) = convert_style_to_class_name(
      ("margin", &PreRuleValue::String("10".to_string())),
      &mut [],
      &mut [],
      &mut [],
      &mut StateManager {
        options: StyleXStateOptions {
          class_name_prefix: 'x'.to_string(),
          style_resolution: StyleResolution::ApplicationOrder,
          dev: false,
          test: false,
          debug: true,
          enable_debug_class_names: false,
          ..Default::default()
        },
        ..Default::default()
      },
    );
    assert!(class_name.starts_with("x"));
    assert!(!class_name.starts_with("margin-x"));
  }

  #[test]
  fn prefixes_classname_with_prefix_only_when_options_debug_is_false() {
    let (_, class_name, _) = convert_style_to_class_name(
      ("margin", &PreRuleValue::String("10".to_string())),
      &mut [],
      &mut [],
      &mut [],
      &mut StateManager {
        options: StyleXStateOptions {
          class_name_prefix: 'x'.to_string(),
          style_resolution: StyleResolution::ApplicationOrder,
          dev: false,
          test: false,
          debug: false,
          ..Default::default()
        },
        ..Default::default()
      },
    );
    assert!(!class_name.starts_with("margin-"));
    assert!(class_name.starts_with("x"));
  }

  #[test]
  fn converts_margin_number_to_px() {
    let result = convert(("margin", &PreRuleValue::String("10".to_string())));

    assert_eq!(result, "margin:10px")
  }

  #[test]
  fn keeps_number_for_z_index() {
    let result = convert(("zIndex", &PreRuleValue::String("10".to_string())));

    assert_eq!(result, "z-index:10")
  }

  #[test]
  fn keeps_fr_for_zero_fraction_values() {
    let result = convert(("gridTemplateRows", &PreRuleValue::String("0fr".to_string())));

    assert_eq!(result, "grid-template-rows:0fr")
  }

  #[test]
  fn keeps_percent_for_zero_percentage_values() {
    let result = convert(("flexBasis", &PreRuleValue::String("0%".to_string())));

    assert_eq!(result, "flex-basis:0%")
  }

  #[test]
  fn keeps_number_for_opacity() {
    let result = convert(("opacity", &PreRuleValue::String("0.25".to_string())));

    assert_eq!(result, "opacity:.25")
  }

  #[test]
  fn handles_array_of_values() {
    let result = convert((
      "height",
      &PreRuleValue::Vec(vec![
        "500".to_string(),
        "100vh".to_string(),
        "100dvh".to_string(),
      ]),
    ));

    assert_eq!(result, "height:500px;height:100vh;height:100dvh")
  }

  #[test]
  fn handles_array_of_values_with_var() {
    let result = convert((
      "height",
      &PreRuleValue::Vec(vec![
        "500".to_string(),
        "var(--height)".to_string(),
        "100dvh".to_string(),
      ]),
    ));

    assert_eq!(result, "height:var(--height,500px);height:100dvh")
  }

  #[test]
  fn handles_array_with_multiple_vars() {
    let result = convert((
      "height",
      &PreRuleValue::Vec(vec![
        "500".to_string(),
        "var(--x)".to_string(),
        "var(--y)".to_string(),
        "100dvh".to_string(),
      ]),
    ));

    assert_eq!(result, "height:var(--y,var(--x,500px));height:100dvh")
  }

  #[test]
  fn handles_array_with_multiple_vars_and_multiple_fallbacks() {
    let result = convert((
      "height",
      &PreRuleValue::Vec(vec![
        "500".to_string(),
        "100vh".to_string(),
        "var(--x)".to_string(),
        "var(--y)".to_string(),
        "100dvh".to_string(),
      ]),
    ));

    assert_eq!(
      result,
      "height:var(--y,var(--x,500px));height:var(--y,var(--x,100vh));height:100dvh"
    )
  }

  #[test]
  fn handles_array_with_variable_default_and_multiple_constant_fallbacks() {
    let result = convert((
      "height",
      &PreRuleValue::Vec(vec![
        "var(--x)".to_string(),
        "500".to_string(),
        "100dvh".to_string(),
      ]),
    ));

    assert_eq!(result, "height:var(--x);height:500px;height:100dvh")
  }

  #[test]
  fn handles_array_with_variable_default_and_multiple_variable_and_constant_fallbacks() {
    let result = convert((
      "height",
      &PreRuleValue::Vec(vec![
        "var(--x)".to_string(),
        "var(--y)".to_string(),
        "var(--z)".to_string(),
        "100dvh".to_string(),
      ]),
    ));

    assert_eq!(result, "height:var(--z,var(--y,var(--x)));height:100dvh")
  }

  #[test]
  fn handles_array_of_all_variables() {
    let result = convert((
      "height",
      &PreRuleValue::Vec(vec![
        "var(--w)".to_string(),
        "var(--x)".to_string(),
        "var(--y)".to_string(),
        "var(--z)".to_string(),
      ]),
    ));

    assert_eq!(result, "height:var(--z,var(--y,var(--x,var(--w))))")
  }
}
