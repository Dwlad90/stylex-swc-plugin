//! Coverage for the two paths the binary-expression node folds through, at the
//! operators real StyleX source cannot reach — comparison, bitwise, `in` and
//! `instanceof`. Everything an author can write is pinned from above, on the
//! `stylex.create` fixtures.

use super::*;
use stylex_ast::ast::convertors::create_ident_expr;
use stylex_ast::ast::convertors::create_null_expr;
use stylex_ast::ast::factories::create_key_value_prop;
use stylex_utils::string::utf16_length;

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
  num_or_str_path_with_fns(bin, &FunctionMap::default())
}

/// The string path, against state of its own.
fn string_path(bin: &BinExpr) -> Result<BinaryExprType, anyhow::Error> {
  let mut state = EvaluationState::new();
  let mut traversal_state = StateManager::default();
  let fns = FunctionMap::default();

  binary_expr_to_string(bin, &mut state, &mut traversal_state, &fns)
}

/// The name a function of the compiler's own is bound to below.
const OWN_FUNCTION: &str = "own";

/// A map binding [`OWN_FUNCTION`] to a function of the compiler's own.
///
/// The one value that has neither a string nor a number: its only string would
/// be its source text, and this evaluator keeps none. Nothing an author writes
/// spells it, so a case about an operand with no reading at all has to build
/// it.
fn a_map_holding_a_function() -> FunctionMap {
  let mut fns = FunctionMap::default();

  fns.identifiers.insert(
    OWN_FUNCTION.into(),
    Box::new(FunctionConfigType::Regular(FunctionConfig {
      fn_ptr: FunctionType::StylexExprFn(|expr, _| expr),
      takes_path: false,
    })),
  );

  fns
}

