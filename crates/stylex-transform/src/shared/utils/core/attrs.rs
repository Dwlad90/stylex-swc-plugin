use std::rc::Rc;

use indexmap::IndexMap;
use stylex_css::css::common::inline_style_to_css_string;

use crate::shared::enums::data_structures::fn_result::FnResult;
use stylex_state::flat_compiled_styles_value::FlatCompiledStylesValue;

use super::{parse_nullable_style::ResolvedArg, props::props_map};

/// The HTML attributes a `stylex.attrs(...)` call is replaced by.
pub(crate) fn attrs(styles: &[ResolvedArg]) -> FnResult {
  let attrs = props_map(styles);

  let mut attrs_map: IndexMap<String, Rc<FlatCompiledStylesValue>> = IndexMap::new();

  if let Some(class_name) = attrs.get("className") {
    attrs_map.insert("class".to_string(), class_name.clone());
  }

  if let Some(data_style_src) = attrs.get("data-style-src") {
    attrs_map.insert("data-style-src".to_string(), data_style_src.clone());
  }

  // An attribute is text, so the inline style is written out as the CSS a
  // `style` attribute holds. `props_map` writes that entry as pairs and nothing
  // else, so there is no other shape to read here.
  if let Some(FlatCompiledStylesValue::KeyValues(pairs)) = attrs.get("style").map(Rc::as_ref) {
    attrs_map.insert(
      "style".to_string(),
      Rc::new(FlatCompiledStylesValue::String(inline_style_to_css_string(
        pairs,
      ))),
    );
  }

  FnResult::Values(attrs_map)
}
