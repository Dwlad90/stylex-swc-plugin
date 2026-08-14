//! Coverage for the two paths the binary-expression node folds through, at the
//! operators real StyleX source cannot reach — comparison, bitwise, `in` and
//! `instanceof`. Everything an author can write is pinned from above, on the
//! `stylex.create` fixtures.

use super::*;
use stylex_ast::ast::convertors::create_ident_expr;

/// The expression under test. Built here rather than at each case so a case
/// reads as its operator and its operands, and nothing else.
fn bin_expr(op: BinaryOp, left: Expr, right: Expr) -> BinExpr {
  BinExpr {
    span: Default::default(),
    op,
    left: Box::new(left),
    right: Box::new(right),
  }
}

/// The number-or-string path, against state of its own.
fn num_or_str_path(bin: &BinExpr) -> Result<BinaryExprType, anyhow::Error> {
  let mut state = EvaluationState::new();
  let mut traversal_state = StateManager::default();
  let fns = FunctionMap::default();

  binary_expr_to_num_or_str(bin, &mut state, &mut traversal_state, &fns)
}

/// The string path, against state of its own.
fn string_path(bin: &BinExpr) -> Result<BinaryExprType, anyhow::Error> {
  let mut state = EvaluationState::new();
  let mut traversal_state = StateManager::default();
  let fns = FunctionMap::default();

  binary_expr_to_string(bin, &mut state, &mut traversal_state, &fns)
}

/// The number a path folded to, or a failure naming what it answered instead.
///
/// Matched rather than unwrapped so a path that refused, or that answered with
/// a string, says which of the two it did — an unwrapped `Err` reports only
/// that something was not `Ok`.
#[track_caller]
fn expect_number(result: Result<BinaryExprType, anyhow::Error>) -> f64 {
  match result {
    Result::Ok(BinaryExprType::Number(number)) => number,
    Result::Ok(other) => panic!("expected a number, folded to {:?}", other),
    Result::Err(error) => panic!("expected a number, refused with {}", error),
  }
}

/// The string a path folded to, on the same terms as [`expect_number`].
#[track_caller]
fn expect_string(result: Result<BinaryExprType, anyhow::Error>) -> String {
  match result {
    Result::Ok(BinaryExprType::String(strng)) => strng,
    Result::Ok(other) => panic!("expected a string, folded to {:?}", other),
    Result::Err(error) => panic!("expected a string, refused with {}", error),
  }
}

/// One operator over two numeric operands, through the number-or-string path.
#[track_caller]
fn fold_numbers(op: BinaryOp, left: f64, right: f64) -> f64 {
  expect_number(num_or_str_path(&bin_expr(
    op,
    create_number_expr(left),
    create_number_expr(right),
  )))
}

/// `+` over two operands, through the string path.
#[track_caller]
fn concatenate(left: Expr, right: Expr) -> String {
  expect_string(string_path(&bin_expr(BinaryOp::Add, left, right)))
}

mod the_number_path {
  use super::*;

  #[test]
  fn addition_adds() {
    assert_eq!(fold_numbers(BinaryOp::Add, 10.0, 2.0), 12.0);
  }

  #[test]
  fn subtraction_subtracts() {
    assert_eq!(fold_numbers(BinaryOp::Sub, 10.0, 2.0), 8.0);
  }

  #[test]
  fn multiplication_multiplies() {
    assert_eq!(fold_numbers(BinaryOp::Mul, 10.0, 2.0), 20.0);
  }

  #[test]
  fn division_divides() {
    assert_eq!(fold_numbers(BinaryOp::Div, 10.0, 2.0), 5.0);
  }

  #[test]
  fn remainder_takes_the_modulus() {
    assert_eq!(fold_numbers(BinaryOp::Mod, 10.0, 2.0), 0.0);
    assert_eq!(fold_numbers(BinaryOp::Mod, 10.0, 3.0), 1.0);
  }

  #[test]
  fn exponentiation_raises_to_the_power() {
    assert_eq!(fold_numbers(BinaryOp::Exp, 10.0, 2.0), 100.0);
    assert_eq!(fold_numbers(BinaryOp::Exp, 2.0, 3.0), 8.0);
  }

  #[test]
  fn bitwise_and_keeps_the_shared_bits() {
    assert_eq!(fold_numbers(BinaryOp::BitAnd, 6.0, 3.0), 2.0);
  }

  #[test]
  fn bitwise_or_keeps_either_side_s_bits() {
    assert_eq!(fold_numbers(BinaryOp::BitOr, 6.0, 3.0), 7.0);
  }

  #[test]
  fn bitwise_xor_keeps_the_bits_only_one_side_has() {
    assert_eq!(fold_numbers(BinaryOp::BitXor, 6.0, 3.0), 5.0);
  }

  #[test]
  fn right_shift_shifts_right() {
    assert_eq!(fold_numbers(BinaryOp::RShift, 6.0, 3.0), 0.0);
  }

  #[test]
  fn left_shift_shifts_left() {
    assert_eq!(fold_numbers(BinaryOp::LShift, 6.0, 3.0), 48.0);
  }

  #[test]
  fn zero_fill_right_shift_shifts_right() {
    assert_eq!(fold_numbers(BinaryOp::ZeroFillRShift, 6.0, 3.0), 0.0);
    assert_eq!(fold_numbers(BinaryOp::ZeroFillRShift, 8.0, 1.0), 4.0);
  }

  #[test]
  fn loose_equality_answers_one_when_equal() {
    assert_eq!(fold_numbers(BinaryOp::EqEq, 5.0, 5.0), 1.0);
    assert_eq!(fold_numbers(BinaryOp::EqEq, 10.0, 2.0), 0.0);
  }

