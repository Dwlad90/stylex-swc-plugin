#[derive(Debug, PartialEq, Clone, Hash)]

pub(crate) struct Pair {
  pub(crate) key: String,
  pub(crate) value: String,
}

impl Pair {
  pub(crate) fn new(key: String, value: String) -> Self {
    Self { key, value }
  }
}
