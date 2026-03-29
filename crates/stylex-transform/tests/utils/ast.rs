use stylex_transform::shared::{
  structures::{functions::FunctionMap, state::EvaluationState, state_manager::StateManager},
  utils::ast::convertors::{convert_expr_to_bool, convert_expr_to_str, expr_to_num},
};
use swc_core::ecma::ast::Expr;

pub(crate) fn convert_expr_to_str_wrapper(expr: &Expr) -> Option<String> {
  convert_expr_to_str(expr, &mut StateManager::default(), &FunctionMap::default())
}

pub(crate) fn convert_expr_to_num_wrapper(expr: &Expr) -> Option<f64> {
  expr_to_num(
    expr,
    &mut EvaluationState::default(),
    &mut StateManager::default(),
    &FunctionMap::default(),
  )
  .ok()
}

pub(crate) fn convert_expr_to_bool_wrapper(expr: &Expr) -> bool {
  convert_expr_to_bool(expr, &mut StateManager::default(), &FunctionMap::default())
}
