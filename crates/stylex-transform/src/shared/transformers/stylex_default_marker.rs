use std::rc::Rc;

use indexmap::IndexMap;

use crate::shared::utils::core::js_to_ast::NestedStringObject;
use stylex_constants::constants::common::COMPILED_KEY;
use stylex_state::{
  flat_compiled_styles_value::FlatCompiledStylesValue, types::FlatCompiledStyles,
};
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
  NestedStringObject::FlatCompiledStylesValues(stylex_default_marker_values(options))
}

/// The marker's entries, for a caller that reads them rather than writes them
/// back as an object.
pub(crate) fn stylex_default_marker_values(options: &StyleXStateOptions) -> FlatCompiledStyles {
  // NOTE: the prefix is always applied, including when it is empty — an
  // unset `classNamePrefix` arrives here already defaulted to `x`, so an
  // empty one was asked for explicitly and keeps its separator.
  let prefix = format!("{}-", options.class_name_prefix);

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

  result
}

#[cfg(test)]
#[path = "../../tests/stylex_default_marker_tests.rs"]
mod tests;
