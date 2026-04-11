use serde::{Deserialize, Serialize};

use crate::structures::injectable_style::{
  InjectableConstStyle, InjectableStyle, InjectableStyleBase, InjectableStyleConstBase,
};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub enum InjectableStyleKind {
  Regular(InjectableStyle),
  Const(InjectableConstStyle),
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Hash, Eq)]
pub enum InjectableStyleBaseKind {
  Regular(InjectableStyleBase),
  Const(InjectableStyleConstBase),
}

impl From<InjectableStyleKind> for InjectableStyleBaseKind {
  fn from(style: InjectableStyleKind) -> Self {
    match style {
      InjectableStyleKind::Regular(style) => {
        InjectableStyleBaseKind::Regular(InjectableStyleBase {
          ltr: style.ltr,
          rtl: style.rtl,
        })
      },
      InjectableStyleKind::Const(style) => {
        InjectableStyleBaseKind::Const(InjectableStyleConstBase {
          ltr: style.ltr,
          rtl: style.rtl,
          const_key: style.const_key,
          const_value: style.const_value,
        })
      },
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_from_regular_kind() {
    let style = InjectableStyleKind::Regular(
      crate::structures::injectable_style::InjectableStyle {
        ltr: "color:red".to_string(),
        rtl: Some("color:blue".to_string()),
        priority: Some(1.0),
      },
    );
    let base: InjectableStyleBaseKind = style.into();
    match base {
      InjectableStyleBaseKind::Regular(b) => {
        assert_eq!(b.ltr, "color:red");
        assert_eq!(b.rtl, Some("color:blue".to_string()));
      },
      _ => panic!("Expected Regular variant"),
    }
  }

  #[test]
  fn test_from_const_kind() {
    let style = InjectableStyleKind::Const(
      crate::structures::injectable_style::InjectableConstStyle {
        ltr: "color:red".to_string(),
        rtl: None,
        priority: Some(0.5),
        const_key: "key".to_string(),
        const_value: "val".to_string(),
      },
    );
    let base: InjectableStyleBaseKind = style.into();
    match base {
      InjectableStyleBaseKind::Const(b) => {
        assert_eq!(b.ltr, "color:red");
        assert_eq!(b.rtl, None);
        assert_eq!(b.const_key, "key");
        assert_eq!(b.const_value, "val");
      },
      _ => panic!("Expected Const variant"),
    }
  }
}
