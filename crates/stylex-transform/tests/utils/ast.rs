use stylex_transform::shared::{
  structures::{functions::FunctionMap, state::EvaluationState, state_manager::StateManager},
  utils::ast::convertors::{convert_expr_to_str, expr_to_num},
};
use swc_core::ecma::ast::Expr;

#[allow(dead_code)]
pub(crate) fn convert_expr_to_str_wrapper(expr: &Expr) -> Option<String> {
  convert_expr_to_str(expr, &mut StateManager::default(), &FunctionMap::default())
}

#[allow(dead_code)]
pub(crate) fn convert_expr_to_num_wrapper(expr: &Expr) -> Option<f64> {
  expr_to_num(
    expr,
    &mut EvaluationState::default(),
    &mut StateManager::default(),
    &FunctionMap::default(),
  )
  .ok()
}

/// An `env` function reads a boolean argument the way the language does, and
/// the language's `ToBoolean` is the coercion crate's -- there is one
/// truthiness table and this reads it. `false` for a value whose kind cannot be
/// read: a test helper has no deopt to take.
#[allow(dead_code)]
pub(crate) fn convert_expr_to_bool_wrapper(expr: &Expr) -> bool {
  stylex_js::coercions::to_js_boolean(expr).unwrap_or(false)
}
