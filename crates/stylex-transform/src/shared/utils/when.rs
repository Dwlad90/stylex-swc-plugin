use log::debug;

use crate::shared::structures::stylex_state_options::StyleXStateOptions;

pub(crate) fn from_proxy(_value: &StyleXStateOptions) -> Option<String> {
  debug!("from_proxy is not implemented");
  None
}

pub(crate) fn from_stylex_style(_value: &StyleXStateOptions) -> Option<String> {
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
  let chars: Vec<char> = selector.chars().collect();
  if chars.is_empty() || chars[0] != '[' || chars[chars.len() - 1] != ']' {
    return false;
  }

  let mut in_single_quote = false;
  let mut in_double_quote = false;
  let mut bracket_count = 0;

  for i in 0..chars.len() {
    let c = chars[i];

    // Track quote state
    if c == '\'' && (i == 0 || chars[i - 1] != '\\') {
      in_single_quote = !in_single_quote;
    } else if c == '"' && (i == 0 || chars[i - 1] != '\\') {
      in_double_quote = !in_double_quote;
    }

    // Track brackets only outside quotes
    if !in_single_quote && !in_double_quote {
      if c == '[' {
        bracket_count += 1;
        // CSS attribute selectors can only have one opening bracket (at the start)
        if bracket_count > 1 {
          return false;
        }
      } else if c == ']' {
        bracket_count -= 1;
        // Closing bracket should only reach 0 at the end
        if bracket_count < 0 || (bracket_count == 0 && i < chars.len() - 1) {
          return false;
        }
      }
    }
  }

  // Should end with exactly one bracket pair and no unclosed quotes
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
pub(crate) fn ancestor(
  pseudo: &str,
  options: Option<&StyleXStateOptions>,
) -> Result<String, String> {
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
pub(crate) fn descendant(
  pseudo: &str,
  options: Option<&StyleXStateOptions>,
) -> Result<String, String> {
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
pub(crate) fn sibling_before(
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
pub(crate) fn sibling_after(
  pseudo: &str,
  options: Option<&StyleXStateOptions>,
) -> Result<String, String> {
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
pub(crate) fn any_sibling(
  pseudo: &str,
  options: Option<&StyleXStateOptions>,
) -> Result<String, String> {
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
    let options = StyleXStateOptions {
      class_name_prefix: "custom".to_string(),
      ..Default::default()
    };
    let result = ancestor(":hover", Some(&options)).unwrap();
    assert_eq!(result, ":where(.custom-default-marker:hover *)");
  }

  #[test]
  fn test_with_empty_prefix() {
    let options = StyleXStateOptions {
      class_name_prefix: "".to_string(),
      ..Default::default()
    };
    let result = ancestor(":hover", Some(&options)).unwrap();
    assert_eq!(result, ":where(.default-marker:hover *)");
  }
}
