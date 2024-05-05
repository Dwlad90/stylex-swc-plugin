use super::{
  pre_rule::{CompiledResult, PreRule, PreRuleValue},
  state_manager::StateManager,
};

#[derive(Debug, Clone, PartialEq)]
pub(crate) struct NullPreRule {}

impl NullPreRule {
  pub(crate) fn new() -> Self {
    NullPreRule {}
  }
}

impl Default for NullPreRule {
  fn default() -> Self {
    NullPreRule::new()
  }
}

impl PreRule for NullPreRule {
  fn get_value(&self) -> Option<PreRuleValue> {
    None
  }
  fn compiled(&mut self, _: &StateManager) -> CompiledResult {
    CompiledResult::Null
  }
  fn equals(&self, _other: &dyn PreRule) -> bool {
    false
  }
}
