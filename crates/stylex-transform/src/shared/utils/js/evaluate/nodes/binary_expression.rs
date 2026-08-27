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

  unwrap_or_panic!(
    binary_expr_to_num_or_str(bin, state, traversal_state, fns)
      .or_else(|num_error| {
        binary_expr_to_string(bin, state, traversal_state, fns).or_else::<String, _>(|str_error| {
          debug!("Binary expression to string error: {}", str_error);
          debug!("Binary expression to number error: {}", num_error);

          Ok(BinaryExprType::Null)
        })
      })
      .map(|result| match result {
        BinaryExprType::Number(num) => Some(EvaluateResultValue::Expr(create_number_expr(num))),
        BinaryExprType::String(strng) =>
          Some(EvaluateResultValue::Expr(create_string_expr(&strng))),
        BinaryExprType::Null => None,
      })
  )
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

  let left = evaluate_operand(
    &binary_expr.left,
    match op {
      BinaryOp::Add => LEFT_HAS_NO_VALUE,
      _ => LEFT_NOT_A_NUMBER,
    },
    state,
    traversal_state,
    fns,
  )?;
  let left_expr = as_expr_or_err!(left, "Left argument not expression");

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

    if is_string(left_expr) || right.as_expr().is_some_and(is_string) {
      return binary_expr_to_string(binary_expr, state, traversal_state, fns);
    }
  }

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

  // The left side's buffer is grown rather than a third one allocated for the
  // join, which is what a chain of `+` folds through once per operand.
  let mut joined = GrownString::of(
    operand_to_string(
      &binary_expr.left,
      LEFT_NOT_A_STRING,
      state,
      traversal_state,
      fns,
    )?,
    CONCATENATION,
  );
  let right = operand_to_string(
    &binary_expr.right,
    RIGHT_NOT_A_STRING,
    state,
    traversal_state,
    fns,
  )?;

  // Measured before the append rather than after: the two operands are already
  // paid for, so what a ceiling can still refuse is the copy that joins them --
  // and refusing it is what stops a chain of doublings from paying for the next
  // one to find out it was too long.
  joined
    .push(
      &right,
      || Expr::Bin(binary_expr.clone()),
      state,
      traversal_state,
    )
    .map_err(|reason| anyhow!("{}", reason))?;

  Result::Ok(BinaryExprType::String(joined.into_text()))
}

/// One side of the expression, evaluated and taken through `ToString` — the
/// coercion the rest of the evaluator already shares, from `stylex_js`.
///
/// This arm used to keep a second, weaker string coercion of its own, which
/// read a string, a number and a big integer and refused the rest — so
/// `'x' + true` failed to fold where JavaScript says `"xtrue"`. The shared one
/// answers for the whole falsy list, for arrays and for objects, and refuses
/// only where no compile-time string exists at all.
///
/// It is also more permissive than the reference implementation on two
/// operands, and deliberately left that way: a big integer and a regular
/// expression both have a string here, where upstream refuses either literal
/// outright with an unsupported-expression diagnostic. The folded strings are
/// what the language says, so the disagreement costs nothing but a build that
/// succeeds where the other fails.
fn operand_to_string(
  operand: &Expr,
  reason: &str,
  state: &mut EvaluationState,
  traversal_state: &mut StateManager,
  fns: &FunctionMap,
) -> Result<String, anyhow::Error> {
  let value = evaluate_operand(operand, reason, state, traversal_state, fns)?;

  match evaluate_result_to_js_string(&value) {
    Some(strng) => Result::Ok(strng),
    None => Result::Err(anyhow!("{}", reason)),
  }
}

#[cfg(test)]
#[path = "tests/binary_expression_tests.rs"]
mod tests;
