use crate::shared::{
  constants::messages::{ILLEGAL_PROP_VALUE, NON_CONTIGUOUS_VARS},
  structures::{
    injectable_style::InjectableStyle, pre_rule::PreRuleValue, state_manager::StateManager,
  },
  utils::{
    common::{create_hash, dashify},
    css::common::{generate_rule, transform_value_cached},
    pre_rule::{sort_at_rules, sort_pseudos},
  },
};

pub(crate) fn convert_style_to_class_name(
  obj_entry: (&str, &PreRuleValue),
  pseudos: &mut [String],
  at_rules: &mut [String],
  state: &mut StateManager,
) -> (String, String, InjectableStyle) {
  let debug = state.options.debug;
  let enable_debug_class_names = state.options.enable_debug_class_names;

  let (key, raw_value) = obj_entry;

  let dashed_key = if key.starts_with("--") {
    key.to_string()
  } else {
    dashify(key)
  };

  let unsorted_pseudos = &mut pseudos.to_vec();
  let sorted_pseudos = sort_pseudos(unsorted_pseudos);

  let unsorted_at_rules = &mut at_rules.to_vec();
  let sorted_at_rules = sort_at_rules(unsorted_at_rules);

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

  let value = match raw_value {
    PreRuleValue::String(value) => PreRuleValue::String(transform_value_cached(key, value, state)),
    PreRuleValue::Vec(vec) => PreRuleValue::Vec(
      vec
        .iter()
        .map(|each_value| transform_value_cached(key, each_value.as_str(), state))
        .collect(),
    ),
    PreRuleValue::Expr(_) | PreRuleValue::Null => panic!("{}", ILLEGAL_PROP_VALUE),
  };

  let value = match value {
    PreRuleValue::String(value) => vec![value.to_string()],
    PreRuleValue::Vec(values) => {
      if values
        .iter()
        .any(|value| value.starts_with("var(") && value.ends_with(')'))
      {
        variable_fallbacks(&values)
      } else {
        values
      }
    }
    PreRuleValue::Expr(_) | PreRuleValue::Null => panic!("{}", ILLEGAL_PROP_VALUE),
  };

  let string_to_hash = format!(
    "<>{}{}{}",
    dashed_key,
    value.join(", "),
    modifier_hash_string
  );

  let prefix = &state.options.class_name_prefix;

  let class_name_hashed = if debug && enable_debug_class_names {
    format!("{}-{}{}", key, prefix, create_hash(&string_to_hash))
  } else {
    format!("{}{}", prefix, create_hash(string_to_hash.as_str()))
  };
  let css_rules = generate_rule(
    class_name_hashed.as_str(),
    dashed_key.as_str(),
    &value,
    pseudos,
    at_rules,
  );

  (key.to_string(), class_name_hashed, css_rules)
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

  assert!(
    !var_values
      .iter()
      .any(|val| !val.starts_with("var(") || !val.ends_with(')')),
    "{}",
    NON_CONTIGUOUS_VARS
  );

  var_values = var_values
    .iter()
    .map(|val| val[4..val.len() - 1].to_string())
    .collect::<Vec<String>>();

  let mut result = Vec::new();

  if !values_before_first_var.is_empty() {
    for val in values_before_first_var {
      let mut to_push = var_values.clone();

      to_push.push(val.to_string());

      result.push(compose_vars(to_push));
    }
  } else {
    result.push(compose_vars(var_values));
  }

  for val in values_after_last_var {
    result.push(val.to_string());
  }

  result
}

fn compose_vars(vars: Vec<String>) -> String {
  match vars.split_first() {
    Some((first, rest)) if !rest.is_empty() => {
      format!("var({},{})", first, compose_vars(rest.to_vec()))
    }
    Some((first, _)) if first.starts_with("--") => {
      format!("var({})", first)
    }
    Some((first, _)) => first.to_string(),
    None => String::new(),
  }
}
