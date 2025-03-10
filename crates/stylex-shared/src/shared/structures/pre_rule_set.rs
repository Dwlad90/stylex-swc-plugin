use crate::shared::structures::pre_rule::{CompiledResult, ComputedStyle};

use super::{
  null_pre_rule::NullPreRule,
  pre_rule::{PreRule, PreRuleValue, PreRules},
  state_manager::StateManager,
};
#[derive(Debug, Clone, PartialEq)]
pub(crate) struct PreRuleSet {
  rules: Vec<PreRules>,
}

impl PreRuleSet {
  pub(crate) fn _new() -> Self {
    PreRuleSet { rules: vec![] }
  }
  pub(crate) fn create(rules: Vec<PreRules>) -> PreRules {
    let flat_rules = rules
      .into_iter()
      .flat_map(|rule| match rule {
        PreRules::PreRuleSet(rule_set) => rule_set.rules,
        _ => vec![rule],
      })
      .collect::<Vec<PreRules>>();

    match flat_rules.len() {
      0 => PreRules::NullPreRule(NullPreRule::new()),
      1 => flat_rules
        .first()
        .expect("First rule item not exist")
        .to_owned(),
      _ => PreRules::PreRuleSet(PreRuleSet { rules: flat_rules }),
    }
  }
}

impl PreRule for PreRuleSet {
  fn equals(&self, _other: &dyn PreRule) -> bool {
    true
  }
  fn compiled(&mut self, state: &mut StateManager) -> CompiledResult {
    let style_tuple = self
      .rules
      .iter_mut()
      .flat_map(|rule| {
        let compiled_rule = match rule {
          PreRules::PreRuleSet(rule_set) => rule_set.compiled(state),
          PreRules::StylesPreRule(styles_pre_rule) => styles_pre_rule.compiled(state),
          PreRules::NullPreRule(null_pre_rule) => null_pre_rule.compiled(state),
        };

        match compiled_rule {
          CompiledResult::ComputedStyles(styles) => styles,
          _ => vec![],
        }
      })
      .collect::<Vec<ComputedStyle>>();

    CompiledResult::ComputedStyles(style_tuple)
  }
  fn get_value(&self) -> Option<PreRuleValue> {
    let rule = self.rules.first().unwrap();

    match &rule {
      PreRules::PreRuleSet(rule_set) => rule_set.get_value(),
      PreRules::StylesPreRule(styles_pre_rule) => styles_pre_rule.get_value(),
      PreRules::NullPreRule(null_pre_rule) => null_pre_rule.get_value(),
    }
  }
}
