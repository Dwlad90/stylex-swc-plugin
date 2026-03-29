use std::rc::Rc;

use indexmap::IndexMap;

use crate::shared::{
  enums::data_structures::{
    flat_compiled_styles_value::FlatCompiledStylesValue, fn_result::FnResult,
  },
  utils::{core::js_to_expr::NestedStringObject, css::common::inline_style_to_css_string},
};

use super::{parse_nullable_style::ResolvedArg, props::props};

pub(crate) fn attrs(styles: &[ResolvedArg]) -> Option<FnResult> {
  let props = props(styles)?;

  let attrs = props.as_props()?.as_values()?;

  let mut attrs_map: IndexMap<String, Rc<FlatCompiledStylesValue>> = IndexMap::new();

  if let Some(class_name) = attrs.get("className") {
    attrs_map.insert("class".to_string(), class_name.clone());
  }

  if let Some(data_style_src) = attrs.get("data-style-src") {
    attrs_map.insert("data-style-src".to_string(), data_style_src.clone());
  }

  if let Some(style_value) = attrs.get("style") {
    match style_value.as_ref() {
      FlatCompiledStylesValue::KeyValues(pairs) => {
        let css_string = inline_style_to_css_string(pairs);
        attrs_map.insert(
          "style".to_string(),
          Rc::new(FlatCompiledStylesValue::String(css_string)),
        );
      },
      _ => {
        attrs_map.insert("style".to_string(), style_value.clone());
      },
    }
  }

  Some(FnResult::Attrs(
    NestedStringObject::FlatCompiledStylesValues(attrs_map),
  ))
}
