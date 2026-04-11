// Tests for metadata shape extraction from injectable style variants.
// Source: crates/stylex-rs-compiler/src/utils/metadata.rs

use stylex_types::structures::injectable_style::{InjectableStyleBase, InjectableStyleConstBase};

use super::*;

#[test]
fn metadata_style_parts_regular_variant() {
  let style = InjectableStyleBaseKind::Regular(InjectableStyleBase {
    ltr: "a{color:red}".to_string(),
    rtl: Some("a{color:blue}".to_string()),
  });

  let parts = metadata_style_parts(&style);
  assert_eq!(parts.ltr, "a{color:red}");
  assert_eq!(parts.rtl, Some("a{color:blue}"));
  assert_eq!(parts.const_key, None);
  assert_eq!(parts.const_value, None);
}

#[test]
fn metadata_style_parts_const_variant() {
  let style = InjectableStyleBaseKind::Const(InjectableStyleConstBase {
    ltr: "a{color:red}".to_string(),
    rtl: None,
    const_key: "themeKey".to_string(),
    const_value: "themeVal".to_string(),
  });

  let parts = metadata_style_parts(&style);
  assert_eq!(parts.ltr, "a{color:red}");
  assert_eq!(parts.rtl, None);
  assert_eq!(parts.const_key, Some("themeKey"));
  assert_eq!(parts.const_value, Some("themeVal"));
}
