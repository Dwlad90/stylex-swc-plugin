use indexmap::IndexMap;

#[derive(Debug, PartialEq, Clone)]
pub enum ValueWithDefault {
  Number(f64),
  String(String),
  Map(IndexMap<String, ValueWithDefault>),
}

#[cfg(not(tarpaulin_include))]
impl std::hash::Hash for ValueWithDefault {
  fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
    match self {
      ValueWithDefault::Number(n) => n.to_bits().hash(state),
      ValueWithDefault::String(s) => s.hash(state),
      ValueWithDefault::Map(map) => {
        for (key, value) in map {
          key.hash(state);
          value.hash(state);
        }
      },
    }
  }
}

impl ValueWithDefault {
  pub fn as_map(&self) -> Option<&IndexMap<String, ValueWithDefault>> {
    match self {
      ValueWithDefault::Map(map) => Some(map),
      _ => None,
    }
  }

  #[cfg(not(tarpaulin_include))]
  fn _as_string(&self) -> Option<&String> {
    match self {
      ValueWithDefault::String(s) => Some(s),
      _ => None,
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_as_map_with_map() {
    let mut inner = IndexMap::new();
    inner.insert(
      "key".to_string(),
      ValueWithDefault::String("val".to_string()),
    );
    let value = ValueWithDefault::Map(inner.clone());
    assert_eq!(value.as_map(), Some(&inner));
  }

  #[test]
  fn test_as_map_with_string() {
    let value = ValueWithDefault::String("hello".to_string());
    assert_eq!(value.as_map(), None);
  }

  #[test]
  fn test_as_map_with_number() {
    let value = ValueWithDefault::Number(42.0);
    assert_eq!(value.as_map(), None);
  }
}
