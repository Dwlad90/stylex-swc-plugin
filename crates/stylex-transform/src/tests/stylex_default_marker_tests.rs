// Tests for default marker object shape and prefix behavior.
// Source: crates/stylex-transform/src/shared/transformers/stylex_default_marker.rs

use super::*;

#[test]
fn test_default_marker_with_prefix() {
  let options = StyleXStateOptions::default().with_class_name_prefix("x");

  let result = stylex_default_marker(&options);

  let map = result
    .as_values()
    .expect("Expected FlatCompiledStylesValues");

  assert!(map.contains_key("x-default-marker"));
  assert!(map.contains_key("$$css"));

  if let Some(FlatCompiledStylesValue::String(s)) = map.get("x-default-marker").map(|v| v.as_ref())
  {
    assert_eq!(s, "x-default-marker");
  } else {
    panic!("Expected string value for marker class");
  }

  if let Some(FlatCompiledStylesValue::Bool(b)) = map.get("$$css").map(|v| v.as_ref()) {
    assert!(b);
  } else {
    panic!("Expected boolean value for $$css");
  }
}

#[test]
fn test_default_marker_with_custom_prefix() {
  let options = StyleXStateOptions::default().with_class_name_prefix("custom");

  let result = stylex_default_marker(&options);

  let map = result
    .as_values()
    .expect("Expected FlatCompiledStylesValues");

  assert!(map.contains_key("custom-default-marker"));

  if let Some(FlatCompiledStylesValue::String(s)) =
    map.get("custom-default-marker").map(|v| v.as_ref())
  {
    assert_eq!(s, "custom-default-marker");
  } else {
    panic!("Expected string value for marker class");
  }
}

#[test]
fn test_default_marker_with_empty_prefix() {
  let options = StyleXStateOptions::default().with_class_name_prefix("");

  let result = stylex_default_marker(&options);

  let map = result
    .as_values()
    .expect("Expected FlatCompiledStylesValues");

  assert!(map.contains_key("default-marker"));

  if let Some(FlatCompiledStylesValue::String(s)) = map.get("default-marker").map(|v| v.as_ref()) {
    assert_eq!(s, "default-marker");
  } else {
    panic!("Expected string value for marker class");
  }
}

#[test]
fn test_default_marker_always_has_css_marker() {
  let options = StyleXStateOptions::default();
  let result = stylex_default_marker(&options);

  let map = result
    .as_values()
    .expect("Expected FlatCompiledStylesValues");

  assert!(map.contains_key("$$css"));
  assert_eq!(map.len(), 2); // marker class + $$css
}
