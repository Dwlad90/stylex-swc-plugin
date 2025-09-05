use std::{ops::Mul, rc::Rc};

use indexmap::IndexMap;
use swc_core::ecma::ast::{Expr, Lit};

use crate::shared::{
  constants::common::SPLIT_TOKEN,
  enums::data_structures::{
    flat_compiled_styles_value::FlatCompiledStylesValue, injectable_style::InjectableStyleKind,
    value_with_default::ValueWithDefault,
  },
  structures::injectable_style::InjectableStyle,
  utils::{
    ast::convertors::{key_value_to_str, lit_to_string},
    common::{create_hash, get_key_values_from_object},
  },
};

pub(crate) fn construct_css_variables_string(
  variables: &IndexMap<String, Rc<FlatCompiledStylesValue>>,
  theme_name_hash: &String,
  typed_variables: &mut IndexMap<String, Rc<FlatCompiledStylesValue>>,
) -> IndexMap<String, Rc<InjectableStyleKind>> {
  let mut rules_by_at_rule: IndexMap<String, Vec<String>> = IndexMap::new();

  for (key, value) in variables.iter() {
    collect_vars_by_at_rules(key, value, &mut rules_by_at_rule, &[], typed_variables);
  }

  let mut result: IndexMap<String, Rc<InjectableStyleKind>> = IndexMap::new();

  for (at_rule, value) in rules_by_at_rule.iter() {
    let suffix = if at_rule == "default" {
      String::default()
    } else {
      format!("-{}", create_hash(at_rule))
    };

    let selector = format!(":root, .{theme_name_hash}");

    let mut ltr = format!("{selector}{{{}}}", value.join(""));

    if at_rule != "default" {
      ltr = wrap_with_at_rules(ltr.as_str(), at_rule);
    }

    result.insert(
      format!("{}{}", theme_name_hash, suffix),
      Rc::new(InjectableStyleKind::Regular(InjectableStyle {
        priority: Some(priority_for_at_rule(at_rule).mul(0.1)),
        ltr,
        rtl: None,
      })),
    );
  }

  result
}

pub(crate) fn collect_vars_by_at_rules(
  key: &String,
  value: &FlatCompiledStylesValue,
  collection: &mut IndexMap<String, Vec<String>>,
  at_rules: &[String],
  typed_variables: &mut IndexMap<String, Rc<FlatCompiledStylesValue>>,
) {
  let Some((hash_name, value, css_type)) = value.as_tuple() else {
    panic!("Props must be an key value pair")
  };

  if let Some(css_type) = css_type {
    let values = css_type.value.as_map().expect("Value must be an map");

    let initial_value = get_nitial_value_of_css_type(values);

    typed_variables.insert(
      hash_name.clone(),
      Rc::new(FlatCompiledStylesValue::CSSType(
        hash_name.clone(),
        css_type.syntax,
        initial_value.clone(),
      )),
    );
  }

  match value {
    Expr::Array(_) => panic!("Array is not supported in defineVars"),
    Expr::Lit(lit) => {
      if let Lit::Null(_) = lit {
        return;
      }

      let val = lit_to_string(lit).expect("Value must be a string");

      let key = if at_rules.is_empty() {
        "default".to_string()
      } else {
        let mut keys = at_rules.to_vec();
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
        let key = key_value_to_str(key_value);

        key == "default"
      }) {
        panic!(r#"Default value is not defined for "{}" variable."#, key);
      }

      for key_value in key_values.iter() {
        let at_rule = key_value_to_str(key_value);

        let extended_at_rules = if at_rule == "default" {
          at_rules.to_vec()
        } else {
          let mut new_at_rule = at_rules.to_vec();
          new_at_rule.push(at_rule.clone());
          new_at_rule
        };

        let value = key_value.value.clone();

        collect_vars_by_at_rules(
          &at_rule,
          &FlatCompiledStylesValue::Tuple(hash_name.clone(), value, None),
          collection,
          &extended_at_rules,
          typed_variables,
        );
      }
    }
    _ => {}
  }
}

fn get_nitial_value_of_css_type(values: &IndexMap<String, ValueWithDefault>) -> String {
  values
    .get("default")
    .map(|value| match value {
      ValueWithDefault::Number(num) => num.to_string(),
      ValueWithDefault::String(strng) => strng.clone(),
      ValueWithDefault::Map(map) => get_nitial_value_of_css_type(map),
    })
    .expect("Default value is not defined")
}

pub(crate) fn wrap_with_at_rules(ltr: &str, at_rule: &str) -> String {
  at_rule
    .split(SPLIT_TOKEN)
    .fold(ltr.to_string(), |acc, at_rule| {
      format!("{}{{{}}}", at_rule, acc)
    })
}

pub(crate) fn priority_for_at_rule(at_rule: &str) -> f64 {
  if at_rule == "default" {
    1.0
  } else {
    at_rule.split(SPLIT_TOKEN).count() as f64
  }
}
