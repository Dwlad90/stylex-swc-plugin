use std::fmt::Debug;

use indexmap::IndexMap;

use crate::shared::utils::core::convert_style_to_class_name::convert_style_to_class_name;
use stylex_css::utils::{
  pre_rule::{sort_at_rules, sort_pseudos},
  pseudo::is_pseudo_selector,
};
use stylex_state::{state_manager::StateManager, types::ClassNameToOriginalPaths};
use stylex_types::structures::style_key::ClassName;
use stylex_utils::types::type_of;

use super::{null_pre_rule::NullPreRule, pre_rule_set::PreRuleSet};
use stylex_structures::pre_rule_value::PreRuleValue;
use stylex_types::structures::injectable_style::InjectableStyle;

#[derive(Debug, Clone, PartialEq)]
pub(crate) struct ComputedStyle(
  pub(crate) ClassName,
  pub(crate) InjectableStyle,
  pub(crate) ClassNameToOriginalPaths,
);

#[derive(Debug, Clone, PartialEq)]
pub(crate) enum CompiledResult {
  Null,
  ComputedStyles(Vec<ComputedStyle>),
}

impl CompiledResult {
  pub(crate) fn _as_computed_styles(&self) -> Option<&Vec<ComputedStyle>> {
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
  const_rules: Vec<String>,
  key_path: Vec<String>,
}

impl StylesPreRule {
  /// Collects the key path segments a rule kind owns, in authored order.
  ///
  /// Each of the three kinds walks the same path and keeps a disjoint slice of
  /// it, so the filter is passed in rather than the path being cloned once per
  /// kind.
  fn select_key_path(key_path: &Option<Vec<String>>, keep: impl Fn(&str) -> bool) -> Vec<String> {
    key_path
      .iter()
      .flatten()
      .filter(|key| keep(key))
      .cloned()
      .collect()
  }

  fn get_pseudos(key_path: &Option<Vec<String>>) -> Vec<String> {
    // The single colon is deliberate: selector assembly needs both kinds, so
    // narrowing this to `::` would drop every pseudo class on the floor.
    let unsorted_pseudos = Self::select_key_path(key_path, |key| {
      is_pseudo_selector(key) || key.starts_with('[')
    });

    sort_pseudos(&unsorted_pseudos)
  }

  fn get_at_rules(key_path: &Option<Vec<String>>) -> Vec<String> {
    let unsorted_at_rules = Self::select_key_path(key_path, |key| key.starts_with('@'));

    sort_at_rules(&unsorted_at_rules)
  }

  /// The `var(--…)` keys of a key path.
  ///
  /// Sorted here, which is a divergence and predates this file's current shape:
  /// upstream's `get constRules()` returns the filtered path **unsorted**, and
  /// only `convertStyleToClassName` sorts, for the hash. `generateCSSRule`
  /// receives the unsorted list and nests the declaration in that order, so two
  /// `var(--…)` keys written out of alphabetical order nest one way here and the
  /// other way in Babel. The class name is unaffected — both sides re-sort the
  /// combined list before hashing — so the emitted rule differs in nesting order
  /// alone. Left as it is rather than changed alongside unrelated work; the sort
  /// is also redundant with the one the caller does.
  fn get_const_rules(key_path: &Option<Vec<String>>) -> Vec<String> {
    let unsorted_const_rules = Self::select_key_path(key_path, |key| key.starts_with("var(--"));

    sort_at_rules(&unsorted_const_rules)
  }
  pub(crate) fn new(property: &str, value: PreRuleValue, key_path: Option<Vec<String>>) -> Self {
    let property_str = property.to_string();

    StylesPreRule {
      property: property_str,
      value,
      pseudos: StylesPreRule::get_pseudos(&key_path),
      at_rules: StylesPreRule::get_at_rules(&key_path),
      const_rules: StylesPreRule::get_const_rules(&key_path),
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

#[cfg_attr(coverage_nightly, coverage(off))]
impl PreRule for StylesPreRule {
  fn get_value(&self) -> Option<PreRuleValue> {
    Some(self.value.to_owned())
  }

  fn compiled(&mut self, state: &mut StateManager) -> CompiledResult {
    let Some((_, class_name, rule)) = convert_style_to_class_name(
      (self.property.as_str(), &self.value),
      &mut self.pseudos,
      &mut self.at_rules,
      &mut self.const_rules,
      state,
    ) else {
      // The value carries no CSS text, so there is no declaration to name.
      return CompiledResult::Null;
    };

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
