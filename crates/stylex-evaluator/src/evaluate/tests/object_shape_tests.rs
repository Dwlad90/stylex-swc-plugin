//! What an object literal folds to, key shape by key shape.
//!
//! Every key an object can be written with becomes a string, and the *same*
//! reader decides whether two keys collide -- so a spelling read one way here
//! and another there is how one property comes to be two in the stylesheet.
//! That is why a number key is rendered as JavaScript spells a number rather
//! than as Rust does: `{ 1e21: x }` names the property `"1e+21"`.
//!
//! Every case is read past a declined fold, because an object the engine can
//! read is folded there. `sx.missing ?? <object>` is that shape: the engine
//! declines the name it cannot carry, and the object behind the `??` is what
//! this evaluator then folds.

use super::source_evaluation::*;
use stylex_constants::constants::evaluation_errors::UNDEFINED_CONST;
use stylex_constants::constants::messages::{ILLEGAL_PROP_ARRAY_VALUE, ILLEGAL_PROP_VALUE};

/// The keys `object` folds to, joined, read past a declined fold.
#[track_caller]
fn keys_of(object: &str) -> String {
  let source = format!("Object.keys(sx.missing ?? ({object}))[0]");

  folded_text_of(evaluated_against_a_function_fold(&source), &source)
}

/// How many own keys `object` folds to.
#[track_caller]
fn key_count_of(object: &str) -> f64 {
  let source = format!("Object.keys(sx.missing ?? ({object})).length");

  match folded_value_of(evaluated_against_a_function_fold(&source), &source)
    .as_expr()
    .and_then(|expr| expr.as_lit())
  {
    Some(swc_core::ecma::ast::Lit::Num(number)) => number.value,
    other => panic!("expected `{}` to count, got {:?}", source, other),
  }
}

#[track_caller]
fn assert_object_refuses(object: &str, reason: &str) {
  let source = format!("Object.keys(sx.missing ?? ({object})).length");

  assert_refused_with(&evaluated_against_a_function_fold(&source), &source, reason);
}

/// A number key is rendered as JavaScript spells a number. Rust's own
/// formatting would name `1e21` as its full digits, which is a different
/// property from the one the source wrote.
#[test]
fn a_number_key_is_spelled_as_the_language_spells_it() {
  assert_eq!(keys_of("{ 1e21: 'a' }"), "1e+21");
  assert_eq!(keys_of("{ 1: 'a' }"), "1");
  assert_eq!(keys_of("{ 0.5: 'a' }"), "0.5");
}

/// A bigint key is its digits, which is what the language names the property.
#[test]
fn a_bigint_key_is_its_digits() {
  assert_eq!(keys_of("{ 1n: 'a' }"), "1");
}

/// A computed key is the string its expression folds to, so a key built by
/// concatenation names the property the concatenation spells.
#[test]
fn a_computed_key_is_what_its_expression_folds_to() {
  assert_eq!(keys_of("{ ['a' + 'b']: 'c' }"), "ab");
  assert_eq!(keys_of("{ [1 + 1]: 'c' }"), "2");
}

/// A computed key whose expression resolves to nothing refuses for that
/// expression's own reason, and one that folds to a value with no string form
/// refuses for the value.
#[test]
fn a_computed_key_that_names_no_string_refuses() {
  assert_object_refuses("{ [unknownName]: 'c' }", UNDEFINED_CONST);
  assert_object_refuses("{ [own]: 'c' }", ILLEGAL_PROP_VALUE);
}

/// A value that resolves to nothing refuses, and the sentence names the key it
/// was written under -- which is the half of the object an author has to find.
#[test]
fn a_value_that_resolves_to_nothing_names_the_key_it_sits_under() {
  assert_object_refuses(
    "{ a: unknownName }",
    "a > Referenced constant is not defined.",
  );
}

/// An array value holding something with no compile-time form refuses, and
/// says what a style array may hold.
#[test]
fn an_array_value_the_evaluator_cannot_write_down_refuses() {
  assert_object_refuses("{ a: [own] }", ILLEGAL_PROP_ARRAY_VALUE);
}

/// One of the compiler's own functions written as a value folds to the object
/// it stands for, rather than refusing: it is an object upstream, and a
/// declaration reading a key off it has to see the same keys.
#[test]
fn one_of_the_compilers_functions_written_as_a_value_folds_to_its_object() {
  assert_eq!(key_count_of("{ a: own }"), 1.0);
  assert_eq!(key_count_of("{ a: sx }"), 1.0);
}

/// A spread contributes the operand's own keys, whatever kind the operand is:
/// the compiler's own fold, a string, an array, and a value with no own keys at
/// all.
#[test]
fn a_spread_contributes_the_operands_own_keys() {
  assert_eq!(key_count_of("{ ...sx }"), 1.0);
  assert_eq!(key_count_of("{ ...own }"), 1.0);
  assert_eq!(key_count_of("{ ...'ab' }"), 2.0);
  assert_eq!(key_count_of("{ ...[1, 2] }"), 2.0);
  assert_eq!(key_count_of("{ ...1 }"), 0.0);
}
