// Tests for the numeric binary operators.
// Source: crates/stylex-js/src/operators.rs
//
// Expected values are what a JavaScript runtime answers for the same operator
// and operands, which is what `@stylexjs/babel-plugin` folds them to.

use super::*;

mod evaluate_bin_expr_tests {
  use super::*;

  // --- Arithmetic operators ---

  #[test]
  fn addition() {
    assert_eq!(evaluate_bin_expr(BinaryOp::Add, 5.0, 3.0), 8.0);
  }

  #[test]
  fn subtraction() {
    assert_eq!(evaluate_bin_expr(BinaryOp::Sub, 10.0, 4.0), 6.0);
  }

  #[test]
  fn multiplication() {
    assert_eq!(evaluate_bin_expr(BinaryOp::Mul, 3.0, 7.0), 21.0);
  }

  #[test]
  fn division() {
    assert_eq!(evaluate_bin_expr(BinaryOp::Div, 15.0, 5.0), 3.0);
  }

  #[test]
  fn modulo() {
    assert_eq!(evaluate_bin_expr(BinaryOp::Mod, 17.0, 5.0), 2.0);
  }

  #[test]
  fn exponentiation() {
    assert_eq!(evaluate_bin_expr(BinaryOp::Exp, 2.0, 10.0), 1024.0);
  }

  #[test]
  fn addition_of_large_magnitudes() {
    assert_eq!(evaluate_bin_expr(BinaryOp::Add, 1e15, 1e15), 2e15);
  }

  #[test]
  fn division_with_an_inexact_result() {
    let result = evaluate_bin_expr(BinaryOp::Div, 1.0, 3.0);

    // A third has no exact binary form, so the test states the bound the
    // caller may rely on rather than a literal that would only restate f64.
    assert!(result > 0.333_333_3 && result < 0.333_333_4);
    assert_eq!(result * 3.0, 1.0);
  }

  // --- Bitwise operators ---

  #[test]
  fn bitwise_or() {
    assert_eq!(evaluate_bin_expr(BinaryOp::BitOr, 5.0, 3.0), 7.0);
  }

  #[test]
  fn bitwise_and() {
    assert_eq!(evaluate_bin_expr(BinaryOp::BitAnd, 5.0, 3.0), 1.0);
  }

  #[test]
  fn bitwise_xor() {
    assert_eq!(evaluate_bin_expr(BinaryOp::BitXor, 5.0, 3.0), 6.0);
  }

  #[test]
  fn left_shift() {
    assert_eq!(evaluate_bin_expr(BinaryOp::LShift, 1.0, 4.0), 16.0);
  }

  #[test]
  fn right_shift() {
    assert_eq!(evaluate_bin_expr(BinaryOp::RShift, 16.0, 2.0), 4.0);
  }

  #[test]
  fn zero_fill_right_shift_positive() {
    assert_eq!(evaluate_bin_expr(BinaryOp::ZeroFillRShift, 16.0, 2.0), 4.0);
  }

  /// `>>>` reads its left side as unsigned, which is the whole of what parts it
  /// from `>>`. Every value below was read out of `node -e 'console.log(x)'`.
  #[test]
  fn zero_fill_right_shift_reads_its_left_side_as_unsigned() {
    assert_eq!(
      evaluate_bin_expr(BinaryOp::ZeroFillRShift, -1.0, 0.0),
      4_294_967_295.0
    );
    assert_eq!(
      evaluate_bin_expr(BinaryOp::ZeroFillRShift, -1.0, 16.0),
      65535.0
    );
    assert_eq!(
      evaluate_bin_expr(BinaryOp::ZeroFillRShift, -9.0, 0.0),
      4_294_967_287.0
    );
    assert_eq!(
      evaluate_bin_expr(BinaryOp::ZeroFillRShift, -8.0, 1.0),
      2_147_483_644.0
    );
  }

  /// `>>` keeps the sign, which is the reading `>>>` above does not take.
  #[test]
  fn right_shift_keeps_the_sign() {
    assert_eq!(evaluate_bin_expr(BinaryOp::RShift, -8.0, 1.0), -4.0);
    assert_eq!(evaluate_bin_expr(BinaryOp::RShift, -1.0, 16.0), -1.0);
  }

