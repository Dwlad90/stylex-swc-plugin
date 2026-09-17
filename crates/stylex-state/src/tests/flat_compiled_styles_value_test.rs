//! What one compiled style value answers about itself.
//!
//! Every accessor here reads one variant and declines the rest, and a namespace
//! holds a mix of them: a class name, a `null` for a property the namespace
//! declares but does not set, a `true` for the compiled marker, an injectable
//! style, a variable pair. So each case asks one accessor for the variant it
//! reads *and* for a variant it does not, because an accessor that answered
//! `Some` for the wrong variant is what would let a `null` be written as a class
//! name.

use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use std::rc::Rc;

use swc_core::{
  common::DUMMY_SP,
  ecma::ast::{Expr, Lit, Str},
};

use stylex_enums::{css_syntax::CSSSyntax, value_with_default::ValueWithDefault};
use stylex_structures::base_css_type::BaseCSSType;
use stylex_styleq::StyleqValue;
use stylex_types::structures::injectable_style::InjectableStyle;

use crate::{flat_compiled_styles_value::FlatCompiledStylesValue, types::FlatCompiledStyles};

fn string_value(value: &str) -> FlatCompiledStylesValue {
  FlatCompiledStylesValue::String(value.to_string())
}

fn injectable() -> InjectableStyle {
  InjectableStyle {
    ltr: ".x1e2nbdu{color:red}".to_string(),
    rtl: None,
    priority: Some(3000.0),
  }
}

fn css_type() -> BaseCSSType {
  BaseCSSType {
    value: ValueWithDefault::String("red".to_string()),
    syntax: CSSSyntax::Color,
  }
}

fn tuple_value() -> FlatCompiledStylesValue {
  FlatCompiledStylesValue::Tuple(
    "--x1abcdef".to_string(),
    Box::new(Expr::Lit(Lit::Str(Str {
      span: DUMMY_SP,
      value: "red".into(),
      raw: None,
    }))),
    Some(css_type()),
  )
}

/// The declarations one inline object holds.
fn object_value() -> FlatCompiledStyles {
  let mut values = FlatCompiledStyles::new();

  values.insert(
    "color".to_owned(),
    Rc::new(FlatCompiledStylesValue::String("blue".to_owned())),
  );

  values
}

/// Every variant, so a case that must name one it does not read has a list to
/// take it from rather than building one more.
fn every_variant() -> Vec<FlatCompiledStylesValue> {
  vec![
    string_value("x1e2nbdu"),
    FlatCompiledStylesValue::Number(0.5),
    FlatCompiledStylesValue::Object(object_value()),
    FlatCompiledStylesValue::Null,
    FlatCompiledStylesValue::InjectableStyle(injectable()),
    FlatCompiledStylesValue::Bool(true),
    tuple_value(),
  ]
}

/// How many variants answered `Some` to one accessor. Every accessor reads
/// exactly one variant, so the count is the whole assertion.
fn count_answering(accessor: impl Fn(&FlatCompiledStylesValue) -> bool) -> usize {
  every_variant()
    .iter()
    .filter(|value| accessor(value))
    .count()
}

#[test]
fn a_tuple_answers_its_three_parts_and_nothing_else_does() {
  let value = tuple_value();

  let Some((key, expr, base)) = value.as_tuple() else {
    panic!("a tuple did not answer its parts");
  };

  assert_eq!(key, "--x1abcdef");
  assert_eq!(
    expr
      .as_lit()
      .and_then(|lit| lit.as_str())
      .and_then(|str_lit| str_lit.value.as_str()),
    Some("red")
  );
  assert_eq!(base.as_ref(), Some(&css_type()));

  assert_eq!(count_answering(|value| value.as_tuple().is_some()), 1);
}

/// A tuple with no CSS type is the shape a plain variable takes, and the third
/// part is `None` rather than the accessor declining.
#[test]
fn a_tuple_without_a_css_type_still_answers_its_parts() {
  let value = FlatCompiledStylesValue::Tuple(
    "--x1abcdef".to_string(),
    Box::new(Expr::Lit(Lit::Str(Str {
      span: DUMMY_SP,
      value: "red".into(),
      raw: None,
    }))),
    None,
  );

  let Some((key, _, base)) = value.as_tuple() else {
    panic!("a tuple without a CSS type did not answer its parts");
  };

  assert_eq!(key, "--x1abcdef");
  assert_eq!(base.as_ref(), None);
}

