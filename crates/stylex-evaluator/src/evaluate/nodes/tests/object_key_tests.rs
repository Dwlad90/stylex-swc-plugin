//! The string an object key names, whichever way it was written.
//!
//! `evaluate_obj_key` is what the transform above asks for the *name* of a
//! property, and every spelling a key can have has to answer the same string
//! the language names the property with -- a key read one way here and another
//! where the object itself is folded is how one property becomes two rules.
//!
//! Asked directly rather than through a folded object, because this is the
//! entry point the transform calls and its refusals are answers rather than
//! deopts recorded on a state.

use crate::evaluate::evaluate_obj_key;
use crate::evaluate::source_evaluation::*;
use stylex_constants::constants::messages::{
  EXPRESSION_IS_NOT_A_STRING, ILLEGAL_PROP_VALUE, KEY_IS_NOT_A_STRING,
};
use stylex_state::{
  evaluate_result_value::EvaluateResultValue, functions::FunctionMap, state_manager::StateManager,
};
use stylex_structures::stylex_options::StyleXOptions;
use swc_core::{
  common::{DUMMY_SP, GLOBALS, Globals},
  ecma::ast::{BigInt, ComputedPropName, Expr, IdentName, KeyValueProp, PropName},
};

use stylex_ast::ast::convertors::{convert_atom_to_string, create_number_expr, create_string_expr};

/// The key `name` names, or the refusal it answered.
fn key_of(name: PropName) -> Result<String, Option<String>> {
  let globals = Globals::new();

  GLOBALS.set(&globals, || {
    let mut state = StateManager::new(StyleXOptions::default());
    let result = evaluate_obj_key(
      &KeyValueProp {
        key: name,
        value: Box::new(create_number_expr(1.0)),
      },
      &mut state,
      &FunctionMap::default(),
    );

    match (result.confident, result.value) {
      (true, Some(EvaluateResultValue::Expr(Expr::Lit(literal)))) => match literal.as_str() {
        Some(text) => Ok(convert_atom_to_string(&text.value)),
        None => panic!("expected a string key, got {:?}", literal),
      },
      (true, other) => panic!("expected a string key, got {:?}", other),
      (false, _) => Err(result.reason),
    }
  })
}

/// A key written as a big-integer literal, which has no other spelling.
fn big_int_key(digits: u64) -> PropName {
  PropName::BigInt(BigInt {
    span: DUMMY_SP,
    value: Box::new(digits.into()),
    raw: None,
  })
}

fn computed(expr: Expr) -> PropName {
  PropName::Computed(ComputedPropName {
    span: DUMMY_SP,
    expr: Box::new(expr),
  })
}

/// The three spellings that name a key without evaluating anything: an
/// identifier, a string, and a number. The number is rendered as JavaScript
/// spells it rather than as Rust does.
#[test]
fn a_written_key_names_itself() {
  assert_eq!(
    key_of(PropName::Ident(IdentName::new("color".into(), DUMMY_SP))),
    Ok(String::from("color"))
  );
  assert_eq!(
    key_of(PropName::Str("font-size".into())),
    Ok(String::from("font-size"))
  );
  assert_eq!(
    key_of(PropName::Num(1e21.into())),
    Ok(String::from("1e+21"))
  );
}

/// A computed key is the string its expression folds to.
#[test]
fn a_computed_key_is_what_it_folds_to() {
  assert_eq!(
    key_of(computed(create_string_expr("color"))),
    Ok(String::from("color"))
  );
  assert_eq!(
    key_of(computed(create_number_expr(2.0))),
    Ok(String::from("2"))
  );
}

/// The three numbers with no digits of their own name themselves, and negative
/// zero names the same property positive zero does.
///
/// Each is a number `f64::to_string` spells differently from the language:
/// `NaN` and `inf` rather than `NaN` and `Infinity`, and `-0` rather than `0`.
/// A key is a CSS property name, so a spelling of the compiler's own would
/// declare a property no browser was asked for.
#[test]
fn the_numbers_with_no_digits_name_themselves() {
  for (value, expected) in [
    (f64::NAN, "NaN"),
    (f64::INFINITY, "Infinity"),
    (f64::NEG_INFINITY, "-Infinity"),
    (-0.0, "0"),
    (0.0, "0"),
  ] {
    assert_eq!(
      key_of(computed(create_number_expr(value))),
      Ok(String::from(expected)),
      "wrong key for the number {}",
      expected
    );
  }
}

/// A computed key whose expression resolves to nothing carries that
/// expression's own refusal, so the sentence names what the author wrote.
#[test]
fn a_computed_key_that_resolves_to_nothing_carries_its_own_refusal() {
  let refusal = key_of(computed(parse_expr("unknownName")));

  assert!(
    matches!(&refusal, Err(Some(reason)) if reason.contains("not defined")),
    "expected the name's own refusal, got {:?}",
    refusal
  );
}