  #[test]
  fn loose_inequality_answers_one_when_different() {
    assert_eq!(fold_numbers(BinaryOp::NotEq, 5.0, 3.0), 1.0);
    assert_eq!(fold_numbers(BinaryOp::NotEq, 5.0, 5.0), 0.0);
  }

  #[test]
  fn strict_equality_answers_one_when_equal() {
    assert_eq!(fold_numbers(BinaryOp::EqEqEq, 5.0, 5.0), 1.0);
    assert_eq!(fold_numbers(BinaryOp::EqEqEq, 10.0, 2.0), 0.0);
  }

  #[test]
  fn strict_inequality_answers_one_when_different() {
    assert_eq!(fold_numbers(BinaryOp::NotEqEq, 5.0, 3.0), 1.0);
    assert_eq!(fold_numbers(BinaryOp::NotEqEq, 5.0, 5.0), 0.0);
  }

  #[test]
  fn greater_than_answers_one_when_greater() {
    assert_eq!(fold_numbers(BinaryOp::Gt, 10.0, 2.0), 1.0);
    assert_eq!(fold_numbers(BinaryOp::Gt, 3.0, 5.0), 0.0);
  }

  #[test]
  fn greater_or_equal_answers_one_when_equal() {
    assert_eq!(fold_numbers(BinaryOp::GtEq, 5.0, 5.0), 1.0);
    assert_eq!(fold_numbers(BinaryOp::GtEq, 3.0, 5.0), 0.0);
  }

  #[test]
  fn less_than_answers_one_when_less() {
    assert_eq!(fold_numbers(BinaryOp::Lt, 3.0, 5.0), 1.0);
    assert_eq!(fold_numbers(BinaryOp::Lt, 10.0, 2.0), 0.0);
  }

  #[test]
  fn less_or_equal_answers_one_when_equal() {
    assert_eq!(fold_numbers(BinaryOp::LtEq, 5.0, 5.0), 1.0);
    assert_eq!(fold_numbers(BinaryOp::LtEq, 10.0, 2.0), 0.0);
  }

  /// `in` asks a question about an object, and this path has already coerced
  /// both sides to numbers — so what it answers is not the operator's meaning.
  /// Pinned as found: nothing real StyleX source can write reaches the arm.
  #[test]
  fn in_answers_on_the_coerced_right_side_being_zero() {
    assert_eq!(fold_numbers(BinaryOp::In, 10.0, 0.0), 1.0);
    assert_eq!(fold_numbers(BinaryOp::In, 10.0, 1.0), 0.0);
  }

  /// `instanceof` is the same shape as `in`, and pinned for the same reason.
  #[test]
  fn instanceof_answers_on_the_coerced_right_side_being_zero() {
    assert_eq!(fold_numbers(BinaryOp::InstanceOf, 10.0, 0.0), 1.0);
    assert_eq!(fold_numbers(BinaryOp::InstanceOf, 10.0, 2.0), 0.0);
  }

  /// The three logical operators never reach this path — the node dispatches
  /// them to their own before it runs — and it refuses rather than coercing
  /// them to a number, which is what it used to do.
  #[test]
  fn the_logical_operators_are_refused_rather_than_coerced() {
    for op in [
      BinaryOp::LogicalOr,
      BinaryOp::LogicalAnd,
      BinaryOp::NullishCoalescing,
    ] {
      let bin = bin_expr(op, create_number_expr(5.0), create_number_expr(3.0));

      assert!(
        num_or_str_path(&bin).is_err(),
        "expected {:?} to be refused by the number path",
        op
      );
    }
  }

  /// A string has no number, and every operator but `+` needs one from both
  /// sides — so the path refuses and the caller deopts.
  #[test]
  fn a_string_operand_under_a_non_addition_operator_is_refused() {
    let bin = bin_expr(
      BinaryOp::Sub,
      create_string_expr("hello"),
      create_number_expr(5.0),
    );

    assert!(num_or_str_path(&bin).is_err());
  }

  /// An operand that cannot be resolved at compile time takes its confidence
  /// with it, and the path refuses rather than folding around it.
  #[test]
  fn an_unresolvable_left_operand_is_refused() {
    let bin = bin_expr(
      BinaryOp::Add,
      create_ident_expr("x"),
      create_number_expr(1.0),
    );

    assert!(num_or_str_path(&bin).is_err());
  }
}

mod the_string_path {
  use super::*;

  #[test]
  fn two_strings_concatenate() {
    assert_eq!(
      concatenate(create_string_expr("hello"), create_string_expr(" world")),
      "hello world"
    );
  }

  #[test]
  fn a_number_on_the_left_concatenates_as_its_spelling() {
    assert_eq!(
      concatenate(create_number_expr(42.0), create_string_expr("world")),
      "42world"
    );
  }

  #[test]
  fn a_number_on_the_right_concatenates_as_its_spelling() {
    assert_eq!(
      concatenate(create_string_expr("hello"), create_number_expr(42.0)),
      "hello42"
    );
  }

  /// Only `+` has a string result. Every other operator arrives here having
  /// already been refused by the number path, and is refused again so the
  /// caller deopts — rather than failing the build over an expression the
  /// language reads as a value.
  #[test]
  fn a_non_addition_operator_is_refused() {
    let bin = bin_expr(
      BinaryOp::Sub,
      create_string_expr("hello"),
      create_string_expr("world"),
    );

    assert!(string_path(&bin).is_err());
  }

  /// An operand with no compile-time value refuses here too, so a `+` whose
  /// right side is only known at runtime falls to the runtime whole.
  #[test]
  fn an_unresolvable_right_operand_is_refused() {
    let bin = bin_expr(
      BinaryOp::Add,
      create_string_expr("foo"),
      create_ident_expr("bar"),
    );

    assert!(string_path(&bin).is_err());
  }
}
