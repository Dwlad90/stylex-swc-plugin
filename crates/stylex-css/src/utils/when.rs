use log::debug;

use stylex_structures::stylex_state_options::StyleXStateOptions;

pub fn from_proxy(_value: &StyleXStateOptions) -> Option<String> {
  debug!("from_proxy is not implemented");
  None
}

pub fn from_stylex_style(_value: &StyleXStateOptions) -> Option<String> {
  debug!("from_stylex_style is not implemented");
  None
}

/// Gets the default marker class name based on options
fn get_default_marker_class_name(options: &StyleXStateOptions) -> String {
  if let Some(value_from_proxy) = from_proxy(options) {
    return value_from_proxy;
  }

  if let Some(value_from_style_xstyle) = from_stylex_style(options) {
    return value_from_style_xstyle;
  }

  let prefix = if !options.class_name_prefix.is_empty() {
    format!("{}-", options.class_name_prefix)
  } else {
    String::new()
  };
  format!("{}default-marker", prefix)
}

/// Validates that a pseudo selector starts with ':' but not '::'
fn validate_pseudo_selector(pseudo: &str) -> Result<(), String> {
  if !pseudo.starts_with(':') && !pseudo.starts_with('[') {
    return Err("Pseudo selector must start with \":\" or \"[\"".to_string());
  }

  if pseudo.starts_with("::") {
    return Err(
      "Pseudo selector cannot start with \"::\" (pseudo-elements are not supported)".to_string(),
    );
  }

  if pseudo.starts_with("[") {
    if !pseudo.ends_with("]") {
      return Err("Attribute selector must end with \"]\"".to_string());
    }

    // Validate proper bracket matching and quote pairing
    if !is_valid_attribute_selector(pseudo) {
      return Err(
        "Attribute selector has invalid format (mismatched brackets or quotes)".to_string(),
      );
    }
  }

  Ok(())
}

/// Validates that an attribute selector has proper bracket and quote matching
fn is_valid_attribute_selector(selector: &str) -> bool {
  if !selector.starts_with('[') || !selector.ends_with(']') {
    return false;
  }

  let mut in_single_quote = false;
  let mut in_double_quote = false;
  let mut bracket_count: i32 = 1;
  let mut prev = '[';

  for c in selector[1..].chars() {
    match c {
      '\'' if prev != '\\' && !in_double_quote => in_single_quote = !in_single_quote,
      '"' if prev != '\\' && !in_single_quote => in_double_quote = !in_double_quote,
      '[' if !in_single_quote && !in_double_quote => {
        bracket_count += 1;
        // CSS attribute selectors can only have one opening bracket (at the start)
        if bracket_count > 1 {
          return false;
        }
      },
      ']' if !in_single_quote && !in_double_quote => {
        bracket_count -= 1;
        if bracket_count < 0 {
          return false;
        }
      },
      _ => {},
    }
    prev = c;
  }

  bracket_count == 0 && !in_single_quote && !in_double_quote
}

/// Creates selector that observes if the given pseudo-class is
/// active on an ancestor with the "defaultMarker"
///
/// # Arguments
/// * `pseudo` - The pseudo selector (e.g., ':hover', ':focus')
/// * `options` - Either a custom marker string or StyleXStateOptions reference
///
/// # Returns
/// A :where() clause for the ancestor observer
pub fn ancestor(pseudo: &str, options: Option<&StyleXStateOptions>) -> Result<String, String> {
  validate_pseudo_selector(pseudo)?;
  let default_marker = options
    .map(get_default_marker_class_name)
    .unwrap_or_else(|| "x-default-marker".to_string());
  Ok(format!(":where(.{}{} *)", default_marker, pseudo))
}

/// Creates selector that observes if the given pseudo-class is
/// active on a descendant with the "defaultMarker"
///
/// # Arguments
/// * `pseudo` - The pseudo selector (e.g., ':hover', ':focus')
/// * `options` - Either a custom marker string or StyleXStateOptions reference
///
/// # Returns
/// A :has() clause for the descendant observer
pub fn descendant(pseudo: &str, options: Option<&StyleXStateOptions>) -> Result<String, String> {
  validate_pseudo_selector(pseudo)?;
  let default_marker = options
    .map(get_default_marker_class_name)
    .unwrap_or_else(|| "x-default-marker".to_string());
  Ok(format!(":where(:has(.{}{}))", default_marker, pseudo))
}

/// Creates selector that observes if the given pseudo-class is
/// active on a previous sibling with the "defaultMarker"
///
/// # Arguments
/// * `pseudo` - The pseudo selector (e.g., ':hover', ':focus')
/// * `options` - Either a custom marker string or StyleXStateOptions reference
///
/// # Returns
/// A :where() clause for the previous sibling observer
pub fn sibling_before(
  pseudo: &str,
  options: Option<&StyleXStateOptions>,
) -> Result<String, String> {
  validate_pseudo_selector(pseudo)?;
  let default_marker = options
    .map(get_default_marker_class_name)
    .unwrap_or_else(|| "x-default-marker".to_string());
  Ok(format!(":where(.{}{} ~ *)", default_marker, pseudo))
}

