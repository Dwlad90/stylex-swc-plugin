use std::rc::Rc;

use stylex_macros::stylex_panic;
use stylex_structures::base_css_type::get_css_value;
use swc_core::ecma::ast::{KeyValueProp, PropName};

use crate::shared::utils::core::define_vars_utils::construct_css_variables_string;
use stylex_ast::ast::convertors::{convert_key_value_to_str, get_key_values_from_object};
use stylex_constants::constants::{
  common::VAR_GROUP_HASH_KEY,
  messages::{EXPORT_ID_NOT_SET, VALUES_MUST_BE_OBJECT},
};
use stylex_state::{
  evaluate_result_value::EvaluateResultValue,
  flat_compiled_styles_value::FlatCompiledStylesValue,
  state_manager::StateManager,
  types::{FlatCompiledStyles, InjectableStylesMap},
};
use stylex_utils::hash::{create_hash, create_key_hash};

pub(crate) fn stylex_define_vars(
  variables: &EvaluateResultValue,
  state: &mut StateManager,
) -> (FlatCompiledStyles, InjectableStylesMap) {
  let export_id = match state.export_id.as_ref() {
    Some(id) => id.clone(),
    None => stylex_panic!("{}", EXPORT_ID_NOT_SET),
  };

  let var_group_hash = format!(
    "{}{}",
    state.options.class_name_prefix,
    create_hash(export_id.as_str())
  );

  let Some(variables) = variables.as_expr().and_then(|expr| expr.as_object()) else {
    stylex_panic!("{}", VALUES_MUST_BE_OBJECT)
  };

  let debug = state.options.debug;
  let enable_debug_class_names = state.options.enable_debug_class_names;
  let class_name_prefix = state.options.class_name_prefix.clone();

  let key_values = get_key_values_from_object(variables);

  // The value each variable takes, under the name the stylesheet writes it
  // with, and beside it the reference the module reads it through.
  let mut variables_map = FlatCompiledStyles::with_capacity(key_values.len());
  let mut theme_variables_objects = FlatCompiledStyles::with_capacity(key_values.len() + 1);

  for key_value in key_values.iter() {
    let key = convert_key_value_to_str(key_value);

    let var_safe_key =
      if key.chars().next().unwrap_or('\0') >= '0' && key.chars().next().unwrap_or('\0') <= '9' {
        format!("_{}", key)
      } else {
        key.clone()
      }
      .chars()
      .map(|c| if c.is_alphanumeric() { c } else { '_' })
      .collect::<String>();

    // Created hashed variable names with fileName//themeName//key
    let name_hash = if key.starts_with("--") {
      key.get(2..).unwrap_or_default().to_string()
    } else if debug && enable_debug_class_names {
      let key_hash = create_key_hash(&export_id, &key);

      format!("{}-{}{}", var_safe_key, class_name_prefix, key_hash)
    } else {
      let key_hash = create_key_hash(&export_id, &key);

      format!("{}{}", class_name_prefix, key_hash)
    };

    let (css_value, css_type) = get_css_value(KeyValueProp {
      key: PropName::Str(key.clone().into()),
      value: key_value.value.clone(),
    });

    theme_variables_objects.insert(
      key.clone(),
      Rc::new(FlatCompiledStylesValue::String(format!(
        "var(--{})",
        name_hash
      ))),
    );

    variables_map.insert(
      key,
      Rc::new(FlatCompiledStylesValue::Tuple(
        name_hash, css_value, css_type,
      )),
    );
  }

  // The `@property` rules the typed variables declare are collected as the
  // variables are read, and the variable rules are appended after them.
  let mut injectable_types = InjectableStylesMap::new();

  let injectable_styles =
    construct_css_variables_string(&variables_map, &var_group_hash, &mut injectable_types);

  theme_variables_objects.insert(
    VAR_GROUP_HASH_KEY.to_owned(),
    Rc::new(FlatCompiledStylesValue::String(var_group_hash)),
  );

  injectable_types.extend(injectable_styles);

  (theme_variables_objects, injectable_types)
}
