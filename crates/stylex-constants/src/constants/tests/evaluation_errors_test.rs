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
fn test_static_constants() {
  assert!(!PATH_WITHOUT_NODE.is_empty());
  assert!(!UNEXPECTED_MEMBER_LOOKUP.is_empty());
  assert!(!IMPORT_PATH_RESOLUTION_ERROR.is_empty());
  assert!(!NON_CONSTANT.is_empty());
  assert!(!UNDEFINED_CONST.is_empty());
  assert!(!OBJECT_METHOD.is_empty());
}
