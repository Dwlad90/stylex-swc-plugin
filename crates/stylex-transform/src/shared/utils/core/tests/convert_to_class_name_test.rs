#[cfg(test)]
mod convert_style_to_class_name {
  use crate::shared::{
    structures::{pre_rule::PreRuleValue, state_manager::StateManager, types::ClassName},
    utils::core::convert_style_to_class_name::convert_style_to_class_name,
  };
  use stylex_enums::style_resolution::StyleResolution;
  use stylex_structures::raw_value::TRawValue;
  use stylex_structures::stylex_state_options::StyleXStateOptions;
  /// The declaration text, for a pair that compiles to one.
  fn convert(styles: (&str, &PreRuleValue)) -> String {
    match try_convert(styles) {
      Some(declaration) => declaration,
      None => panic!("expected `{}` to compile to a declaration", styles.0),
    }
  }

  /// The declaration text, or `None` when the pair carries no CSS text and is
  /// left undeclared.
  fn try_convert(styles: (&str, &PreRuleValue)) -> Option<String> {
    convert_style_to_class_name(
      styles,
      &mut [],
      &mut [],
      &mut [],
      &mut StateManager::default(),
    )
    .map(|(_, _, rule)| extract_body(rule.ltr))
  }

  /// The class name a pair compiles to, under the given options.
  fn class_name_of(styles: (&str, &PreRuleValue), state: &mut StateManager) -> ClassName {
    match convert_style_to_class_name(styles, &mut [], &mut [], &mut [], state) {
      Some((_, class_name, _)) => class_name,
      None => panic!("expected `{}` to compile to a declaration", styles.0),
    }
  }

  fn extract_body(s: String) -> String {
    let start = s.find('{').unwrap_or(0) + 1;
    let end = s.len() - 1;
    s[start..end].to_string()
  }

  #[test]
  fn converts_style_to_class_name() {
    let result = convert(("margin", &PreRuleValue::number(10.0)));

    assert_eq!(result, "margin:10px")
  }

  #[test]
  fn prefixes_classname_with_property_name_when_options_debug_is_true() {
    let class_name = class_name_of(
      ("margin", &PreRuleValue::number(10.0)),
      &mut StateManager::for_test(
        None,
        StyleXStateOptions::default()
          .with_class_name_prefix("x")
          .with_style_resolution(StyleResolution::PropertySpecificity)
          .with_dev(false)
          .with_test(false)
          .with_debug(true)
          .with_enable_debug_class_names(true),
      ),
    );
    assert!(class_name.as_str().starts_with("margin-"))
  }

  #[test]
  fn prefixes_classname_with_prefix_only_when_options_enable_debug_class_names_is_false() {
    let class_name = class_name_of(
      ("margin", &PreRuleValue::number(10.0)),
      &mut StateManager::for_test(
        None,
        StyleXStateOptions::default()
          .with_class_name_prefix("x")
          .with_style_resolution(StyleResolution::PropertySpecificity)
          .with_dev(false)
          .with_test(false)
          .with_debug(true)
          .with_enable_debug_class_names(false),
      ),
    );
    assert!(class_name.as_str().starts_with("x"));
    assert!(!class_name.as_str().starts_with("margin-x"));
  }

  #[test]
  fn prefixes_classname_with_prefix_only_when_options_debug_is_false() {
    let class_name = class_name_of(
      ("margin", &PreRuleValue::number(10.0)),
      &mut StateManager::for_test(
        None,
        StyleXStateOptions::default()
          .with_class_name_prefix("x")
          .with_style_resolution(StyleResolution::PropertySpecificity)
          .with_dev(false)
          .with_test(false)
          .with_debug(false),
      ),
    );
    assert!(!class_name.as_str().starts_with("margin-"));
    assert!(class_name.as_str().starts_with("x"));
  }

  #[test]
  fn converts_margin_number_to_px() {
    let result = convert(("margin", &PreRuleValue::number(10.0)));

    assert_eq!(result, "margin:10px")
  }

  #[test]
  fn keeps_number_for_z_index() {
    let result = convert(("zIndex", &PreRuleValue::number(10.0)));

    assert_eq!(result, "z-index:10")
  }

  #[test]
  fn keeps_fr_for_zero_fraction_values() {
    let result = convert(("gridTemplateRows", &PreRuleValue::string("0fr")));

    assert_eq!(result, "grid-template-rows:0fr")
  }

  #[test]
  fn keeps_percent_for_zero_percentage_values() {
    let result = convert(("flexBasis", &PreRuleValue::string("0%")));

    assert_eq!(result, "flex-basis:0%")
  }

  #[test]
  fn keeps_number_for_opacity() {
    let result = convert(("opacity", &PreRuleValue::string("0.25")));

    assert_eq!(result, "opacity:.25")
  }

