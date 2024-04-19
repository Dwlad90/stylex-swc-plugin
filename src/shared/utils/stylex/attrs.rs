use indexmap::IndexMap;

use crate::shared::{
  enums::{FlatCompiledStylesValue, FnResult},
  utils::stylex::js_to_expr::NestedStringObject,
};

use super::{parse_nullable_style::ResolvedArg, props::props};

pub(crate) fn attrs(styles: &Vec<ResolvedArg>) -> Option<FnResult> {
  let props = props(styles);

  let Some(props) = props
    .and_then(|props| props.as_props().cloned())
    .and_then(|props| props.as_values().cloned())
  else {
    return None;
  };

  let mut attrs_map: IndexMap<String, FlatCompiledStylesValue> = IndexMap::new();

  if let Some(class_name) = props.get("className") {
    attrs_map.insert("class".to_string(), class_name.clone());
  };

  if let Some(inline_style) = props.get("style") {
    panic!("Implement inline style");
  };

  return Some(FnResult::Attrs(
    NestedStringObject::FlatCompiledStylesValues(attrs_map),
  ));
}
