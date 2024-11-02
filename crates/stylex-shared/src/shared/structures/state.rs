use std::collections::{HashMap, HashSet};

use swc_core::ecma::ast::Expr;

use super::functions::FunctionMap;

#[derive(Debug)]
pub struct EvaluationState {
  pub(crate) confident: bool,
  pub(crate) deopt_path: Option<Expr>,
  pub(crate) added_imports: HashSet<String>,
  pub(crate) functions: FunctionMap,
}

impl Default for EvaluationState {
  fn default() -> Self {
    EvaluationState {
      confident: true,
      deopt_path: None,
      added_imports: HashSet::new(),
      functions: FunctionMap {
        identifiers: HashMap::new(),
        member_expressions: HashMap::new(),
      },
    }
  }
}

impl EvaluationState {
  pub(crate) fn new() -> Self {
    EvaluationState {
      confident: true,
      deopt_path: None,
      added_imports: HashSet::new(),
      functions: FunctionMap {
        identifiers: HashMap::new(),
        member_expressions: HashMap::new(),
      },
    }
  }
}
