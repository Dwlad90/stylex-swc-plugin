use std::rc::Rc;

use indexmap::IndexMap;

use crate::shared::{
  enums::data_structures::{
    flat_compiled_styles_value::FlatCompiledStylesValue, fn_result::FnResult,
  },
  utils::core::js_to_expr::NestedStringObject,
};

use super::{parse_nullable_style::ResolvedArg, props::props};

pub(crate) fn attrs(styles: &[ResolvedArg]) -> Option<FnResult> {
  let props = props(styles)?;

  let props = props.as_props()?.as_values()?;

  let mut attrs_map: IndexMap<String, &Rc<FlatCompiledStylesValue>> = IndexMap::new();

  if let Some(class_name) = props.get("className") {
    attrs_map.insert("class".to_string(), class_name);
  }

  if let Some(data_style_src) = props.get("data-style-src") {
    attrs_map.insert("data-style-src".to_string(), data_style_src);
  }

  if props.get("style").is_some() {
    panic!("Implement inline style");
  }

  Some(FnResult::Attrs(
    NestedStringObject::FlatCompiledStylesValues(
      attrs_map.into_iter().map(|(k, v)| (k, v.clone())).collect(),
    ),
  ))
}