/// The number-or-string path, against a map the case built.
///
/// The map goes on the state as well as beside it. A name resolves through
/// `state.functions`, so a map handed only as the argument would leave the name
/// unresolved and the case would read the operand walk's refusal rather than
/// the coercion's.
fn num_or_str_path_with_fns(
  bin: &BinExpr,
  fns: &FunctionMap,
) -> Result<BinaryExprType, anyhow::Error> {
  let mut state = EvaluationState::new();
  let mut traversal_state = StateManager::default();

  state.functions = std::rc::Rc::new(fns.clone());

  binary_expr_to_num_or_str(bin, &mut state, &mut traversal_state, fns)
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
///
/// The count the string carries is checked against a fresh reading of the text
/// on the way through, because every case here then holds the carried count as
/// well as the text: a count that drifts from what it labels would spend the
/// character ceiling against a length nothing has.
#[track_caller]
fn expect_string(result: Result<BinaryExprType, anyhow::Error>) -> String {
  match result {
    Result::Ok(BinaryExprType::String { text, units }) => {
      assert_eq!(
        units,
        utf16_length(&text),
        "the count carried with {:?} is not its length",
        text
      );

      text
    },
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

/// One equality operator over two operands, through the number-or-string path.
///
/// Two expressions rather than two numbers, because which reading each operator
/// takes is decided by what the sides *are* rather than by what they coerce to.
#[track_caller]
fn fold_equality(op: BinaryOp, left: Expr, right: Expr) -> f64 {
  expect_number(num_or_str_path(&bin_expr(op, left, right)))
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

  /// Operands on which the signed and the unsigned readings part company, so
  /// the case fails against either one written for the other. Every value was
  /// read out of `node -e 'console.log(x)'`.
  #[test]
  fn right_shift_shifts_right_and_keeps_the_sign() {
    assert_eq!(fold_numbers(BinaryOp::RShift, 6.0, 3.0), 0.0);
    assert_eq!(fold_numbers(BinaryOp::RShift, -8.0, 1.0), -4.0);
    assert_eq!(fold_numbers(BinaryOp::RShift, -1.0, 16.0), -1.0);
  }

  #[test]
  fn left_shift_shifts_left() {
    assert_eq!(fold_numbers(BinaryOp::LShift, 6.0, 3.0), 48.0);
    assert_eq!(fold_numbers(BinaryOp::LShift, 1.0, 31.0), -2_147_483_648.0);
  }

  /// `>>>` reads its left side as unsigned, so a negative operand answers the
  /// large number rather than itself. Read as signed, `-1 >>> 0` answered `-1`.
  #[test]
  fn zero_fill_right_shift_reads_its_left_side_as_unsigned() {
    assert_eq!(fold_numbers(BinaryOp::ZeroFillRShift, 6.0, 3.0), 0.0);
    assert_eq!(fold_numbers(BinaryOp::ZeroFillRShift, 8.0, 1.0), 4.0);
    assert_eq!(
      fold_numbers(BinaryOp::ZeroFillRShift, -1.0, 0.0),
      4_294_967_295.0
    );
    assert_eq!(fold_numbers(BinaryOp::ZeroFillRShift, -1.0, 16.0), 65535.0);
    assert_eq!(
      fold_numbers(BinaryOp::ZeroFillRShift, -8.0, 1.0),
      2_147_483_644.0
    );
  }

  /// A side past the signed 32-bit range wraps rather than saturating, and a
  /// count of the word width or more shifts by nothing.
  ///
  /// Both are what a Rust cast does not do: `4294967296 | 0` answered
  /// 2147483647, and `1 << 32` stopped a debug build with a shift overflow.
  #[test]
  fn the_bitwise_operators_wrap_into_thirty_two_bits() {
    assert_eq!(fold_numbers(BinaryOp::BitOr, 4_294_967_296.0, 0.0), 0.0);
    assert_eq!(
      fold_numbers(BinaryOp::BitOr, 3_000_000_000.0, 0.0),
      -1_294_967_296.0
    );
    assert_eq!(fold_numbers(BinaryOp::BitAnd, 1e21, -1.0), -559_939_584.0);
    assert_eq!(fold_numbers(BinaryOp::LShift, 1.0, 32.0), 1.0);
    assert_eq!(fold_numbers(BinaryOp::LShift, 1.0, 33.0), 2.0);
    assert_eq!(fold_numbers(BinaryOp::LShift, 1.0, -1.0), -2_147_483_648.0);
    assert_eq!(fold_numbers(BinaryOp::ZeroFillRShift, 5.0, -1.0), 0.0);

    // A side with no integer answers zero rather than refusing, which is what
    // `ToInt32` says of a `NaN` and of either infinity.
    assert_eq!(fold_numbers(BinaryOp::BitOr, f64::NAN, 0.0), 0.0);
    assert_eq!(fold_numbers(BinaryOp::BitOr, f64::INFINITY, 0.0), 0.0);
    assert_eq!(fold_numbers(BinaryOp::BitOr, f64::NEG_INFINITY, 0.0), 0.0);
  }

  /// The three numbers with no digits, as operands of the arithmetic. Each is a
  /// value the language answers with rather than a refusal, and `NaN` is the
  /// one that equals nothing -- including itself.
  #[test]
  fn the_numbers_with_no_digits_are_operands_like_any_other() {
    assert!(fold_numbers(BinaryOp::Add, f64::NAN, 1.0).is_nan());
    assert!(fold_numbers(BinaryOp::Sub, f64::INFINITY, f64::INFINITY).is_nan());
    assert_eq!(
      fold_numbers(BinaryOp::Add, f64::INFINITY, 1.0),
      f64::INFINITY
    );
    assert_eq!(fold_numbers(BinaryOp::Div, 1.0, 0.0), f64::INFINITY);

    // `-0` is what `0 * -1` answers, and `assert_eq!` against `0.0` cannot see
    // it: the two zeroes compare equal. The sign is what parts `1 / -0` from
    // `1 / 0`.
    let negative_zero = fold_numbers(BinaryOp::Mul, 0.0, -1.0);

    assert_eq!(negative_zero, 0.0);
    assert!(negative_zero.is_sign_negative());

    // Every comparison against `NaN` is false, and the inequality is the one
    // that is true.
    assert_eq!(fold_numbers(BinaryOp::EqEqEq, f64::NAN, f64::NAN), 0.0);
    assert_eq!(fold_numbers(BinaryOp::NotEqEq, f64::NAN, f64::NAN), 1.0);
    assert_eq!(fold_numbers(BinaryOp::Lt, f64::NAN, 1.0), 0.0);
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

  /// Which reading each of the four operators takes, over the pairs where the
  /// two readings part company.
  ///
  /// `!=` sits with the strict pair, which is how the reference implementation
  /// writes it rather than how the language reads it. Compared as two numbers,
  /// as this path did before, `1 != '1'` answered `false` and put the other arm
  /// of a conditional into the stylesheet.
  #[test]
  fn the_four_equality_operators_compare_two_primitives() {
    let one = create_number_expr(1.0);
    let one_as_text = create_string_expr("1");

    assert_eq!(
      fold_equality(BinaryOp::EqEq, one.clone(), one_as_text.clone()),
      1.0
    );
    assert_eq!(
      fold_equality(BinaryOp::NotEq, one.clone(), one_as_text.clone()),
      1.0
    );
    assert_eq!(
      fold_equality(BinaryOp::EqEqEq, one.clone(), one_as_text.clone()),
      0.0
    );
    assert_eq!(fold_equality(BinaryOp::NotEqEq, one, one_as_text), 1.0);
  }

  /// Two pairs a numeric reading has no answer for: two strings that are not
  /// numeric literals, and the two nullish spellings. Both refused before, and
  /// the language answers `true` for both under `==`.
  #[test]
  fn two_values_with_no_number_still_compare() {
    assert_eq!(
      fold_equality(
        BinaryOp::EqEq,
        create_string_expr("red"),
        create_string_expr("red")
      ),
      1.0
    );
    assert_eq!(
      fold_equality(
        BinaryOp::EqEqEq,
        create_string_expr("red"),
        create_string_expr("blue")
      ),
      0.0
    );
    assert_eq!(
      fold_equality(BinaryOp::EqEq, create_null_expr(), undefined_expr()),
      1.0
    );
    assert_eq!(
      fold_equality(BinaryOp::EqEqEq, create_null_expr(), undefined_expr()),
      0.0
    );
  }

  /// A side with no primitive refuses, and names what it could not read rather
  /// than the comparison. The language compares two objects by reference, which
  /// this evaluator does not hold.
  #[test]
  fn a_side_with_no_primitive_is_refused() {
    let refused = num_or_str_path(&bin_expr(
      BinaryOp::EqEq,
      Expr::Object(create_object_lit(vec![])),
      create_number_expr(0.0),
    ));

    assert_refuses_with(refused, "is not a number");
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

      assert_refuses_with(num_or_str_path(&bin), &unsupported_operator(op.as_str()));
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

    // The coercion's own sentence rather than the side's: the operand *is*
    // there, and what has no reading is its text.
    assert_refuses_with(num_or_str_path(&bin), "Value is not a number: hello");
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

    assert_refuses_with(num_or_str_path(&bin), LEFT_HAS_NO_VALUE);
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

    assert_refuses_with(string_path(&bin), "only addition is supported");
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

    assert_refuses_with(string_path(&bin), RIGHT_NOT_A_STRING);
  }
}

/// An operand with no number refuses on the numeric path, and the refusal is
/// the coercion's own rather than the operand walk's: the side evaluated, and
/// then had no number to give. `{ toString: 1 }` is such a value -- neither
/// conversion method is callable, so the language itself throws where a number
/// was wanted.
#[test]
fn a_left_operand_that_evaluated_but_has_no_number_is_refused() {
  let bin = bin_expr(
    BinaryOp::Mul,
    create_object_lit(vec![create_key_value_prop(
      "toString",
      create_number_expr(1.0),
    )])
    .into(),
    create_number_expr(2.0),
  );

  assert_refuses_with(num_or_str_path(&bin), "is not a number");
}

/// A `+` concatenates as soon as either side is a string, and a side with no
/// string at all refuses there. A function is the value with none: its only
/// string is its source text, which neither compiler keeps.
#[test]
fn a_concatenation_over_a_value_with_no_string_is_refused() {
  let fns = a_map_holding_a_function();

  for (left, right, expected) in [
    (
      create_ident_expr(OWN_FUNCTION),
      create_string_expr("x"),
      LEFT_NOT_A_STRING,
    ),
    (
      create_string_expr("x"),
      create_ident_expr(OWN_FUNCTION),
      RIGHT_NOT_A_STRING,
    ),
  ] {
    let bin = bin_expr(BinaryOp::Add, left, right);

    assert_refuses_with(num_or_str_path_with_fns(&bin, &fns), expected);
  }
}

/// The right side of an operator that is not `+` is read after the left has
/// coerced, and each of the three ways it can fail is answered where it fails.
///
/// The left side's three are pinned above. These are the right's, and they are
/// a different order of events: `+` asks its right side before either coerces,
/// so only the other operators reach a right side with the left already read.
#[test]
fn a_right_operand_with_no_number_is_refused_where_it_fails() {
  let fns = a_map_holding_a_function();

  for (right, expected) in [
    // Evaluated, and then no number: neither conversion method is callable, so
    // the language itself throws where a number was wanted.
    (
      Expr::from(create_object_lit(vec![create_key_value_prop(
        "toString",
        create_number_expr(1.0),
      )])),
      "is not a number",
    ),
    // A value the module holds that is not an expression at all, which is what
    // a name bound to one of this compiler's own functions resolves to.
    (
      create_ident_expr(OWN_FUNCTION),
      "Right argument not expression",
    ),
  ] {
    let bin = bin_expr(BinaryOp::Sub, create_number_expr(1.0), right);

    assert_refuses_with(num_or_str_path_with_fns(&bin, &fns), expected);
  }
}

/// A right side that answered nothing at all refuses before any coercion, which
/// is the third way and the one that names the side rather than the reading.
#[test]
fn a_right_operand_that_answered_nothing_is_refused() {
  let bin = bin_expr(
    BinaryOp::Sub,
    create_number_expr(1.0),
    create_ident_expr("missing"),
  );

  assert_refuses_with(num_or_str_path(&bin), RIGHT_NOT_A_NUMBER);
}

/// Asserts a path refused, and that the sentence is the one `expected`.
///
/// The sentence rather than only the refusal: which side had no reading, and
/// which of the two readings asked for it, are the whole of what these cases
/// are about -- and a refusal from the other side reads to a caller exactly
/// like the right one.
#[track_caller]
fn assert_refuses_with(result: Result<BinaryExprType, anyhow::Error>, expected: &str) {
  match result {
    Result::Ok(folded) => panic!("expected a refusal, folded to {:?}", folded),
    Result::Err(error) => {
      let sentence = error.to_string();

      assert!(
        sentence.contains(expected),
        "expected the refusal to say `{}`, got `{}`",
        expected,
        sentence
      );
    },
  }
}
