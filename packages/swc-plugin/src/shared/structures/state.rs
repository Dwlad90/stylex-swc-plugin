use std::collections::{HashMap, HashSet};

use swc_core::ecma::ast::Expr;

use super::{functions::FunctionMap, state_manager::StateManager};

#[derive(Debug)]
pub struct EvaluationState {
  pub(crate) confident: bool,
  pub(crate) deopt_path: Option<Box<Expr>>,
  pub(crate) added_imports: HashSet<String>,
  pub(crate) functions: FunctionMap,
  pub(crate) traversal_state: StateManager,
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
      traversal_state: StateManager::default(),
    }
  }
}

impl EvaluationState {
  pub(crate) fn new(traversal_state: &StateManager) -> Self {
    EvaluationState {
      confident: true,
      deopt_path: None,
      added_imports: HashSet::new(),
      functions: FunctionMap {
        identifiers: HashMap::new(),
        member_expressions: HashMap::new(),
      },
      traversal_state: traversal_state.clone(),
    }
  }
}
