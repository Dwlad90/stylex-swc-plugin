use super::*;
use crate::shared::structures::types::FunctionConfigMap;
use std::rc::Rc;
use stylex_ast::ast::convertors::{create_ident_expr, create_null_expr};
use stylex_structures::fold_ceilings::MAX_FOLDED_CHARACTERS_LIMIT;
use swc_core::{
  common::DUMMY_SP,
  ecma::ast::{UnaryExpr, UnaryOp},
};

fn void_expr(arg: Expr) -> Expr {
  Expr::Unary(UnaryExpr {
    span: DUMMY_SP,
    op: UnaryOp::Void,
    arg: Box::new(arg),
  })
}

#[test]
fn the_nullish_bridge_answers_for_the_three_spellings_of_nullish() {
  assert!(evaluate_result_is_nullish(&EvaluateResultValue::Expr(
    create_null_expr()
  )));
  assert!(evaluate_result_is_nullish(&EvaluateResultValue::Expr(
    create_ident_expr("undefined")
  )));
  assert!(evaluate_result_is_nullish(&EvaluateResultValue::Expr(
    void_expr(create_string_expr("red"))
  )));
}

#[test]
fn the_nullish_bridge_answers_for_the_absent_value() {
  assert!(evaluate_result_is_nullish(&EvaluateResultValue::Null));
}

#[test]
fn the_nullish_bridge_refuses_the_falsy_values_that_are_not_nullish() {
  assert!(!evaluate_result_is_nullish(&EvaluateResultValue::Expr(
    create_string_expr("")
  )));
  assert!(!evaluate_result_is_nullish(&EvaluateResultValue::Expr(
    create_number_expr(0.0)
  )));
  assert!(!evaluate_result_is_nullish(&EvaluateResultValue::Expr(
    create_bool_expr(false)
  )));
}

#[test]
fn the_nullish_bridge_answers_no_for_the_evaluator_s_own_variants() {
  assert!(!evaluate_result_is_nullish(&EvaluateResultValue::Vec(
    vec![]
  )));
  assert!(!evaluate_result_is_nullish(&EvaluateResultValue::Map(
    IndexMap::default()
  )));
  assert!(!evaluate_result_is_nullish(
    &EvaluateResultValue::EnvObject(IndexMap::default().into())
  ));
  assert!(!evaluate_result_is_nullish(
    &EvaluateResultValue::FunctionConfigMap(FunctionConfigMap::default())
  ));
}

/// The two coercions that have to tell an object from a function once said
/// different things about a folded function map, and a template that
/// interpolated the namespace refused because of it. They are separate
/// exhaustive matches by design -- so that a new variant cannot be added
/// without classifying it in both -- and this is what stops the two
/// classifications from drifting apart again.
#[test]
fn the_two_bridges_agree_a_function_map_is_an_object() {
  let map = EvaluateResultValue::FunctionConfigMap(FunctionConfigMap::default());

  assert_eq!(
    evaluate_result_to_js_string(&map).as_deref(),
    Some(coercions::OBJECT_TO_STRING),
    "the string bridge must give a function map the object default"
  );

  assert!(
    matches!(
      evaluate_result_to_js_object(&map),
      Some(coercions::ObjectCoercion::Object)
    ),
    "the object bridge must read a function map as an object, not as a function"
  );
}

/// The other two variants of the family are functions, and both bridges have to
/// say so. `Refuse` is the form the style-value consumers use, and a function
/// under it has no string at all.
#[test]
fn the_two_bridges_agree_a_callback_is_a_function() {
  let callback = EvaluateResultValue::Callback(Rc::new(|_args, _fns| create_null_expr()));

  assert_eq!(
    evaluate_result_to_js_string(&callback),
    None,
    "a function has no compile-time string under the refusing form"
  );

  assert!(
    matches!(
      evaluate_result_to_js_object(&callback),
      Some(coercions::ObjectCoercion::Function)
    ),
    "the object bridge must read a callback as a function"
  );
}

/// The whole of the character ceiling's arithmetic, tested without a compile:
/// the boundary in both directions, and what a "character" is counted as.
#[test]
fn the_character_ceiling_admits_exactly_the_ceiling() {
  assert_eq!(units_within(0, "abcd", 4), Some(4));
  assert_eq!(units_within(4, "efgh", 8), Some(8));
  assert_eq!(units_within(4, "efghi", 8), None);

  // An empty append grows nothing, so it can never be the piece that refuses --
  // not even against a buffer already sitting exactly on the ceiling.
  assert_eq!(units_within(8, "", 8), Some(8));
}

/// Counted in UTF-16 code units, which is the length JavaScript reports. An
/// astral character occupies two of them and spells as four bytes, so neither the
/// scalar count nor the byte count would answer here.
#[test]
fn the_character_ceiling_counts_code_units() {
  assert_eq!(units_within(0, "\u{1F600}", 2), Some(2));
  assert_eq!(units_within(0, "\u{1F600}", 1), None);
  assert_eq!(units_within(2, "\u{1F600}", 4), Some(4));
  assert_eq!(units_within(2, "\u{1F600}", 3), None);

  // Two bytes, one code unit — the direction a byte count would get wrong the
  // other way, by refusing a string the ceiling allows.
  assert_eq!(units_within(0, "é", 1), Some(1));
}

/// The sum exists to be refused on, so it saturates: a wrapped one would come
/// back small and admit the very append it was asked about. Measured against the
/// largest ceiling a project can ask for, because that is what the clamped option
/// can actually be -- a saturated sum only has to beat *that*, and no buffer
/// holding `usize::MAX` code units is reachable through a compile, which is why
/// this is asserted here at all.
#[test]
fn the_character_ceiling_refuses_rather_than_wrapping() {
  assert_eq!(
    units_within(usize::MAX, "x", MAX_FOLDED_CHARACTERS_LIMIT),
    None
  );
  assert_eq!(
    units_within(usize::MAX - 1, "xx", MAX_FOLDED_CHARACTERS_LIMIT),
    None
  );
}

/// A ceiling of zero reaches this arithmetic only if something upstream let it:
/// `Ceiling::clamped` reads an unset option as the default, so what the compiler
/// spends is never zero. Pinned anyway, because the arithmetic is the layer that
/// would silently refuse everything if that ever changed.
#[test]
fn a_zero_ceiling_admits_only_an_empty_append() {
  assert_eq!(units_within(0, "", 0), Some(0));
  assert_eq!(units_within(0, "x", 0), None);
}
