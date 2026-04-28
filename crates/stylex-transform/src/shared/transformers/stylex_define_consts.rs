use std::rc::Rc;

use stylex_macros::{stylex_panic, stylex_unimplemented};

use crate::shared::{
  enums::data_structures::{
    evaluate_result_value::EvaluateResultValue,
    flat_compiled_styles_value::FlatCompiledStylesValue, obj_map_type::ObjMapType,
  },
  structures::{
    state_manager::StateManager,
    types::{FlatCompiledStyles, InjectableStylesMap},
  },
  utils::{common::serialize_value_to_json_string, object::obj_map},
};
use stylex_constants::constants::messages::{
  EXPORT_ID_NOT_SET, INJECTABLE_STYLE_NOT_SUPPORTED, VALUES_MUST_BE_OBJECT,
};
use stylex_types::{
  enums::data_structures::injectable_style::InjectableStyleKind,
  structures::injectable_style::InjectableConstStyle,
};
use stylex_utils::hash::create_key_hash;

pub(crate) fn stylex_define_consts(
  constants: &EvaluateResultValue,
  state: &mut StateManager,
) -> (FlatCompiledStyles, InjectableStylesMap) {
  let Some(constants) = constants.as_expr().and_then(|expr| expr.as_object()) else {
    #[cfg_attr(coverage_nightly, coverage(off))]
    {
      stylex_panic!("{}", VALUES_MUST_BE_OBJECT)
    }
  };

  let class_name_prefix = state.options.class_name_prefix.clone();
  let debug = state.options.debug;
  let enable_debug_class_names = state.options.enable_debug_class_names;
  let export_id = match state.export_id.clone() {
    Some(id) => id,
    #[cfg_attr(coverage_nightly, coverage(off))]
    None => stylex_panic!("{}", EXPORT_ID_NOT_SET),
  };

  let js_output = obj_map(
    ObjMapType::Object(constants.clone()),
    state,
    |item, _| -> Rc<FlatCompiledStylesValue> {
      let result = match item.as_ref() {
        #[cfg_attr(coverage_nightly, coverage(off))]
        FlatCompiledStylesValue::InjectableStyle(_) => {
          stylex_panic!("{}", INJECTABLE_STYLE_NOT_SUPPORTED)
        },
        FlatCompiledStylesValue::Tuple(_key, value, _) => {
          let serialized_value =
            serialize_value_to_json_string(EvaluateResultValue::Expr(*value.clone()));

          FlatCompiledStylesValue::String(serialized_value)
        },
        #[cfg_attr(coverage_nightly, coverage(off))]
        _ => stylex_unimplemented!(
          "FlatCompiledStylesValue variant not supported in stylex_define_consts"
        ),
      };

      Rc::new(result)
    },
  );

  let injectable_types = js_output
    .iter()
    .filter_map(|(key, value)| {
      if let FlatCompiledStylesValue::String(value) = value.as_ref() {
        let var_safe_key = if key.chars().next().unwrap_or('\0') >= '0'
          && key.chars().next().unwrap_or('\0') <= '9'
        {
          format!("_{}", key)
        } else {
          key.to_string()
        }
        .chars()
        .map(|c| if c.is_alphanumeric() { c } else { '_' })
        .collect::<String>();

        let const_key = if key.starts_with("--") {
          // Preserve user-authored CSS custom property name without the leading `--`
          key.chars().skip(2).collect::<String>()
        } else {
          let key_hash = create_key_hash(&export_id, key);

          if debug && enable_debug_class_names {
            format!("{}-{}{}", var_safe_key, class_name_prefix, key_hash)
          } else {
            format!("{}{}", class_name_prefix, key_hash)
          }
        };

        Some((
          const_key.to_owned().into(),
          Rc::new(InjectableStyleKind::Const(InjectableConstStyle {
            ltr: String::default(),
            rtl: None,
            priority: Some(0.0),
            const_key,
            const_value: value.to_owned(),
          })),
        ))
      } else {
        None
      }
    })
    .collect();

  (js_output, injectable_types)
}
