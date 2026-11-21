use std::borrow::Cow;

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

  /// Appends this pair to `out` as `key:value;`.
  ///
  /// Each half is written as it is, also when it is blank. A blank value
  /// fails the build before a pair is made, and a blank key is kept, as the
  /// reference keeps it.
  pub fn push_css_text(&self, out: &mut String) {
    push_css_text(out, &self.key, &self.value);
  }

  /// The length of the text that [`Pair::push_css_text`] writes, so a caller
  /// can size its buffer once.
  pub fn css_text_len(&self) -> usize {
    css_text_len(&self.key, &self.value)
  }
}

/// The length of `key:value;`: the two halves, a colon and a semicolon.
fn css_text_len(key: &str, value: &str) -> usize {
  key.len() + value.len() + 2
}

/// Appends `key:value;` to `out`. Both pair kinds use it, so the format is
/// written once.
fn push_css_text(out: &mut String, key: &str, value: &str) {
  out.reserve(css_text_len(key, value));
  out.push_str(key);
  out.push(':');
  out.push_str(value);
  out.push(';');
}

#[derive(Debug, PartialEq, Clone, Hash)]
pub struct PairCow<'a> {
  pub key: Cow<'a, str>,
  pub value: Cow<'a, str>,
}

impl<'a> PairCow<'a> {
  /// Appends this pair to `out` as `key:value;`, the same text that
  /// [`Pair::push_css_text`] writes, without making the pair own its halves.
  pub fn push_css_text(&self, out: &mut String) {
    push_css_text(out, &self.key, &self.value);
  }

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
