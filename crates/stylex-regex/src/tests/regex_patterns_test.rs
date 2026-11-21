//! Focused tests for high-impact shared regex patterns.

use crate::regex::{NPM_NAME_REGEX, SANITIZE_CLASS_NAME_REGEX, pseudo_parts};

/// Ensure invalid class-name characters are detected while valid names pass.
#[test]
fn sanitize_class_name_regex_matches_invalid_characters() {
  assert!(SANITIZE_CLASS_NAME_REGEX.is_match("a b").unwrap());
  assert!(!SANITIZE_CLASS_NAME_REGEX.is_match("valid_name-1").unwrap());
}

/// Ensure package names accepted by npm naming conventions are matched.
#[test]
fn npm_name_regex_accepts_valid_scoped_names() {
  assert!(NPM_NAME_REGEX.is_match("@scope/pkg-name").unwrap());
  assert!(NPM_NAME_REGEX.is_match("plain-package").unwrap());
  assert!(!NPM_NAME_REGEX.is_match("Invalid Package").unwrap());
}

/// The pseudo parts of a key come back in order, with the argument of a
/// functional pseudo-class. A key without a colon has none.
#[test]
fn pseudo_parts_lists_each_part_in_order() {
  assert_eq!(pseudo_parts(":hover::after"), vec![":hover", "::after"]);
  assert_eq!(
    pseudo_parts(":nth-child(2):focus"),
    vec![":nth-child(2)", ":focus"]
  );
  assert!(pseudo_parts("color").is_empty());
}
