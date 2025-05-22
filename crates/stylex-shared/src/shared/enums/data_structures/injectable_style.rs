use serde::{Deserialize, Serialize};

use crate::shared::structures::injectable_style::{
  InjectableConstStyle, InjectableStyle, InjectableStyleBase, InjectableStyleConstBase,
};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub(crate) enum InjectableStyleKind {
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
      }
      InjectableStyleKind::Const(style) => {
        InjectableStyleBaseKind::Const(InjectableStyleConstBase {
          ltr: style.ltr,
          rtl: style.rtl,
          const_key: style.const_key,
          const_value: style.const_value,
        })
      }
    }
  }
}
