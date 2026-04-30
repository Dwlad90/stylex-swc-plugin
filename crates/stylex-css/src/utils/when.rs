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

/// Gets the default marker class name based on options.
fn get_default_marker_class_name(options: &StyleXStateOptions) -> String {
  from_proxy(options)
    .or_else(|| from_stylex_style(options))
    .unwrap_or_else(|| {
      let prefix = if !options.class_name_prefix.is_empty() {
        format!("{}-", options.class_name_prefix)
      } else {
        String::new()
      };
      format!("{}default-marker", prefix)
    })
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
#[path = "../tests/when_tests.rs"]
mod tests;