#[test]
fn a_string_answers_itself_and_nothing_else_does() {
  assert_eq!(
    string_value("x1e2nbdu").as_string(),
    Some(&"x1e2nbdu".to_string())
  );
  assert_eq!(count_answering(|value| value.as_string().is_some()), 1);
}

/// An empty class name is still a string: nothing about the accessor treats the
/// empty string as an absent one.
#[test]
fn an_empty_string_is_still_a_string() {
  assert_eq!(string_value("").as_string(), Some(&String::new()));
}

#[test]
fn an_injectable_style_answers_itself_and_nothing_else_does() {
  let value = FlatCompiledStylesValue::InjectableStyle(injectable());

  assert_eq!(value.as_injectable_style(), Some(&injectable()));
  assert_eq!(
    count_answering(|value| value.as_injectable_style().is_some()),
    1
  );
}

#[test]
fn a_bool_answers_itself_and_nothing_else_does() {
  assert_eq!(FlatCompiledStylesValue::Bool(true)._as_bool(), Some(&true));
  assert_eq!(
    FlatCompiledStylesValue::Bool(false)._as_bool(),
    Some(&false)
  );
  assert_eq!(count_answering(|value| value._as_bool().is_some()), 1);
}

#[test]
fn a_null_answers_itself_and_nothing_else_does() {
  assert_eq!(FlatCompiledStylesValue::Null._as_null(), Some(()));
  assert_eq!(count_answering(|value| value._as_null().is_some()), 1);
}

/// A number is its own variant, and the text spelling of the same number is
/// not it: `opacity: 0.5` and `opacity: '0.5'` are two declarations. Neither
/// of them reads back as the text a class name is.
#[test]
fn a_number_is_not_the_text_that_spells_it() {
  assert_ne!(FlatCompiledStylesValue::Number(0.5), string_value("0.5"));
  assert_eq!(FlatCompiledStylesValue::Number(0.5).as_class_name(), None);
  assert_eq!(FlatCompiledStylesValue::Number(2.0).as_class_name(), None);
}

#[test]
fn an_object_answers_its_declarations_and_nothing_else_does() {
  let value = FlatCompiledStylesValue::Object(object_value());

  assert_eq!(value.as_object(), Some(&object_value()));
  assert_eq!(count_answering(|value| value.as_object().is_some()), 1);
}

/// An object holding nothing is an object, not an absent one -- a `props` call
/// given `{}` beside a compiled style reaches this shape.
#[test]
fn an_object_holding_nothing_is_still_an_object() {
  let value = FlatCompiledStylesValue::Object(FlatCompiledStyles::new());

  assert_eq!(value.as_object(), Some(&FlatCompiledStyles::new()));
}

/// Two values that are equal hash alike, which the derived spelling cannot
/// give a number. The pair of zeroes is the one place equality and the bits
/// part company, and the reader spells both `0`, as the language does.
#[test]
fn a_value_hashes_by_what_it_holds() {
  assert_eq!(
    hash_of(&FlatCompiledStylesValue::Number(0.5)),
    hash_of(&FlatCompiledStylesValue::Number(0.5))
  );
  assert_ne!(
    hash_of(&FlatCompiledStylesValue::Number(0.5)),
    hash_of(&FlatCompiledStylesValue::Number(1.5))
  );
  assert_eq!(
    hash_of(&FlatCompiledStylesValue::Object(object_value())),
    hash_of(&FlatCompiledStylesValue::Object(object_value()))
  );
  assert_ne!(
    hash_of(&FlatCompiledStylesValue::Object(object_value())),
    hash_of(&FlatCompiledStylesValue::Object(FlatCompiledStyles::new()))
  );
}

/// No two variants hash alike while they hold the same thing, because the
/// variant itself is hashed first. A string `"true"` and a `true` are two
/// values a merge must tell apart.
#[test]
fn two_variants_holding_the_same_thing_hash_apart() {
  assert_ne!(
    hash_of(&string_value("true")),
    hash_of(&FlatCompiledStylesValue::Bool(true))
  );
}

