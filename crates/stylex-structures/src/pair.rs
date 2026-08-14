use std::borrow::Cow;

use stylex_utils::string::is_blank_css_text;

#[derive(Debug, PartialEq, Clone, Hash)]
pub struct Pair {
  pub key: String,
  pub value: String,
}

impl Pair {
  pub fn new(key: impl Into<String>, value: impl Into<String>) -> Self {
    Self {
      key: key.into(),
      value: value.into(),
    }
  }

  /// The CSS declaration this pair spells, or `None` when it spells none.
  ///
  /// A half that carries no CSS text leaves nothing to declare: `top:` is not
  /// valid CSS and a browser discards it, so the declaration is omitted rather
  /// than emitted empty. Every at-rule body assembled from pairs asks this, so
  /// that a blank value drops before the body is hashed and the name is the one
  /// a body without that declaration produces.
  pub fn as_declaration(&self) -> Option<String> {
    if is_blank_css_text(&self.key) || is_blank_css_text(&self.value) {
      return None;
    }

    let mut declaration = String::with_capacity(self.key.len() + self.value.len() + 2);
    declaration.push_str(&self.key);
    declaration.push(':');
    declaration.push_str(&self.value);
    declaration.push(';');

    Some(declaration)
  }
}

#[derive(Debug, PartialEq, Clone, Hash)]
pub struct PairCow<'a> {
  pub key: Cow<'a, str>,
  pub value: Cow<'a, str>,
}

impl<'a> PairCow<'a> {
  pub fn borrowed(pair: &'a Pair) -> Self {
    Self {
      key: Cow::Borrowed(pair.key.as_str()),
      value: Cow::Borrowed(pair.value.as_str()),
    }
  }

  pub fn into_owned(self) -> Pair {
    Pair {
      key: self.key.into_owned(),
      value: self.value.into_owned(),
    }
  }
}

impl PartialEq<Pair> for PairCow<'_> {
  fn eq(&self, other: &Pair) -> bool {
    self.key == other.key && self.value == other.value
  }
}

#[cfg(test)]
#[path = "tests/pair_test.rs"]
mod tests;
