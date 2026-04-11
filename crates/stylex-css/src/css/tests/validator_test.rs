#[cfg(test)]
mod unprefixed_custom_properties_tests {
  use crate::css::common::swc_parse_css;
  use crate::css::validators::unprefixed_custom_properties::unprefixed_custom_properties_validator;

  #[test]
  fn accepts_prefixed_custom_property() {
    let (result, _) = swc_parse_css("* { color: var(--foo) }");
    // Should not panic
    unprefixed_custom_properties_validator(&result.unwrap());
  }

  #[test]
  fn accepts_prefixed_with_fallback() {
    let (result, _) = swc_parse_css("* { color: var(--foo, red) }");
    unprefixed_custom_properties_validator(&result.unwrap());
  }

  #[test]
  #[should_panic(expected = "Unprefixed custom properties")]
  fn rejects_unprefixed_custom_property() {
    let (result, _) = swc_parse_css("* { color: var(foo) }");
    unprefixed_custom_properties_validator(&result.unwrap());
  }

  #[test]
  fn accepts_non_var_functions() {
    let (result, _) = swc_parse_css("* { color: rgb(255, 0, 0) }");
    unprefixed_custom_properties_validator(&result.unwrap());
  }

  #[test]
  fn accepts_no_functions() {
    let (result, _) = swc_parse_css("* { color: red }");
    unprefixed_custom_properties_validator(&result.unwrap());
  }

  #[test]
  fn accepts_multiple_valid_vars() {
    let (result, _) = swc_parse_css("* { color: var(--a); background: var(--b) }");
    unprefixed_custom_properties_validator(&result.unwrap());
  }

  #[test]
  #[should_panic(expected = "Unprefixed custom properties")]
  fn rejects_unprefixed_single_letter() {
    let (result, _) = swc_parse_css("* { color: var(x) }");
    unprefixed_custom_properties_validator(&result.unwrap());
  }

  #[test]
  fn accepts_empty_stylesheet() {
    let (result, _) = swc_parse_css("");
    unprefixed_custom_properties_validator(&result.unwrap());
  }

  #[test]
  fn accepts_rule_without_var() {
    let (result, _) = swc_parse_css("* { margin: 10px; padding: 0 }");
    unprefixed_custom_properties_validator(&result.unwrap());
  }

  #[test]
  fn accepts_var_with_double_dash_prefix() {
    let (result, _) = swc_parse_css("* { color: var(--xAbCdEf) }");
    unprefixed_custom_properties_validator(&result.unwrap());
  }

  #[test]
  fn accepts_calc_function() {
    let (result, _) = swc_parse_css("* { width: calc(100% - 20px) }");
    unprefixed_custom_properties_validator(&result.unwrap());
  }

  #[test]
  fn accepts_nested_var_in_calc() {
    let (result, _) = swc_parse_css("* { width: calc(var(--x) + 10px) }");
    unprefixed_custom_properties_validator(&result.unwrap());
  }

  #[test]
  fn accepts_multiple_declarations_with_var() {
    let (result, _) = swc_parse_css(
      "* { color: var(--xColor); background: var(--xBg); border-color: var(--xBorder) }",
    );
    unprefixed_custom_properties_validator(&result.unwrap());
  }

  #[test]
  fn accepts_min_max_clamp_functions() {
    let (result, _) = swc_parse_css("* { width: min(100%, 500px) }");
    unprefixed_custom_properties_validator(&result.unwrap());
  }

  #[test]
  fn accepts_non_qualified_rules() {
    // at-rules are not QualifiedRules, validator should skip them
    let (result, _) = swc_parse_css("@media screen { * { color: red } }");
    unprefixed_custom_properties_validator(&result.unwrap());
  }

  #[test]
  fn accepts_var_with_fallback_var() {
    let (result, _) = swc_parse_css("* { color: var(--a, var(--b)) }");
    unprefixed_custom_properties_validator(&result.unwrap());
  }

  #[test]
  fn accepts_multiple_properties_with_functions() {
    let (result, _) = swc_parse_css("* { color: rgb(var(--r), 0, 0); background: var(--bg) }");
    unprefixed_custom_properties_validator(&result.unwrap());
  }

  #[test]
  fn accepts_empty_function() {
    let (result, _) = swc_parse_css("* { content: attr(data-value) }");
    unprefixed_custom_properties_validator(&result.unwrap());
  }

  #[test]
  fn accepts_non_var_function_with_ident_arg() {
    let (result, _) = swc_parse_css("* { width: max(100px, 200px) }");
    unprefixed_custom_properties_validator(&result.unwrap());
  }

  #[test]
  fn accepts_declaration_without_function() {
    let (result, _) = swc_parse_css("* { display: block; color: blue }");
    unprefixed_custom_properties_validator(&result.unwrap());
  }
}