/// A computed key that folded to a value with no expression form is a key this
/// does not read -- an ordinary refusal rather than a broken invariant.
#[test]
fn a_computed_key_with_no_expression_form_refuses() {
  assert_eq!(
    key_of(computed(parse_expr("[1, 2]"))),
    Err(Some(ILLEGAL_PROP_VALUE.to_string()))
  );
}

/// A computed key is the string its expression names, so an expression with no
/// string form names no key and the object refuses. An object literal is such
/// an expression: it evaluates, and then has no key spelling to give.
#[test]
fn a_computed_key_with_no_string_form_refuses() {
  assert_deopt_reason_contains("({ [{}]: 'x' })", EXPRESSION_IS_NOT_A_STRING);
}

/// A key written as a big-integer literal names its digits, which is what the
/// language names the property: `1n` and `1` are the same key. Pinned against
/// the reference implementation, which declares `1: red` for `{ 1n: 'red' }`.
#[test]
fn a_big_integer_key_names_its_digits() {
  assert_eq!(key_of(big_int_key(1)), Ok(String::from("1")));
  assert_eq!(key_of(big_int_key(0)), Ok(String::from("0")));
  assert_eq!(
    key_of(big_int_key(u64::MAX)),
    Ok(String::from("18446744073709551615")),
    "a value past what a number holds is still written out in full"
  );
}

/// The same key written in source, so the spelling and the entry point agree.
#[test]
fn a_big_integer_key_written_in_source_names_the_same_string() {
  assert_folds_to_object_keys("({ 1n: 'x' })", &["1"]);
  // An integer key is ordered ahead of a written one, which is the order the
  // language gives own keys and the order the reference implementation emits
  // the two rules in.
  assert_folds_to_object_keys("({ color: 'red', 2n: 'blue' })", &["2", "color"]);
}

/// A computed key that folds to a value this compiler writes no string for
/// refuses. It says so as a key rather than as a value, because the key is the
/// half the author changes.
///
/// Five spellings, and each of them is a boolean, `null` or an object. The
/// reference implementation names the property `String(key)` instead, so
/// `{ [true]: 'red' }` declares `true: red` there. Recorded rather than changed
/// here, because the coercion decides a CSS property name -- ticket 49 of
/// `.scratch/split-transform-crate` settles which answer each key gets.
#[test]
fn a_computed_key_that_names_no_string_refuses_as_a_key() {
  for source in ["true", "false", "null", "({})", "!1"] {
    assert_eq!(
      key_of(computed(parse_expr(source))),
      Err(Some(KEY_IS_NOT_A_STRING.to_string())),
      "wrong refusal for the computed key `{}`",
      source
    );
  }
}

/// A comparison read as a key names `0` here and `false` in the language, so
/// the two compilers write two different property names for one source, with no
/// error either side.
///
/// Recorded rather than endorsed. A comparison is folded through the numeric
/// reading of a binary expression before the key is asked for a string, which
/// is where the `0` comes from. Pinned so the answer changes visibly when the
/// key coercion is settled by ticket 49 of `.scratch/split-transform-crate`.
#[test]
fn a_comparison_read_as_a_key_names_the_number_it_folded_through() {
  assert_eq!(key_of(computed(parse_expr("1 > 2"))), Ok(String::from("0")));
  assert_eq!(key_of(computed(parse_expr("2 > 1"))), Ok(String::from("1")));
}

// ==================== a key written twice ====================

/// A key written twice is one property carrying the later value, however the
/// second spelling arrived.
///
/// Three routes reach the same set -- a second literal property, a spread that
/// carries the key, and a computed key that folds to it -- and the fold reads
/// them at three different moments. The language answers all three the same
/// way, so a route that kept both would write a property the source does not
/// describe, and a route that kept the first value would write the wrong one.
#[test]
fn a_key_written_twice_keeps_one_property_and_the_later_value() {
  for source in [
    "({ a: 1, a: 2 })",
    "({ a: 1, ...{ a: 2 } })",
    "({ a: 1, ['a']: 2 })",
  ] {
    assert_folds_to_object_keys(source, &["a"]);
    assert_folds_to_number(&format!("({source}).a"), 2.0);
  }
}

/// The empty string is a key like any other, and an array-index key sorts ahead
/// of it -- which is the order the language answers and the order a class name
/// is hashed from.
#[test]
fn the_empty_string_is_a_key_and_an_index_sorts_ahead_of_it() {
  assert_folds_to_object_keys("({ '': 1, 0: 2 })", &["0", ""]);
  assert_folds_to_object_keys("({ 0: 1, '': 2, a: 3 })", &["0", "", "a"]);

  assert_folds_to_number("({ '': 1, 0: 2 })['']", 1.0);
  assert_folds_to_number("({ '': 1, 0: 2 })[0]", 2.0);
}
