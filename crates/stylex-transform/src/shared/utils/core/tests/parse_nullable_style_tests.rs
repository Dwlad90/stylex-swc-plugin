//! Tests for the reading of one `stylex.props` argument into the style it
//! stands for.

use std::rc::Rc;

use indexmap::IndexMap;
use stylex_state::{
  evaluate_result_value::EvaluateResultValue, flat_compiled_styles_value::FlatCompiledStylesValue,
  functions::FunctionMap, state_manager::StateManager,
};
use swc_core::ecma::ast::Lit;

use super::{
  StyleObject, parse_compiled_styles, parse_nullable_literal, parse_nullable_object,
  parse_nullable_style,
};
use crate::tests::support::expr;

fn read(code: &str) -> StyleObject {
  parse_nullable_style(
    &expr(code),
    &mut StateManager::default(),
    &FunctionMap::default(),
  )
}

fn styles() -> IndexMap<String, Rc<FlatCompiledStylesValue>> {
  IndexMap::new()
}

/// The two ways an author writes "no style here" are the same answer, and the
/// merge skips both.
#[test]
fn reads_an_absent_argument_as_absent() {
  assert_eq!(read("null"), StyleObject::Nullable);
  assert_eq!(read("undefined"), StyleObject::Nullable);
}

/// A parenthesis is not a different argument, so the shape inside is what is
/// read.
#[test]
fn reads_through_a_parenthesis() {
  assert_eq!(read("(null)"), StyleObject::Nullable);
}

/// Anything the compiler cannot fold to a compiled style is handed to the
/// runtime, which is what `Other` says.
#[test]
fn hands_an_unreadable_argument_to_the_runtime() {
  assert_eq!(read("'a string'"), StyleObject::Other);
  assert_eq!(read("1"), StyleObject::Other);
  assert_eq!(read("name"), StyleObject::Other);
  assert_eq!(read("makeStyles()"), StyleObject::Other);
  assert_eq!(read("a + b"), StyleObject::Other);
}

/// An object the compiler folded is read as the style it spells, class names
/// and all.
#[test]
fn reads_a_folded_object_as_a_style() {
  match read("({ color: 'xa', $$css: true })") {
    StyleObject::Style(style) => {
      assert_eq!(
        style["color"].as_ref(),
        &FlatCompiledStylesValue::String("xa".to_owned())
      );
      assert_eq!(
        style["$$css"].as_ref(),
        &FlatCompiledStylesValue::Bool(true)
      );
    },
    other => panic!("the object was not read as a style: {other:?}"),
  }
}

/// A property written as absent is kept, carrying that absence, so a style
/// after it unsets the property rather than being shadowed by it.
#[test]
fn keeps_a_property_written_as_absent() {
  match read("({ color: null })") {
    StyleObject::Style(style) => {
      assert_eq!(style["color"].as_ref(), &FlatCompiledStylesValue::Null)
    },
    other => panic!("the object was not read as a style: {other:?}"),
  }
}

/// An array of styles is one style, merged in the order it was written.
#[test]
fn reads_an_array_of_objects_as_one_style() {
  let mut compiled = styles();

  let result = parse_compiled_styles(
    &mut compiled,
    &EvaluateResultValue::Vec(vec![
      EvaluateResultValue::Expr(expr("{ color: 'xa' }")),
      EvaluateResultValue::Expr(expr("{ margin: 'xb' }")),
    ]),
  );

  match result {
    Some(StyleObject::Style(style)) => {
      assert_eq!(
        style.keys().cloned().collect::<Vec<_>>(),
        ["color", "margin"]
      );
    },
    other => panic!("the array was not read as one style: {other:?}"),
  }
}

/// An array holding an array is flattened, because the runtime reads it the
/// same way.
#[test]
fn reads_a_nested_array_as_part_of_the_same_style() {
  let mut compiled = styles();

  let result = parse_compiled_styles(
    &mut compiled,
    &EvaluateResultValue::Vec(vec![EvaluateResultValue::Vec(vec![
      EvaluateResultValue::Expr(expr("{ color: 'xa' }")),
    ])]),
  );

  match result {
    Some(StyleObject::Style(style)) => assert_eq!(style.len(), 1),
    other => panic!("the nested array was not read as one style: {other:?}"),
  }
}

