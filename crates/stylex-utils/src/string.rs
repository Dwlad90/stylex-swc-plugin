use stylex_regex::regex::DASHIFY_REGEX;

/// Converts a camelCase or PascalCase string to its hyphenated (kebab-case)
/// equivalent by inserting hyphens before uppercase letters and lowercasing
/// the result.
///
/// This is used to convert JavaScript-style CSS property names (e.g.
/// `marginTop`, `WebkitTransform`) to their CSS equivalents (`margin-top`,
/// `-webkit-transform`).
pub fn dashify(s: &str) -> String {
  DASHIFY_REGEX.replace_all(s, "-$1").to_lowercase()
}

/// Strips surrounding double-quote characters from a string.
pub fn remove_quotes(s: &str) -> String {
  s.trim_matches('"').to_string()
}

/// Wraps a key in double quotes when `should_wrap_in_quotes` is true,
/// otherwise returns the key unchanged.
pub fn wrap_key_in_quotes(key: &str, should_wrap_in_quotes: bool) -> String {
  if should_wrap_in_quotes {
    format!("\"{}\"", key)
  } else {
    key.to_string()
  }
}

/// Returns the Unicode code point of the character at the given index,
/// or `None` if the index is out of bounds.
pub fn char_code_at(s: &str, index: usize) -> Option<u32> {
  s.chars().nth(index).map(|c| c as u32)
}
