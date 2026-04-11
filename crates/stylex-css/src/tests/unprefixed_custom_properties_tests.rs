// Tests for validating custom-property prefix enforcement inside var() references.
// Source: crates/stylex-css/src/css/validators/unprefixed_custom_properties.rs

use super::unprefixed_custom_properties_validator;
use crate::css::common::swc_parse_css;

/// Ensures the validator rejects custom properties that are missing the `--` prefix.
#[test]
#[should_panic(expected = "Unprefixed custom properties")]
fn disallow_unprefixed_custom_properties() {
  let (result, _) = swc_parse_css("* { color: var(foo); }");

  unprefixed_custom_properties_validator(&result.unwrap());
}
