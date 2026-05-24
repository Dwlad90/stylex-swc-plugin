use stylex_evaluator::nested::{
  flatten_nested_overrides_config, object_lit_to_nested_string_config,
  object_lit_to_nested_vars_config,
};
use stylex_macros::stylex_panic;
use stylex_structures::nested::flatten_nested_string_config;

use crate::shared::{
  enums::data_structures::evaluate_result_value::EvaluateResultValue,
  structures::{
    state_manager::StateManager,
    types::{FlatCompiledStyles, InjectableStylesMap},
  },
  transformers::stylex_create_theme::stylex_create_theme,
  utils::core::stylex_nested_utils::{expr_map_to_evaluate_result, string_map_to_evaluate_result},
};
use stylex_constants::constants::messages::{THEME_VARS_MUST_BE_OBJECT, VALUES_MUST_BE_OBJECT};

pub(crate) fn stylex_create_theme_nested(
  theme_vars: &mut EvaluateResultValue,
  nested_overrides: &EvaluateResultValue,
  state: &mut StateManager,
  typed_variables: &mut FlatCompiledStyles,
) -> (FlatCompiledStyles, InjectableStylesMap) {
  let mut flat_theme_vars = match theme_vars {
    EvaluateResultValue::Expr(expr) => {
      let Some(obj) = expr.as_object() else {
        stylex_panic!("{}", THEME_VARS_MUST_BE_OBJECT)
      };

      let nested_theme_vars = object_lit_to_nested_string_config(obj);
      let flat_theme_vars = flatten_nested_string_config(&nested_theme_vars);
      string_map_to_evaluate_result(flat_theme_vars)
    },
    EvaluateResultValue::ThemeRef(_) => theme_vars.clone(),
    _ => stylex_panic!("{}", THEME_VARS_MUST_BE_OBJECT),
  };

  let Some(overrides) = nested_overrides.as_expr().and_then(|expr| expr.as_object()) else {
    stylex_panic!("{}", VALUES_MUST_BE_OBJECT)
  };

  let nested_overrides = object_lit_to_nested_vars_config(overrides);
  let flat_overrides = flatten_nested_overrides_config(&nested_overrides);
  let flat_overrides = expr_map_to_evaluate_result(flat_overrides);

  stylex_create_theme(
    &mut flat_theme_vars,
    &flat_overrides,
    state,
    typed_variables,
  )
}
