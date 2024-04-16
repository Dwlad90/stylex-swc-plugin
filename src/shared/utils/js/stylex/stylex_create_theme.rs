use std::cmp::Ordering;

use indexmap::IndexMap;

use crate::shared::{
  constants::constants::COMPILED_KEY,
  enums::FlatCompiledStylesValue,
  structures::{
    evaluate_result::EvaluateResultValue, functions::FunctionMap,
    injectable_style::InjectableStyle, state_manager::StateManager,
  },
  utils::{
    common::{create_hash, expr_to_str, get_key_str, get_key_values_from_object},
    stylex::define_vars_utils::{
      collect_vars_by_at_rules, priority_for_at_rule, wrap_with_at_rules,
    },
    validators::validate_theme_variables,
  },
};

pub(crate) fn stylex_create_theme(
  theme_vars: &EvaluateResultValue,
  variables: &EvaluateResultValue,
  state: &mut StateManager,
) -> (
  IndexMap<String, FlatCompiledStylesValue>,
  IndexMap<String, InjectableStyle>,
) {
  dbg!(theme_vars, variables);

  let theme_name_key_value = validate_theme_variables(theme_vars);

  let mut rules_by_at_rule: IndexMap<String, Vec<String>> = IndexMap::new();

  let mut key_values =
    get_key_values_from_object(variables.as_expr().unwrap().as_object().unwrap());

  key_values.sort_by(|a, b| {
    let a_key = get_key_str(a);
    let b_key = get_key_str(b);

    a_key.cmp(&b_key)
  });

  let theme_vars_props = theme_vars.as_expr().unwrap().as_object().unwrap();

  let theme_vars_key_values = get_key_values_from_object(theme_vars_props);

  for key_value in key_values.into_iter() {
    let key = get_key_str(&key_value);

    let theme_vars_item = theme_vars_key_values
      .clone()
      .into_iter()
      .find(|key_value| {
        let local_key = get_key_str(key_value);

        local_key == key
      })
      .expect("Theme variable not found");

    let theme_vars_str_value = expr_to_str(
      theme_vars_item.value.as_ref(),
      state,
      &FunctionMap::default(),
    );

    let name_hash = theme_vars_str_value[6..theme_vars_str_value.len() - 1].to_string();

    let value = FlatCompiledStylesValue::Tuple(name_hash, key_value.value);

    collect_vars_by_at_rules(&key, &value, &mut rules_by_at_rule, &vec![]);
  }

  dbg!(&rules_by_at_rule);

  // Sort @-rules to get a consistent unique hash value
  // But also put "default" first
  let mut sorted_at_rules = rules_by_at_rule.keys().collect::<Vec<&String>>();

  sorted_at_rules.sort_by(|a, b| {
    if a.as_str() == "default" {
      Ordering::Less
    } else if b.as_str() == "default" {
      Ordering::Greater
    } else {
      a.cmp(b)
    }
  });

  dbg!(&sorted_at_rules);

  let at_rules_string_for_hash = sorted_at_rules
    .clone()
    .into_iter()
    .map(|at_rule| {
      let rult_by_at_rule = rules_by_at_rule.get(at_rule).unwrap().join("");

      wrap_with_at_rules(rult_by_at_rule.as_str(), at_rule)
    })
    .collect::<Vec<String>>()
    .join("");

  dbg!(&at_rules_string_for_hash);

  // Create a class name hash
  let override_class_name = format!(
    "{}{}",
    state.options.class_name_prefix,
    create_hash(at_rules_string_for_hash.as_str())
  );

  dbg!(&override_class_name);

  let mut resolved_theme_vars: IndexMap<String, FlatCompiledStylesValue> = IndexMap::new();
  let mut styles_to_inject: IndexMap<String, InjectableStyle> = IndexMap::new();

  dbg!(&sorted_at_rules);

  for at_rule in sorted_at_rules.into_iter() {
    let decls = rules_by_at_rule.get(at_rule).unwrap().join("");
    let rule = format!(".{}{{{}}}", override_class_name.clone(), decls);

    if at_rule == "default" {
      styles_to_inject.insert(
        override_class_name.clone(),
        InjectableStyle {
          ltr: rule.clone(),
          rtl: None,
          priority: Some(0.5),
        },
      );
    } else {
      let key = format!("{}-{}", override_class_name, create_hash(at_rule));
      let ltr = wrap_with_at_rules(rule.as_str(), at_rule);
      let priority = 0.5 + 0.1 * priority_for_at_rule(at_rule);

      styles_to_inject.insert(
        key,
        InjectableStyle {
          ltr,
          rtl: None,
          priority: Some(priority),
        },
      );
    }
  }
  dbg!(&styles_to_inject);

  resolved_theme_vars.insert(
    COMPILED_KEY.to_string(),
    FlatCompiledStylesValue::Bool(true),
  );

  let theme_name_str_value = expr_to_str(
    theme_name_key_value.value.as_ref(),
    state,
    &FunctionMap::default(),
  );

  resolved_theme_vars.insert(
    theme_name_str_value,
    FlatCompiledStylesValue::String(override_class_name),
  );

  (resolved_theme_vars, styles_to_inject)
}
