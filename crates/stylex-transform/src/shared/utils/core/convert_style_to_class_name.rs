use std::borrow::Cow;

use stylex_macros::stylex_panic;

use crate::shared::{
  structures::{
    pre_rule::PreRuleValue,
    state_manager::StateManager,
    types::{ClassName, RuleKey},
  },
  utils::css::common::{generate_css_rule, transform_value_cached},
};
use stylex_constants::constants::messages::{ILLEGAL_PROP_VALUE, NON_CONTIGUOUS_VARS};
use stylex_css::utils::pre_rule::{sort_at_rules, sort_pseudos};
use stylex_types::structures::injectable_style::InjectableStyle;
use stylex_utils::{
  hash::create_hash,
  string::{dashify, is_blank_css_text},
};

/// Compiles a resolved property/value pair into the class name that carries it
/// and the rule that class name injects.
///
/// `None` when the transformed value carries no CSS text: the declaration would
/// be `color:`, which is invalid CSS a browser discards, so the property is left
/// undeclared instead. The test is on the *transformed* value rather than the
/// authored one, because transformation is what decides whether a blank value
/// spells anything -- a blank `content` is quoted into `""`, which does.
pub(crate) fn convert_style_to_class_name(
  obj_entry: (&str, &PreRuleValue),
  pseudos: &mut [String],
  at_rules: &mut [String],
  const_rules: &mut [String],
  state: &mut StateManager,
) -> Option<(RuleKey, ClassName, InjectableStyle)> {
  let debug = state.options.debug;
  let enable_debug_class_names = state.options.enable_debug_class_names;

  let (key, raw_value) = obj_entry;

  let dashed_key = if key.starts_with("--") {
    Cow::Borrowed(key)
  } else {
    dashify(key)
  };

  let unsorted_pseudos = &mut pseudos.to_vec();
  let sorted_pseudos = sort_pseudos(unsorted_pseudos);

  let mut combined_at_rules = Vec::with_capacity(at_rules.len() + const_rules.len());

  combined_at_rules.extend_from_slice(at_rules);
  combined_at_rules.extend_from_slice(const_rules);

  let sorted_at_rules = sort_at_rules(&combined_at_rules);

  let at_rule_hash_string = sorted_at_rules.join("");
  let pseudo_hash_string = sorted_pseudos.join("");

  let modifier_hash_string = if at_rule_hash_string.is_empty() && pseudo_hash_string.is_empty() {
    // NOTE: 'null' is used to keep existing hashes stable.
    // This should be removed in a future version.
    "null".to_string()
  } else {
    // TODO: set correct order when will be answer from the Meta team
    // Link to discussion: https://github.com/facebook/stylex/discussions/744
    format!("{}{}", pseudo_hash_string, at_rule_hash_string)
  };

  let value: Vec<String> = match raw_value {
    PreRuleValue::Raw(raw_value) => vec![transform_value_cached(key, raw_value, state)],
    PreRuleValue::Vec(values) => {
      // A blank entry drops before the fallback chain is built, so the class
      // name is hashed from the entries that survive: a blank beside `red`
      // yields the class name a lone `red` yields. It also has to go before
      // `variable_fallbacks`, which requires the `var()` entries it composes to
      // be contiguous.
      let values: Vec<String> = values
        .iter()
        .map(|each_value| transform_value_cached(key, each_value, state))
        .filter(|value| !is_blank_css_text(value))
        .collect();

      if values
        .iter()
        .any(|value| value.starts_with("var(") && value.ends_with(')'))
      {
        variable_fallbacks(&values)
      } else {
        values
      }
    },
    PreRuleValue::Expr(_) | PreRuleValue::Null => stylex_panic!("{}", ILLEGAL_PROP_VALUE),
  };

  // A lone value is judged here; a fallback array arrives with its blank
  // entries already gone, so `all` over no values -- also `true` -- is what
  // answers for an array that emptied.
  if value.iter().all(|text| is_blank_css_text(text)) {
    return None;
  }

  let string_to_hash = format!(
    "<>{}{}{}",
    dashed_key.as_ref(),
    value.join(", "),
    modifier_hash_string
  );

  let prefix = &state.options.class_name_prefix;

  let class_name_hashed = if debug && enable_debug_class_names {
    format!("{}-{}{}", key, prefix, create_hash(&string_to_hash))
  } else {
    format!("{}{}", prefix, create_hash(string_to_hash.as_str()))
  };

  let css_rules = generate_css_rule(
    class_name_hashed.as_str(),
    dashed_key.as_ref(),
    &value,
    pseudos,
    at_rules,
    const_rules,
    &state.options,
  );

  Some((
    RuleKey::from(key),
    ClassName::from(class_name_hashed),
    css_rules,
  ))
}

fn variable_fallbacks(values: &[String]) -> Vec<String> {
  let first_var = values
    .iter()
    .position(|val| val.starts_with("var(") && val.ends_with(')'));

  let last_var = values
    .iter()
    .rev()
    .position(|val| val.starts_with("var(") && val.ends_with(')'))
    .map(|i| values.len() - 1 - i);

  let values_before_first_var = &values[0..first_var.unwrap_or(0)];

  let mut var_values: Vec<String> = values
    [first_var.unwrap_or(0)..last_var.unwrap_or(values.len()) + 1]
    .iter()
    .rev()
    .cloned()
    .collect::<Vec<String>>();

  let values_after_last_var = &values[last_var.unwrap_or(values.len()) + 1..];

  if !var_values
    .iter()
    .all(|val| val.starts_with("var(") && val.ends_with(')'))
  {
    stylex_panic!("{}", NON_CONTIGUOUS_VARS);
  }

  var_values = var_values
    .iter()
    .map(|val| val[4..val.len() - 1].to_string())
    .collect::<Vec<String>>();

  let result_capacity = if values_before_first_var.is_empty() {
    1
  } else {
    values_before_first_var.len()
  } + values_after_last_var.len();
  let mut result = Vec::with_capacity(result_capacity);

  if !values_before_first_var.is_empty() {
    // The var prefix is the same for every iteration, so it is laid down once
    // and only the trailing value is swapped — rather than cloning the whole
    // prefix per value, which is what this cost before.
    let mut to_push = Vec::with_capacity(var_values.len() + 1);
    to_push.extend_from_slice(&var_values);
    to_push.push(String::new());

    for val in values_before_first_var {
      if let Some(last) = to_push.last_mut() {
        last.clear();
        last.push_str(val);
      }

      result.push(compose_vars(&to_push));
    }
  } else {
    result.push(compose_vars(&var_values));
  }

  for val in values_after_last_var {
    result.push(val.to_string());
  }

  result
}

fn compose_vars(vars: &[String]) -> String {
  match vars.split_first() {
    Some((first, rest)) if !rest.is_empty() => {
      let fallback = compose_vars(rest);
      let mut result = String::with_capacity(first.len() + fallback.len() + 6);
      result.push_str("var(");
      result.push_str(first);
      result.push(',');
      result.push_str(&fallback);
      result.push(')');
      result
    },
    Some((first, _)) if first.starts_with("--") => {
      let mut result = String::with_capacity(first.len() + 5);
      result.push_str("var(");
      result.push_str(first);
      result.push(')');
      result
    },
    Some((first, _)) => first.to_string(),
    None => String::new(),
  }
}
