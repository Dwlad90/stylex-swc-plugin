use std::rc::Rc;

use indexmap::IndexMap;

use crate::shared::{
  enums::data_structures::flat_compiled_styles_value::FlatCompiledStylesValue,
  structures::stylex_state_options::StyleXStateOptions,
  utils::core::js_to_expr::NestedStringObject,
};

/// Creates a default marker object that can be used with stylex.props()
/// to add a marker class for ancestor/sibling state observers.
///
/// # Arguments
/// * `options` - Reference to StyleXStateOptions to get the class name prefix
///
/// # Returns
/// A map with the default marker class name as both key and value,
/// plus a `$$css` marker set to true
pub(crate) fn stylex_default_marker(options: &StyleXStateOptions) -> NestedStringObject {
  let prefix = if !options.class_name_prefix.is_empty() {
    format!("{}-", options.class_name_prefix)
  } else {
    String::new()
  };

  let marker_class = format!("{}default-marker", prefix);

  let mut result = IndexMap::new();

  result.insert(
    marker_class.clone(),
    Rc::new(FlatCompiledStylesValue::String(marker_class)),
  );

  result.insert(
    "$$css".to_string(),
    Rc::new(FlatCompiledStylesValue::Bool(true)),
  );

  NestedStringObject::FlatCompiledStylesValues(result)
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_default_marker_with_prefix() {
    let options = StyleXStateOptions {
      class_name_prefix: "x".to_string(),
      ..Default::default()
    };

    let result = stylex_default_marker(&options);

    let map = result
      .as_values()
      .expect("Expected FlatCompiledStylesValues");

    assert!(map.contains_key("x-default-marker"));
    assert!(map.contains_key("$$css"));

    if let Some(FlatCompiledStylesValue::String(s)) =
      map.get("x-default-marker").map(|v| v.as_ref())
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
    let options = StyleXStateOptions {
      class_name_prefix: "custom".to_string(),
      ..Default::default()
    };

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
    let options = StyleXStateOptions {
      class_name_prefix: "".to_string(),
      ..Default::default()
    };

    let result = stylex_default_marker(&options);

    let map = result
      .as_values()
      .expect("Expected FlatCompiledStylesValues");

    assert!(map.contains_key("default-marker"));

    if let Some(FlatCompiledStylesValue::String(s)) = map.get("default-marker").map(|v| v.as_ref())
    {
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
}