/// An object is one value however its names were written down, so two that
/// hold the same declarations in either order hash alike. The map itself
/// compares that way, and a hash that read the names in order would make one
/// value into two.
#[test]
fn an_object_hashes_the_same_in_either_order() {
  let mut written_one_way = FlatCompiledStyles::new();

  written_one_way.insert(
    "color".to_owned(),
    Rc::new(FlatCompiledStylesValue::String("blue".to_owned())),
  );
  written_one_way.insert(
    "opacity".to_owned(),
    Rc::new(FlatCompiledStylesValue::Number(0.5)),
  );

  let mut written_the_other = FlatCompiledStyles::new();

  written_the_other.insert(
    "opacity".to_owned(),
    Rc::new(FlatCompiledStylesValue::Number(0.5)),
  );
  written_the_other.insert(
    "color".to_owned(),
    Rc::new(FlatCompiledStylesValue::String("blue".to_owned())),
  );

  assert_eq!(
    FlatCompiledStylesValue::Object(written_one_way.clone()),
    FlatCompiledStylesValue::Object(written_the_other.clone())
  );
  assert_eq!(
    hash_of(&FlatCompiledStylesValue::Object(written_one_way)),
    hash_of(&FlatCompiledStylesValue::Object(written_the_other))
  );
}

/// Two objects of different shapes hash apart, even where the names and the
/// values they hold read out in the same order. The count of each object is
/// what separates them, and a merge that keys a cached style on the hash alone
/// would otherwise answer one style with another style's class names.
#[test]
fn two_objects_of_different_shapes_hash_apart() {
  let mut flat = FlatCompiledStyles::new();

  flat.insert(
    "x".to_owned(),
    Rc::new(FlatCompiledStylesValue::Object(FlatCompiledStyles::new())),
  );
  flat.insert("y".to_owned(), Rc::new(FlatCompiledStylesValue::Null));

  let mut deep_inner = FlatCompiledStyles::new();

  deep_inner.insert("y".to_owned(), Rc::new(FlatCompiledStylesValue::Null));

  let mut deep = FlatCompiledStyles::new();

  deep.insert(
    "x".to_owned(),
    Rc::new(FlatCompiledStylesValue::Object(deep_inner)),
  );

  assert_ne!(
    hash_of(&FlatCompiledStylesValue::Object(flat)),
    hash_of(&FlatCompiledStylesValue::Object(deep))
  );
}

/// Every variant has a hash of its own, and no two of them share one. The
/// merge keys a cached style on this, so a variant left out of the reading
/// would answer with another style's class names.
#[test]
fn every_variant_hashes_apart_from_every_other() {
  let mut hashes = every_variant().iter().map(hash_of).collect::<Vec<_>>();
  let count = hashes.len();

  hashes.sort_unstable();
  hashes.dedup();

  assert_eq!(hashes.len(), count);
}

fn hash_of(value: &FlatCompiledStylesValue) -> u64 {
  let mut hasher = DefaultHasher::new();

  value.hash(&mut hasher);

  hasher.finish()
}

/// The three questions `styleq` asks of a namespace entry. A class name is the
/// string, `null` marks a property the namespace clears, and `true` is the
/// compiled marker -- so only the string variant names a class.
#[test]
fn styleq_reads_a_class_name_out_of_the_string_alone() {
  assert_eq!(string_value("x1e2nbdu").as_class_name(), Some("x1e2nbdu"));
  assert_eq!(count_answering(|value| value.as_class_name().is_some()), 1);
}

#[test]
fn styleq_reads_null_out_of_the_null_alone() {
  assert!(FlatCompiledStylesValue::Null.is_null());
  assert_eq!(count_answering(StyleqValue::is_null), 1);
}

/// No variant stands for a property that was not given.
///
/// The question exists because an inline style skips such a property whole: it
/// writes nothing, defines nothing, and leaves the property for a later style.
/// A compiled style has nothing to answer it with -- a property the author left
/// out is absent from the map rather than present holding nothing -- so every
/// variant answers no, and a variant added later that does stand for one has to
/// come back here.
#[test]
fn no_compiled_value_stands_for_a_property_that_was_not_given() {
  assert_eq!(count_answering(StyleqValue::is_undefined), 0);
}

/// `false` is not the compiled marker. The marker is written as `true` and only
/// `true`, so a namespace carrying `$$css: false` is not treated as compiled.
#[test]
fn styleq_reads_the_compiled_marker_out_of_a_true_alone() {
  assert!(FlatCompiledStylesValue::Bool(true).is_true_bool());
  assert!(!FlatCompiledStylesValue::Bool(false).is_true_bool());
  assert_eq!(count_answering(StyleqValue::is_true_bool), 1);
}
