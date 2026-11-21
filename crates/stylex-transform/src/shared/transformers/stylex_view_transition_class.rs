use std::fmt::Write;

use indexmap::IndexMap;
use stylex_macros::stylex_panic;

use crate::shared::utils::object::{
  obj_transform_values_and_map_keys, preprocess_object_properties,
};
use stylex_ast::ast::convertors::{convert_key_value_to_str, get_key_values_from_object};
use stylex_constants::constants::messages::VALUES_MUST_BE_OBJECT;
use stylex_state::{evaluate_result_value::EvaluateResultValue, state_manager::StateManager};
use stylex_structures::pair::Pair;
use stylex_types::{
  enums::data_structures::injectable_style::InjectableStyleKind,
  structures::injectable_style::InjectableStyle,
};
use stylex_utils::{hash::create_hash, string::dashify};

/// One animation part: the selector it is written under, and the CSS text of
/// the declarations below it.
type ViewTransitionParts = IndexMap<String, String>;

pub(crate) fn stylex_view_transition_class(
  styles: &EvaluateResultValue,
  state: &mut StateManager,
) -> (String, InjectableStyleKind) {
  // NOTE: an unset `classNamePrefix` arrives here already defaulted to `x`,
  // so an empty one was asked for explicitly and is honoured as empty.
  let class_name_prefix = state.options.class_name_prefix.clone();

  let Some(styles) = styles.as_expr().and_then(|expr| expr.as_object()) else {
    stylex_panic!("{}", VALUES_MUST_BE_OBJECT)
  };

  let key_values = get_key_values_from_object(styles);

  let mut parts = ViewTransitionParts::with_capacity(key_values.len());

  for key_value in key_values.iter() {
    let part = convert_key_value_to_str(key_value);

    let entries = preprocess_object_properties(&key_value.value, state);
    let declarations =
      obj_transform_values_and_map_keys(&entries, state, |key| dashify(key).into_owned());

    // The exact length of the text, so the buffer is not grown.
    let capacity = declarations.iter().map(Pair::css_text_len).sum();
    let mut css_text = String::with_capacity(capacity);

    for declaration in &declarations {
      declaration.push_css_text(&mut css_text);
    }

    parts.insert(format!("::view-transition-{}", dashify(&part)), css_text);
  }

  let view_transition_class_name =
    class_name_prefix + create_hash(&string_to_hash(&parts)).as_str();

  let style = construct_final_view_transition_css_str(&parts, &view_transition_class_name);

  (
    view_transition_class_name,
    InjectableStyleKind::Regular(InjectableStyle {
      ltr: style,
      rtl: None,
      priority: Some(1.0),
    }),
  )
}

fn construct_final_view_transition_css_str(
  parts: &ViewTransitionParts,
  class_name: &str,
) -> String {
  let capacity = parts
    .iter()
    .map(|(part, css_text)| part.len() + class_name.len() + css_text.len() + 6)
    .sum();

  let mut result = String::with_capacity(capacity);

  for (part, css_text) in parts.iter() {
    let _ = write!(result, "{}(*.{}){{{}}}", part, class_name, css_text);
  }

  result
}

/// The string the class name is hashed from.
///
/// This writes `selector:body;`, which is not a rule's CSS text at all, so it
/// does not use [`Pair::push_css_text`]. A selector whose body is empty still
/// has to appear, because the reference hashes it that way and the name has to
/// agree. What the emitted CSS carries is set by
/// [`construct_final_view_transition_css_str`], not here.
fn string_to_hash(parts: &ViewTransitionParts) -> String {
  let mut result = String::with_capacity(parts.len() * 16);

  for (part, css_text) in parts.iter() {
    let _ = write!(result, "{}:{};", part, css_text);
  }

  result
}
