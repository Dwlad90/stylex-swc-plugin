use indexmap::IndexMap;
use serde::{Deserialize, Serialize, Serializer};

use crate::shared::structures::injectable_style::InjectableStyle;

use super::injectable_style::InjectableStyleBase;

fn f32_to_int<S>(priority: &f32, serializer: S) -> Result<S::Ok, S::Error>
where
  S: Serializer,
{
  if priority.fract() == 0.0 {
    return serializer.serialize_i32(*priority as i32);
  }

  serializer.serialize_f32(*priority as f32)
}

#[derive(Debug, Serialize, Deserialize, Clone)]

pub(crate) struct MetaData {
  class_name: String,
  style: InjectableStyleBase,
  #[serde(serialize_with = "f32_to_int")]
  priority: f32,
}

impl MetaData {
  pub(crate) fn new(class_name: String, injectable_style: InjectableStyle) -> Self {
    Self {
      class_name,
      style: InjectableStyleBase::from(injectable_style.clone()),
      priority: injectable_style.priority.unwrap(),
    }
  }
  pub(crate) fn _get_style(&self) -> &InjectableStyleBase {
    &self.style
  }

  pub(crate) fn get_css(&self) -> String {
    self.style.ltr.clone()
  }

  pub(crate) fn get_css_rtl(&self) -> Option<String> {
    self.style.rtl.clone()
  }

  pub(crate) fn get_class_name(&self) -> &str {
    &self.class_name
  }

  pub(crate) fn get_priority(&self) -> &f32 {
    &self.priority
  }

  // fn set_priority(key: &str) -> u16 {
  //     get_priority(key)
  // }

  pub(crate) fn convert_from_injected_styles_map(
    injected_styles_map: IndexMap<String, InjectableStyle>,
  ) -> Vec<MetaData> {
    injected_styles_map
      .into_iter()
      .map(|(class_name, injectable_style)| MetaData::new(class_name.clone(), injectable_style))
      .collect::<Vec<MetaData>>()
  }
}
