use std::rc::Rc;

use indexmap::IndexMap;
use stylex_macros::stylex_panic;
use swc_core::ecma::ast::{Expr, Lit, ObjectLit};

use crate::shared::{
  enums::data_structures::flat_compiled_styles_value::FlatCompiledStylesValue,
  structures::types::{ClassPathsInNamespace, FlatCompiledStyles, InjectableStylesMap},
  utils::ast::convertors::{convert_key_value_to_str, convert_lit_to_string},
};
use stylex_ast::ast::convertors::get_key_values_from_object;
use stylex_constants::constants::{
  common::SPLIT_TOKEN,
  messages::{EXPECTED_CSS_VAR, VALUES_MUST_BE_OBJECT, missing_default_value},
};
use stylex_enums::value_with_default::ValueWithDefault;
use stylex_types::structures::injectable_style::InjectableStyle;
use stylex_utils::hash::create_hash;

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
      format!("{}{}", theme_name_hash, suffix).into(),
      InjectableStyle::regular(ltr, Some(var_group_priority(at_rule))),
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
        initial_value,
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
        keys.sort_unstable();
        keys.join(SPLIT_TOKEN)
      };

      collection
        .entry(key)
        .or_default()
        .push(format!("--{}:{};", hash_name, val));
    },
    Expr::Object(obj) => {
      if !object_carries_a_default(obj) {
        stylex_panic!("{}", missing_default_value(key));
      }

      let key_values = get_key_values_from_object(obj);

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

        // The variable's own name travels down the recursion, not the at-rule
        // the level is standing on: an author looking for `cornerRadius` is
        // told about `cornerRadius` however deep the object nests, which is
        // the name the reference implementation carries down here too.
        collect_vars_by_at_rules(
          key,
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

/// Whether an object written as a variable's value is a CSS type rather than a
/// value map.
///
/// A CSS type carries its own `value` under a `syntax`, so it is not a map of
/// at-rules and has no `default` key of its own to look for -- the reference
/// implementation tests for one ahead of the missing-default check for the same
/// reason. The pair of keys is the test, because either alone appears on
/// ordinary value maps.
pub(crate) fn object_is_a_css_type(obj: &ObjectLit) -> bool {
  let keys: Vec<String> = get_key_values_from_object(obj)
    .iter()
    .map(convert_key_value_to_str)
    .collect();

  keys.iter().any(|key| key == "syntax") && keys.iter().any(|key| key == "value")
}

/// Whether an object written as a variable's value carries the `default` key.
///
/// Shared with the step that expands a variable's value, which asks the same
/// question one stage earlier: an object with no `default` is refused for the
/// shape it is before anything looks at what it holds, which is the order the
/// reference implementation checks in. Asked through one function so the two
/// stages cannot come to disagree about what carrying a default means.
pub(crate) fn object_carries_a_default(obj: &ObjectLit) -> bool {
  get_key_values_from_object(obj)
    .iter()
    .any(|key_value| convert_key_value_to_str(key_value) == "default")
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

fn priority_for_at_rule(at_rule: &str) -> f64 {
  if at_rule == "default" {
    1.0
  } else {
    1.0 + at_rule.split(SPLIT_TOKEN).count() as f64
  }
}

/// Priority of a var group's rule at `at_rule`.
///
/// Neither this nor [`theme_override_priority`] rounds its result, and neither
/// may start to. The computed value *is* the priority: the stylesheet sort
/// compares priorities for equality and falls through to a by-content tie-break
/// on a tie, so rounding one is not a cosmetic change — it moves rules relative
/// to each other, and can collapse two priorities onto one.
///
/// A single at-rule makes that observable. This function returns exactly `0.6`
/// five at-rules deep, while [`theme_override_priority`] returns
/// `0.6000000000000001` for one at-rule; round either to a single decimal place
/// and they tie, at which point the override can sort ahead of the rule it
/// overrides — both declare the same custom property at equal specificity, so
/// order decides the winner.
///
/// The gap is an artefact of the arithmetic, not a guarantee: at most depths
/// `0.4 + n / 10.0` is exact and the two functions genuinely do collide (two
/// at-rules against a group six deep both give exactly `0.7`). Nothing here
/// separates that pair — only the sum's own precision does, where it has any.
pub(crate) fn var_group_priority(at_rule: &str) -> f64 {
  priority_for_at_rule(at_rule) / 10.0
}

/// Priority of a theme override's rule at `at_rule`.
///
/// See [`var_group_priority`] for why this result is never rounded.
pub(crate) fn theme_override_priority(at_rule: &str) -> f64 {
  0.4 + priority_for_at_rule(at_rule) / 10.0
}
