use super::{
  pre_rule::{CompiledResult, PreRule},
  state_manager::StateManager,
};
use stylex_structures::pre_rule_value::PreRuleValue;

#[derive(Debug, Clone, PartialEq, Copy, Default)]
pub(crate) struct NullPreRule {}

impl NullPreRule {
  #[must_use]
  pub(crate) fn new() -> Self {
    Self::default()
  }
}

#[cfg_attr(coverage_nightly, coverage(off))]
impl PreRule for NullPreRule {
  fn get_value(&self) -> Option<PreRuleValue> {
    None
  }
  fn compiled(&mut self, _: &mut StateManager) -> CompiledResult {
    CompiledResult::Null
  }
  fn equals(&self, _other: &dyn PreRule) -> bool {
    false
  }
}
