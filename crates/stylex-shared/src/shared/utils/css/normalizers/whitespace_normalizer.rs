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
      let num1 = &caps[1];
      let op = &caps[2];
      let num2 = &caps[3];

      if op == "%" {
        format!("{}{} {}", num1, op, num2)
      } else {
        format!("{} {} {}", num1, op, num2)
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
