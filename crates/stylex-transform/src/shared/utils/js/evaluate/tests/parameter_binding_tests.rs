//! Which arguments an author's own arrow can bind a parameter to.
//!
//! One question asked in two places — the call that refuses an argument and the
//! binding that reads it — so the cases here are about the pair agreeing. A
//! value one accepted and the other dropped would leave the parameter unbound
//! and hand the arrow's body back unevaluated, which reaches an author as an
//! internal note rather than as a sentence about the call they wrote.

use indexmap::IndexMap;

use crate::shared::{
  enums::data_structures::evaluate_result_value::EvaluateResultValue,
  structures::{
    functions::{FunctionConfig, FunctionType},
    theme_ref::ThemeRef,
  },
  utils::js::evaluate::{binds_a_parameter, evaluate_result_as_expr},
};

use stylex_ast::ast::convertors::{create_null_expr, create_number_expr, create_string_expr};

fn theme_ref() -> ThemeRef {
  ThemeRef::new("vars.stylex.js", "vars", "x")
}

/// The ordinary case: a value with an expression form binds as that expression.
#[test]
fn a_value_that_writes_itself_down_binds() {
  for value in [
    EvaluateResultValue::Expr(create_string_expr("red")),
    EvaluateResultValue::Expr(create_number_expr(1.0)),
    EvaluateResultValue::Expr(create_null_expr()),
    EvaluateResultValue::Vec(vec![EvaluateResultValue::Expr(create_string_expr("a"))]),
    EvaluateResultValue::Vec(vec![]),
  ] {
    assert!(binds_a_parameter(&value));
  }
}

/// A theme reference has no expression form and binds anyway, through the
/// factory a module's own token import binds through — which is the whole of why
/// this question is not `evaluate_result_as_expr(..).is_some()`.
#[test]
fn a_theme_reference_binds_without_an_expression_form() {
  let value = EvaluateResultValue::ThemeRef(theme_ref());

  assert!(evaluate_result_as_expr(&value).is_none());
  assert!(binds_a_parameter(&value));
}

/// A theme reference nested inside an array does *not* bind, because the array
/// binds as one expression and a reference has none to contribute. Refusing is
/// the honest answer: the parameter would otherwise hold an array with a hole
/// where the source wrote a value.
#[test]
fn a_theme_reference_inside_an_array_does_not_bind() {
  let value = EvaluateResultValue::Vec(vec![EvaluateResultValue::ThemeRef(theme_ref())]);

  assert!(!binds_a_parameter(&value));
}

/// A function has no form to bind either way. The reference compiler folds one
/// as its source text and this compiler keeps none, so the call refuses rather
/// than binding a parameter to nothing.
#[test]
fn a_function_does_not_bind() {
  let config = FunctionConfig {
    fn_ptr: FunctionType::Callback(Box::new(create_null_expr())),
    takes_path: false,
  };

  assert!(!binds_a_parameter(&EvaluateResultValue::FunctionConfig(
    config
  )));
}

/// The remaining values the evaluator has of its own. None writes an expression
/// down, and none has a second form the binding could read, so each refuses.
#[test]
fn the_evaluators_own_values_do_not_bind() {
  assert!(!binds_a_parameter(&EvaluateResultValue::Map(
    IndexMap::default()
  )));
  assert!(!binds_a_parameter(&EvaluateResultValue::Null));
}
