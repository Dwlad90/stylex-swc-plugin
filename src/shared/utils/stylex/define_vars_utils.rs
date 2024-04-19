use std::ops::Mul;

use indexmap::IndexMap;
use swc_core::ecma::ast::{Expr, Lit};

use crate::shared::{
  constants::common::SPLIT_TOKEN,
  enums::FlatCompiledStylesValue,
  structures::injectable_style::InjectableStyle,
  utils::common::{create_hash, get_key_str, get_key_values_from_object, get_string_val_from_lit},
};

pub(crate) fn construct_css_variables_string(
  variables: &IndexMap<String, FlatCompiledStylesValue>,
  theme_name_hash: &String,
) -> IndexMap<String, InjectableStyle> {
  let mut rules_by_at_rule: IndexMap<String, Vec<String>> = IndexMap::new();

  for (key, value) in variables.iter() {
    collect_vars_by_at_rules(&key, &value, &mut rules_by_at_rule, &vec![]);
  }

  dbg!(&rules_by_at_rule);

  let mut result: IndexMap<String, InjectableStyle> = IndexMap::new();

  for (at_rule, value) in rules_by_at_rule.iter() {
    let suffix = if at_rule == "default" {
      "".to_string()
    } else {
      format!("-{}", create_hash(at_rule))
    };

    let mut ltr = format!(":root{{{}}}", value.join(""));

    if at_rule != "default" {
      ltr = wrap_with_at_rules(ltr.as_str(), at_rule);
    }

    result.insert(
      format!("{}{}", theme_name_hash, suffix),
      InjectableStyle {
        priority: Option::Some(priority_for_at_rule(at_rule).mul(0.1)),
        ltr,
        rtl: Option::None,
      },
    );
  }

  result
}

pub(crate) fn collect_vars_by_at_rules(
  key: &String,
  value: &FlatCompiledStylesValue,
  collection: &mut IndexMap<String, Vec<String>>,
  at_rules: &Vec<String>,
) {
  let Some((hash_name, value)) = value.as_tuple() else {
    panic!("Props must be an key value pair")
  };

  match value.as_ref() {
    Expr::Array(_) => panic!("Array is not supported in stylex.defineVars"),
    Expr::Lit(lit) => {
      if let Lit::Null(_) = lit {
        return;
      }

      let val = get_string_val_from_lit(lit).expect("Value must be a string");

      let key = if at_rules.is_empty() {
        "default".to_string()
      } else {
        let mut keys = at_rules.clone();
        keys.sort();
        keys.join(SPLIT_TOKEN)
      };

      collection
        .entry(key)
        .or_default()
        .push(format!("--{}:{};", hash_name, val));
    }
    Expr::Object(obj) => {
      let key_values = get_key_values_from_object(obj);

      if !key_values.iter().any(|key_value| {
        let key = get_key_str(key_value);

        key == "default"
      }) {
        panic!("Default value is not defined for {} variable.", key);
      }

      for key_value in key_values.iter() {
        let at_rule = get_key_str(key_value);

        let value = key_value.value.clone();

        let extended_at_rules = if at_rule == "default" {
          at_rules.clone()
        } else {
          let mut new_at_rule = at_rules.clone();
          new_at_rule.push(at_rule.clone());
          new_at_rule
        };

        collect_vars_by_at_rules(
          &at_rule,
          &FlatCompiledStylesValue::Tuple(hash_name.clone(), value),
          collection,
          &extended_at_rules,
        );
      }
    }
    _ => {}
  }
}

pub(crate) fn wrap_with_at_rules(ltr: &str, at_rule: &str) -> String {
  at_rule
    .split(SPLIT_TOKEN)
    .fold(ltr.to_string(), |acc, at_rule| {
      format!("{}{{{}}}", at_rule, acc)
    })
}

pub(crate) fn priority_for_at_rule(at_rule: &str) -> f32 {
  if at_rule == "default" {
    0.0
  } else {
    at_rule.split(SPLIT_TOKEN).count() as f32
  }
}
