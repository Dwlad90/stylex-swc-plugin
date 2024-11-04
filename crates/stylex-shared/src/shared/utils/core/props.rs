use std::rc::Rc;

use indexmap::IndexMap;

use crate::{
  shared::{
    enums::data_structures::{
      flat_compiled_styles_value::FlatCompiledStylesValue, fn_result::FnResult,
    },
    utils::core::js_to_expr::NestedStringObject,
  },
  transform::styleq::common::{styleq, StyleQResult},
};

use super::parse_nullable_style::ResolvedArg;

pub(crate) fn props(styles: &[ResolvedArg]) -> Option<FnResult> {
  let StyleQResult {
    class_name,
    inline_style,
  } = styleq(styles);

  let mut props_map: IndexMap<String, Rc<FlatCompiledStylesValue>> = IndexMap::new();

  if !class_name.is_empty() {
    props_map.insert(
      "className".to_string(),
      Rc::new(FlatCompiledStylesValue::String(class_name)),
    );
  }

  if let Some(_inline_style) = inline_style {
    unimplemented!("Inline style");
  }

  Some(FnResult::Props(
    NestedStringObject::FlatCompiledStylesValues(props_map),
  ))
}
