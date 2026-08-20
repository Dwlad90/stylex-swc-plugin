use super::*;
use stylex_ast::ast::convertors::{create_ident_expr, create_null_expr};
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
  assert!(!evaluate_result_is_nullish(&EvaluateResultValue::Entries(
    IndexMap::default()
  )));
  assert!(!evaluate_result_is_nullish(
    &EvaluateResultValue::EnvObject(IndexMap::default())
  ));
  assert!(!evaluate_result_is_nullish(
    &EvaluateResultValue::FunctionConfigMap(IndexMap::default())
  ));
}
