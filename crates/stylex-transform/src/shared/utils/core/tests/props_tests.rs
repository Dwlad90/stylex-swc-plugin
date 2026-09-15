//! Tests for the React properties and the HTML attributes a merge is written
//! back as.

use std::rc::Rc;

use stylex_state::{
  flat_compiled_styles_value::FlatCompiledStylesValue, types::FlatCompiledStyles,
};

use super::style_args::{inline, inline_pair, style_of, styles};
use crate::shared::enums::data_structures::fn_result::FnResult;
use crate::shared::utils::core::{
  attrs::attrs,
  parse_nullable_style::{ResolvedArg, StyleObject},
  props::props,
};

/// The values a result holds, keyed the way it named them.
fn values_of(result: FnResult) -> FlatCompiledStyles {
  let object = match (result.as_props(), result.as_attrs()) {
    (Some(object), _) | (_, Some(object)) => object.clone(),
    _ => panic!("the result names neither properties nor attributes"),
  };

  match object.as_values() {
    Some(values) => values.clone(),
    None => panic!("the result holds no values"),
  }
}

fn text_of(values: &FlatCompiledStyles, key: &str) -> String {
  match values.get(key).map(Rc::as_ref) {
    Some(FlatCompiledStylesValue::String(text)) => text.clone(),
    other => panic!("the key {key} does not hold text: {other:?}"),
  }
}

#[test]
fn names_the_merged_classes_as_one_class_name() {
  let values = values_of(props(&[style_of(&[("color", "xa"), ("margin", "xb")])]));

  assert_eq!(text_of(&values, "className"), "xa xb");
}

/// A merge that wrote no class writes no `className`, rather than one holding
/// nothing.
#[test]
fn writes_no_class_name_when_the_merge_wrote_no_class() {
  assert!(values_of(props(&[])).is_empty());
  assert!(values_of(props(&[ResolvedArg::style_object(StyleObject::Nullable)])).is_empty());
}

/// An inline style becomes the pairs a `style` property holds, and each key is
/// written the way CSS spells it.
#[test]
fn names_an_inline_style_as_dashed_pairs() {
  let values = values_of(props(&[styles(inline(&[("marginTop", "1px")]))]));

  match values["style"].as_ref() {
    FlatCompiledStylesValue::KeyValues(pairs) => {
      assert_eq!(pairs.len(), 1);
      assert_eq!(pairs[0].key, "margin-top");
      assert_eq!(pairs[0].value, "1px");
    },
    other => panic!("the style is not a set of pairs: {other:?}"),
  }
}

/// A custom property is already spelled the way CSS spells it, so its name is
/// left alone.
#[test]
fn keeps_the_name_of_a_custom_property() {
  let values = values_of(props(&[styles(inline(&[("--myColor", "red")]))]));

  match values["style"].as_ref() {
    FlatCompiledStylesValue::KeyValues(pairs) => assert_eq!(pairs[0].key, "--myColor"),
    other => panic!("the style is not a set of pairs: {other:?}"),
  }
}

/// An inline value that spells no CSS text is left out. Only text can be
/// written into a `style` property.
#[test]
fn leaves_out_an_inline_value_that_spells_no_text() {
  let values = values_of(props(&[styles(inline_pair("margin", "1px"))]));

  match values["style"].as_ref() {
    FlatCompiledStylesValue::KeyValues(pairs) => assert!(pairs.is_empty()),
    other => panic!("the style is not a set of pairs: {other:?}"),
  }
}

/// The debug source is written only when the merge has one to write.
#[test]
fn names_the_debug_source_only_when_there_is_one() {
  let mut style = FlatCompiledStyles::new();

  style.insert(
    stylex_constants::constants::common::COMPILED_KEY.to_owned(),
    Rc::new(FlatCompiledStylesValue::Bool(true)),
  );

  assert!(!values_of(props(&[styles(style)])).contains_key("data-style-src"));
}

#[test]
fn names_the_merged_classes_as_a_class_attribute() {
  let values = values_of(attrs(&[style_of(&[("color", "xa")])]));

  assert_eq!(text_of(&values, "class"), "xa");
  assert!(!values.contains_key("className"));
}

/// An attribute is text, so the inline style is written out as the CSS a
/// `style` attribute holds.
#[test]
fn writes_an_inline_style_out_as_css_text() {
  let values = values_of(attrs(&[styles(inline(&[
    ("marginTop", "1px"),
    ("color", "red"),
  ]))]));

  assert_eq!(text_of(&values, "style"), "margin-top:1px;color:red");
}

#[test]
fn writes_no_attribute_for_a_merge_that_wrote_nothing() {
  assert!(values_of(attrs(&[])).is_empty());
}

/// An inline style holding nothing that spells CSS text writes an empty `style`
/// attribute rather than none, because the property it came from was written.
#[test]
fn writes_an_empty_style_attribute_for_a_style_that_spells_no_text() {
  let values = values_of(attrs(&[styles(inline_pair("margin", "1px"))]));

  assert_eq!(text_of(&values, "style"), "");
}
