// Tests for pseudo-selector observer helpers and marker generation.
// Source: crates/stylex-css/src/utils/when.rs

use super::*;

#[test]
fn test_validate_pseudo_selector_valid() {
  assert!(validate_pseudo_selector(":hover").is_ok());
  assert!(validate_pseudo_selector(":focus").is_ok());
  assert!(validate_pseudo_selector(":active").is_ok());
}

#[test]
fn test_validate_pseudo_selector_invalid_no_colon() {
  let result = validate_pseudo_selector("hover");
  assert!(result.is_err());
  assert_eq!(
    result.unwrap_err(),
    "Pseudo selector must start with \":\" or \"[\""
  );
}

#[test]
fn test_validate_pseudo_selector_invalid_double_colon() {
  let result = validate_pseudo_selector("::before");
  assert!(result.is_err());
  assert!(result.unwrap_err().contains("pseudo-elements"));
}

#[test]
fn test_validate_pseudo_selector_valid_attribute() {
  assert!(validate_pseudo_selector("[data-state=\"open\"]").is_ok());
  assert!(validate_pseudo_selector("[data-state='open']").is_ok());
  assert!(validate_pseudo_selector("[disabled]").is_ok());
  assert!(validate_pseudo_selector("[aria-label*=\"test\"]").is_ok());
}

#[test]
fn test_validate_pseudo_selector_invalid_attribute_missing_bracket() {
  let result = validate_pseudo_selector("[data-state=\"open\"");
  assert!(result.is_err());
  assert!(result.unwrap_err().contains("must end with"));
}

#[test]
fn test_validate_pseudo_selector_invalid_attribute_mismatched_quotes() {
  let result = validate_pseudo_selector("[data-state=\"open']");
  assert!(result.is_err());
  assert!(result.unwrap_err().contains("invalid format"));
}

#[test]
fn test_validate_pseudo_selector_invalid_attribute_unclosed_quotes() {
  let result = validate_pseudo_selector("[data-state=\"open]");
  assert!(result.is_err());
  assert!(result.unwrap_err().contains("invalid format"));
}

#[test]
fn test_validate_pseudo_selector_invalid_attribute_nested_bracket() {
  let result = validate_pseudo_selector("[data-[nested]=\"open\"]");
  assert!(result.is_err());
  assert!(result.unwrap_err().contains("invalid format"));
}

/// `[a][b]` has two sequential bracket pairs.  The first `]` brings
/// `bracket_count` to 0, then the second `[` increments it back to 1
/// — exercising the `bracket_count <= 1` (false) branch of the
/// `if bracket_count > 1` guard.
#[test]
fn test_validate_pseudo_selector_sequential_attribute_selectors() {
  let result = validate_pseudo_selector("[a][b]");
  assert!(result.is_ok());
}

#[test]
fn test_ancestor_with_default_options() {
  let result = ancestor(":hover", None).unwrap();
  assert_eq!(result, ":where(.x-default-marker:hover *)");
}

#[test]
fn test_descendant_with_default_options() {
  let result = descendant(":focus", None).unwrap();
  assert_eq!(result, ":where(:has(.x-default-marker:focus))");
}

#[test]
fn test_sibling_before_with_default_options() {
  let result = sibling_before(":hover", None).unwrap();
  assert_eq!(result, ":where(.x-default-marker:hover ~ *)");
}

#[test]
fn test_sibling_after_with_default_options() {
  let result = sibling_after(":focus", None).unwrap();
  assert_eq!(result, ":where(:has(~ .x-default-marker:focus))");
}

#[test]
fn test_any_sibling_with_default_options() {
  let result = any_sibling(":active", None).unwrap();
  assert_eq!(
    result,
    ":where(.x-default-marker:active ~ *, :has(~ .x-default-marker:active))"
  );
}

