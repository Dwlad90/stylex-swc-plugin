use super::super::*;
use super::logical_expression;
use anyhow::anyhow;
use stylex_macros::as_expr_or_err;
use swc_core::ecma::ast::{BinExpr, BinaryOp};

pub(in super::super) fn evaluate(
  bin: &BinExpr,
  state: &mut EvaluationState,
  traversal_state: &mut StateManager,
  fns: &FunctionMap,
) -> Option<EvaluateResultValue> {
  // The reference implementation keeps `||`, `&&` and `??` on a node of their
  // own, evaluated over values and tested ahead of this one. SWC folds all
  // three into `BinExpr`, so the split happens here instead — before either of
  // the paths below, which coerce their operands and so cannot answer for an
  // operator that returns one of them unchanged.
  if let Some(op) = logical_expression::LogicalOp::of(bin.op) {
    return logical_expression::evaluate(op, bin, state, traversal_state, fns);
  }

  match fold_binary_expr(bin, state, traversal_state, fns)? {
    BinaryExprType::Number(num) => Some(EvaluateResultValue::Expr(create_number_expr(num))),
    BinaryExprType::String { text, .. } => {
      Some(EvaluateResultValue::Expr(create_string_expr(&text)))
    },
    BinaryExprType::Null => None,
  }
}

/// `ToNumber` over a boolean: the number a comparison result becomes when the
/// surrounding expression is being evaluated as a number.
#[inline]
fn convert_bool_to_number(value: bool) -> f64 {
  if value { 1.0 } else { 0.0 }
}

/// Whether an already-evaluated operand *is* a string, rather than whether it
/// converts to one. An evaluated string always arrives as a string literal, so
/// nothing else can answer yes here — a number does not become a string by
/// having a spelling.
#[inline]
fn is_string(expr: &Expr) -> bool {
  matches!(expr, Expr::Lit(Lit::Str(_)))
}

/// The reason each side records when it cannot be read as what the path it is
/// on needs. Named rather than built at the call site so the operand helpers
/// stay at one argument per thing they actually need.
const LEFT_NOT_A_NUMBER: &str = "Left expression is not a number";
const RIGHT_NOT_A_NUMBER: &str = "Right expression is not a number";
const LEFT_NOT_A_STRING: &str = "Left expression is not a string";
const RIGHT_NOT_A_STRING: &str = "Right expression is not a string";

/// The reasons the `+` dispatch records, which runs before either path has
/// claimed the operator and so cannot borrow either one's wording: an operand
/// of `+` is on its way to concatenation as readily as to addition, and naming
/// it a failed number would describe a coercion nothing was about to perform.
const LEFT_HAS_NO_VALUE: &str = "Left expression could not be evaluated";
const RIGHT_HAS_NO_VALUE: &str = "Right expression could not be evaluated";

/// One side of the expression, evaluated. An operand that resolves to nothing
/// while the evaluator is still confident is this path's own bug rather than an
/// expression it cannot fold, which is why the two answers differ.
fn evaluate_operand(
  operand: &Expr,
  reason: &str,
  state: &mut EvaluationState,
  traversal_state: &mut StateManager,
  fns: &FunctionMap,
) -> Result<EvaluateResultValue, anyhow::Error> {
  match evaluate_cached(operand, state, traversal_state, fns) {
    Some(value) => Result::Ok(value),
    None if !state.confident => Result::Err(anyhow!("{}", reason)),
    None => stylex_panic!("{}", reason),
  }
}

/// The left operand of a binary expression, as much of it as the level above
/// can use.
///
/// A `+` whose left side is another `+` is the one shape that carries more than
/// a value: the text below was measured while it was grown, and the level above
/// appends to it. Every other operand is just the value the evaluator resolved.
enum LeftOperand {
  /// A concatenation this same path folded one level down, still carrying the
  /// count it was measured to.
  Measured { text: String, units: usize },
  /// Every other operand.
  Value(EvaluateResultValue),
}

impl LeftOperand {
  /// Whether this side makes a `+` a concatenation rather than an addition.
  fn is_string(&self) -> bool {
    match self {
      Self::Measured { .. } => true,
      Self::Value(value) => value.as_expr().is_some_and(is_string),
    }
  }

  /// The expression behind an operand that is not a folded concatenation.
  ///
  /// `None` for a measured one, which the numeric path never sees: a folded
  /// concatenation is a string, and a string on either side sends the `+` to
  /// the concatenation path before any coercion to a number is asked for.
  fn as_expr(&self) -> Option<&Expr> {
    match self {
      Self::Measured { .. } => None,
      Self::Value(value) => value.as_expr(),
    }
  }
}

