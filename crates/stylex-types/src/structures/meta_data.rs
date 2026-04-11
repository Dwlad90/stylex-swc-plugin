use std::hash::{Hash, Hasher};

use serde::{Deserialize, Serialize, Serializer};

use crate::{
  enums::data_structures::injectable_style::{InjectableStyleBaseKind, InjectableStyleKind},
  traits::InjectableStylesMap,
};
use stylex_utils::hash::hash_f64;

fn f64_to_int<S>(priority: &f64, serializer: S) -> Result<S::Ok, S::Error>
where
  S: Serializer,
{
  if priority.fract() == 0.0 {
    return serializer.serialize_i32(*priority as i32);
  }

  serializer.serialize_f64(*priority)
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]

pub struct MetaData {
  class_name: String,
  style: InjectableStyleBaseKind,
  #[serde(serialize_with = "f64_to_int")]
  priority: f64,
}

#[cfg(not(tarpaulin_include))]
impl Hash for MetaData {
  fn hash<H: Hasher>(&self, state: &mut H) {
    self.class_name.hash(state);
    self.style.hash(state);
    hash_f64(self.priority);
  }
}

impl Eq for MetaData {}

impl MetaData {
  pub fn new(class_name: impl Into<String>, injectable_style: InjectableStyleKind) -> Self {
    Self {
      class_name: class_name.into(),
      priority: match &injectable_style {
        InjectableStyleKind::Regular(style) => style.priority.unwrap_or(0.0),
        InjectableStyleKind::Const(style) => style.priority.unwrap_or(0.0),
      },
      style: InjectableStyleBaseKind::from(injectable_style),
    }
  }
  pub fn get_style(&self) -> &InjectableStyleBaseKind {
    &self.style
  }

  pub fn get_css(&self) -> &str {
    match &self.style {
      InjectableStyleBaseKind::Regular(style) => style.ltr.as_str(),
      InjectableStyleBaseKind::Const(style) => style.ltr.as_str(),
    }
  }

  pub fn get_const_key(&self) -> Option<&str> {
    match &self.style {
      InjectableStyleBaseKind::Const(style) => Some(style.const_key.as_str()),
      _ => None,
    }
  }
  pub fn get_const_value(&self) -> Option<&str> {
    match &self.style {
      InjectableStyleBaseKind::Const(style) => Some(style.const_value.as_str()),
      _ => None,
    }
  }

  pub fn get_css_rtl(&self) -> Option<&str> {
    match &self.style {
      InjectableStyleBaseKind::Regular(style) => style.rtl.as_deref(),
      InjectableStyleBaseKind::Const(style) => style.rtl.as_deref(),
    }
  }

  pub fn get_class_name(&self) -> &str {
    self.class_name.as_str()
  }

  pub fn get_priority(&self) -> &f64 {
    &self.priority
  }

  pub fn convert_from_injected_styles_map(
    injected_styles_map: &InjectableStylesMap,
  ) -> Vec<MetaData> {
    injected_styles_map
      .iter()
      .map(|(class_name, injectable_style)| {
        MetaData::new(class_name.clone(), injectable_style.as_ref().clone())
      })
      .collect()
  }
}

#[cfg(test)]
mod tests {
  use std::rc::Rc;

  use indexmap::IndexMap;

  use super::*;
  use crate::{
    enums::data_structures::injectable_style::InjectableStyleKind,
    structures::injectable_style::{InjectableConstStyle, InjectableStyle},
  };

  #[test]
  fn test_new_with_regular_style() {
    let style = InjectableStyleKind::Regular(InjectableStyle {
      ltr: ".x{color:red}".to_string(),
      rtl: Some(".x{color:blue}".to_string()),
      priority: Some(1.0),
    });
    let meta = MetaData::new("x123".to_string(), style);
    assert_eq!(meta.get_class_name(), "x123");
    assert_eq!(meta.get_css(), ".x{color:red}");
    assert_eq!(meta.get_css_rtl(), Some(".x{color:blue}"));
    assert_eq!(*meta.get_priority(), 1.0);
    assert_eq!(meta.get_const_key(), None);
    assert_eq!(meta.get_const_value(), None);
  }

  #[test]
  fn test_new_with_const_style() {
    let style = InjectableStyleKind::Const(InjectableConstStyle {
      ltr: ".y{font:bold}".to_string(),
      rtl: None,
      priority: None,
      const_key: "ck".to_string(),
      const_value: "cv".to_string(),
    });
    let meta = MetaData::new("y456".to_string(), style);
    assert_eq!(meta.get_class_name(), "y456");
    assert_eq!(meta.get_css(), ".y{font:bold}");
    assert_eq!(meta.get_css_rtl(), None);
    assert_eq!(*meta.get_priority(), 0.0);
    assert_eq!(meta.get_const_key(), Some("ck"));
    assert_eq!(meta.get_const_value(), Some("cv"));
  }

  #[test]
  fn test_get_style_returns_ref() {
    let style = InjectableStyleKind::Regular(InjectableStyle {
      ltr: "a".to_string(),
      rtl: None,
      priority: Some(2.0),
    });
    let meta = MetaData::new("cls".to_string(), style);
    match meta.get_style() {
      InjectableStyleBaseKind::Regular(b) => assert_eq!(b.ltr, "a"),
      _ => panic!("Expected Regular"),
    }
  }

  #[test]
  fn test_convert_from_injected_styles_map() {
    let mut map: InjectableStylesMap = IndexMap::new();
    map.insert(
      "cls1".to_string(),
      Rc::new(InjectableStyleKind::Regular(InjectableStyle {
        ltr: "css1".to_string(),
        rtl: None,
        priority: Some(0.5),
      })),
    );
    map.insert(
      "cls2".to_string(),
      Rc::new(InjectableStyleKind::Const(InjectableConstStyle {
        ltr: "css2".to_string(),
        rtl: Some("css2-rtl".to_string()),
        priority: Some(1.0),
        const_key: "k".to_string(),
        const_value: "v".to_string(),
      })),
    );
    let result = MetaData::convert_from_injected_styles_map(&map);
    assert_eq!(result.len(), 2);
    assert_eq!(result[0].get_class_name(), "cls1");
    assert_eq!(result[1].get_class_name(), "cls2");
  }
}
