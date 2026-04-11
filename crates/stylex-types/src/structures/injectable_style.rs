use std::{hash::Hash, rc::Rc};

use serde::{Deserialize, Serialize};

use crate::enums::data_structures::injectable_style::InjectableStyleKind;
use stylex_utils::hash::hash_f64;

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
pub struct InjectableConstStyle {
  pub ltr: String,
  pub rtl: Option<String>,
  pub priority: Option<f64>,
  pub const_key: String,
  pub const_value: String,
}

#[cfg(not(tarpaulin_include))]
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
  pub fn regular(ltr: impl Into<String>, priority: Option<f64>) -> Rc<InjectableStyleKind> {
    Rc::new(InjectableStyleKind::Regular(InjectableStyle {
      ltr: ltr.into(),
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
  pub fn with_rtl(
    ltr: impl Into<String>,
    rtl: impl Into<String>,
    priority: Option<f64>,
  ) -> Rc<InjectableStyleKind> {
    Rc::new(InjectableStyleKind::Regular(InjectableStyle {
      ltr: ltr.into(),
      rtl: Some(rtl.into()),
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

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_regular_factory() {
    let style = InjectableStyle::regular("color:red".to_string(), Some(0.5));
    match style.as_ref() {
      InjectableStyleKind::Regular(s) => {
        assert_eq!(s.ltr, "color:red");
        assert_eq!(s.rtl, None);
        assert_eq!(s.priority, Some(0.5));
      },
      _ => panic!("Expected Regular"),
    }
  }

  #[test]
  fn test_with_rtl_factory() {
    let style = InjectableStyle::with_rtl(
      "margin-left:10px".to_string(),
      "margin-right:10px".to_string(),
      Some(1.0),
    );
    match style.as_ref() {
      InjectableStyleKind::Regular(s) => {
        assert_eq!(s.ltr, "margin-left:10px");
        assert_eq!(s.rtl, Some("margin-right:10px".to_string()));
        assert_eq!(s.priority, Some(1.0));
      },
      _ => panic!("Expected Regular"),
    }
  }

  #[test]
  fn test_default_injectable_style() {
    let style = InjectableStyle::default();
    assert_eq!(style.ltr, "");
    assert_eq!(style.rtl, None);
    assert_eq!(style.priority, Some(0.0));
  }

  #[test]
  fn test_default_injectable_const_style() {
    let style = InjectableConstStyle::default();
    assert_eq!(style.ltr, "");
    assert_eq!(style.rtl, None);
    assert_eq!(style.priority, Some(0.0));
    assert_eq!(style.const_key, "");
    assert_eq!(style.const_value, "");
  }

  #[test]
  fn test_from_injectable_style_to_base() {
    let style = InjectableStyle {
      ltr: "color:red".to_string(),
      rtl: Some("color:blue".to_string()),
      priority: Some(1.0),
    };
    let base: InjectableStyleBase = style.into();
    assert_eq!(base.ltr, "color:red");
    assert_eq!(base.rtl, Some("color:blue".to_string()));
  }

  #[test]
  fn test_from_injectable_const_style_to_base() {
    let style = InjectableConstStyle {
      ltr: "color:red".to_string(),
      rtl: None,
      priority: Some(0.5),
      const_key: "k".to_string(),
      const_value: "v".to_string(),
    };
    let base: InjectableStyleConstBase = style.into();
    assert_eq!(base.ltr, "color:red");
    assert_eq!(base.rtl, None);
    assert_eq!(base.const_key, "k");
    assert_eq!(base.const_value, "v");
  }
}
