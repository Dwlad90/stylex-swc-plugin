use std::borrow::Borrow;

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize, derive_more::Display)]
#[display("{}", _0)]
#[repr(transparent)]
pub struct ClassName(pub String);

impl ClassName {
  #[inline]
  pub fn as_str(&self) -> &str {
    &self.0
  }

  #[inline]
  pub fn into_string(self) -> String {
    self.0
  }
}

impl From<String> for ClassName {
  #[inline]
  fn from(value: String) -> Self {
    Self(value)
  }
}

impl From<&str> for ClassName {
  #[inline]
  fn from(value: &str) -> Self {
    Self(value.to_string())
  }
}

impl AsRef<str> for ClassName {
  #[inline]
  fn as_ref(&self) -> &str {
    self.as_str()
  }
}

impl Borrow<str> for ClassName {
  #[inline]
  fn borrow(&self) -> &str {
    self.as_str()
  }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize, derive_more::Display)]
#[display("{}", _0)]
#[repr(transparent)]
pub struct RuleKey(pub String);

impl RuleKey {
  #[inline]
  pub fn as_str(&self) -> &str {
    &self.0
  }

  #[inline]
  pub fn into_string(self) -> String {
    self.0
  }
}

impl From<String> for RuleKey {
  #[inline]
  fn from(value: String) -> Self {
    Self(value)
  }
}

impl From<&str> for RuleKey {
  #[inline]
  fn from(value: &str) -> Self {
    Self(value.to_string())
  }
}

impl AsRef<str> for RuleKey {
  #[inline]
  fn as_ref(&self) -> &str {
    self.as_str()
  }
}

impl Borrow<str> for RuleKey {
  #[inline]
  fn borrow(&self) -> &str {
    self.as_str()
  }
}

#[cfg(test)]
mod tests {
  use indexmap::IndexMap;

  use super::{ClassName, RuleKey};

  #[test]
  fn class_name_round_trips_through_json() {
    let class_name = ClassName::from("x1abc");
    let json = match serde_json::to_string(&class_name) {
      Ok(json) => json,
      Err(error) => panic!("failed to serialize ClassName: {error}"),
    };
    let deserialized = match serde_json::from_str::<ClassName>(&json) {
      Ok(class_name) => class_name,
      Err(error) => panic!("failed to deserialize ClassName: {error}"),
    };

    assert_eq!(json, "\"x1abc\"");
    assert_eq!(deserialized, class_name);
  }

  #[test]
  fn rule_key_supports_str_lookup_without_temporary_key() {
    let mut map: IndexMap<RuleKey, usize> = IndexMap::new();
    map.insert(RuleKey::from("color"), 1);

    assert_eq!(map.get("color"), Some(&1));
  }

  /// ```compile_fail
  /// use stylex_types::structures::style_key::{ClassName, RuleKey};
  ///
  /// let class_name = ClassName::from("x1abc");
  /// let rule_key: RuleKey = class_name;
  /// ```
  #[allow(dead_code)]
  struct ClassNameAndRuleKeyAreDistinct;
}
