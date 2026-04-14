//! Coverage and behavioral tests for shared constants used by the transform
//! pipeline.

use crate::constants::common::{
  COLOR_FUNCTION_LISTED_NORMALIZED_PROPERTY_VALUES,
  COLOR_RELATIVE_VALUES_LISTED_NORMALIZED_PROPERTY_VALUES, COMPILED_KEY, CONSTS_FILE_EXTENSION,
  CSS_CONTENT_FUNCTIONS, CSS_CONTENT_KEYWORDS, DEFAULT_INJECT_PATH, INVALID_METHODS,
  LOGICAL_FLOAT_END_VAR, LOGICAL_FLOAT_START_VAR, MUTATING_ARRAY_METHODS, MUTATING_OBJECT_METHODS,
  ROOT_FONT_SIZE, RUNTIME_JSX_CALL_NAMES, SPLIT_TOKEN, VALID_CALLEES,
  VALID_POSITION_TRY_PROPERTIES, VALID_VIEW_TRANSITION_CLASS_PROPERTIES, VAR_GROUP_HASH_KEY,
};

/// Scalar constants should remain stable because they are externally consumed
/// identifiers.
#[test]
fn scalar_constants_match_expected_contracts() {
  assert_eq!(DEFAULT_INJECT_PATH, "@stylexjs/stylex/lib/stylex-inject");
  assert_eq!(COMPILED_KEY, "$$css");
  assert_eq!(SPLIT_TOKEN, "__$$__");
  assert_eq!(ROOT_FONT_SIZE, 16);
  assert_eq!(VAR_GROUP_HASH_KEY, "__varGroupHash__");
  assert_eq!(CONSTS_FILE_EXTENSION, ".const");
  assert_eq!(LOGICAL_FLOAT_START_VAR, "--stylex-logical-start");
  assert_eq!(LOGICAL_FLOAT_END_VAR, "--stylex-logical-end");
}

/// Method allow/deny sets should expose expected representative entries.
#[test]
fn method_sets_include_expected_members() {
  assert!(VALID_CALLEES.contains("String"));
  assert!(VALID_CALLEES.contains("Array"));

  assert!(MUTATING_ARRAY_METHODS.contains("push"));
  assert!(MUTATING_ARRAY_METHODS.contains("splice"));

  assert!(MUTATING_OBJECT_METHODS.contains("assign"));
  assert!(MUTATING_OBJECT_METHODS.contains("defineProperty"));

  assert!(INVALID_METHODS.contains("random"));
  assert!(INVALID_METHODS.contains("freeze"));
}

/// Lazy lists used by normalizers and runtime checks should be initialized and
/// validated.
#[test]
fn lazy_constant_lists_have_expected_shape() {
  assert_eq!(COLOR_FUNCTION_LISTED_NORMALIZED_PROPERTY_VALUES.len(), 9);
  assert!(COLOR_FUNCTION_LISTED_NORMALIZED_PROPERTY_VALUES.contains(&"oklch"));
  assert!(COLOR_FUNCTION_LISTED_NORMALIZED_PROPERTY_VALUES.contains(&"radial-gradient"));

  assert_eq!(
    COLOR_RELATIVE_VALUES_LISTED_NORMALIZED_PROPERTY_VALUES.len(),
    7
  );
  assert!(COLOR_RELATIVE_VALUES_LISTED_NORMALIZED_PROPERTY_VALUES.contains(&" l "));

  assert_eq!(CSS_CONTENT_FUNCTIONS.len(), 7);
  assert!(CSS_CONTENT_FUNCTIONS.contains(&"url("));
  assert!(CSS_CONTENT_FUNCTIONS.contains(&"var(--"));

  assert_eq!(CSS_CONTENT_KEYWORDS.len(), 11);
  assert!(CSS_CONTENT_KEYWORDS.contains(&"normal"));
  assert!(CSS_CONTENT_KEYWORDS.contains(&"revert-layer"));
}

/// Position/transition property allow-lists should keep their canonical
/// entries.
#[test]
fn property_allow_lists_stay_consistent() {
  assert_eq!(VALID_POSITION_TRY_PROPERTIES.len(), 40);
  assert!(VALID_POSITION_TRY_PROPERTIES.contains(&"top"));
  assert!(VALID_POSITION_TRY_PROPERTIES.contains(&"placeSelf"));

  assert_eq!(VALID_VIEW_TRANSITION_CLASS_PROPERTIES.len(), 4);
  assert!(VALID_VIEW_TRANSITION_CLASS_PROPERTIES.contains(&"group"));
  assert!(VALID_VIEW_TRANSITION_CLASS_PROPERTIES.contains(&"new"));
}

/// Runtime JSX helper names should include both classic and modern factory
/// calls.
#[test]
fn runtime_jsx_call_names_include_expected_variants() {
  assert!(RUNTIME_JSX_CALL_NAMES.contains(&"jsx"));
  assert!(RUNTIME_JSX_CALL_NAMES.contains(&"createElement"));
  assert!(RUNTIME_JSX_CALL_NAMES.contains(&"createVNode"));
}
