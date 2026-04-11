//! Tests for user-facing message generation functions (argument length, static values, etc).

use crate::constants::messages::*;

#[test]
fn test_illegal_argument_length_singular() {
  assert_eq!(
    illegal_argument_length("create", 1),
    "create() should have 1 argument."
  );
}

#[test]
fn test_illegal_argument_length_plural() {
  assert_eq!(
    illegal_argument_length("create", 2),
    "create() should have 2 arguments."
  );
}

#[test]
fn test_non_static_value() {
  assert_eq!(
    non_static_value("create"),
    "Only static values are allowed inside of a create() call."
  );
}

#[test]
fn test_non_style_object() {
  assert_eq!(
    non_style_object("create"),
    "create() can only accept an object."
  );
}

#[test]
fn test_non_export_named_declaration() {
  assert_eq!(
    non_export_named_declaration("create"),
    "The return value of create() must be bound to a named export."
  );
}

#[test]
fn test_unbound_call_value() {
  assert_eq!(
    unbound_call_value("create"),
    "create() calls must be bound to a bare variable."
  );
}

#[test]
fn test_cannot_generate_hash() {
  let result = cannot_generate_hash("create");
  assert!(result.starts_with("Unable to generate hash for create()"));
}

#[test]
fn test_expected_call_expression() {
  let result = expected_call_expression("defineVars");
  assert!(result.starts_with("defineVars(): Expected a call expression"));
}