/// A measured left side is remembered as the string literal it spells, which is
/// all the tree can hold, and read back as an ordinary value -- so a chain
/// answered from the memo is measured where it lands, exactly as it was before
/// the count started travelling.
impl Memoized for LeftOperand {
  fn from_memo(remembered: EvaluateResultValue) -> Self {
    Self::Value(remembered)
  }

  fn to_memo(&self) -> EvaluateResultValue {
    match self {
      Self::Measured { text, .. } => EvaluateResultValue::Expr(create_string_expr(text)),
      Self::Value(value) => value.clone(),
    }
  }
}

/// One side of a binary expression, evaluated -- folding the left side of a `+`
/// chain here rather than through the evaluator's own dispatch, so a
/// concatenation keeps the count it was measured to.
///
/// The dispatch hands a folded `+` back as a plain string literal, which has
/// nowhere to carry a length. Measured again one level up, and copied into a
/// fresh buffer beside it, a chain spends the length of everything already
/// joined once per remaining link -- the square of its text rather than its
/// length. Folded here, the buffer and the count travel together and each
/// operand is read exactly once.
///
/// It is the *same* level of the ceiling and the *same* memo either way, which
/// is what [`folded_once`] is for: a chain refuses at the link it always
/// refused at, and a subtree written twice still answers from the first
/// reading. Only the measurement is extra, and only on the way down -- a
/// remembered answer is a literal like any other, and is measured.
///
/// The mutation check the dispatch makes is not repeated: it asks about an
/// assignment, an update and a `delete`, and this arm has already matched a
/// binary expression.
fn evaluate_left_operand(
  binary_expr: &BinExpr,
  reason: &str,
  state: &mut EvaluationState,
  traversal_state: &mut StateManager,
  fns: &FunctionMap,
) -> Result<LeftOperand, anyhow::Error> {
  if matches!(binary_expr.op, BinaryOp::Add)
    && let Expr::Bin(inner) = normalize_expr(&binary_expr.left)
    && matches!(inner.op, BinaryOp::Add)
  {
    let folded = folded_once(
      &binary_expr.left,
      state,
      traversal_state,
      |state, traversal_state| match fold_binary_expr(inner, state, traversal_state, fns)? {
        BinaryExprType::String { text, units } => Some(LeftOperand::Measured { text, units }),
        BinaryExprType::Number(number) => Some(LeftOperand::Value(EvaluateResultValue::Expr(
          create_number_expr(number),
        ))),
        BinaryExprType::Null => None,
      },
    );

    // Reported on the same terms an operand resolving to nothing is reported on
    // anywhere else, which is `evaluate_operand`'s below.
    return match folded {
      Some(left) => Result::Ok(left),
      None if !state.confident => Result::Err(anyhow!("{}", reason)),
      None => stylex_panic!("{}", reason),
    };
  }

  evaluate_operand(&binary_expr.left, reason, state, traversal_state, fns).map(LeftOperand::Value)
}

/// A binary expression folded to its value rather than to an expression: the
/// number-or-string path, falling back to the string path.
///
/// The same two-step [`evaluate`] performs, minus the boxing back into the
/// tree, so the descent above reads a measured string where `evaluate` would
/// hand back a string literal. `Null` is the refusal, as it is there.
fn fold_binary_expr(
  binary_expr: &BinExpr,
  state: &mut EvaluationState,
  traversal_state: &mut StateManager,
  fns: &FunctionMap,
) -> Option<BinaryExprType> {
  if !state.confident {
    return None;
  }

  Some(
    binary_expr_to_num_or_str(binary_expr, state, traversal_state, fns).unwrap_or_else(
      |num_error| {
        binary_expr_to_string(binary_expr, state, traversal_state, fns).unwrap_or_else(
          |str_error| {
            debug!("Binary expression to string error: {}", str_error);
            debug!("Binary expression to number error: {}", num_error);

            BinaryExprType::Null
          },
        )
      },
    ),
  )
}

