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
  convert_namespaces_to_ast, convert_values_to_ast, remove_objects_with_spreads,
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
    properties_of(&convert_values_to_ast(&values)),
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
    properties_of(&convert_namespaces_to_ast(&styles)),
    [("root".to_owned(), "object:1".to_owned())]
  );
}

#[test]
fn writes_an_empty_object_for_no_style() {
  assert!(properties_of(&convert_namespaces_to_ast(&StylesObjectMap::new())).is_empty());
  assert!(properties_of(&convert_values_to_ast(&FlatCompiledStyles::new())).is_empty());
}

/// Only the three kinds above can be written back. Any other value means the
/// styles were not flattened, so the call is refused rather than written as
/// something the runtime cannot read.
#[test]
#[should_panic(expected = "Encountered an unsupported value type during AST conversion.")]
fn refuses_a_value_it_cannot_write() {
  convert_values_to_ast(&values_of(&[(
    "color",
    FlatCompiledStylesValue::KeyValue(Pair::new("color".to_owned(), "red".to_owned())),
  )]));
}

#[test]
fn keeps_every_namespace_it_is_given() {
  let mut styles: StylesObjectMap = IndexMap::new();

  styles.insert("root".to_owned(), Rc::new(FlatCompiledStyles::new()));
  styles.insert("dark".to_owned(), Rc::new(FlatCompiledStyles::new()));

  let result = remove_objects_with_spreads(&styles);

  assert_eq!(result.keys().cloned().collect::<Vec<_>>(), ["root", "dark"]);
}

/// Every prop of a written object is a key-value under a name that can be read
/// back, whatever the name and the value are.
///
/// The finalize sweep in `visit_mut_var_declarator` reads these objects and
/// leaves one alone that holds anything else -- a spread, a method, a key with
/// no compile-time text. Nothing it can be handed holds one, and this is where
/// that is true: the invariant is asserted where the object is built, rather
/// than argued at the reader.
#[test]
fn writes_every_prop_as_a_key_value_under_a_name() {
  let mut styles: StylesObjectMap = IndexMap::new();

  // A namespace whose name has to be quoted, and a declaration whose name has
  // to be quoted, are the two names that are not plain identifiers.
  styles.insert(
    "a-namespace".to_owned(),
    Rc::new(values_of(&[
      (
        "color-kMwMTN",
        FlatCompiledStylesValue::String("xabc".to_owned()),
      ),
      ("margin", FlatCompiledStylesValue::Null),
    ])),
  );

  let written = convert_namespaces_to_ast(&styles);

  // `key_of` refuses a spread and a prop that is not a key-value, and it reads
  // the name of every key a written object can carry.
  let namespaces = properties_of(&written);

  assert_eq!(
    namespaces,
    [("a-namespace".to_owned(), "object:2".to_owned())]
  );

  assert_eq!(
    properties_under(&written, "a-namespace"),
    [
      ("color-kMwMTN".to_owned(), "string:xabc".to_owned()),
      ("margin".to_owned(), "null".to_owned()),
    ]
  );
}

/// The properties of the object one named key holds, read the same way the
/// answer itself is. A namespace and an inline style are both such an object.
fn properties_under(expr: &Expr, key: &str) -> Vec<(String, String)> {
  let object = match expr {
    Expr::Object(object) => object,
    other => panic!("the answer is not an object expression: {other:?}"),
  };

  match object.props.iter().find(|prop| key_of(prop) == key) {
    Some(prop) => match prop.as_prop().and_then(|prop| prop.as_key_value()) {
      Some(key_value) => properties_of(key_value.value.as_ref()),
      None => panic!("the key {key} is not a key-value prop"),
    },
    None => panic!("the answer holds no key {key}"),
  }
}

/// The inline style a `props` merge built is written as the object a `style`
/// property holds, one declaration per pair.
///
/// The text stays text, because an inline style is what the author wrote. A
/// compiled value that reads as a number is written as one, but `gridRow: '1'`
/// is a string in the source and stays a string here.
#[test]
fn writes_an_inline_style_as_the_object_a_style_property_holds() {
  let values = values_of(&[
    (
      "className",
      FlatCompiledStylesValue::String("xabc".to_owned()),
    ),
    (
      "style",
      FlatCompiledStylesValue::KeyValues(vec![
        Pair::new("color".to_owned(), "blue".to_owned()),
        Pair::new("gridRow".to_owned(), "1".to_owned()),
      ]),
    ),
  ]);

  let written = convert_values_to_ast(&values);

  assert_eq!(
    properties_of(&written),
    [
      ("className".to_owned(), "string:xabc".to_owned()),
      ("style".to_owned(), "object:2".to_owned()),
    ]
  );

  assert_eq!(
    properties_under(&written, "style"),
    [
      ("color".to_owned(), "string:blue".to_owned()),
      ("gridRow".to_owned(), "string:1".to_owned()),
    ]
  );
}

/// An inline style that holds no pair is written as an empty object, because
/// the merge wrote the property it sits under.
///
/// A source does reach this shape: `stylex.props({ color: true })` keeps the
/// declaration through the merge, and the step that writes the properties
/// leaves it out because it is not text. The shape is built here and not
/// compiled, because that source is the one the compiler must stop leaving
/// out -- ticket 84 of `.scratch/split-transform-crate` -- so a test compiled
/// from it would pin the gap rather than the writer.
#[test]
fn writes_an_inline_style_that_holds_no_pair_as_an_empty_object() {
  let values = values_of(&[("style", FlatCompiledStylesValue::KeyValues(vec![]))]);

  assert_eq!(
    properties_of(&convert_values_to_ast(&values)),
    [("style".to_owned(), "object:0".to_owned())]
  );
}
