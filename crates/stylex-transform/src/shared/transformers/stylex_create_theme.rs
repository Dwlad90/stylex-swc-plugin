use std::{cmp::Ordering, rc::Rc};

use indexmap::IndexMap;
use stylex_macros::{stylex_panic, stylex_unimplemented};
use swc_core::ecma::ast::KeyValueProp;

use crate::shared::{
  enums::data_structures::{
    evaluate_result_value::EvaluateResultValue, flat_compiled_styles_value::FlatCompiledStylesValue,
  },
  structures::{
    functions::FunctionMap,
    state_manager::StateManager,
    types::{FlatCompiledStyles, InjectableStylesMap},
  },
  utils::{
    ast::convertors::{convert_expr_to_str, convert_key_value_to_str},
    common::{get_css_value, get_key_values_from_object},
    core::define_vars_utils::{collect_vars_by_at_rules, priority_for_at_rule, wrap_with_at_rules},
    validators::validate_theme_variables,
  },
};
use stylex_constants::constants::{
  common::{COMPILED_KEY, VAR_GROUP_HASH_KEY},
  messages::{
    AT_RULE_NOT_FOUND, EXPECTED_CSS_VAR, EXPRESSION_IS_NOT_A_STRING, THEME_VARS_MUST_BE_OBJECT,
  },
};
use stylex_types::structures::injectable_style::InjectableStyle;
use stylex_utils::{
  collection::find_and_swap_remove, hash::create_hash, math::round_to_decimal_places,
};

