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
