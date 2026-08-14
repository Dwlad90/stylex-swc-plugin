//! Tests for evaluation error message functions and static constants.

use crate::constants::evaluation_errors::*;

#[test]
fn test_unsupported_operator() {
  assert_eq!(unsupported_operator("+"), "Unsupported operator: +\n\n");
  assert_eq!(unsupported_operator("**"), "Unsupported operator: **\n\n");
}

#[test]
fn test_unsupported_expression() {
  assert_eq!(
    unsupported_expression("AwaitExpression"),
    "Unsupported expression: AwaitExpression\n\n"
  );
}

#[test]
fn test_uncoercible_value() {
  assert_eq!(
    uncoercible_value("String"),
    "Cannot coerce this value at compile time.\nOnly static values can be passed to String().\n\n"
  );
  assert!(uncoercible_value("Number").contains("Number()"));
}

#[test]
fn test_array_length_too_large() {
  assert_eq!(
    array_length_too_large(65_536),
    "Array length is too large to evaluate at compile time.\nAt most 65536 elements are supported.\n\n"
  );
}

#[test]
fn test_not_a_function() {
  assert_eq!(
    not_a_function("Math"),
    "Math is not a function.\nOnly its methods can be called.\n\n"
  );
}

#[test]
fn test_static_constants() {
  assert!(!PATH_WITHOUT_NODE.is_empty());
  assert!(!UNEXPECTED_MEMBER_LOOKUP.is_empty());
  assert!(!IMPORT_PATH_RESOLUTION_ERROR.is_empty());
  assert!(!NON_CONSTANT.is_empty());
  assert!(!UNDEFINED_CONST.is_empty());
  assert!(!OBJECT_METHOD.is_empty());
}
