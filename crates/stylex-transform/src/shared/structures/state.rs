use std::rc::Rc;

use rustc_hash::FxHashSet;
use swc_core::ecma::ast::Expr;

use super::functions::FunctionMap;

#[derive(Debug, Clone)]
pub struct EvaluationState {
  pub(crate) confident: bool,
  pub(crate) deopt_path: Option<Expr>,
  pub(crate) added_imports: FxHashSet<String>,
  pub(crate) functions: Rc<FunctionMap>,
  pub(crate) deopt_reason: Option<String>,
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
  pub(crate) fn new() -> Self {
    EvaluationState::default()
  }
}