/// Every binary operator but the three logical ones, folded to whichever of a
/// number and a string its operands decide.
///
/// Named for both because `+` returns either: it is the one operator whose
/// result type is not the path's to choose, and reading the name as a promise
/// of a number is how a caller comes to treat `'1' + 2` as arithmetic. The two
/// callers outside this node want a number specifically and refuse the other
/// answer themselves.
pub(crate) fn binary_expr_to_num_or_str(
  binary_expr: &BinExpr,
  state: &mut EvaluationState,
  traversal_state: &mut StateManager,
  fns: &FunctionMap,
) -> Result<BinaryExprType, anyhow::Error> {
  let op = binary_expr.op;

  let left = evaluate_left_operand(
    binary_expr,
    match op {
      BinaryOp::Add => LEFT_HAS_NO_VALUE,
      _ => LEFT_NOT_A_NUMBER,
    },
    state,
    traversal_state,
    fns,
  )?;

  // `+` is the one operator whose result type its operands decide rather than
  // the path that claimed it: JavaScript concatenates as soon as either side is
  // a string, and only adds when neither is. Asked after the numeric coercion
  // instead, `'1' + 2` would answer `3` — both sides coerce, so the number path
  // would never yield to the string one.
  //
  // It is therefore also the only operator that asks anything of its right side
  // before its left has coerced. The rest are left in the order they had, so a
  // left side with no numeric form goes on refusing there rather than deopting
  // on a right side the refusal never needed. `evaluate_cached` memoises, so
  // the second look below costs nothing.
  if matches!(op, BinaryOp::Add) {
    let right = evaluate_operand(
      &binary_expr.right,
      RIGHT_HAS_NO_VALUE,
      state,
      traversal_state,
      fns,
    )?;

    // Concatenated here rather than by handing the expression back to the
    // string path, which would evaluate both operands a second time -- and
    // would drop the left side's measurement on the way, since only a value can
    // carry one.
    if left.is_string() || right.as_expr().is_some_and(is_string) {
      return concatenate(binary_expr, left, &right, state, traversal_state);
    }
  }

  let left_expr = as_expr_or_err!(left, "Left argument not expression");
  let left_num = expr_to_num(left_expr, state, traversal_state, fns)?;

  let right = evaluate_operand(
    &binary_expr.right,
    RIGHT_NOT_A_NUMBER,
    state,
    traversal_state,
    fns,
  )?;
  let right_expr = as_expr_or_err!(right, "Right argument not expression");
  let right_num = expr_to_num(right_expr, state, traversal_state, fns)?;

  let result = match &op {
    BinaryOp::Add => left_num + right_num,
    BinaryOp::Sub => left_num - right_num,
    BinaryOp::Mul => left_num * right_num,
    BinaryOp::Div => left_num / right_num,
    BinaryOp::Mod => left_num % right_num,
    BinaryOp::Exp => left_num.powf(right_num),
    BinaryOp::RShift => ((left_num as i32) >> right_num as i32) as f64,
    BinaryOp::LShift => ((left_num as i32) << right_num as i32) as f64,
    BinaryOp::BitAnd => ((left_num as i32) & right_num as i32) as f64,
    BinaryOp::BitOr => ((left_num as i32) | right_num as i32) as f64,
    BinaryOp::BitXor => ((left_num as i32) ^ right_num as i32) as f64,
    // `in` and `instanceof` ask a question about an object, which this path has
    // already coerced away to a number. What they answer here is therefore not
    // the operator's meaning; it is left as found, because nothing real StyleX
    // source can write reaches either arm.
    BinaryOp::In => convert_bool_to_number(right_num == 0.0),
    BinaryOp::InstanceOf => convert_bool_to_number(right_num == 0.0),
    BinaryOp::EqEq => convert_bool_to_number(left_num == right_num),
    BinaryOp::NotEq => convert_bool_to_number(left_num != right_num),
    BinaryOp::EqEqEq => convert_bool_to_number(left_num == right_num),
    BinaryOp::NotEqEq => convert_bool_to_number(left_num != right_num),
    BinaryOp::Lt => convert_bool_to_number(left_num < right_num),
    BinaryOp::LtEq => convert_bool_to_number(left_num <= right_num),
    BinaryOp::Gt => convert_bool_to_number(left_num > right_num),
    BinaryOp::GtEq => convert_bool_to_number(left_num >= right_num),
    BinaryOp::ZeroFillRShift => ((left_num as i32) >> right_num as i32) as f64,
    // Unreachable: the three logical operators are dispatched to their own node
    // before this path can run, and there they return an operand rather than a
    // number. Refused on the same terms as any other operator this path has no
    // answer for, rather than coerced to one.
    BinaryOp::LogicalOr | BinaryOp::LogicalAnd | BinaryOp::NullishCoalescing => {
      return Result::Err(anyhow!(unsupported_operator(op.as_str())));
    },
  };

  Result::Ok(BinaryExprType::Number(result))
}

