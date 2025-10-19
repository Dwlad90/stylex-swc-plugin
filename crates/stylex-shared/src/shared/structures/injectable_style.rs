use std::{hash::Hash, rc::Rc};

use serde::{Deserialize, Serialize};

use crate::shared::{enums::data_structures::injectable_style::InjectableStyleKind, utils::common::hash_f64};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Hash, Eq)]
pub struct InjectableStyleBase {
  pub rtl: Option<String>,
  pub ltr: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Hash, Eq)]
pub struct InjectableStyleConstBase {
  pub rtl: Option<String>,
  pub ltr: String,
  pub const_key: String,
  pub const_value: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct InjectableStyle {
  pub ltr: String,
  pub rtl: Option<String>,
  pub priority: Option<f64>,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub(crate) struct InjectableConstStyle {
  pub ltr: String,
  pub rtl: Option<String>,
  pub priority: Option<f64>,
  pub const_key: String,
  pub const_value: String,
}

impl Hash for InjectableStyle {
  fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
    self.ltr.hash(state);
    self.rtl.hash(state);
    hash_f64(self.priority.unwrap_or(0.0));
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

impl InjectableStyle {
  /// Creates a new InjectableStyle wrapped in Rc<InjectableStyleKind> with only LTR content.
  ///
  /// # Example
  /// ```ignore
  /// let style = InjectableStyle::regular(css_string, Some(0.5));
  /// ```
  #[inline]
  pub(crate) fn regular(ltr: String, priority: Option<f64>) -> Rc<InjectableStyleKind> {
    Rc::new(InjectableStyleKind::Regular(InjectableStyle {
      ltr,
      rtl: None,
      priority,
    }))
  }

  /// Creates a new InjectableStyle wrapped in Rc<InjectableStyleKind> with both LTR and RTL content.
  ///
  /// # Example
  /// ```ignore
  /// let style = InjectableStyle::with_rtl(ltr_css, rtl_css, Some(0.5));
  /// ```
  #[inline]
  pub(crate) fn with_rtl(ltr: String, rtl: String, priority: Option<f64>) -> Rc<InjectableStyleKind> {
    Rc::new(InjectableStyleKind::Regular(InjectableStyle {
      ltr,
      rtl: Some(rtl),
      priority,
    }))
  }
}

impl Default for InjectableStyle {
  fn default() -> Self {
    InjectableStyle {
      ltr: "".to_string(),
      rtl: None,
      priority: Some(0.0),
    }
  }
}

impl From<InjectableConstStyle> for InjectableStyleConstBase {
  fn from(style: InjectableConstStyle) -> Self {
    InjectableStyleConstBase {
      ltr: style.ltr,
      rtl: style.rtl,
      const_key: style.const_key,
      const_value: style.const_value,
    }
  }
}
impl Default for InjectableConstStyle {
  fn default() -> Self {
    InjectableConstStyle {
      ltr: "".to_string(),
      rtl: None,
      priority: Some(0.0),
      const_key: "".to_string(),
      const_value: "".to_string(),
    }
  }
}
