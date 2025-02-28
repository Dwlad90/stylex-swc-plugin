use std::fmt::Debug;

use indexmap::IndexMap;
use swc_core::ecma::ast::Expr;

use crate::shared::utils::{
  common::type_of,
  core::convert_style_to_class_name::convert_style_to_class_name,
  pre_rule::{sort_at_rules, sort_pseudos},
};

use super::{
  injectable_style::InjectableStyle, null_pre_rule::NullPreRule, pre_rule_set::PreRuleSet,
  state_manager::StateManager, types::ClassesToOriginalPaths,
};

#[derive(Debug, Clone, PartialEq)]
pub(crate) enum PreRuleValue {
  #[allow(dead_code)]
  Expr(Expr),
  String(String),
  Vec(Vec<String>),
  Null,
}

#[derive(Debug, Clone, PartialEq)]
pub(crate) struct ComputedStyle(
  pub(crate) String,
  pub(crate) InjectableStyle,
  pub(crate) ClassesToOriginalPaths,
);

#[derive(Debug, Clone, PartialEq)]
pub(crate) enum CompiledResult {
  Null,
  ComputedStyles(Vec<ComputedStyle>),
}

impl CompiledResult {
  pub(crate) fn as_computed_styles(&self) -> Option<&Vec<ComputedStyle>> {
    match self {
      CompiledResult::ComputedStyles(computed_styles) => Some(computed_styles),
      _ => None,
    }
  }
}

pub(crate) trait PreRule: Debug {
  #[allow(dead_code)]
  fn get_value(&self) -> Option<PreRuleValue>;
  fn compiled(&mut self, state: &mut StateManager) -> CompiledResult;
  #[allow(dead_code)]
  fn equals(&self, other: &dyn PreRule) -> bool;
}

#[derive(Debug, Clone, PartialEq)]
pub(crate) enum PreRules {
  PreRuleSet(PreRuleSet),
  StylesPreRule(StylesPreRule),
  NullPreRule(NullPreRule),
}

#[derive(Debug, Clone, PartialEq)]
pub(crate) struct StylesPreRule {
  property: String,
  value: PreRuleValue,
  pseudos: Vec<String>,
  at_rules: Vec<String>,
  key_path: Vec<String>,
}

impl StylesPreRule {
  fn get_pseudos(key_path: &Option<Vec<String>>) -> Vec<String> {
    let mut unsorted_pseudos = key_path.clone().unwrap_or_default();

    unsorted_pseudos = unsorted_pseudos
      .iter()
      .filter(|key| key.starts_with(':'))
      .cloned()
      .collect();

    sort_pseudos(&unsorted_pseudos)
  }

  fn get_at_rules(key_path: &Option<Vec<String>>) -> Vec<String> {
    let mut unsorted_at_rules = key_path.clone().unwrap_or_default();

    unsorted_at_rules = unsorted_at_rules
      .iter()
      .filter(|key| key.starts_with('@'))
      .cloned()
      .collect();

    sort_at_rules(&unsorted_at_rules)
  }
  pub(crate) fn new(property: &str, value: PreRuleValue, key_path: Option<Vec<String>>) -> Self {
    let property_str = property.to_string();

    StylesPreRule {
      property: property_str,
      value,
      pseudos: StylesPreRule::get_pseudos(&key_path),
      at_rules: StylesPreRule::get_at_rules(&key_path),
      key_path: key_path.unwrap_or_default(),
    }
  }
  pub(crate) fn _get_property(&self) -> Option<&str> {
    Some(&self.property)
  }
  pub(crate) fn _get_pseudos(&self) -> Option<Vec<String>> {
    Some(self.pseudos.to_owned())
  }
  pub(crate) fn _get_at_rules(&self) -> Option<Vec<String>> {
    Some(self.at_rules.to_owned())
  }
}

impl PreRule for StylesPreRule {
  fn get_value(&self) -> Option<PreRuleValue> {
    Some(self.value.to_owned())
  }

  fn compiled(&mut self, state: &mut StateManager) -> CompiledResult {
    let (_, class_name, rule) = convert_style_to_class_name(
      (self.property.as_str(), &self.value),
      &mut self.pseudos,
      &mut self.at_rules,
      state,
    );

    let mut classes_to_original_paths = IndexMap::new();

    classes_to_original_paths.insert(class_name.clone(), self.key_path.clone());

    CompiledResult::ComputedStyles(vec![ComputedStyle(
      class_name,
      rule,
      classes_to_original_paths,
    )])
  }

  fn equals(&self, other: &dyn PreRule) -> bool {
    type_of(other) == type_of(self)
  }
}