  /// A side past the signed 32-bit range wraps rather than saturating, and a
  /// count of the word width or more shifts by nothing. A 64-bit cast answered
  /// 2147483647 for the first row, and a Rust shift panicked on the third.
  #[test]
  fn the_bitwise_operators_wrap_into_thirty_two_bits() {
    assert_eq!(
      evaluate_bin_expr(BinaryOp::BitOr, 4_294_967_296.0, 0.0),
      0.0
    );
    assert_eq!(
      evaluate_bin_expr(BinaryOp::BitOr, 3_000_000_000.0, 0.0),
      -1_294_967_296.0
    );
    assert_eq!(evaluate_bin_expr(BinaryOp::LShift, 1.0, 32.0), 1.0);
    assert_eq!(evaluate_bin_expr(BinaryOp::LShift, 1.0, 33.0), 2.0);
    assert_eq!(
      evaluate_bin_expr(BinaryOp::LShift, 1.0, 31.0),
      -2_147_483_648.0
    );
    assert_eq!(evaluate_bin_expr(BinaryOp::BitOr, f64::NAN, 0.0), 0.0);
    assert_eq!(evaluate_bin_expr(BinaryOp::BitOr, f64::INFINITY, 0.0), 0.0);
    assert_eq!(
      evaluate_bin_expr(BinaryOp::BitAnd, 1e21, -1.0),
      -559_939_584.0
    );
  }

  // --- Edge cases ---

  #[test]
  fn division_by_zero_yields_infinity() {
    let result = evaluate_bin_expr(BinaryOp::Div, 1.0, 0.0);
    assert!(result.is_infinite() && result.is_sign_positive());
  }

  #[test]
  fn division_negative_by_zero_yields_neg_infinity() {
    let result = evaluate_bin_expr(BinaryOp::Div, -1.0, 0.0);
    assert!(result.is_infinite() && result.is_sign_negative());
  }

  #[test]
  fn zero_divided_by_zero_is_nan() {
    let result = evaluate_bin_expr(BinaryOp::Div, 0.0, 0.0);
    assert!(result.is_nan());
  }

  #[test]
  fn modulo_by_zero_is_nan() {
    let result = evaluate_bin_expr(BinaryOp::Mod, 5.0, 0.0);
    assert!(result.is_nan());
  }

  #[test]
  fn addition_with_negative_numbers() {
    assert_eq!(evaluate_bin_expr(BinaryOp::Add, -3.0, -7.0), -10.0);
  }

  #[test]
  fn subtraction_with_negative_numbers() {
    assert_eq!(evaluate_bin_expr(BinaryOp::Sub, -3.0, -7.0), 4.0);
  }

  #[test]
  fn multiplication_with_negative_numbers() {
    assert_eq!(evaluate_bin_expr(BinaryOp::Mul, -3.0, 7.0), -21.0);
  }

  #[test]
  fn addition_with_zero() {
    assert_eq!(evaluate_bin_expr(BinaryOp::Add, 0.0, 0.0), 0.0);
  }

  #[test]
  fn multiplication_with_zero() {
    assert_eq!(evaluate_bin_expr(BinaryOp::Mul, 42.0, 0.0), 0.0);
  }

  #[test]
  fn exponentiation_zero_power() {
    assert_eq!(evaluate_bin_expr(BinaryOp::Exp, 5.0, 0.0), 1.0);
  }

  #[test]
  fn exponentiation_negative_exponent() {
    assert_eq!(evaluate_bin_expr(BinaryOp::Exp, 2.0, -1.0), 0.5);
  }

  #[test]
  fn right_shift_negative_number() {
    // -16 >> 2 == -4 in two's complement
    assert_eq!(evaluate_bin_expr(BinaryOp::RShift, -16.0, 2.0), -4.0);
  }

  #[test]
  #[should_panic(expected = "Unsupported binary operator")]
  fn unsupported_operator_panics() {
    // EqEq is not handled by evaluate_bin_expr
    evaluate_bin_expr(BinaryOp::EqEq, 1.0, 1.0);
  }
}
