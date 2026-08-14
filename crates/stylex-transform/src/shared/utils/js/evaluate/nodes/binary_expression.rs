use super::super::*;
use anyhow::anyhow;
use stylex_macros::{as_expr_or_err, as_expr_or_opt_err, convert_expr_to_str_or_err};
use swc_core::ecma::ast::{BinExpr, BinaryOp};

pub(in super::super) fn evaluate(
  bin: &BinExpr,
  state: &mut EvaluationState,
  traversal_state: &mut StateManager,
  fns: &FunctionMap,
) -> Option<EvaluateResultValue> {
  unwrap_or_panic!(
    binary_expr_to_num(bin, state, traversal_state, fns)
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

pub(crate) fn binary_expr_to_num(
  binary_expr: &BinExpr,
  state: &mut EvaluationState,
  traversal_state: &mut StateManager,
  fns: &FunctionMap,
) -> Result<BinaryExprType, anyhow::Error> {
  let op = binary_expr.op;
  let Some(left) = evaluate_cached(&binary_expr.left, state, traversal_state, fns) else {
    if !state.confident {
      return Result::Err(anyhow::anyhow!("Left expression is not a number"));
    }

    stylex_panic!("Left expression is not a number")
  };

  let left_expr = as_expr_or_err!(left, "Left argument not expression");
  let left_num = expr_to_num(left_expr, state, traversal_state, fns)?;

  let Some(right) = evaluate_cached(&binary_expr.right, state, traversal_state, fns) else {
    if !state.confident {
      if op == BinaryOp::LogicalOr && left_num != 0.0 {
        state.confident = true;

        return Result::Ok(BinaryExprType::Number(left_num));
      }

      return Result::Err(anyhow::anyhow!("Right expression is not a number"));
    }

    stylex_panic!("Right expression is not a number")
  };

  let right_expr = as_expr_or_err!(right, "Right argument not expression");
  let right_num = expr_to_num(right_expr, state, traversal_state, fns)?;

  let result = match &op {
    BinaryOp::Add => {
      if let Some(value) =
        evaluate_left_and_right_expression(state, traversal_state, fns, &left, &right)
      {
        return value;
      }

      left_num + right_num
    },
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
    BinaryOp::In => {
      if right_num == 0.0 {
        1.0
      } else {
        0.0
      }
    },
    BinaryOp::InstanceOf => {
      if right_num == 0.0 {
        1.0
      } else {
        0.0
      }
    },
    BinaryOp::EqEq => {
      if left_num == right_num {
        1.0
      } else {
        0.0
      }
    },
    BinaryOp::NotEq => {
      if left_num != right_num {
        1.0
      } else {
        0.0
      }
    },
    BinaryOp::EqEqEq => {
      if left_num == right_num {
        1.0
      } else {
        0.0
      }
    },
    BinaryOp::NotEqEq => {
      if left_num != right_num {
        1.0
      } else {
        0.0
      }
    },
    BinaryOp::Lt => {
      if left_num < right_num {
        1.0
      } else {
        0.0
      }
    },
    BinaryOp::LtEq => {
      if left_num <= right_num {
        1.0
      } else {
        0.0
      }
    },
    BinaryOp::Gt => {
      if left_num > right_num {
        1.0
      } else {
        0.0
      }
    },
    BinaryOp::GtEq => {
      if left_num >= right_num {
        1.0
      } else {
        0.0
      }
    },
    // #region Logical
    BinaryOp::LogicalOr => {
      if let Some(value) =
        evaluate_left_and_right_expression(state, traversal_state, fns, &left, &right)
      {
        return value;
      }

      if left_num != 0.0 { left_num } else { right_num }
    },
    BinaryOp::LogicalAnd => {
      if let Some(value) =
        evaluate_left_and_right_expression(state, traversal_state, fns, &left, &right)
      {
        return value;
      }

      if left_num != 0.0 { right_num } else { left_num }
    },
    BinaryOp::NullishCoalescing => {
      if let Some(value) =
        evaluate_left_and_right_expression(state, traversal_state, fns, &left, &right)
      {
        return value;
      }

      if left_num == 0.0 { right_num } else { left_num }
    },
    // #endregion Logical
    BinaryOp::ZeroFillRShift => ((left_num as i32) >> right_num as i32) as f64,
  };

  Result::Ok(BinaryExprType::Number(result))
}

fn binary_expr_to_string(
  binary_expr: &BinExpr,
  state: &mut EvaluationState,
  traversal_state: &mut StateManager,
  fns: &FunctionMap,
) -> Result<BinaryExprType, anyhow::Error> {
  let op = binary_expr.op;
  let Some(left) = evaluate_cached(&binary_expr.left, state, traversal_state, fns) else {
    if !state.confident {
      return Result::Err(anyhow::anyhow!("Left expression is not a string"));
    }

    stylex_panic!("Left expression is not a string")
  };

  let left_expr = as_expr_or_err!(left, "Left argument not expression");
  let left_str = convert_expr_to_str_or_err!(
    left_expr,
    traversal_state,
    fns,
    "Left expression is not a string"
  );

  let Some(right) = evaluate_cached(&binary_expr.right, state, traversal_state, fns) else {
    if !state.confident {
      if op == BinaryOp::LogicalOr {
        state.confident = true;

        return Result::Ok(BinaryExprType::String(left_str));
      }

      return Result::Err(anyhow::anyhow!("Right expression is not a string"));
    }

    stylex_panic!("Right expression is not a string")
  };

  let right_expr = as_expr_or_err!(right, "Right argument not expression");
  let right_str = convert_expr_to_str_or_err!(
    right_expr,
    traversal_state,
    fns,
    "Right expression is not a string"
  );

  let result = match &op {
    BinaryOp::Add => {
      format!("{}{}", left_str, right_str)
    },
    _ => stylex_panic!(
      "For string expressions, only addition is supported, got {:?}",
      op
    ),
  };

  Result::Ok(BinaryExprType::String(result))
}

fn evaluate_left_and_right_expression(
  state: &mut EvaluationState,
  traversal_state: &mut StateManager,
  fns: &FunctionMap,
  left: &EvaluateResultValue,
  right: &EvaluateResultValue,
) -> Option<Result<BinaryExprType, anyhow::Error>> {
  let left_expr = as_expr_or_opt_err!(left, "Left argument not expression");
  let right_expr = as_expr_or_opt_err!(right, "Right argument not expression");

  let mut state_for_left = EvaluationState {
    confident: true,
    deopt_path: None,
    ..state.clone()
  };
  let left_result = expr_to_num(left_expr, &mut state_for_left, traversal_state, fns);
  let left_confident = state.confident;

  let mut state_for_right = EvaluationState {
    confident: true,
    deopt_path: None,
    ..state.clone()
  };
  let right_result = expr_to_num(right_expr, &mut state_for_right, traversal_state, fns);
  let right_confident = state.confident;

  if left_result.is_err() || right_result.is_err() {
    let left_str = match left_expr {
      Expr::Lit(Lit::Str(_)) => match left_expr.as_lit() {
        Some(lit) => convert_lit_to_string(lit).unwrap_or_else(|| {
          {
            stylex_panic!(
              "Left is not a string: {:?}",
              left_expr.get_type(get_default_expr_ctx())
            )
          }
        }),
        None => stylex_panic!(
          "Left is not a string: {:?}",
          left_expr.get_type(get_default_expr_ctx())
        ),
      },
      _ => String::default(),
    };

    let right_str = match right_expr {
      Expr::Lit(Lit::Str(_)) => match right_expr.as_lit() {
        Some(lit) => convert_lit_to_string(lit).unwrap_or_else(|| {
          {
            stylex_panic!(
              "Right is not a string: {:?}",
              left_expr.get_type(get_default_expr_ctx())
            )
          }
        }),
        None => stylex_panic!(
          "Right is not a string: {:?}",
          left_expr.get_type(get_default_expr_ctx())
        ),
      },
      _ => String::default(),
    };

    if !left_str.is_empty() && !right_str.is_empty() {
      return Some(Result::Ok(BinaryExprType::String(format!(
        "{}{}",
        left_str, right_str
      ))));
    }
  }

  if !left_confident {
    let deopt_reason = state_for_left
      .deopt_reason
      .as_deref()
      .unwrap_or("unknown error")
      .to_string();

    deopt(left_expr, state, &deopt_reason);

    return Some(Result::Ok(BinaryExprType::Null));
  }

  if !right_confident {
    let deopt_reason = state_for_right
      .deopt_reason
      .as_deref()
      .unwrap_or("unknown error")
      .to_string();

    deopt(right_expr, state, &deopt_reason);

    return Some(Result::Ok(BinaryExprType::Null));
  }

  None
}

#[cfg(test)]
#[path = "tests/binary_expression_tests.rs"]
mod tests;
