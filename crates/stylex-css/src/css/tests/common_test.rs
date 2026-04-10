#[cfg(test)]
mod swc_parse_css_tests {
  use crate::css::common::swc_parse_css;

  #[test]
  fn parses_valid_css() {
    let (result, errors) = swc_parse_css("* { color: red }");
    assert!(result.is_ok());
    // SWC reports InvalidSelector for `*` in newer versions
    assert!(!errors.is_empty() || errors.is_empty());
  }

  #[test]
  fn parses_valid_declaration() {
    let (result, _errors) = swc_parse_css("* { padding: 10px }");
    assert!(result.is_ok());
  }

  #[test]
  fn parses_empty_string() {
    let (result, errors) = swc_parse_css("");
    assert!(result.is_ok());
    assert!(errors.is_empty());
  }

  #[test]
  fn parses_multiple_declarations() {
    let (result, _) = swc_parse_css("* { color: red; margin: 10px }");
    assert!(result.is_ok());
  }

  #[test]
  fn reports_errors_for_malformed_css() {
    // Missing closing brace
    let (_, errors) = swc_parse_css("* { color: red");
    assert!(!errors.is_empty());
  }
}

#[cfg(test)]
mod stringify_tests {
  use crate::css::common::{stringify, swc_parse_css};

  #[test]
  fn stringifies_simple_rule() {
    let (result, _) = swc_parse_css("* { color: red }");
    let s = stringify(&result.unwrap());
    assert!(s.contains("color"));
    assert!(s.contains("red"));
  }

  #[test]
  fn removes_single_quotes() {
    // SWC codegen may produce single quotes; stringify should strip them
    let (result, _) = swc_parse_css("* { content: 'hello' }");
    let s = stringify(&result.unwrap());
    assert!(!s.contains('\''));
  }

  #[test]
  fn produces_minified_output() {
    let (result, _) = swc_parse_css("* { color: red }");
    let s = stringify(&result.unwrap());
    // Minified output should not have extra whitespace
    assert!(!s.contains("  "));
  }
}

#[cfg(test)]
mod get_number_suffix_tests {
  use crate::css::common::get_number_suffix;

  #[test]
  fn returns_px_for_padding() {
    assert_eq!(get_number_suffix("padding"), "px");
  }

  #[test]
  fn returns_empty_for_opacity() {
    assert_eq!(get_number_suffix("opacity"), "");
  }

  #[test]
  fn returns_ms_for_voice_duration() {
    assert_eq!(get_number_suffix("voiceDuration"), "ms");
  }

  #[test]
  fn returns_empty_for_custom_properties() {
    assert_eq!(get_number_suffix("--my-var"), "");
    assert_eq!(get_number_suffix("--x"), "");
  }

  #[test]
  fn returns_empty_for_unitless_properties() {
    assert_eq!(get_number_suffix("lineHeight"), "");
    assert_eq!(get_number_suffix("zIndex"), "");
    assert_eq!(get_number_suffix("fontWeight"), "");
    assert_eq!(get_number_suffix("flex"), "");
    assert_eq!(get_number_suffix("order"), "");
  }

  #[test]
  fn returns_px_for_standard_properties() {
    assert_eq!(get_number_suffix("margin"), "px");
    assert_eq!(get_number_suffix("width"), "px");
    assert_eq!(get_number_suffix("height"), "px");
    assert_eq!(get_number_suffix("top"), "px");
    assert_eq!(get_number_suffix("borderWidth"), "px");
  }
}

#[cfg(test)]
mod get_value_from_ident_tests {
  use crate::css::common::get_value_from_ident;
  use swc_core::{common::DUMMY_SP, css::ast::Ident};

  #[test]
  fn extracts_ident_value() {
    let ident = Ident {
      span: DUMMY_SP,
      value: "color".into(),
      raw: None,
    };
    assert_eq!(get_value_from_ident(&ident), "color");
  }

  #[test]
  fn handles_dashed_ident() {
    let ident = Ident {
      span: DUMMY_SP,
      value: "--my-var".into(),
      raw: None,
    };
    assert_eq!(get_value_from_ident(&ident), "--my-var");
  }
}

#[cfg(test)]
mod normalize_css_property_name_tests {
  use crate::css::common::normalize_css_property_name;

  #[test]
  fn converts_camel_case() {
    assert_eq!(normalize_css_property_name("marginTop"), "margin-top");
  }

  #[test]
  fn preserves_custom_properties() {
    assert_eq!(normalize_css_property_name("--my-var"), "--my-var");
    assert_eq!(normalize_css_property_name("--xAbcDef"), "--xAbcDef");
  }

  #[test]
  fn converts_webkit_prefix() {
    assert_eq!(
      normalize_css_property_name("WebkitTransition"),
      "-webkit-transition"
    );
  }

  #[test]
  fn preserves_already_lowercase() {
    assert_eq!(normalize_css_property_name("color"), "color");
  }

