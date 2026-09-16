use std::rc::Rc;
use stylex_structures::pair::Pair;

use indexmap::IndexMap;

use crate::shared::{
  enums::data_structures::fn_result::FnResult,
  utils::core::styleq::{StyleQResult, styleq},
};
use stylex_state::{
  flat_compiled_styles_value::FlatCompiledStylesValue, types::FlatCompiledStyles,
};

use super::parse_nullable_style::ResolvedArg;

/// The properties a `stylex.props(...)` call is replaced by.
pub(crate) fn props(styles: &[ResolvedArg]) -> FnResult {
  FnResult::Values(props_map(styles))
}

/// The properties the merged styles become, before they are named as a result.
///
/// Split out because `attrs` needs the same map and reads it by another set of
/// names. Asking `props` for it would have `attrs` take the result apart again,
/// and every step of that is a case the result can never be in.
pub(crate) fn props_map(styles: &[ResolvedArg]) -> FlatCompiledStyles {
  let StyleQResult {
    class_name,
    inline_style,
    data_style_src,
  } = styleq(styles);

  // Three names at most: the class, the inline style and the debug source.
  let mut props_map: FlatCompiledStyles = IndexMap::with_capacity(3);

  if !class_name.is_empty() {
    props_map.insert(
      "className".to_string(),
      Rc::new(FlatCompiledStylesValue::String(class_name)),
    );
  }

  if let Some(inline_style) = inline_style {
    // Each name is kept as the author spelled it, because the runtime reads
    // this property as a style object and `marginTop` is the name such an
    // object carries. The CSS spelling is asked for where CSS text is made.
    //
    // The merged map is owned here, so each name moves into the pair it makes.
    // The value is behind a shared pointer, so it is copied.
    let mut pairs: Vec<Pair> = Vec::with_capacity(inline_style.len());

    pairs.extend(
      inline_style
        .into_iter()
        .filter_map(|(key, value)| value.as_string().map(|text| Pair::new(key, text.clone()))),
    );

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

  props_map
}

#[cfg(test)]
#[path = "tests/props_tests.rs"]
mod tests;
