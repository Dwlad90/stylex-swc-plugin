use super::*;
use stylex_ast::ast::convertors::create_null_expr;
use swc_core::common::DUMMY_SP;

fn bin(op: BinaryOp, left: Expr, right: Expr) -> BinExpr {
  BinExpr {
    span: DUMMY_SP,
    op,
    left: Box::new(left),
    right: Box::new(right),
  }
}

fn fold_expr(op: BinaryOp, left: Expr, right: Expr) -> (Option<Expr>, Option<String>) {
  let mut state = EvaluationState::new();
  let mut traversal_state = StateManager::default();
  let fns = FunctionMap::default();

  let logical_op = match LogicalOp::of(op) {
    Some(logical_op) => logical_op,
    None => panic!("{:?} is not a logical operator", op),
  };

  let result = evaluate(
    logical_op,
    &bin(op, left, right),
    &mut state,
    &mut traversal_state,
    &fns,
  );

  let value = result.and_then(|value| value.as_expr().cloned());

  (value, state.deopt_reason)
}

fn folded_string(op: BinaryOp, left: Expr, right: Expr) -> String {
  match fold_expr(op, left, right) {
    (Some(Expr::Lit(Lit::Str(strng))), _) => convert_atom_to_string(&strng.value),
    (value, reason) => panic!("expected a folded string, got {:?} ({:?})", value, reason),
  }
}

#[test]
fn nullish_takes_the_right_side_for_null() {
  assert_eq!(
    folded_string(
      BinaryOp::NullishCoalescing,
      create_null_expr(),
      create_string_expr("red"),
    ),
    "red"
  );
}

#[test]
fn nullish_takes_the_right_side_for_undefined() {
  assert_eq!(
    folded_string(
      BinaryOp::NullishCoalescing,
      create_ident_expr("undefined"),
      create_string_expr("red"),
    ),
    "red"
  );
}

#[test]
fn nullish_keeps_a_left_side_that_is_neither() {
  assert_eq!(
    folded_string(
      BinaryOp::NullishCoalescing,
      create_string_expr("blue"),
      create_string_expr("red"),
    ),
    "blue"
  );
}

/// Upstream's nullish guard tests the left side's truthiness rather than its
/// nullishness, so a left side that is falsy but present refuses to fold. The
/// restriction is inherited rather than corrected: folding here where upstream
/// does not would be a silent CSS difference between two builds of the same
/// source.
#[test]
fn nullish_refuses_a_falsy_left_side_that_is_not_nullish() {
  for left in [
    create_number_expr(0.0),
    create_bool_expr(false),
    create_string_expr(""),
  ] {
    let (value, reason) = fold_expr(
      BinaryOp::NullishCoalescing,
      left.clone(),
      create_string_expr("red"),
    );

    assert!(value.is_none(), "expected {:?} ?? 'red' to refuse", left);
    assert_eq!(reason.as_deref(), Some("unknown error"));
  }
}

#[test]
fn or_takes_the_left_side_when_it_is_truthy() {
  assert_eq!(
    folded_string(
      BinaryOp::LogicalOr,
      create_string_expr("blue"),
      create_string_expr("red"),
    ),
    "blue"
  );
}

#[test]
fn or_takes_the_right_side_when_the_left_is_falsy() {
  assert_eq!(
    folded_string(
      BinaryOp::LogicalOr,
      create_string_expr(""),
      create_string_expr("red"),
    ),
    "red"
  );
}

#[test]
fn and_takes_the_right_side_when_the_left_is_truthy() {
  assert_eq!(
    folded_string(
      BinaryOp::LogicalAnd,
      create_string_expr("blue"),
      create_string_expr("red"),
    ),
    "red"
  );
}

/// A falsy confident left side is returned as it is — upstream returns the same
/// value, and what an empty value means for the surrounding declaration is
/// decided downstream.
#[test]
fn and_returns_a_falsy_left_side_verbatim() {
  assert_eq!(
    folded_string(
      BinaryOp::LogicalAnd,
      create_string_expr(""),
      create_string_expr("red"),
    ),
    ""
  );
}

/// The winning operand is not normalised on its way out: an object stays the
/// object it was.
#[test]
fn the_winning_operand_keeps_its_own_shape() {
  let (value, reason) = fold_expr(
    BinaryOp::NullishCoalescing,
    create_null_expr(),
    create_object_expression(vec![]),
  );

  assert!(
    matches!(value, Some(Expr::Object(_))),
    "expected the object operand back, got {:?} ({:?})",
    value,
    reason
  );
}

/// An unresolvable side the fold never consults cannot deopt the expression:
/// the truthy left side wins on its own.
#[test]
fn an_unresolvable_right_side_does_not_deopt_a_decided_or() {
  assert_eq!(
    folded_string(
      BinaryOp::LogicalOr,
      create_string_expr("blue"),
      create_ident_expr("unknown"),
    ),
    "blue"
  );
}

/// An unresolvable side the fold does need deopts with that side's own reason.
#[test]
fn an_unresolvable_right_side_deopts_an_undecided_or() {
  let (value, reason) = fold_expr(
    BinaryOp::LogicalOr,
    create_string_expr(""),
    create_ident_expr("unknown"),
  );

  assert!(value.is_none(), "expected the expression to deopt");
  assert!(reason.is_some(), "expected a deopt reason");
}

/// Only the three the reference implementation puts on a node of their own are
/// recognised; every other binary operator belongs to the paths that coerce.
#[test]
fn only_the_three_logical_operators_are_recognised() {
  assert!(LogicalOp::of(BinaryOp::LogicalOr).is_some());
  assert!(LogicalOp::of(BinaryOp::LogicalAnd).is_some());
  assert!(LogicalOp::of(BinaryOp::NullishCoalescing).is_some());

  for op in [BinaryOp::Add, BinaryOp::Sub, BinaryOp::EqEqEq, BinaryOp::In] {
    assert!(
      LogicalOp::of(op).is_none(),
      "expected {:?} not to be a logical operator",
      op
    );
  }
}
