use rustc_hash::{FxHashMap, FxHashSet};
use swc_core::ecma::ast::Expr;

use super::functions::FunctionMap;

#[derive(Debug)]
pub struct EvaluationState {
  pub(crate) confident: bool,
  pub(crate) deopt_path: Option<Expr>,
  pub(crate) added_imports: FxHashSet<String>,
  pub(crate) functions: FunctionMap,
}

impl Default for EvaluationState {
  fn default() -> Self {
    EvaluationState {
      confident: true,
      deopt_path: None,
      added_imports: FxHashSet::default(),
      functions: FunctionMap {
        identifiers: FxHashMap::default(),
        member_expressions: FxHashMap::default(),
      },
    }
  }
}

impl EvaluationState {
  pub(crate) fn new() -> Self {
    EvaluationState {
      confident: true,
      deopt_path: None,
      added_imports: FxHashSet::default(),
      functions: FunctionMap {
        identifiers: FxHashMap::default(),
        member_expressions: FxHashMap::default(),
      },
    }
  }
}
