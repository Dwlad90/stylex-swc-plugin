use std::borrow::Cow;
use std::rc::Rc;

use indexmap::IndexMap;
use stylex_css::css::common::inline_style_to_css_string;
use stylex_macros::stylex_unreachable;
use stylex_structures::pair::PairCow;
use stylex_utils::number::to_js_string;

use crate::shared::enums::data_structures::fn_result::FnResult;
use stylex_state::flat_compiled_styles_value::FlatCompiledStylesValue;

use super::{parse_nullable_style::ResolvedArg, props::props_map};

/// The text a JavaScript object spells when a string is asked of it.
const OBJECT_AS_TEXT: &str = "[object Object]";

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
  // `style` attribute holds. `props_map` writes that entry as an object and
  // nothing else, so there is no other shape to read here.
  if let Some(FlatCompiledStylesValue::Object(style)) = attrs.get("style").map(Rc::as_ref) {
    let declarations = style
      .iter()
      .map(|(key, value)| PairCow {
        key: Cow::Borrowed(key.as_str()),
        value: inline_value_as_text(value),
      })
      .collect::<Vec<PairCow<'_>>>();

    attrs_map.insert(
      "style".to_string(),
      Rc::new(FlatCompiledStylesValue::String(inline_style_to_css_string(
        &declarations,
      ))),
    );
  }

  FnResult::Values(attrs_map)
}

/// The text one inline declaration spells inside a `style` attribute.
///
/// An attribute holds text alone, so each value is spelled the way JavaScript
/// spells it: a number as its own digits, a boolean as `true` or `false`, and
/// an object as the text every plain object spells, whatever it holds.
///
/// A value kind an inline style cannot hold means the styles were not
/// flattened, and the call is refused rather than written short -- the same
/// answer the writer of a `style` property gives, so one unreadable value has
/// one outcome and not two.
///
/// A `null` is not among the kinds read here. The merge drops a declaration
/// set to null before it writes the style, and one held inside an object is
/// never reached, because an object spells its own text without being read
/// into.
fn inline_value_as_text(value: &FlatCompiledStylesValue) -> Cow<'_, str> {
  match value {
    FlatCompiledStylesValue::String(text) => Cow::Borrowed(text.as_str()),
    FlatCompiledStylesValue::Number(number) => Cow::Owned(to_js_string(*number)),
    FlatCompiledStylesValue::Bool(value) => Cow::Borrowed(match value {
      true => "true",
      false => "false",
    }),
    FlatCompiledStylesValue::Object(_) => Cow::Borrowed(OBJECT_AS_TEXT),
    _ => stylex_unreachable!("Encountered an unsupported value type in an inline style."),
  }
}