#[test]
fn test_with_custom_options() {
  let options = StyleXStateOptions::default().with_class_name_prefix("custom");
  let result = ancestor(":hover", Some(&options)).unwrap();
  assert_eq!(result, ":where(.custom-default-marker:hover *)");
}

#[test]
fn test_with_empty_prefix() {
  let options = StyleXStateOptions::default().with_class_name_prefix("");
  let result = ancestor(":hover", Some(&options)).unwrap();
  assert_eq!(result, ":where(.default-marker:hover *)");
}

// --- ancestor additional tests ---

#[test]
fn ancestor_invalid_pseudo_no_colon() {
  let result = ancestor("hover", None);
  assert!(result.is_err());
  assert!(result.unwrap_err().contains("must start with"));
}

#[test]
fn ancestor_invalid_pseudo_double_colon() {
  let result = ancestor("::before", None);
  assert!(result.is_err());
  assert!(result.unwrap_err().contains("pseudo-elements"));
}

#[test]
fn ancestor_with_focus_visible() {
  let result = ancestor(":focus-visible", None).unwrap();
  assert_eq!(result, ":where(.x-default-marker:focus-visible *)");
}

#[test]
fn ancestor_with_functional_pseudo() {
  let result = ancestor(":nth-child(2)", None).unwrap();
  assert_eq!(result, ":where(.x-default-marker:nth-child(2) *)");
}

#[test]
fn ancestor_with_attribute_selector() {
  let result = ancestor("[data-state=\"open\"]", None).unwrap();
  assert_eq!(result, ":where(.x-default-marker[data-state=\"open\"] *)");
}

// --- descendant additional tests ---

#[test]
fn descendant_invalid_pseudo_no_colon() {
  let result = descendant("focus", None);
  assert!(result.is_err());
  assert!(result.unwrap_err().contains("must start with"));
}

#[test]
fn descendant_invalid_pseudo_double_colon() {
  let result = descendant("::after", None);
  assert!(result.is_err());
  assert!(result.unwrap_err().contains("pseudo-elements"));
}

#[test]
fn descendant_with_custom_options() {
  let options = StyleXStateOptions::default().with_class_name_prefix("my");
  let result = descendant(":hover", Some(&options)).unwrap();
  assert_eq!(result, ":where(:has(.my-default-marker:hover))");
}

#[test]
fn descendant_with_empty_prefix() {
  let options = StyleXStateOptions::default().with_class_name_prefix("");
  let result = descendant(":active", Some(&options)).unwrap();
  assert_eq!(result, ":where(:has(.default-marker:active))");
}

#[test]
fn descendant_with_focus_visible() {
  let result = descendant(":focus-visible", None).unwrap();
  assert_eq!(result, ":where(:has(.x-default-marker:focus-visible))");
}

#[test]
fn descendant_with_attribute_selector() {
  let result = descendant("[disabled]", None).unwrap();
  assert_eq!(result, ":where(:has(.x-default-marker[disabled]))");
}

// --- sibling_before additional tests ---

#[test]
fn sibling_before_invalid_pseudo_no_colon() {
  let result = sibling_before("active", None);
  assert!(result.is_err());
  assert!(result.unwrap_err().contains("must start with"));
}

#[test]
fn sibling_before_invalid_pseudo_double_colon() {
  let result = sibling_before("::first-line", None);
  assert!(result.is_err());
  assert!(result.unwrap_err().contains("pseudo-elements"));
}

#[test]
fn sibling_before_with_custom_options() {
  let options = StyleXStateOptions::default().with_class_name_prefix("sb");
  let result = sibling_before(":hover", Some(&options)).unwrap();
  assert_eq!(result, ":where(.sb-default-marker:hover ~ *)");
}

#[test]
fn sibling_before_with_empty_prefix() {
  let options = StyleXStateOptions::default().with_class_name_prefix("");
  let result = sibling_before(":focus", Some(&options)).unwrap();
  assert_eq!(result, ":where(.default-marker:focus ~ *)");
}

