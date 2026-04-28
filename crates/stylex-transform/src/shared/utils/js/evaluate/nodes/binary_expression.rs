use super::super::*;
use swc_core::ecma::ast::BinExpr;

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