/// Creates selector that observes if the given pseudo-class is
/// active on a next sibling with the "defaultMarker"
///
/// # Arguments
/// * `pseudo` - The pseudo selector (e.g., ':hover', ':focus')
/// * `options` - Either a custom marker string or StyleXStateOptions reference
///
/// # Returns
/// A :has() clause for the next sibling observer
pub fn sibling_after(pseudo: &str, options: Option<&StyleXStateOptions>) -> Result<String, String> {
  validate_pseudo_selector(pseudo)?;
  let default_marker = options
    .map(get_default_marker_class_name)
    .unwrap_or_else(|| "x-default-marker".to_string());
  Ok(format!(":where(:has(~ .{}{}))", default_marker, pseudo))
}

/// Creates selector that observes if the given pseudo-class is
/// active on any sibling with the "defaultMarker"
///
/// # Arguments
/// * `pseudo` - The pseudo selector (e.g., ':hover', ':focus')
/// * `options` - Either a custom marker string or StyleXStateOptions reference
///
/// # Returns
/// A :where() clause for the any sibling observer
pub fn any_sibling(pseudo: &str, options: Option<&StyleXStateOptions>) -> Result<String, String> {
  validate_pseudo_selector(pseudo)?;
  let default_marker = options
    .map(get_default_marker_class_name)
    .unwrap_or_else(|| "x-default-marker".to_string());
  Ok(format!(
    ":where(.{}{} ~ *, :has(~ .{}{}))",
    default_marker, pseudo, default_marker, pseudo
  ))
}

#[cfg(test)]
mod tests {
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
    assert_eq!(
      result,
      ":where(.x-default-marker[data-state=\"open\"] *)"
    );
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
    let options = StyleXStateOptions {
      class_name_prefix: "my".to_string(),
      ..Default::default()
    };
    let result = descendant(":hover", Some(&options)).unwrap();
    assert_eq!(result, ":where(:has(.my-default-marker:hover))");
  }

  #[test]
  fn descendant_with_empty_prefix() {
    let options = StyleXStateOptions {
      class_name_prefix: "".to_string(),
      ..Default::default()
    };
    let result = descendant(":active", Some(&options)).unwrap();
    assert_eq!(result, ":where(:has(.default-marker:active))");
  }

  #[test]
  fn descendant_with_focus_visible() {
    let result = descendant(":focus-visible", None).unwrap();
    assert_eq!(
      result,
      ":where(:has(.x-default-marker:focus-visible))"
    );
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
    let options = StyleXStateOptions {
      class_name_prefix: "sb".to_string(),
      ..Default::default()
    };
    let result = sibling_before(":hover", Some(&options)).unwrap();
    assert_eq!(result, ":where(.sb-default-marker:hover ~ *)");
  }

  #[test]
  fn sibling_before_with_empty_prefix() {
    let options = StyleXStateOptions {
      class_name_prefix: "".to_string(),
      ..Default::default()
    };
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
    assert_eq!(
      result,
      ":where(.x-default-marker[data-active] ~ *)"
    );
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
    let options = StyleXStateOptions {
      class_name_prefix: "sa".to_string(),
      ..Default::default()
    };
    let result = sibling_after(":hover", Some(&options)).unwrap();
    assert_eq!(result, ":where(:has(~ .sa-default-marker:hover))");
  }

  #[test]
  fn sibling_after_with_empty_prefix() {
    let options = StyleXStateOptions {
      class_name_prefix: "".to_string(),
      ..Default::default()
    };
    let result = sibling_after(":active", Some(&options)).unwrap();
    assert_eq!(result, ":where(:has(~ .default-marker:active))");
  }

  #[test]
  fn sibling_after_with_active() {
    let result = sibling_after(":active", None).unwrap();
    assert_eq!(
      result,
      ":where(:has(~ .x-default-marker:active))"
    );
  }

  #[test]
  fn sibling_after_with_attribute_selector() {
    let result = sibling_after("[aria-selected]", None).unwrap();
    assert_eq!(
      result,
      ":where(:has(~ .x-default-marker[aria-selected]))"
    );
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
    let options = StyleXStateOptions {
      class_name_prefix: "as".to_string(),
      ..Default::default()
    };
    let result = any_sibling(":hover", Some(&options)).unwrap();
    assert_eq!(
      result,
      ":where(.as-default-marker:hover ~ *, :has(~ .as-default-marker:hover))"
    );
  }

  #[test]
  fn any_sibling_with_empty_prefix() {
    let options = StyleXStateOptions {
      class_name_prefix: "".to_string(),
      ..Default::default()
    };
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
    let opts = StyleXStateOptions {
      class_name_prefix: "app".to_string(),
      dev: true,
      ..Default::default()
    };
    assert_eq!(from_proxy(&opts), None);
  }

  #[test]
  fn from_stylex_style_with_custom_options_returns_none() {
    let opts = StyleXStateOptions {
      class_name_prefix: "app".to_string(),
      test: true,
      ..Default::default()
    };
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
    assert_eq!(
      result,
      ":where(.x-default-marker[data-theme='dark'] *)"
    );
  }

  #[test]
  fn descendant_with_attribute_contains_selector() {
    let result = descendant("[class*=\"active\"]", None).unwrap();
    assert_eq!(
      result,
      ":where(:has(.x-default-marker[class*=\"active\"]))"
    );
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
}
