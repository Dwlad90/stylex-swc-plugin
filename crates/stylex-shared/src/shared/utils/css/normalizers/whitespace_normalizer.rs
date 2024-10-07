use std::borrow::Cow;

use crate::shared::regex::{
  CSS_RULE_REGEX, CSS_URL_REGEX, HASH_WHITESPACE_NORMALIZER_REGEX,
  WHITESPACE_FUNC_NORMALIZER_REGEX, WHITESPACE_NORMALIZER_MATH_SIGNS_REGEX,
  WHITESPACE_NORMALIZER_REGEX, WHITESPACE_NORMALIZER_SPACES_EMPTY_STRING_REGEX,
  WHITESPACE_NORMALIZER_SPACES_REGEX,
};

pub(crate) fn whitespace_normalizer(result: String) -> String {
  let css_string: &str = if result.contains('{') {
    CSS_RULE_REGEX
      .captures(result.as_str())
      .unwrap()
      .get(1)
      .unwrap_or_else(|| panic!("Failed to get CSS rule of: {}", result))
      .as_str()
  } else {
    result.as_str()
  };

  let normalized_css_string = if CSS_URL_REGEX.is_match(css_string) {
    Cow::Borrowed(css_string)
  } else {
    WHITESPACE_NORMALIZER_MATH_SIGNS_REGEX.replace_all(css_string, " $1 $2")
  };

  let normalized_css_string =
    WHITESPACE_NORMALIZER_REGEX.replace_all(&normalized_css_string, "$1$3 $2$4");

  let normalized_css_string =
    WHITESPACE_NORMALIZER_SPACES_EMPTY_STRING_REGEX.replace_all(&normalized_css_string, "$1$2");

  let normalized_css_string =
    WHITESPACE_NORMALIZER_SPACES_REGEX.replace_all(&normalized_css_string, "$1$2");

  let normalized_css_string =
    HASH_WHITESPACE_NORMALIZER_REGEX.replace_all(&normalized_css_string, "$1 #");

  let normalized_css_string =
    WHITESPACE_FUNC_NORMALIZER_REGEX.replace_all(&normalized_css_string, "($1),");

  normalized_css_string.trim().to_string()
}
