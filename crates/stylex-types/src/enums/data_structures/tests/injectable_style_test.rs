//! Tests for InjectableStyleKind → InjectableStyleBaseKind conversion
//! (From trait implementation).

use crate::{
  enums::data_structures::injectable_style::{InjectableStyleBaseKind, InjectableStyleKind},
  structures::injectable_style::{InjectableConstStyle, InjectableStyle},
};

#[test]
fn test_from_regular_kind() {
  let style = InjectableStyleKind::Regular(InjectableStyle {
    ltr: "color:red".to_string(),
    rtl: Some("color:blue".to_string()),
    priority: Some(1.0),
  });
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
  let style = InjectableStyleKind::Const(InjectableConstStyle {
    ltr: "color:red".to_string(),
    rtl: None,
    priority: Some(0.5),
    const_key: "key".to_string(),
    const_value: "val".to_string(),
  });
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
