use std::rc::Rc;

use rustc_hash::FxHashSet;
use swc_core::ecma::ast::Expr;

use stylex_state::functions::FunctionMap;

#[derive(Debug, Clone)]
pub struct EvaluationState {
  pub confident: bool,
  pub deopt_path: Option<Expr>,
  pub added_imports: FxHashSet<String>,
  pub functions: Rc<FunctionMap>,
  pub deopt_reason: Option<String>,
}

impl Default for EvaluationState {
  fn default() -> Self {
    EvaluationState {
      confident: true,
      deopt_path: None,
      added_imports: FxHashSet::default(),
      deopt_reason: None,
      functions: Rc::new(FunctionMap::default()),
    }
  }
}

impl EvaluationState {
  pub fn new() -> Self {
    EvaluationState::default()
  }
}
