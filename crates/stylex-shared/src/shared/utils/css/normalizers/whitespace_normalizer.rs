use crate::shared::regex::{
  CSS_RULE_REGEX, CSS_URL_REGEX, HASH_WHITESPACE_NORMALIZER_REGEX,
  WHITESPACE_FUNC_NORMALIZER_REGEX, WHITESPACE_NORMALIZER_MATH_SIGNS_REGEX,
  WHITESPACE_NORMALIZER_REGEX, WHITESPACE_NORMALIZER_SPACES_EMPTY_STRING_REGEX,
  WHITESPACE_NORMALIZER_SPACES_REGEX,
};

pub(crate) fn whitespace_normalizer(result: String) -> String {
  if let Some(captures) = CSS_URL_REGEX.captures(result.as_str()) {
    if let Some(url) = captures.get(0) {
      return url.as_str().to_string();
    } else {
      panic!("Failed to get URL from captures: {}", result);
    }
  }

  let css_string: &str = if result.contains('{') {
    match CSS_RULE_REGEX.captures(result.as_str()) {
      Some(captures) => match captures.get(1) {
        Some(rule) => rule.as_str(),
        None => panic!("Failed to get CSS rule of: {}", result),
      },
      None => panic!("Failed to parse CSS rule of: {}", result),
    }
  } else {
    result.as_str()
  };

  let normalized_css_string =
    WHITESPACE_NORMALIZER_MATH_SIGNS_REGEX.replace_all(css_string, " $1 $2");

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
