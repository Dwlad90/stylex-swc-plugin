use swc_core::css::ast::{
  ComponentValue, Declaration, Function, FunctionName, QualifiedRule, Rule, Stylesheet,
};

use crate::css::common::get_value_from_ident;
use stylex_constants::constants::messages::UNPREFIXED_CUSTOM_PROPERTIES;

fn process_function(func: &Function) {
  if let FunctionName::Ident(func_name_ident) = &func.name {
    let func_name = get_value_from_ident(func_name_ident);
    if func_name == "var"
      && let Some(ComponentValue::Ident(ident)) = func.value.first()
    {
      let value = get_value_from_ident(ident.as_ref());
      assert!(value.starts_with("--"), "{}", UNPREFIXED_CUSTOM_PROPERTIES);
    }
  }
}

fn process_declaration(declaration: &Declaration) {
  for value in declaration.value.iter() {
    if let ComponentValue::Function(func) = value {
      process_function(func);
    }
  }
}

/// SWC always emits `ComponentValue::Declaration` inside qualified-rule
/// blocks, so the non-declaration arm is defensive only.
fn as_declaration(cv: &ComponentValue) -> Option<&Declaration> {
  match cv {
    ComponentValue::Declaration(d) => Some(d),
    _ => None,
  }
}

fn process_qualified_rule(qualified_rule: &QualifiedRule) {
  qualified_rule
    .block
    .value
    .iter()
    .filter_map(as_declaration)
    .for_each(process_declaration);
}

/// Validates that all `var()` references in a stylesheet use properly
/// prefixed custom properties (i.e. names starting with `--`).
pub fn unprefixed_custom_properties_validator(ast: &Stylesheet) {
  for rule in ast.rules.iter() {
    if let Rule::QualifiedRule(qualified_rule) = rule {
      process_qualified_rule(qualified_rule);
    }
  }
}

#[cfg(test)]
#[path = "../../tests/unprefixed_custom_properties_tests.rs"]
mod tests;
