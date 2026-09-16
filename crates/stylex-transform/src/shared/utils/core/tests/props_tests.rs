//! Tests for the React properties and the HTML attributes a merge is written
//! back as.

use std::rc::Rc;

use stylex_state::{
  flat_compiled_styles_value::FlatCompiledStylesValue, types::FlatCompiledStyles,
};

use crate::shared::enums::data_structures::fn_result::FnResult;
use crate::shared::utils::core::tests::style_args::{
  ResultReader, inline, inline_pair, style_of, styles,
};
use crate::shared::utils::core::{
  attrs::attrs,
  parse_nullable_style::{ResolvedArg, StyleObject},
  props::props,
  stylex::stylex,
};

/// The values a result holds, keyed the way it named them.
fn values_of(result: FnResult) -> FlatCompiledStyles {
  match result.as_values() {
    Some(values) => values.clone(),
    None => panic!("the result holds no values: {result:?}"),
  }
}

fn text_of(values: &FlatCompiledStyles, key: &str) -> String {
  match values.get(key).map(Rc::as_ref) {
    Some(FlatCompiledStylesValue::String(text)) => text.clone(),
    other => panic!("the key {key} does not hold text: {other:?}"),
  }
}

/// The declarations the `style` property holds, as name and value.
fn style_pairs_of(values: &FlatCompiledStyles) -> Vec<(String, String)> {
  match values.get("style").map(Rc::as_ref) {
    Some(FlatCompiledStylesValue::KeyValues(pairs)) => pairs
      .iter()
      .map(|pair| (pair.key.clone(), pair.value.clone()))
      .collect(),
    other => panic!("the style is not a set of pairs: {other:?}"),
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

/// An inline style becomes the pairs a `style` property holds, each under the
/// name the author wrote. The runtime reads the property as a style object,
/// and `marginTop` is the name such an object carries.
///
/// The test `writes_an_inline_style_out_as_css_text` holds the other half of
/// the rule: it reads the same name as an attribute and gets `margin-top`.
/// Each result spells the name the way its own reader expects.
#[test]
fn names_an_inline_style_as_the_pairs_the_author_wrote() {
  let values = values_of(props(&[styles(inline(&[("marginTop", "1px")]))]));

  assert_eq!(
    style_pairs_of(&values),
    [("marginTop".to_owned(), "1px".to_owned())]
  );
}

/// A custom property carries the name CSS gives it, so the author's spelling
/// and the CSS spelling are one name. It is the one name both results write
/// the same way, and neither may dash it further -- `--myColor` holds a
/// capital that the CSS spelling of any other name would dash.
///
/// Both results are read here, because this is the only name for which they
/// must agree, and nothing else pins the attribute half of it.
#[test]
fn keeps_the_name_of_a_custom_property() {
  let declaration = [styles(inline(&[("--myColor", "red")]))];

  assert_eq!(
    style_pairs_of(&values_of(props(&declaration))),
    [("--myColor".to_owned(), "red".to_owned())]
  );
  assert_eq!(
    text_of(&values_of(attrs(&declaration)), "style"),
    "--myColor:red"
  );
}

/// An inline value that spells no CSS text is left out. Only text can be
/// written into a `style` property.
///
/// The value is built here, because the kind it is -- a single key-value pair
/// -- has no producer at all outside tests. The kind a source really writes
/// is a boolean, which is left out the same way -- ticket 84 of
/// `.scratch/split-transform-crate`.
#[test]
fn leaves_out_an_inline_value_that_spells_no_text() {
  let values = values_of(props(&[styles(inline_pair("margin", "1px"))]));

  assert!(style_pairs_of(&values).is_empty());
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

/// The three calls answer two kinds, and each names the kind it made: a merge
/// asked for properties or attributes answers values, and one asked for a class
/// name answers a string.
#[test]
fn each_call_answers_the_kind_it_makes() {
  let merged = [style_of(&[("color", "xa")])];

  assert!(props(&merged).as_values().is_some());
  assert!(attrs(&merged).as_values().is_some());
  assert!(stylex(&merged).as_class_name().is_some());
}