/// An absent element of an array declares nothing and is passed over.
#[test]
fn passes_over_an_absent_element_of_an_array() {
  let mut compiled = styles();

  let result = parse_compiled_styles(
    &mut compiled,
    &EvaluateResultValue::Vec(vec![
      EvaluateResultValue::Null,
      EvaluateResultValue::Expr(expr("{ color: 'xa' }")),
    ]),
  );

  match result {
    Some(StyleObject::Style(style)) => assert_eq!(style.len(), 1),
    other => panic!("the array was not read as one style: {other:?}"),
  }
}

/// An array that declares nothing is handed to the runtime rather than read as
/// an empty style, which would say the merge had something to apply.
#[test]
fn hands_an_array_that_declares_nothing_to_the_runtime() {
  assert_eq!(
    parse_compiled_styles(&mut styles(), &EvaluateResultValue::Vec(vec![])),
    Some(StyleObject::Other)
  );
}

/// A theme reference is not a style, and the runtime is what reads it.
#[test]
fn hands_a_theme_reference_to_the_runtime() {
  let theme_ref = EvaluateResultValue::ThemeRef(stylex_state::theme_ref::ThemeRef::new(
    "Theme.stylex.js",
    "vars",
    "x",
  ));

  assert_eq!(
    parse_compiled_styles(&mut styles(), &theme_ref),
    Some(StyleObject::Other)
  );
}

/// A folded value that is not an object says nothing about the style, so the
/// answer stays the one the caller had.
#[test]
fn says_nothing_about_a_folded_value_that_is_not_an_object() {
  assert_eq!(
    parse_compiled_styles(&mut styles(), &EvaluateResultValue::Expr(expr("1"))),
    None
  );
}

/// A class name, the compiled marker and an absent value are what a compiled
/// style holds, and each is read as the kind it is.
#[test]
fn reads_each_kind_of_value_a_compiled_style_holds() {
  let mut compiled = styles();

  parse_nullable_object(
    &mut compiled,
    &expr("{ color: 'xa', $$css: true, margin: null }"),
  );

  assert_eq!(
    compiled["color"].as_ref(),
    &FlatCompiledStylesValue::String("xa".to_owned())
  );
  assert_eq!(
    compiled["$$css"].as_ref(),
    &FlatCompiledStylesValue::Bool(true)
  );
  assert_eq!(compiled["margin"].as_ref(), &FlatCompiledStylesValue::Null);
}

/// An inline style is the object the author wrote, so a value keeps the kind
/// they gave it. A number stays a number and is not the text that spells it,
/// because `opacity: 0.5` and `opacity: '0.5'` are two declarations.
#[test]
fn reads_each_kind_of_value_an_inline_style_holds() {
  let mut inline = styles();

  parse_nullable_object(
    &mut inline,
    &expr("{ opacity: 0.5, color: true, ':hover': { color: 'blue', margin: null } }"),
  );

  assert_eq!(
    inline["opacity"].as_ref(),
    &FlatCompiledStylesValue::Number(0.5)
  );
  assert_eq!(
    inline["color"].as_ref(),
    &FlatCompiledStylesValue::Bool(true)
  );

  let Some(nested) = inline[":hover"].as_object() else {
    panic!("the pseudo-class does not hold an object: {:?}", inline);
  };

  assert_eq!(
    nested["color"].as_ref(),
    &FlatCompiledStylesValue::String("blue".to_owned())
  );
  assert_eq!(nested["margin"].as_ref(), &FlatCompiledStylesValue::Null);
}

/// An object holds objects as deep as the author wrote them.
#[test]
fn reads_an_object_inside_an_object() {
  let mut inline = styles();

  parse_nullable_object(
    &mut inline,
    &expr("{ ':hover': { ':focus': { color: 'red' } } }"),
  );

  let nested = inline[":hover"]
    .as_object()
    .and_then(|hover| hover[":focus"].as_object())
    .cloned();

  assert_eq!(
    nested.map(|values| values["color"].as_ref().clone()),
    Some(FlatCompiledStylesValue::String("red".to_owned()))
  );
}

