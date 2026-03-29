use std::rc::Rc;
use stylex_structures::pair::Pair;

use indexmap::IndexMap;

use crate::{
  shared::{
    enums::data_structures::{
      flat_compiled_styles_value::FlatCompiledStylesValue, fn_result::FnResult,
    },
    structures::types::FlatCompiledStyles,
    utils::{core::js_to_expr::NestedStringObject, css::common::normalize_css_property_name},
  },
  transform::styleq::common::{StyleQResult, styleq},
};

use super::parse_nullable_style::ResolvedArg;

pub(crate) fn props(styles: &[ResolvedArg]) -> Option<FnResult> {
  let StyleQResult {
    class_name,
    inline_style,
    data_style_src,
  } = styleq(styles);

  let mut props_map: FlatCompiledStyles = IndexMap::new();

  if !class_name.is_empty() {
    props_map.insert(
      "className".to_string(),
      Rc::new(FlatCompiledStylesValue::String(class_name)),
    );
  }

  if let Some(inline_style) = inline_style {
    let pairs: Vec<Pair> = inline_style
      .iter()
      .filter_map(|(k, v)| {
        if let FlatCompiledStylesValue::String(val) = v.as_ref() {
          Some(Pair::new(normalize_css_property_name(k), val.clone()))
        } else {
          None
        }
      })
      .collect();

    props_map.insert(
      "style".to_string(),
      Rc::new(FlatCompiledStylesValue::KeyValues(pairs)),
    );
  }

  if let Some(data_style_src) = data_style_src
    && !data_style_src.is_empty()
  {
    props_map.insert(
      "data-style-src".to_string(),
      Rc::new(FlatCompiledStylesValue::String(data_style_src)),
    );
  }

  Some(FnResult::Props(
    NestedStringObject::FlatCompiledStylesValues(props_map),
  ))
}