#[test]
fn sibling_before_with_focus() {
  let result = sibling_before(":focus", None).unwrap();
  assert_eq!(result, ":where(.x-default-marker:focus ~ *)");
}

#[test]
fn sibling_before_with_attribute_selector() {
  let result = sibling_before("[data-active]", None).unwrap();
  assert_eq!(result, ":where(.x-default-marker[data-active] ~ *)");
}

// --- sibling_after additional tests ---

#[test]
fn sibling_after_invalid_pseudo_no_colon() {
  let result = sibling_after("focus", None);
  assert!(result.is_err());
  assert!(result.unwrap_err().contains("must start with"));
}

#[test]
fn sibling_after_invalid_pseudo_double_colon() {
  let result = sibling_after("::selection", None);
  assert!(result.is_err());
  assert!(result.unwrap_err().contains("pseudo-elements"));
}

#[test]
fn sibling_after_with_custom_options() {
  let options = StyleXStateOptions::default().with_class_name_prefix("sa");
  let result = sibling_after(":hover", Some(&options)).unwrap();
  assert_eq!(result, ":where(:has(~ .sa-default-marker:hover))");
}

#[test]
fn sibling_after_with_empty_prefix() {
  let options = StyleXStateOptions::default().with_class_name_prefix("");
  let result = sibling_after(":active", Some(&options)).unwrap();
  assert_eq!(result, ":where(:has(~ .default-marker:active))");
}

#[test]
fn sibling_after_with_active() {
  let result = sibling_after(":active", None).unwrap();
  assert_eq!(result, ":where(:has(~ .x-default-marker:active))");
}

#[test]
fn sibling_after_with_attribute_selector() {
  let result = sibling_after("[aria-selected]", None).unwrap();
  assert_eq!(result, ":where(:has(~ .x-default-marker[aria-selected]))");
}

// --- any_sibling additional tests ---

#[test]
fn any_sibling_invalid_pseudo_no_colon() {
  let result = any_sibling("active", None);
  assert!(result.is_err());
  assert!(result.unwrap_err().contains("must start with"));
}

#[test]
fn any_sibling_invalid_pseudo_double_colon() {
  let result = any_sibling("::placeholder", None);
  assert!(result.is_err());
  assert!(result.unwrap_err().contains("pseudo-elements"));
}

#[test]
fn any_sibling_with_custom_options() {
  let options = StyleXStateOptions::default().with_class_name_prefix("as");
  let result = any_sibling(":hover", Some(&options)).unwrap();
  assert_eq!(
    result,
    ":where(.as-default-marker:hover ~ *, :has(~ .as-default-marker:hover))"
  );
}

#[test]
fn any_sibling_with_empty_prefix() {
  let options = StyleXStateOptions::default().with_class_name_prefix("");
  let result = any_sibling(":focus", Some(&options)).unwrap();
  assert_eq!(
    result,
    ":where(.default-marker:focus ~ *, :has(~ .default-marker:focus))"
  );
}

#[test]
fn any_sibling_with_hover() {
  let result = any_sibling(":hover", None).unwrap();
  assert_eq!(
    result,
    ":where(.x-default-marker:hover ~ *, :has(~ .x-default-marker:hover))"
  );
}

#[test]
fn any_sibling_with_attribute_selector() {
  let result = any_sibling("[data-checked]", None).unwrap();
  assert_eq!(
    result,
    ":where(.x-default-marker[data-checked] ~ *, \
:has(~ .x-default-marker[data-checked]))"
  );
}

// --- from_proxy / from_stylex_style ---

#[test]
fn from_proxy_returns_none() {
  let opts = StyleXStateOptions::default();
  assert_eq!(from_proxy(&opts), None);
}

#[test]
fn from_stylex_style_returns_none() {
  let opts = StyleXStateOptions::default();
  assert_eq!(from_stylex_style(&opts), None);
}

