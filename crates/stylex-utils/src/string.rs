use std::borrow::Cow;

use stylex_regex::regex::DASHIFY_REGEX;

/// Converts a camelCase or PascalCase string to its hyphenated (kebab-case)
/// equivalent by inserting hyphens before uppercase letters and lowercasing
/// the result.
///
/// This is used to convert JavaScript-style CSS property names (e.g.
/// `marginTop`, `WebkitTransform`) to their CSS equivalents (`margin-top`,
/// `-webkit-transform`).
pub fn dashify(s: &str) -> Cow<'_, str> {
  if !s.chars().any(char::is_uppercase) {
    return Cow::Borrowed(s);
  }

  Cow::Owned(DASHIFY_REGEX.replace_all(s, "-$1").to_lowercase())
}

/// Strips surrounding double-quote characters from a string.
pub fn remove_quotes(s: &str) -> Cow<'_, str> {
  let trimmed = s.trim_matches('"');

  if trimmed.len() == s.len() {
    Cow::Borrowed(s)
  } else {
    Cow::Borrowed(trimmed)
  }
}

/// Wraps a key in double quotes when `should_wrap_in_quotes` is true,
/// otherwise returns the key unchanged.
pub fn wrap_key_in_quotes(key: &str, should_wrap_in_quotes: bool) -> Cow<'_, str> {
  if should_wrap_in_quotes {
    Cow::Owned(format!("\"{}\"", key))
  } else {
    Cow::Borrowed(key)
  }
}

/// Returns the UTF-16 code unit at the given index, or `None` if the index is
/// out of bounds.
///
/// `charCodeAt` indexes by UTF-16 code unit and returns a single code unit, so
/// an astral scalar occupies two indices and reads back as its surrogate halves
/// — `"🎉"` is `0xD83C` at 0 and `0xDF89` at 1, never the `0x1F389` scalar.
/// Indexing by `char` instead would return the whole scalar and shift every
/// index that follows one.
pub fn char_code_at(s: &str, index: usize) -> Option<u32> {
  s.encode_utf16().nth(index).map(u32::from)
}

#[cfg(test)]
#[path = "tests/string_test.rs"]
mod tests;
