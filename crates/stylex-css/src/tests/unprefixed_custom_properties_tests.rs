// Tests for validating custom-property prefix enforcement inside var()
// references. Source: crates/stylex-css/src/css/validators/
// unprefixed_custom_properties.rs

use super::{as_declaration, unprefixed_custom_properties_validator};
use crate::css::common::swc_parse_css;
use swc_core::{common::DUMMY_SP, css::ast::ComponentValue};

/// Ensures the validator rejects custom properties that are missing the `--`
/// prefix.
#[test]
#[should_panic(expected = "Unprefixed custom properties")]
fn disallow_unprefixed_custom_properties() {
  let (result, _) = swc_parse_css("* { color: var(foo); }");

  unprefixed_custom_properties_validator(&result.unwrap());
}

/// A CSS dashed-function name (e.g. `--custom-fn()`) is parsed as a
/// `DashedIdent`, not `Ident`. The validator should silently skip such
/// functions without panicking.
#[test]
fn dashed_ident_function_is_silently_skipped() {
  let (result, _) = swc_parse_css("* { color: --custom-fn(red); }");

  // Should not panic — the function name is `DashedIdent`, so the
  // `if let FunctionName::Ident` guard is false and the function
  // body is not entered.
  unprefixed_custom_properties_validator(&result.unwrap());
}

#[test]
fn non_declaration_component_value_is_skipped() {
  let component_value = ComponentValue::Integer(Box::new(swc_core::css::ast::Integer {
    span: DUMMY_SP,
    value: 1,
    raw: None,
  }));

  assert!(as_declaration(&component_value).is_none());
}
