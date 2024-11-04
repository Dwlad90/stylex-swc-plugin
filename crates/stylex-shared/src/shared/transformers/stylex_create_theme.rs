use std::{cmp::Ordering, rc::Rc};

use indexmap::IndexMap;

use crate::shared::{
  constants::common::{COMPILED_KEY, THEME_NAME_KEY},
  enums::data_structures::{
    evaluate_result_value::EvaluateResultValue, flat_compiled_styles_value::FlatCompiledStylesValue,
  },
  structures::{
    functions::FunctionMap, injectable_style::InjectableStyle, state_manager::StateManager,
  },
  utils::{
    ast::convertors::expr_to_str,
    common::{create_hash, get_css_value, get_key_str, get_key_values_from_object},
    core::define_vars_utils::{collect_vars_by_at_rules, priority_for_at_rule, wrap_with_at_rules},
    validators::validate_theme_variables,
  },
};

pub(crate) fn stylex_create_theme(
  theme_vars: &mut EvaluateResultValue,
  variables: &EvaluateResultValue,
  state: &mut StateManager,
  typed_variables: &mut IndexMap<String, Rc<FlatCompiledStylesValue>>,
) -> (
  IndexMap<String, Rc<FlatCompiledStylesValue>>,
  IndexMap<String, Rc<InjectableStyle>>,
) {
  let theme_name_key_value = validate_theme_variables(theme_vars);

  let mut rules_by_at_rule: IndexMap<String, Vec<String>> = IndexMap::new();

  let mut variables_key_values = Box::new(get_key_values_from_object(
    variables
      .as_expr()
      .and_then(|expr| expr.as_object())
      .expect("Variables must be an object"),
  ));

  variables_key_values.sort_by_key(get_key_str);

  for key_value in variables_key_values.into_iter() {
    let key = get_key_str(&key_value);

    let theme_vars_str_value = match theme_vars {
      EvaluateResultValue::Expr(expr) => {
        let theme_vars_key_values = get_key_values_from_object(expr.as_object().unwrap());
        let theme_vars_item = theme_vars_key_values
          .iter()
          .find(|key_value| get_key_str(key_value) == key)
          .expect("Theme variable not found");

        expr_to_str(
          theme_vars_item.value.as_ref(),
          state,
          &FunctionMap::default(),
        )
      }
      EvaluateResultValue::ThemeRef(theme_ref) => theme_ref.get(key.as_str()).clone(),
      _ => unimplemented!("Unsupported theme vars type"),
    };

    let name_hash = theme_vars_str_value[6..theme_vars_str_value.len() - 1].to_string();

    let css_value = get_css_value(key_value);

    let value = FlatCompiledStylesValue::Tuple(name_hash, css_value.0, css_value.1);

    collect_vars_by_at_rules(&key, &value, &mut rules_by_at_rule, &[], typed_variables);
  }

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

  let at_rules_string_for_hash = sorted_at_rules
    .iter()
    .map(|at_rule| {
      let rule_by_at_rule = rules_by_at_rule.get(*at_rule).unwrap().join("");

      wrap_with_at_rules(rule_by_at_rule.as_str(), at_rule)
    })
    .collect::<Vec<String>>()
    .join("");

  // Create a class name hash
  let override_class_name = format!(
    "{}{}",
    state.options.class_name_prefix,
    create_hash(at_rules_string_for_hash.as_str())
  );

  let mut resolved_theme_vars: IndexMap<String, Rc<FlatCompiledStylesValue>> = IndexMap::new();
  let mut styles_to_inject: IndexMap<String, Rc<InjectableStyle>> = IndexMap::new();

  for at_rule in sorted_at_rules.into_iter() {
    let decls = rules_by_at_rule.get(at_rule).unwrap().join("");
    let rule = format!(".{override_class_name}, .{override_class_name}:root{{{decls}}}");

    if at_rule == "default" {
      styles_to_inject.insert(
        override_class_name.clone(),
        Rc::new(InjectableStyle {
          ltr: rule,
          rtl: None,
          priority: Some(0.5),
        }),
      );
    } else {
      let key = format!("{}-{}", override_class_name, create_hash(at_rule));
      let ltr = wrap_with_at_rules(rule.as_str(), at_rule);
      let priority = 0.5 + 0.1 * priority_for_at_rule(at_rule);

      styles_to_inject.insert(
        key,
        Rc::new(InjectableStyle {
          ltr,
          rtl: None,
          priority: Some(priority),
        }),
      );
    }
  }

  resolved_theme_vars.insert(
    COMPILED_KEY.to_string(),
    Rc::new(FlatCompiledStylesValue::Bool(true)),
  );

  let theme_name_str_value = match theme_vars {
    EvaluateResultValue::Expr(_) => expr_to_str(
      theme_name_key_value.value.as_ref(),
      state,
      &FunctionMap::default(),
    ),
    EvaluateResultValue::ThemeRef(theme_ref) => theme_ref.get(THEME_NAME_KEY).to_owned(),
    _ => unimplemented!("Unsupported theme vars type"),
  };

  resolved_theme_vars.insert(
    theme_name_str_value,
    Rc::new(FlatCompiledStylesValue::String(override_class_name)),
  );

  (resolved_theme_vars, styles_to_inject)
}