/// An object holding nothing is read as an object holding nothing, rather than
/// as a declaration that was never written.
#[test]
fn reads_an_object_holding_nothing() {
  let mut inline = styles();

  parse_nullable_object(&mut inline, &expr("{ ':hover': {} }"));

  assert_eq!(inline[":hover"].as_object().map(IndexMap::len), Some(0));
}

/// A name written twice keeps the place of its first writing and the value of
/// its last, the way the language reads such an object.
#[test]
fn keeps_a_repeated_name_in_its_first_place() {
  let mut inline = styles();

  parse_nullable_object(
    &mut inline,
    &expr("{ opacity: 0.5, color: 'red', opacity: 1 }"),
  );

  assert_eq!(
    inline.keys().cloned().collect::<Vec<_>>(),
    ["opacity", "color"]
  );
  assert_eq!(
    inline["opacity"].as_ref(),
    &FlatCompiledStylesValue::Number(1.0)
  );
}

/// A property that is not a key-value pair declares nothing to read.
#[test]
fn passes_over_a_property_that_is_not_a_key_value_pair() {
  let mut compiled = styles();

  parse_nullable_object(&mut compiled, &expr("{ ...rest, color: 'xa' }"));

  assert_eq!(compiled.keys().cloned().collect::<Vec<_>>(), ["color"]);
}

/// A declaration holds a literal or an object. Anything else was never
/// evaluated, and reading it as a class name would write the wrong class.
#[test]
#[should_panic(
  expected = "Encountered an unsupported expression type while parsing a nullable style array."
)]
fn refuses_a_value_that_is_neither_a_literal_nor_an_object() {
  parse_nullable_object(&mut styles(), &expr("{ color: name }"));
}

#[test]
#[should_panic(
  expected = "Encountered an unsupported expression type while parsing a nullable style array."
)]
fn refuses_a_compiled_style_that_is_not_an_object() {
  parse_nullable_object(&mut styles(), &expr("[1, 2]"));
}

/// A big integer is not text, a number, a boolean or an absence, so it is none
/// of the kinds a declaration can hold.
#[test]
#[should_panic(expected = "Unhandled literal type in nullable style parsing array")]
fn refuses_a_literal_a_declaration_cannot_hold() {
  let lit = match expr("1n") {
    swc_core::ecma::ast::Expr::Lit(lit) => lit,
    other => panic!("the fixture is not a literal: {other:?}"),
  };

  parse_nullable_literal(&lit);
}

/// A string holding half a surrogate pair crosses and comes back, because the
/// text is read as it stands rather than through a converter that spells it.
#[test]
fn reads_a_class_name_holding_half_a_surrogate_pair() {
  let lit = Lit::Str(swc_core::ecma::ast::Str {
    span: swc_core::common::DUMMY_SP,
    value: "xa".into(),
    raw: None,
  });

  assert_eq!(
    parse_nullable_literal(&lit),
    FlatCompiledStylesValue::String("xa".to_owned())
  );
}

/// Only a style, an array of styles and an absent value can stand in an array
/// of style arguments. Anything else means the argument was not compiled, and
/// merging it would write a class the author never asked for.
#[test]
#[should_panic(
  expected = "Encountered an unsupported evaluation result while parsing a nullable style array."
)]
fn refuses_an_array_element_that_is_not_a_style() {
  parse_compiled_styles(
    &mut styles(),
    &EvaluateResultValue::Vec(vec![EvaluateResultValue::Map(IndexMap::new())]),
  );
}

#[test]
#[should_panic(
  expected = "Encountered an unsupported evaluation result while parsing a nullable style."
)]
fn refuses_a_folded_value_that_is_not_a_style() {
  parse_compiled_styles(&mut styles(), &EvaluateResultValue::Map(IndexMap::new()));
}
