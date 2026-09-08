//! What one compiled style value answers about itself.
//!
//! Every accessor here reads one variant and declines the rest, and a namespace
//! holds a mix of them: a class name, a `null` for a property the namespace
//! declares but does not set, a `true` for the compiled marker, an injectable
//! style, a variable pair. So each case asks one accessor for the variant it
//! reads *and* for a variant it does not, because an accessor that answered
//! `Some` for the wrong variant is what would let a `null` be written as a class
//! name.

use swc_core::{
  common::DUMMY_SP,
  ecma::ast::{Expr, Lit, Str},
};

use stylex_enums::{css_syntax::CSSSyntax, value_with_default::ValueWithDefault};
use stylex_structures::{base_css_type::BaseCSSType, pair::Pair};
use stylex_styleq::StyleqValue;
use stylex_types::structures::injectable_style::InjectableStyle;

use crate::flat_compiled_styles_value::FlatCompiledStylesValue;

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

/// Every variant, so a case that must name one it does not read has a list to
/// take it from rather than building one more.
fn every_variant() -> Vec<FlatCompiledStylesValue> {
  vec![
    string_value("x1e2nbdu"),
    FlatCompiledStylesValue::KeyValue(Pair::new("color", "red")),
    FlatCompiledStylesValue::KeyValues(vec![Pair::new("color", "red")]),
    FlatCompiledStylesValue::Null,
    FlatCompiledStylesValue::InjectableStyle(injectable()),
    FlatCompiledStylesValue::Bool(true),
    tuple_value(),
    FlatCompiledStylesValue::CSSType(
      "--x1abcdef".to_string(),
      CSSSyntax::Color,
      "red".to_string(),
    ),
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

#[test]
fn a_key_value_answers_its_pair_and_nothing_else_does() {
  let pair = Pair::new("color", "red");
  let value = FlatCompiledStylesValue::KeyValue(pair.clone());

  assert_eq!(value.as_key_value(), Some(&pair));
  assert_eq!(count_answering(|value| value.as_key_value().is_some()), 1);
}

/// A list of pairs is its own variant: one pair and a list holding that pair are
/// two values, and neither accessor reads the other.
#[test]
fn key_values_answer_their_list_and_nothing_else_does() {
  let pairs = vec![Pair::new("color", "red"), Pair::new("color", "blue")];
  let value = FlatCompiledStylesValue::KeyValues(pairs.clone());

  assert_eq!(value.as_key_values(), Some(&pairs));
  assert_eq!(count_answering(|value| value.as_key_values().is_some()), 1);
}

/// An empty list is a list, not an absent one -- a fallback set that resolved to
/// nothing reaches here.
#[test]
fn an_empty_key_value_list_is_still_a_list() {
  let value = FlatCompiledStylesValue::KeyValues(vec![]);

  assert_eq!(value.as_key_values(), Some(&vec![]));
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

/// `false` is not the compiled marker. The marker is written as `true` and only
/// `true`, so a namespace carrying `$$css: false` is not treated as compiled.
#[test]
fn styleq_reads_the_compiled_marker_out_of_a_true_alone() {
  assert!(FlatCompiledStylesValue::Bool(true).is_true_bool());
  assert!(!FlatCompiledStylesValue::Bool(false).is_true_bool());
  assert_eq!(count_answering(StyleqValue::is_true_bool), 1);
}