#[test]
fn from_proxy_with_custom_options_returns_none() {
  let opts = StyleXStateOptions::default()
    .with_class_name_prefix("app")
    .with_dev(true);
  assert_eq!(from_proxy(&opts), None);
}

#[test]
fn from_stylex_style_with_custom_options_returns_none() {
  let opts = StyleXStateOptions::default()
    .with_class_name_prefix("app")
    .with_test(true);
  assert_eq!(from_stylex_style(&opts), None);
}

// --- get_default_marker_class_name indirect tests ---

#[test]
fn default_marker_with_prefix_x() {
  // Default prefix is "x", so marker should be "x-default-marker"
  let result = ancestor(":hover", None).unwrap();
  assert!(result.contains("x-default-marker"));
}

#[test]
fn default_marker_none_options_uses_fallback() {
  // When no options, the fallback "x-default-marker" is used
  let result = descendant(":hover", None).unwrap();
  assert!(result.contains("x-default-marker"));
}

// --- Validate attribute selectors with functions ---

#[test]
fn ancestor_with_attribute_value_selector() {
  let result = ancestor("[data-theme='dark']", None).unwrap();
  assert_eq!(result, ":where(.x-default-marker[data-theme='dark'] *)");
}

#[test]
fn descendant_with_attribute_contains_selector() {
  let result = descendant("[class*=\"active\"]", None).unwrap();
  assert_eq!(result, ":where(:has(.x-default-marker[class*=\"active\"]))");
}

// --- Validate invalid attribute selectors with functions ---

#[test]
fn ancestor_with_invalid_attribute_missing_bracket() {
  let result = ancestor("[data-state", None);
  assert!(result.is_err());
  assert!(result.unwrap_err().contains("must end with"));
}

#[test]
fn descendant_with_invalid_attribute_mismatched_quotes() {
  let result = descendant("[data=\"open']", None);
  assert!(result.is_err());
  assert!(result.unwrap_err().contains("invalid format"));
}

// --- is_valid_attribute_selector edge cases ---

#[test]
fn is_valid_attribute_selector_empty() {
  assert!(!is_valid_attribute_selector(""));
}

#[test]
fn is_valid_attribute_selector_no_open_bracket() {
  assert!(!is_valid_attribute_selector("disabled]"));
}

#[test]
fn is_valid_attribute_selector_no_close_bracket() {
  assert!(!is_valid_attribute_selector("[disabled"));
}

#[test]
fn is_valid_attribute_selector_bracket_in_double_quotes() {
  assert!(is_valid_attribute_selector("[data-x=\"[\"]"));
}

#[test]
fn is_valid_attribute_selector_bracket_in_single_quotes() {
  assert!(is_valid_attribute_selector("[data-x='[']"));
}

#[test]
fn is_valid_attribute_selector_close_bracket_mid_string() {
  assert!(!is_valid_attribute_selector("[a]b]"));
}

#[test]
fn is_valid_attribute_selector_with_escaped_quotes() {
  assert!(is_valid_attribute_selector("[data-x=\"val\\\"ue\"]"));
}

#[test]
fn is_valid_attribute_selector_simple_valid() {
  assert!(is_valid_attribute_selector("[disabled]"));
  assert!(is_valid_attribute_selector("[data-state=\"open\"]"));
}

// --- Additional pseudo selectors ---

#[test]
fn validate_pseudo_selector_starts_with_bracket() {
  assert!(validate_pseudo_selector("[aria-label]").is_ok());
}

#[test]
fn sibling_before_with_nth_child() {
  let result = sibling_before(":nth-child(2n+1)", None).unwrap();
  assert!(result.contains(":nth-child(2n+1)"));
}

#[test]
fn sibling_after_with_first_child() {
  let result = sibling_after(":first-child", None).unwrap();
  assert!(result.contains(":first-child"));
}

#[test]
fn any_sibling_with_last_child() {
  let result = any_sibling(":last-child", None).unwrap();
  assert!(result.contains(":last-child"));
}