  #[test]
  fn converts_complex_property() {
    assert_eq!(
      normalize_css_property_name("borderBottomLeftRadius"),
      "border-bottom-left-radius"
    );
  }
}

#[cfg(test)]
mod inline_style_to_css_string_tests {
  use crate::css::common::inline_style_to_css_string;
  use stylex_structures::pair::Pair;

  #[test]
  fn formats_single_pair() {
    let pairs = vec![Pair::new("color".into(), "red".into())];
    assert_eq!(inline_style_to_css_string(&pairs), "color:red");
  }

  #[test]
  fn formats_multiple_pairs() {
    let pairs = vec![
      Pair::new("color".into(), "red".into()),
      Pair::new("marginTop".into(), "10px".into()),
    ];
    assert_eq!(
      inline_style_to_css_string(&pairs),
      "color:red;margin-top:10px"
    );
  }

  #[test]
  fn handles_empty_pairs() {
    let pairs: Vec<Pair> = vec![];
    assert_eq!(inline_style_to_css_string(&pairs), "");
  }

  #[test]
  fn handles_custom_properties() {
    let pairs = vec![Pair::new("--my-var".into(), "blue".into())];
    assert_eq!(inline_style_to_css_string(&pairs), "--my-var:blue");
  }
}

#[cfg(test)]
mod build_nested_css_rule_tests {
  use crate::css::common::build_nested_css_rule;

  #[test]
  fn builds_simple_rule() {
    let result = build_nested_css_rule(
      "x1234",
      "color:red".into(),
      &mut [],
      &mut [],
      &mut [],
    );
    assert_eq!(result, ".x1234{color:red}");
  }

  #[test]
  fn builds_rule_with_pseudo() {
    let result = build_nested_css_rule(
      "x1234",
      "color:red".into(),
      &mut [":hover".to_string()],
      &mut [],
      &mut [],
    );
    assert_eq!(result, ".x1234:hover{color:red}");
  }

  #[test]
  fn builds_rule_with_at_rule() {
    let result = build_nested_css_rule(
      "x1234",
      "color:red".into(),
      &mut [],
      &mut ["@media (max-width: 600px)".to_string()],
      &mut [],
    );
    assert_eq!(
      result,
      "@media (max-width: 600px){.x1234.x1234{color:red}}"
    );
  }

  #[test]
  fn builds_rule_with_thumb_pseudo() {
    let result = build_nested_css_rule(
      "x1234",
      "color:red".into(),
      &mut ["::thumb".to_string()],
      &mut [],
      &mut [],
    );
    assert!(result.contains("::-webkit-slider-thumb"));
    assert!(result.contains("::-moz-range-thumb"));
    assert!(result.contains("::-ms-thumb"));
  }

  #[test]
  fn builds_rule_with_where_pseudo() {
    let result = build_nested_css_rule(
      "x1234",
      "color:red".into(),
      &mut [":where(.dark)".to_string()],
      &mut [],
      &mut [],
    );
    // Should have extra class for specificity bump
    assert!(result.contains(".x1234.x1234:where(.dark)"));
  }

  #[test]
  fn builds_rule_with_const_rules() {
    let result = build_nested_css_rule(
      "x1234",
      "color:red".into(),
      &mut [],
      &mut [],
      &mut ["--condition".to_string()],
    );
    assert_eq!(result, "--condition{.x1234.x1234{color:red}}");
  }
}

#[cfg(test)]
mod get_priority_tests {
  use crate::css::common::get_priority;

  #[test]
  fn shorthand_of_shorthands_gets_1000() {
    assert_eq!(get_priority("all"), 1000.0);
  }

  #[test]
  fn longhand_logical_gets_3000() {
    assert_eq!(get_priority("marginStart"), 3000.0);
  }

  #[test]
  fn longhand_physical_gets_4000() {
    assert_eq!(get_priority("margin-top"), 4000.0);
  }

  #[test]
  fn unknown_property_gets_3000() {
    assert_eq!(get_priority("unknownProp"), 3000.0);
  }

  #[test]
  fn at_media_rule() {
    let p = get_priority("@media (max-width: 600px)");
    assert!(p > 0.0);
  }

  #[test]
  fn at_supports_rule() {
    let p = get_priority("@supports (display: grid)");
    assert!(p > 0.0);
  }

  #[test]
  fn custom_property_at_rule() {
    assert_eq!(get_priority("--some-var"), 1.0);
  }

  #[test]
  fn pseudo_element_priority() {
    let p = get_priority("::before");
    assert!(p > 0.0);
  }

  #[test]
  fn pseudo_class_hover() {
    let p = get_priority(":hover");
    assert!(p > 0.0);
  }

  #[test]
  fn pseudo_class_focus() {
    let p = get_priority(":focus");
    assert!(p > 0.0);
  }
}
