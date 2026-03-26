use crate::shared::regex::{
  CSS_RULE_REGEX, CSS_URL_REGEX, HASH_WHITESPACE_NORMALIZER_REGEX,
  WHITESPACE_BRACKET_NORMALIZER_REGEX, WHITESPACE_FUNC_NORMALIZER_REGEX,
  WHITESPACE_NORMALIZER_EXTRA_SPACES_REGEX, WHITESPACE_NORMALIZER_MATH_SIGNS_REGEX,
};

pub(crate) fn whitespace_normalizer(content: String) -> String {
  // Early return: If content is a URL, return it as-is
  if let Ok(Some(captures)) = CSS_URL_REGEX.captures(&content)
    && let Some(url) = captures.get(0)
  {
    return url.as_str().to_string();
  }

  // Extract CSS value from rule if present (e.g., "color: red;" -> "red")
  let mut css = if content.contains('{') {
    CSS_RULE_REGEX
      .captures(&content)
      .ok()
      .flatten()
      .and_then(|c| c.get(1))
      .map(|m| m.as_str().to_string())
      .unwrap_or(content)
  } else {
    content
  };

  // Normalize whitespace around math operators (+, -, *, /, %)
  css = WHITESPACE_NORMALIZER_MATH_SIGNS_REGEX
    .replace_all(&css, |caps: &fancy_regex::Captures| {
      // Using named groups for better readability
      let left = caps.name("left").map(|m| m.as_str()).unwrap_or("");
      let op = caps.name("op").map(|m| m.as_str()).unwrap_or("");
      let right_space = caps.name("rspace").map(|m| m.as_str()).unwrap_or("");
      let right = caps.name("right").map(|m| m.as_str()).unwrap_or("");

      match op {
        // Percent: attach to left, space before right (e.g., "50% 10" not "50 % 10")
        "%" => format!("{}{} {}", left, op, right),
        // Minus: preserve negative numbers (e.g., "5 -3" vs "5- 3")
        "-" => {
          if right_space.is_empty() {
            format!("{} {}{}", left, op, right)
          } else {
            format!("{} {} {}", left, op, right)
          }
        }
        // Other operators: always add space around
        _ => format!("{} {} {}", left, op, right),
      }
    })
    .to_string();

  // Normalize brackets and quotes: ")a" -> ") a", """" -> ""
  css = WHITESPACE_BRACKET_NORMALIZER_REGEX
    .replace_all(&css, "$1$3 $2$4")
    .to_string();

  // Remove extra spaces in specific cases (empty quotes, multiple parens)
  css = WHITESPACE_NORMALIZER_EXTRA_SPACES_REGEX
    .replace_all(&css, |caps: &fancy_regex::Captures| {
      // Match empty string quotes: "" -> ""
      if let (Some(q1), Some(q2)) = (caps.get(1), caps.get(2)) {
        format!("{}{}", q1.as_str(), q2.as_str())
      }
      // Match multiple closing parens: ") )" -> "))"
      else if let (Some(p1), Some(p2)) = (caps.get(3), caps.get(4)) {
        format!("{}{}", p1.as_str(), p2.as_str())
      } else {
        caps[0].to_string()
      }
    })
    .to_string();

  // Add space before hash in colors: "a#fff" -> "a #fff"
  css = HASH_WHITESPACE_NORMALIZER_REGEX
    .replace_all(&css, "$1 #")
    .to_string();

  // Normalize function arguments: "( arg )" -> "(arg)"
  css = WHITESPACE_FUNC_NORMALIZER_REGEX
    .replace_all(&css, "($1),")
    .to_string();

  css.trim().to_string()
}
