use std::rc::Rc;

use indexmap::IndexMap;
use stylex_macros::stylex_unimplemented;

use crate::{
  shared::{
    enums::data_structures::{
      flat_compiled_styles_value::FlatCompiledStylesValue, fn_result::FnResult,
    },
    structures::types::FlatCompiledStyles,
    utils::core::js_to_expr::NestedStringObject,
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

  if let Some(_inline_style) = inline_style {
    stylex_unimplemented!(
      "Inline style objects from dynamic style functions are not yet supported in props()."
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
