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
