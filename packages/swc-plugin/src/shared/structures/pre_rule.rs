use std::fmt::Debug;

use swc_core::ecma::ast::Expr;

use crate::shared::utils::{common::type_of, css::utils::convert_style_to_class_name};

use super::{
  included_style::IncludedStyle, injectable_style::InjectableStyle, null_pre_rule::NullPreRule,
  pre_included_styles_rule::PreIncludedStylesRule, pre_rule_set::PreRuleSet,
  state_manager::StateManager,
};

#[derive(Debug, Clone)]
pub(crate) struct StyleWithDirections {
  pub(crate) rtl: Option<String>,
  pub(crate) ltr: String,
}

#[derive(Debug, Clone)]
pub(crate) enum Styles {
  IncludedStyle(IncludedStyle),
  InjectableStyle(InjectableStyle),
  // Add more types as needed
}

#[derive(Debug, Clone, PartialEq)]
pub(crate) enum PreRuleValue {
  Expr(Expr),
  String(String),
  Vec(Vec<String>),
  Null,
}

#[derive(Debug, Clone, PartialEq)]
pub(crate) struct ComputedStyle(pub(crate) String, pub(crate) InjectableStyle);

#[derive(Debug, Clone, PartialEq)]
pub(crate) enum CompiledResult {
  Null,
  IncludedStyle(IncludedStyle),
  ComputedStyles(Vec<ComputedStyle>),
}

impl CompiledResult {
  pub(crate) fn as_included_style(&self) -> Option<&IncludedStyle> {
    match self {
      CompiledResult::IncludedStyle(included_style) => Some(included_style),
      _ => None,
    }
  }

  pub(crate) fn as_computed_styles(&self) -> Option<&Vec<ComputedStyle>> {
    match self {
      CompiledResult::ComputedStyles(computed_styles) => Some(computed_styles),
      _ => None,
    }
  }
}

pub(crate) trait PreRule: Debug {
  fn get_value(&self) -> Option<PreRuleValue>;
  fn compiled(&mut self, state: &StateManager) -> CompiledResult;
  fn equals(&self, other: &dyn PreRule) -> bool;
}

#[derive(Debug, Clone, PartialEq)]
pub(crate) enum PreRules {
  PreIncludedStylesRule(PreIncludedStylesRule),
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
}

impl StylesPreRule {
  pub(crate) fn new(
    property: &str,
    value: PreRuleValue,
    pseudos: Option<Vec<String>>,
    at_rules: Option<Vec<String>>,
  ) -> Self {
    let pseudos = pseudos.unwrap_or_default();
    let at_rules = at_rules.unwrap_or_default();
    let property = property.to_string();

    // dbg!(&property, &value, &pseudos, &at_rules);

    StylesPreRule {
      property: property.clone(),
      value,
      pseudos: if property.as_str().starts_with(':') {
        let mut extended_pseudos = vec![property.clone()];
        extended_pseudos.extend(pseudos);
        extended_pseudos
      } else {
        pseudos
      },
      at_rules: if property.as_str().starts_with('@') {
        let mut extender_at_rules = vec![property];
        extender_at_rules.extend(at_rules);
        extender_at_rules
      } else {
        at_rules
      },
    }
  }
  pub(crate) fn _get_property(&self) -> Option<String> {
    Option::Some(self.property.clone())
  }
  pub(crate) fn _get_pseudos(&self) -> Option<Vec<String>> {
    Option::Some(self.pseudos.clone())
  }
  pub(crate) fn _get_at_rules(&self) -> Option<Vec<String>> {
    Option::Some(self.at_rules.clone())
  }
}

impl PreRule for StylesPreRule {
  fn get_value(&self) -> Option<PreRuleValue> {
    Option::Some(self.value.clone())
  }

  fn compiled(&mut self, state: &StateManager) -> CompiledResult {
    let (_, class_name, rule) = convert_style_to_class_name(
      (self.property.as_str(), &self.value),
      &mut self.pseudos,
      &mut self.at_rules,
      &state.options.class_name_prefix,
      state,
    );

    CompiledResult::ComputedStyles(vec![ComputedStyle(class_name, rule)])
  }

  fn equals(&self, other: &dyn PreRule) -> bool {
    type_of(other) == type_of(self)
  }
}
