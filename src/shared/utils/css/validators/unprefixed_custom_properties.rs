use swc_core::css::ast::{ComponentValue, FunctionName, Rule, Stylesheet};

use crate::shared::utils::css::get_value_from_ident;
#[cfg(test)]
use crate::shared::utils::css::swc_parse_css;

pub(crate) fn unprefixed_custom_properties_validator(ast: Stylesheet) {
  ast.rules.iter().for_each(|rule| match rule {
    Rule::QualifiedRule(qualified_rule) => {
      qualified_rule
        .block
        .value
        .iter()
        .for_each(|declaration| match &declaration {
          ComponentValue::Declaration(declaration) => {
            declaration.value.iter().for_each(|value| match &value {
              ComponentValue::Function(func) => match &func.name {
                FunctionName::Ident(func_name_ident) => {
                  let func_name = get_value_from_ident(&func_name_ident);

                  if func_name == "var" {
                    let function_value = func.value.get(0).unwrap();

                    match function_value {
                      ComponentValue::Ident(ident) => {
                        let value = get_value_from_ident(ident.as_ref());

                        assert!(value.starts_with("--"), "Unprefixed custom properties");
                      }
                      _ => {}
                    }
                  }
                }
                _ => {}
              },
              _ => {}
            });
          }
          _ => {}
        })
    }
    _ => {}
  });
}

#[test]
#[should_panic(expected = "Unprefixed custom properties")]
fn disallow_unprefixed_custom_properties() {
  let (result, _) = swc_parse_css("* { color: var(foo); }");

  unprefixed_custom_properties_validator(result.unwrap());
}
