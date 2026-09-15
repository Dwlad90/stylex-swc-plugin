//! Tests for the object expression the compiled styles are written back as.

use std::rc::Rc;

use indexmap::IndexMap;
use stylex_state::{
  flat_compiled_styles_value::FlatCompiledStylesValue,
  types::{FlatCompiledStyles, StylesObjectMap},
};
use stylex_structures::pair::Pair;
use swc_core::{
  atoms::Wtf8Atom,
  ecma::ast::{Expr, Lit, PropName, PropOrSpread},
};

use crate::shared::utils::core::js_to_ast::{
  NestedStringObject, convert_object_to_ast, remove_objects_with_spreads,
};

/// The text of an atom. A lone surrogate spells no text, and no case here
/// writes one.
fn text_of(atom: &Wtf8Atom) -> String {
  match atom.as_str() {
    Some(text) => text.to_owned(),
    None => panic!("the atom spells no text: {atom:?}"),
  }
}

/// The key of a property, however it is spelled.
fn key_of(prop: &PropOrSpread) -> String {
  let key_value = match prop {
    PropOrSpread::Prop(prop) => match prop.as_key_value() {
      Some(key_value) => key_value,
      None => panic!("the property is not a key-value pair: {prop:?}"),
    },
    PropOrSpread::Spread(_) => panic!("the answer holds a spread"),
  };

  match &key_value.key {
    PropName::Ident(ident) => ident.sym.to_string(),
    PropName::Str(text) => text_of(&text.value),
    other => panic!("the key is spelled in a way no answer uses: {other:?}"),
  }
}

/// The value of a property, rendered so a case can name it in one word.
fn value_of(prop: &PropOrSpread) -> String {
  let key_value = match prop {
    PropOrSpread::Prop(prop) => match prop.as_key_value() {
      Some(key_value) => key_value,
      None => panic!("the property is not a key-value pair: {prop:?}"),
    },
    PropOrSpread::Spread(_) => panic!("the answer holds a spread"),
  };

  match key_value.value.as_ref() {
    Expr::Lit(Lit::Str(text)) => format!("string:{}", text_of(&text.value)),
    Expr::Lit(Lit::Num(number)) => format!("number:{}", number.value),
    Expr::Lit(Lit::Bool(value)) => format!("bool:{}", value.value),
    Expr::Lit(Lit::Null(_)) => "null".to_owned(),
    Expr::Object(object) => format!("object:{}", object.props.len()),
    other => panic!("the value is of a kind no answer uses: {other:?}"),
  }
}

fn properties_of(expr: &Expr) -> Vec<(String, String)> {
  match expr {
    Expr::Object(object) => object
      .props
      .iter()
      .map(|prop| (key_of(prop), value_of(prop)))
      .collect(),
    other => panic!("the answer is not an object expression: {other:?}"),
  }
}

fn values_of(pairs: &[(&str, FlatCompiledStylesValue)]) -> FlatCompiledStyles {
  let mut values: FlatCompiledStyles = IndexMap::new();

  for (key, value) in pairs {
    values.insert((*key).to_owned(), Rc::new(value.clone()));
  }

  values
}

/// Every kind of value a compiled style holds has a spelling of its own, and a
/// string that reads as a number is written as one.
#[test]
fn writes_each_kind_of_value_the_way_javascript_spells_it() {
  let values = values_of(&[
    ("color", FlatCompiledStylesValue::String("xabc".to_owned())),
    ("zIndex", FlatCompiledStylesValue::String("2".to_owned())),
    ("$$css", FlatCompiledStylesValue::Bool(true)),
    ("margin", FlatCompiledStylesValue::Null),
  ]);

  assert_eq!(
    properties_of(&convert_object_to_ast(
      &NestedStringObject::FlatCompiledStylesValues(values)
    )),
    [
      ("color".to_owned(), "string:xabc".to_owned()),
      ("zIndex".to_owned(), "number:2".to_owned()),
      ("$$css".to_owned(), "bool:true".to_owned()),
      ("margin".to_owned(), "null".to_owned()),
    ]
  );
}

/// A namespace becomes an object of its own, keyed by the name the author
/// wrote.
#[test]
fn writes_a_namespace_as_an_object_of_its_own() {
  let mut styles: StylesObjectMap = IndexMap::new();

  styles.insert(
    "root".to_owned(),
    Rc::new(values_of(&[(
      "color",
      FlatCompiledStylesValue::String("xabc".to_owned()),
    )])),
  );

  assert_eq!(
    properties_of(&convert_object_to_ast(
      &NestedStringObject::FlatCompiledStyles(styles)
    )),
    [("root".to_owned(), "object:1".to_owned())]
  );
}

#[test]
fn writes_an_empty_object_for_no_style() {
  assert!(
    properties_of(&convert_object_to_ast(
      &NestedStringObject::FlatCompiledStyles(StylesObjectMap::new())
    ))
    .is_empty()
  );
  assert!(
    properties_of(&convert_object_to_ast(
      &NestedStringObject::FlatCompiledStylesValues(FlatCompiledStyles::new())
    ))
    .is_empty()
  );
}

/// Only the three kinds above can be written back. Any other value means the
/// styles were not flattened, so the call is refused rather than written as
/// something the runtime cannot read.
#[test]
#[should_panic(expected = "Encountered an unsupported value type during AST conversion.")]
fn refuses_a_value_it_cannot_write() {
  convert_object_to_ast(&NestedStringObject::FlatCompiledStylesValues(values_of(&[
    (
      "color",
      FlatCompiledStylesValue::KeyValue(Pair::new("color".to_owned(), "red".to_owned())),
    ),
  ])));
}

/// The values are there to be read only when the object holds values. Asking a
/// namespace map for them answers nothing rather than the map itself.
#[test]
fn only_a_value_object_hands_its_values_back() {
  let values = values_of(&[("color", FlatCompiledStylesValue::String("xabc".to_owned()))]);

  assert_eq!(
    NestedStringObject::FlatCompiledStylesValues(values.clone()).as_values(),
    Some(&values)
  );
  assert_eq!(
    NestedStringObject::FlatCompiledStyles(StylesObjectMap::new()).as_values(),
    None
  );
}

#[test]
fn keeps_every_namespace_it_is_given() {
  let mut styles: StylesObjectMap = IndexMap::new();

  styles.insert("root".to_owned(), Rc::new(FlatCompiledStyles::new()));
  styles.insert("dark".to_owned(), Rc::new(FlatCompiledStyles::new()));

  let result = remove_objects_with_spreads(&styles);

  assert_eq!(result.keys().cloned().collect::<Vec<_>>(), ["root", "dark"]);
}
