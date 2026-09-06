//! Not every test binary uses every helper here, so an item unused by the
//! one being compiled is expected rather than dead. Said once for the
//! module: this is a shared helper library, and per-item attributes were
//! the same fact repeated at each of them.
#![allow(dead_code)]

use stylex_evaluator::convertors::expr_to_num;
use stylex_evaluator::state::EvaluationState;
use stylex_state::resolution::convertors::convert_expr_to_str;
use stylex_state::{functions::FunctionMap, state_manager::StateManager};
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

/// An `env` function reads a boolean argument the way the language does, and
/// the language's `ToBoolean` is the coercion crate's -- there is one
/// truthiness table and this reads it. `false` for a value whose kind cannot be
/// read: a test helper has no deopt to take.
pub(crate) fn convert_expr_to_bool_wrapper(expr: &Expr) -> bool {
  stylex_js::coercions::to_js_boolean(expr).unwrap_or(false)
}
