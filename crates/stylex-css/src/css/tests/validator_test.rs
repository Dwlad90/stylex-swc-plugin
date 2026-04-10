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
}
