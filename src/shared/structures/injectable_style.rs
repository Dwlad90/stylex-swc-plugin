use std::hash::Hash;

use serde::{Deserialize, Serialize};

use crate::shared::utils::common::hash_f32;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub(crate) struct InjectableStyleBase {
  pub(crate) rtl: Option<String>,
  pub(crate) ltr: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub(crate) struct InjectableStyle {
  pub(crate) ltr: String,
  pub(crate) rtl: Option<String>,
  pub(crate) priority: Option<f32>,
}

impl Hash for InjectableStyle {
  fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
    self.ltr.hash(state);
    self.rtl.hash(state);
    hash_f32(self.priority.unwrap_or(0.0));
  }
}

impl From<InjectableStyle> for InjectableStyleBase {
  fn from(style: InjectableStyle) -> Self {
    // Assuming InjectableStyleBase and InjectableStyle have similar fields
    InjectableStyleBase {
      ltr: style.ltr,
      rtl: style.rtl,
    }
  }
}
