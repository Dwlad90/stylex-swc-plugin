use indexmap::IndexMap;
use stylex_evaluator::nested::{flatten_nested_vars_config, object_lit_to_nested_vars_config};
use stylex_macros::stylex_panic;

use crate::shared::{
  enums::data_structures::evaluate_result_value::EvaluateResultValue,
  structures::{state_manager::StateManager, types::InjectableStylesMap},
  transformers::stylex_define_vars::stylex_define_vars,
  utils::core::stylex_nested_utils::{
    UnflattenedCompiledStylesValue, expr_map_to_evaluate_result, unflatten_object,
  },
};
use stylex_constants::constants::{
  common::VAR_GROUP_HASH_KEY,
  messages::{PROPERTY_NOT_FOUND, VALUES_MUST_BE_OBJECT},
};

pub(crate) fn stylex_define_vars_nested(
  nested_variables: &EvaluateResultValue,
  state: &mut StateManager,
) -> (
  IndexMap<String, UnflattenedCompiledStylesValue>,
  InjectableStylesMap,
) {
  let Some(variables) = nested_variables.as_expr().and_then(|expr| expr.as_object()) else {
    stylex_panic!("{}", VALUES_MUST_BE_OBJECT)
  };

  let nested_variables = object_lit_to_nested_vars_config(variables);
  let flat_variables = flatten_nested_vars_config(&nested_variables);
  let flat_variables = expr_map_to_evaluate_result(flat_variables);
  let (mut flat_result, injectable_styles) = stylex_define_vars(&flat_variables, state);

  let var_group_hash = flat_result
    .shift_remove(VAR_GROUP_HASH_KEY)
    .unwrap_or_else(|| stylex_panic!("{}", PROPERTY_NOT_FOUND));

  let mut nested_var_refs = unflatten_object(&flat_result);
  nested_var_refs.insert(
    VAR_GROUP_HASH_KEY.to_string(),
    UnflattenedCompiledStylesValue::Leaf(var_group_hash),
  );

  (nested_var_refs, injectable_styles)
}
