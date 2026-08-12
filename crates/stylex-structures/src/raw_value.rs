use std::{borrow::Cow, fmt};

use stylex_utils::number::to_js_string;

/// A style value as authored: `number | string`. Paired with an `Option`, where
/// `None` is a `null` value.
///
/// The two cases have to stay distinguishable all the way to
/// `transform_value`, because only a `Number` gets a unit suffix appended:
/// `width: 1` compiles to `1px`, `width: '1'` to `1`.
#[derive(Debug, PartialEq, Clone)]
pub enum TRawValue {
  String(String),
  Number(f64),
}

impl TRawValue {
  /// Renders the value as CSS text, exactly as JS string coercion would.
  pub fn as_css_text(&self) -> Cow<'_, str> {
    match self {
      TRawValue::String(value) => Cow::Borrowed(value),
      TRawValue::Number(value) => Cow::Owned(to_js_string(*value)),
    }
  }

  /// The authored number, or `None` for a string — including a string that
  /// happens to look numeric, which JS never treats as a number.
  pub fn as_number(&self) -> Option<f64> {
    match self {
      TRawValue::String(_) => None,
      TRawValue::Number(value) => Some(*value),
    }
  }

  /// Whether JS would treat this value as falsy — `0`, `-0`, `NaN` or `""`.
  ///
  /// Note that the *string* `"0"` is truthy, so a falsy check cannot be done on
  /// the rendered CSS text.
  pub fn is_falsy(&self) -> bool {
    match self {
      TRawValue::String(value) => value.is_empty(),
      TRawValue::Number(value) => *value == 0.0 || value.is_nan(),
    }
  }

  /// A key that tells two values apart the way JS does, by type as well as by
  /// text: the number `0` and the string `"0"` are distinct.
  ///
  /// Used wherever values are compared as JS values rather than as CSS —
  /// deduplication and caching both need the type to be part of the identity.
  pub fn identity_key(&self) -> String {
    let (tag, text) = match self {
      TRawValue::String(value) => ('s', Cow::Borrowed(value.as_str())),
      TRawValue::Number(value) => ('n', Cow::Owned(to_js_string(*value))),
    };

    let mut key = String::with_capacity(text.len() + 1);
    key.push(tag);
    key.push_str(&text);
    key
  }
}

/// An absent value splits as the empty string, matching the CSS text a missing
/// shorthand part contributes.
impl Default for TRawValue {
  fn default() -> Self {
    TRawValue::String(String::new())
  }
}

/// Comparisons against string literals read the value as CSS text, so callers
/// that only care about a keyword (`"auto"`, `"start"`) stay unchanged whether
/// the author wrote a string or a number.
impl PartialEq<str> for TRawValue {
  fn eq(&self, other: &str) -> bool {
    self.as_css_text() == other
  }
}

impl PartialEq<&str> for TRawValue {
  fn eq(&self, other: &&str) -> bool {
    self.as_css_text() == *other
  }
}

impl fmt::Display for TRawValue {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    f.write_str(&self.as_css_text())
  }
}

impl From<String> for TRawValue {
  fn from(value: String) -> Self {
    TRawValue::String(value)
  }
}

impl From<&str> for TRawValue {
  fn from(value: &str) -> Self {
    TRawValue::String(value.to_string())
  }
}

impl From<f64> for TRawValue {
  fn from(value: f64) -> Self {
    TRawValue::Number(value)
  }
}

#[cfg(test)]
#[path = "tests/raw_value_test.rs"]
mod tests;
