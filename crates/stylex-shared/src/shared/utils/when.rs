use crate::shared::structures::stylex_state_options::StyleXStateOptions;

/// Gets the default marker class name based on options
#[allow(dead_code)]
fn get_default_marker_class_name(options: &StyleXStateOptions) -> String {
  let prefix = if !options.class_name_prefix.is_empty() {
    format!("{}-", options.class_name_prefix)
  } else {
    String::new()
  };
  format!("{}default-marker", prefix)
}

/// Validates that a pseudo selector starts with ':' but not '::'
#[allow(dead_code)]
fn validate_pseudo_selector(pseudo: &str) -> Result<(), String> {
  if !pseudo.starts_with(':') {
    return Err("Pseudo selector must start with \":\"".to_string());
  }
  if pseudo.starts_with("::") {
    return Err(
      "Pseudo selector cannot start with \"::\" (pseudo-elements are not supported)".to_string(),
    );
  }
  Ok(())
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
  Ok(format!(":has(.{}{})", default_marker, pseudo))
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
  Ok(format!(":has(~ .{}{})", default_marker, pseudo))
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
    assert_eq!(result.unwrap_err(), "Pseudo selector must start with \":\"");
  }

  #[test]
  fn test_validate_pseudo_selector_invalid_double_colon() {
    let result = validate_pseudo_selector("::before");
    assert!(result.is_err());
    assert!(result.unwrap_err().contains("pseudo-elements"));
  }

  #[test]
  fn test_ancestor_with_default_options() {
    let result = ancestor(":hover", None).unwrap();
    assert_eq!(result, ":where(.x-default-marker:hover *)");
  }

  #[test]
  fn test_descendant_with_default_options() {
    let result = descendant(":focus", None).unwrap();
    assert_eq!(result, ":has(.x-default-marker:focus)");
  }

  #[test]
  fn test_sibling_before_with_default_options() {
    let result = sibling_before(":hover", None).unwrap();
    assert_eq!(result, ":where(.x-default-marker:hover ~ *)");
  }

  #[test]
  fn test_sibling_after_with_default_options() {
    let result = sibling_after(":focus", None).unwrap();
    assert_eq!(result, ":has(~ .x-default-marker:focus)");
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
