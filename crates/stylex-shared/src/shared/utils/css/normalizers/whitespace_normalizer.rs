use crate::shared::regex::{
  CSS_RULE_REGEX, CSS_URL_REGEX, HASH_WHITESPACE_NORMALIZER_REGEX,
  WHITESPACE_BRACKET_NORMALIZER_REGEX, WHITESPACE_FUNC_NORMALIZER_REGEX,
  WHITESPACE_NORMALIZER_EXTRA_SPACES_REGEX, WHITESPACE_NORMALIZER_MATH_SIGNS_REGEX,
};

pub(crate) fn whitespace_normalizer(content: String) -> String {
  // Handle URL case
  if let Some(url) = CSS_URL_REGEX.captures(&content).and_then(|c| c.get(0)) {
    return url.as_str().to_string();
  }

  // Extract CSS rule if present
  let mut css = if content.contains('{') {
    CSS_RULE_REGEX
      .captures(&content)
      .and_then(|c| c.get(1))
      .map(|m| m.as_str().to_string())
      .unwrap_or(content)
  } else {
    content
  };

  // Normalize math signs
  css = WHITESPACE_NORMALIZER_MATH_SIGNS_REGEX
    .replace_all(&css, |caps: &regex::Captures| {
      let left = &caps[1];
      let op = &caps[3];
      // Represents the trailing whitespace captured by the regular expression.
      // Will be empty if there is no whitespace after the value.
      let right_space = &caps[4];
      let right = &caps[5];

      if op == "%" {
        // Attach percent to left with no space, then one space before right.
        format!("{}{} {}", left, op, right)
      } else if op == "-" {
        if right_space.is_empty() {
          format!("{} {}{}", left, op, right)
        } else {
          format!("{} {} {}", left, op, right)
        }
      } else {
        // Other operators, always normalize to one space around.
        format!("{} {} {}", left, op, right)
      }
    })
    .to_string();

  // Normalize brackets
  css = WHITESPACE_BRACKET_NORMALIZER_REGEX
    .replace_all(&css, "$1$3 $2$4")
    .to_string();

  // Normalize extra spaces
  css = WHITESPACE_NORMALIZER_EXTRA_SPACES_REGEX
    .replace_all(&css, |caps: &regex::Captures| {
      if let (Some(q1), Some(q2)) = (caps.get(1), caps.get(2)) {
        format!("{}{}", q1.as_str(), q2.as_str())
      } else if let (Some(p1), Some(p2)) = (caps.get(3), caps.get(4)) {
        format!("{}{}", p1.as_str(), p2.as_str())
      } else {
        caps[0].to_string()
      }
    })
    .to_string();

  // Normalize hash
  css = HASH_WHITESPACE_NORMALIZER_REGEX
    .replace_all(&css, "$1 #")
    .to_string();

  // Normalize functions
  css = WHITESPACE_FUNC_NORMALIZER_REGEX
    .replace_all(&css, "($1),")
    .to_string();

  css.trim().to_string()
}
