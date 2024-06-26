use indexmap::IndexMap;

use crate::shared::{
  enums::data_structures::{
    flat_compiled_styles_value::FlatCompiledStylesValue, fn_result::FnResult,
  },
  utils::core::js_to_expr::NestedStringObject,
};

use super::{parse_nullable_style::ResolvedArg, props::props};

pub(crate) fn attrs(styles: &Vec<ResolvedArg>) -> Option<FnResult> {
  let props = props(styles);

  let props = props
    .and_then(|props| props.as_props().cloned())
    .and_then(|props| props.as_values().cloned())?;

  let mut attrs_map: IndexMap<String, Box<FlatCompiledStylesValue>> = IndexMap::new();

  if let Some(class_name) = props.get("className") {
    attrs_map.insert("class".to_string(), class_name.clone());
  };

  if let Some(_inline_style) = props.get("style") {
    panic!("Implement inline style");
  };

  Some(FnResult::Attrs(
    NestedStringObject::FlatCompiledStylesValues(attrs_map),
  ))
}