pub(crate) fn stylex_create_theme(
  theme_vars: &mut EvaluateResultValue,
  variables: &EvaluateResultValue,
  state: &mut StateManager,
  typed_variables: &mut FlatCompiledStyles,
) -> (FlatCompiledStyles, InjectableStylesMap) {
  let theme_name_key_value = validate_theme_variables(theme_vars, state);

  let mut rules_by_at_rule = IndexMap::new();

  let variables_obj = match variables.as_expr().and_then(|expr| expr.as_object()) {
    Some(obj) => obj,
    #[cfg_attr(coverage_nightly, coverage(off))]
    None => stylex_panic!("{}", THEME_VARS_MUST_BE_OBJECT),
  };
  let mut variables_key_values = Box::new(get_key_values_from_object(variables_obj));

  variables_key_values.sort_by_key(convert_key_value_to_str);

  #[allow(unused_assignments)]
  let mut var_group_hash: String = String::new();
  let mut theme_vars_key_values: Vec<KeyValueProp> = Vec::new();

  match theme_vars {
    EvaluateResultValue::Expr(expr) => {
      let theme_obj = match expr.as_object() {
        Some(obj) => obj,
        #[cfg_attr(coverage_nightly, coverage(off))]
        None => stylex_panic!("{}", THEME_VARS_MUST_BE_OBJECT),
      };
      theme_vars_key_values = get_key_values_from_object(theme_obj);

      var_group_hash = theme_vars_key_values
        .iter()
        .find(|key_value| convert_key_value_to_str(key_value) == VAR_GROUP_HASH_KEY)
        .map(|key_value| {
          match convert_expr_to_str(&key_value.value, state, &FunctionMap::default()) {
            Some(s) => s,
            #[cfg_attr(coverage_nightly, coverage(off))]
            None => stylex_panic!("{}", EXPRESSION_IS_NOT_A_STRING),
          }
        })
        .unwrap_or_default();
    },
    EvaluateResultValue::ThemeRef(theme_ref) => {
      var_group_hash = match theme_ref.get(VAR_GROUP_HASH_KEY, state).as_css_var() {
        Some(v) => v.to_owned(),
        #[cfg_attr(coverage_nightly, coverage(off))]
        None => stylex_panic!("{}", EXPECTED_CSS_VAR),
      };
    },
    #[cfg_attr(coverage_nightly, coverage(off))]
    _ => {
      stylex_unimplemented!("Unsupported theme vars type {:?}", theme_vars)
    },
  }

  for key_value in variables_key_values.into_iter() {
    let key = convert_key_value_to_str(&key_value);

    let theme_vars_str_value = match theme_vars {
      EvaluateResultValue::Expr(_) => {
        let theme_vars_item = match find_and_swap_remove(&mut theme_vars_key_values, |key_value| {
          convert_key_value_to_str(key_value) == key
        }) {
          Some(item) => item,
          #[cfg_attr(coverage_nightly, coverage(off))]
          None => stylex_panic!(
            "The referenced theme variable was not found. Ensure it was declared in defineVars()."
          ),
        };

        match convert_expr_to_str(
          theme_vars_item.value.as_ref(),
          state,
          &FunctionMap::default(),
        ) {
          Some(s) => s,
          #[cfg_attr(coverage_nightly, coverage(off))]
          None => stylex_panic!("{}", EXPRESSION_IS_NOT_A_STRING),
        }
      },
      EvaluateResultValue::ThemeRef(theme_ref) => {
        match theme_ref.get(key.as_str(), state).as_css_var() {
          Some(v) => v.to_string(),
          None => stylex_panic!("{}", EXPECTED_CSS_VAR),
        }
      },
      #[cfg_attr(coverage_nightly, coverage(off))]
      _ => stylex_unimplemented!("Unsupported theme vars type"),
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
      let rule_by_at_rule = match rules_by_at_rule.get(*at_rule) {
        Some(v) => v.join(""),
        #[cfg_attr(coverage_nightly, coverage(off))]
        None => stylex_panic!("{}", AT_RULE_NOT_FOUND),
      };

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

  let mut resolved_theme_vars = IndexMap::new();
  let mut styles_to_inject = IndexMap::new();

  for at_rule in sorted_at_rules.into_iter() {
    let decls = match rules_by_at_rule.get(at_rule) {
      Some(v) => v.join(""),
      #[cfg_attr(coverage_nightly, coverage(off))]
      None => stylex_panic!("{}", AT_RULE_NOT_FOUND),
    };
    let rule = format!(".{override_class_name}, .{override_class_name}:root{{{decls}}}");

    let priority = round_to_decimal_places(0.4 + priority_for_at_rule(at_rule) / 10.0, 1);

    let (suffix, ltr) = if at_rule == "default" {
      (String::new(), rule)
    } else {
      (
        format!("-{}", create_hash(at_rule)),
        wrap_with_at_rules(&rule, at_rule),
      )
    };

    styles_to_inject.insert(
      format!("{}{}", override_class_name, suffix).into(),
      InjectableStyle::regular(ltr, Some(priority)),
    );
  }

  let theme_name_str_value = match theme_vars {
    EvaluateResultValue::Expr(_) => {
      match convert_expr_to_str(
        theme_name_key_value.value.as_ref(),
        state,
        &FunctionMap::default(),
      ) {
        Some(s) => s,
        #[cfg_attr(coverage_nightly, coverage(off))]
        None => stylex_panic!("{}", EXPRESSION_IS_NOT_A_STRING),
      }
    },
    EvaluateResultValue::ThemeRef(theme_ref) => {
      match theme_ref.get(VAR_GROUP_HASH_KEY, state).as_css_var() {
        Some(v) => v.to_owned(),
        #[cfg_attr(coverage_nightly, coverage(off))]
        None => stylex_panic!("{}", EXPECTED_CSS_VAR),
      }
    },
    #[cfg_attr(coverage_nightly, coverage(off))]
    _ => stylex_unimplemented!("Unsupported theme vars type"),
  };

  let theme_class = format!("{override_class_name} {var_group_hash}");

  resolved_theme_vars.insert(
    theme_name_str_value,
    Rc::new(FlatCompiledStylesValue::String(theme_class)),
  );

  resolved_theme_vars.insert(
    COMPILED_KEY.to_string(),
    Rc::new(FlatCompiledStylesValue::Bool(true)),
  );

  (resolved_theme_vars, styles_to_inject)
}
