use super::{
  pre_rule::{CompiledResult, PreRule, PreRuleValue},
  state_manager::StateManager,
};

#[derive(Debug, Clone)]
pub(crate) struct NullPreRule {}

impl NullPreRule {
  pub(crate) fn new() -> Self {
    NullPreRule {}
  }
}

impl PreRule for NullPreRule {
  fn get_value(&self) -> Option<PreRuleValue> {
    None
  }
  fn compiled(&mut self, _prefix: &str, _: &StateManager) -> CompiledResult {
    CompiledResult::Null
  }
  fn equals(&self, _other: &dyn PreRule) -> bool {
    false
  }
}
