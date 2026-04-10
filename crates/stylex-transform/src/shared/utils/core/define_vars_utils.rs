use std::rc::Rc;

use indexmap::IndexMap;
use stylex_macros::stylex_panic;
use swc_core::ecma::ast::{Expr, Lit};

use crate::shared::enums::data_structures::flat_compiled_styles_value::FlatCompiledStylesValue;
use crate::shared::structures::types::{
  ClassPathsInNamespace, FlatCompiledStyles, InjectableStylesMap,
};
use crate::shared::utils::ast::convertors::{convert_key_value_to_str, convert_lit_to_string};
use crate::shared::utils::common::get_key_values_from_object;
use stylex_constants::constants::common::SPLIT_TOKEN;
use stylex_utils::hash::create_hash;
use stylex_utils::math::round_to_decimal_places;
use stylex_constants::constants::messages::{EXPECTED_CSS_VAR, VALUES_MUST_BE_OBJECT};
use stylex_enums::value_with_default::ValueWithDefault;
use stylex_types::structures::injectable_style::InjectableStyle;

pub(crate) fn construct_css_variables_string(
  variables: &FlatCompiledStyles,
  theme_name_hash: &String,
  typed_variables: &mut FlatCompiledStyles,
) -> InjectableStylesMap {
  let mut rules_by_at_rule = IndexMap::new();

  for (key, value) in variables.iter() {
    collect_vars_by_at_rules(key, value, &mut rules_by_at_rule, &[], typed_variables);
  }

  let mut result: InjectableStylesMap = IndexMap::new();

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
      InjectableStyle::regular(
        ltr,
        // Round to avoid floating-point precision issues (0.1 + 0.2 = 0.30000000000000004)
        Some(round_to_decimal_places(
          priority_for_at_rule(at_rule) / 10.0,
          1,
        )),
      ),
    );
  }

  result
}

pub(crate) fn collect_vars_by_at_rules(
  key: &String,
  value: &FlatCompiledStylesValue,
  collection: &mut ClassPathsInNamespace,
  at_rules: &[String],
  typed_variables: &mut FlatCompiledStyles,
) {
  let Some((hash_name, value, css_type)) = value.as_tuple() else {
    stylex_panic!("{}", VALUES_MUST_BE_OBJECT)
  };

  if let Some(css_type) = css_type {
    let values = match css_type.value.as_map() {
      Some(v) => v,
      None => stylex_panic!("Value must be a map"),
    };

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
    Expr::Array(_) => stylex_panic!(
      "Array values are not supported in defineVars(). Use a string, number, or nested object."
    ),
    Expr::Lit(lit) => {
      if let Lit::Null(_) = lit {
        return;
      }

      let val = match convert_lit_to_string(lit) {
        Some(v) => v,
        None => stylex_panic!("{}", EXPECTED_CSS_VAR),
      };

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
    },
    Expr::Object(obj) => {
      let key_values = get_key_values_from_object(obj);

      if !key_values.iter().any(|key_value| {
        let key = convert_key_value_to_str(key_value);

        key == "default"
      }) {
        stylex_panic!(r#"Default value is not defined for "{}" variable."#, key);
      }

      for key_value in key_values.iter() {
        let at_rule = convert_key_value_to_str(key_value);

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
    },
    _ => {},
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
    .unwrap_or_else(|| stylex_panic!("CSS type requires a default value but none was provided."))
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
    1.0 + at_rule.split(SPLIT_TOKEN).count() as f64
  }
}
