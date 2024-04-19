use std::fmt::Debug;

use swc_core::ecma::ast::Expr;

use crate::shared::utils::common::type_of;

use super::{
  included_style::IncludedStyle,
  pre_rule::{CompiledResult, PreRule, PreRuleValue},
  state_manager::StateManager,
};

#[derive(Debug, Clone)]
pub(crate) struct PreIncludedStylesRule {
  pub(crate) included_styles: Expr,
}

impl PreIncludedStylesRule {
  pub(crate) fn new(included_styles: Expr) -> Self {
    PreIncludedStylesRule { included_styles }
  }
}

impl PreRule for PreIncludedStylesRule {
  fn get_value(&self) -> Option<PreRuleValue> {
    Option::Some(PreRuleValue::Expr(self.included_styles.clone()))
  }
  fn compiled(&mut self, _prefix: &str, _: &StateManager) -> CompiledResult {
    CompiledResult::IncludedStyle(IncludedStyle::new(self.included_styles.clone()))
  }
  fn equals(&self, other: &dyn PreRule) -> bool {
    type_of(other) == type_of(self)
  }
}
