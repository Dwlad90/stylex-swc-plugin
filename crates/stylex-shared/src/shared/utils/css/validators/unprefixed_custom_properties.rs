use swc_core::css::ast::{
  ComponentValue, Declaration, Function, FunctionName, QualifiedRule, Rule, Stylesheet,
};

use crate::shared::constants::messages::UNPREFIXED_CUSTOM_PROPERTIES;
use crate::shared::utils::css::common::get_value_from_ident;
#[cfg(test)]
use crate::shared::utils::css::common::swc_parse_css;
fn process_function(func: &Function) {
  if let FunctionName::Ident(func_name_ident) = &func.name {
    let func_name = get_value_from_ident(func_name_ident);
    if func_name == "var" {
      if let Some(ComponentValue::Ident(ident)) = func.value.first() {
        let value = get_value_from_ident(ident.as_ref());
        assert!(value.starts_with("--"), "{}", UNPREFIXED_CUSTOM_PROPERTIES);
      }
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

fn process_qualified_rule(qualified_rule: &QualifiedRule) {
  for declaration in qualified_rule.block.value.iter() {
    if let ComponentValue::Declaration(declaration) = declaration {
      process_declaration(declaration);
    }
  }
}

pub(crate) fn unprefixed_custom_properties_validator(ast: Stylesheet) {
  for rule in ast.rules.iter() {
    if let Rule::QualifiedRule(qualified_rule) = rule {
      process_qualified_rule(qualified_rule);
    }
  }
}

#[test]
#[should_panic(expected = "Unprefixed custom properties")]
fn disallow_unprefixed_custom_properties() {
  let (result, _) = swc_parse_css("* { color: var(foo); }");

  unprefixed_custom_properties_validator(result.unwrap());
}