  /// `color:` is not a declaration a browser accepts, so a value that carries
  /// no CSS text leaves the property undeclared instead.
  #[test]
  fn declares_nothing_for_a_value_with_no_css_text() {
    assert_eq!(try_convert(("color", &PreRuleValue::string(""))), None);
    assert_eq!(try_convert(("color", &PreRuleValue::string(" "))), None);
    assert_eq!(try_convert(("color", &PreRuleValue::string("  \t "))), None);
  }

  /// The test is on the transformed value, not the authored one: quoting is what
  /// gives a blank `content` its text.
  ///
  /// That ordering is load-bearing. `content` acquires its quotes before the
  /// blank check that leaves other properties undeclared, so a blank `content`
  /// never reaches it — whatever the blank is made of.
  #[test]
  fn declares_a_blank_content_value_as_empty_quotes() {
    for blank in [" ", "   ", "\t", ""] {
      assert_eq!(
        try_convert(("content", &PreRuleValue::string(blank))),
        Some("content:\"\"".to_string()),
        "blank content value {blank:?}"
      );
    }

    assert_eq!(
      try_convert(("hyphenateCharacter", &PreRuleValue::string("   "))),
      Some("hyphenate-character:\"\"".to_string())
    );
  }

  /// A blank entry drops out of a fallback array rather than emitting an empty
  /// declaration beside the ones that spell a value.
  #[test]
  fn drops_a_blank_entry_from_a_fallback_array() {
    assert_eq!(
      try_convert((
        "color",
        &PreRuleValue::Vec(vec![" ".into(), "red".into(), "".into()])
      )),
      Some("color:red".to_string())
    );
  }

  /// Blank entries drop before the `var()` chain is composed, so they cannot
  /// break the contiguity the chain requires.
  #[test]
  fn drops_a_blank_entry_between_variable_fallbacks() {
    assert_eq!(
      try_convert((
        "height",
        &PreRuleValue::Vec(vec!["var(--x)".into(), " ".into(), "var(--y)".into()])
      )),
      Some("height:var(--y,var(--x))".to_string())
    );
  }

  /// An array with nothing left to declare answers the same as a lone blank.
  #[test]
  fn declares_nothing_for_a_fallback_array_of_blanks() {
    assert_eq!(
      try_convert(("color", &PreRuleValue::Vec(vec![" ".into(), "".into()]))),
      None
    );
  }

  /// `0` carries text even though JS calls it falsy.
  #[test]
  fn declares_a_zero_value() {
    assert_eq!(
      try_convert(("zIndex", &PreRuleValue::number(0.0))),
      Some("z-index:0".to_string())
    );
  }

  #[test]
  fn handles_array_of_values() {
    let result = convert((
      "height",
      &PreRuleValue::Vec(vec![
        TRawValue::Number(500.0),
        "100vh".into(),
        "100dvh".into(),
      ]),
    ));

    assert_eq!(result, "height:500px;height:100vh;height:100dvh")
  }

  #[test]
  fn handles_array_of_values_with_var() {
    let result = convert((
      "height",
      &PreRuleValue::Vec(vec![
        TRawValue::Number(500.0),
        "var(--height)".into(),
        "100dvh".into(),
      ]),
    ));

    assert_eq!(result, "height:var(--height,500px);height:100dvh")
  }

  #[test]
  fn handles_array_with_multiple_vars() {
    let result = convert((
      "height",
      &PreRuleValue::Vec(vec![
        TRawValue::Number(500.0),
        "var(--x)".into(),
        "var(--y)".into(),
        "100dvh".into(),
      ]),
    ));

    assert_eq!(result, "height:var(--y,var(--x,500px));height:100dvh")
  }

  #[test]
  fn handles_array_with_multiple_vars_and_multiple_fallbacks() {
    let result = convert((
      "height",
      &PreRuleValue::Vec(vec![
        TRawValue::Number(500.0),
        "100vh".into(),
        "var(--x)".into(),
        "var(--y)".into(),
        "100dvh".into(),
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
        "var(--x)".into(),
        TRawValue::Number(500.0),
        "100dvh".into(),
      ]),
    ));

    assert_eq!(result, "height:var(--x);height:500px;height:100dvh")
  }

  #[test]
  fn handles_array_with_variable_default_and_multiple_variable_and_constant_fallbacks() {
    let result = convert((
      "height",
      &PreRuleValue::Vec(vec![
        "var(--x)".into(),
        "var(--y)".into(),
        "var(--z)".into(),
        "100dvh".into(),
      ]),
    ));

    assert_eq!(result, "height:var(--z,var(--y,var(--x)));height:100dvh")
  }

  #[test]
  fn handles_array_of_all_variables() {
    let result = convert((
      "height",
      &PreRuleValue::Vec(vec![
        "var(--w)".into(),
        "var(--x)".into(),
        "var(--y)".into(),
        "var(--z)".into(),
      ]),
    ));

    assert_eq!(result, "height:var(--z,var(--y,var(--x,var(--w))))")
  }
}
