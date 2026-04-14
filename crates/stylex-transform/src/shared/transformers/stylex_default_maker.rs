use std::rc::Rc;

use indexmap::IndexMap;

use crate::shared::{
  enums::data_structures::flat_compiled_styles_value::FlatCompiledStylesValue,
  utils::core::js_to_expr::NestedStringObject,
};
use stylex_constants::constants::common::COMPILED_KEY;
use stylex_structures::stylex_state_options::StyleXStateOptions;

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
    COMPILED_KEY.to_string(),
    Rc::new(FlatCompiledStylesValue::Bool(true)),
  );

  NestedStringObject::FlatCompiledStylesValues(result)
}

#[cfg(test)]
#[path = "../../tests/stylex_default_maker_tests.rs"]
mod tests;
