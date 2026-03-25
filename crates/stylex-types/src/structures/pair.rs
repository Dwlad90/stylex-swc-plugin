#[derive(Debug, PartialEq, Clone, Hash)]

pub struct Pair {
  pub key: String,
  pub value: String,
}

impl Pair {
  pub fn new(key: String, value: String) -> Self {
    Self { key, value }
  }
}