/// `+` over operands at least one of which is a string: `ToString` of each
/// side, joined.
///
/// Only `+` has a string result. Every other operator reaches here through the
/// number path's fallback, having already refused there, and is refused again
/// so the caller deopts rather than failing the build — which is also what the
/// language asks for, since `'a' * 'b'` is a value rather than an error.
///
/// Reached for a `+` only when the number path refused one — an operand with no
/// numeric reading and no string either side, such as an array. The `+` the
/// number path *did* claim concatenates through [`concatenate`] directly,
/// keeping the operands it has already evaluated.
fn binary_expr_to_string(
  binary_expr: &BinExpr,
  state: &mut EvaluationState,
  traversal_state: &mut StateManager,
  fns: &FunctionMap,
) -> Result<BinaryExprType, anyhow::Error> {
  let op = binary_expr.op;

  if !matches!(op, BinaryOp::Add) {
    return Result::Err(anyhow!(
      "For string expressions, only addition is supported, got {:?}",
      op
    ));
  }

  // Both operands through the plain reading, not the measured one: a left side
  // that folds to a string never arrives here, because the number-or-string
  // path claims that `+` and concatenates it itself. What reaches this fallback
  // is a `+` that path refused -- an operand with no numeric form and no string
  // either side, such as an array -- so a descent here could only pay for a
  // measurement nothing would carry.
  let left = evaluate_operand(
    &binary_expr.left,
    LEFT_NOT_A_STRING,
    state,
    traversal_state,
    fns,
  )
  .map(LeftOperand::Value)?;
  let right = evaluate_operand(
    &binary_expr.right,
    RIGHT_NOT_A_STRING,
    state,
    traversal_state,
    fns,
  )?;

  concatenate(binary_expr, left, &right, state, traversal_state)
}

/// Two evaluated operands of a `+`, joined through one measured buffer.
///
/// Both sides grow the same buffer, which measures every piece before it lands
/// -- so a chain of doublings is refused at the append that passes the ceiling
/// rather than after the next one has allocated, and an operand that is an
/// array is refused at the element that passes it rather than after its whole
/// join.
///
/// A left side this path folded one level down is *adopted* rather than copied
/// in: it is the same text and the same count, already checked against the same
/// ceiling, so re-reading it would spend the length of everything joined so far
/// once per remaining link. Any other left side is written through the coercion
/// like the right one, because an array's join is only measurable while it is
/// being written.
///
/// Each side is taken through `ToString`, the coercion the rest of the evaluator
/// already shares from `stylex_js`. This arm used to keep a second, weaker one
/// of its own, which read a string, a number and a big integer and refused the
/// rest -- so `'x' + true` failed to fold where JavaScript says `"xtrue"`. The
/// shared one answers for the whole falsy list, for arrays and for objects, and
/// refuses only where no compile-time string exists at all.
///
/// It is also more permissive than the reference implementation on two
/// operands, and deliberately left that way: a big integer and a regular
/// expression both have a string here, where upstream refuses either literal
/// outright with an unsupported-expression diagnostic. The folded strings are
/// what the language says, so the disagreement costs nothing but a build that
/// succeeds where the other fails.
fn concatenate(
  binary_expr: &BinExpr,
  left: LeftOperand,
  right: &EvaluateResultValue,
  state: &mut EvaluationState,
  traversal_state: &mut StateManager,
) -> Result<BinaryExprType, anyhow::Error> {
  // The sentence for an operand with no string at all names which side it was;
  // a ceiling refusal carries the buffer's own, which names the concatenation
  // rather than a side, because either side reaching the ceiling is the same
  // expression growing too large.
  let refusal = |reason: &'static str| {
    move |refused: StringAppend| match refused {
      StringAppend::NoStringForm => anyhow!("{}", reason),
      StringAppend::TooLarge(sentence) => anyhow!("{}", sentence),
    }
  };
  let path = || Expr::Bin(binary_expr.clone());

  let mut joined = match left {
    LeftOperand::Measured { text, units } => GrownString::adopt(text, units, CONCATENATION),
    LeftOperand::Value(value) => {
      let mut joined = GrownString::new(CONCATENATION);

      joined
        .push_string_of(&value, path, state, traversal_state)
        .map_err(refusal(LEFT_NOT_A_STRING))?;

      joined
    },
  };

  joined
    .push_string_of(right, path, state, traversal_state)
    .map_err(refusal(RIGHT_NOT_A_STRING))?;

  let (text, units) = joined.into_measured();

  Result::Ok(BinaryExprType::String { text, units })
}

#[cfg(test)]
#[path = "tests/binary_expression_tests.rs"]
mod tests;
