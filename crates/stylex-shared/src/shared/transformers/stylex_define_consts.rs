use std::rc::Rc;

use indexmap::IndexMap;

use crate::shared::{
  constants::messages,
  enums::data_structures::{
    evaluate_result_value::EvaluateResultValue,
    flat_compiled_styles_value::FlatCompiledStylesValue, injectable_style::InjectableStyleKind,
    obj_map_type::ObjMapType,
  },
  structures::{injectable_style::InjectableConstStyle, state_manager::StateManager},
  utils::{
    common::{create_hash, serialize_value_to_json_string},
    object::obj_map,
  },
};

pub(crate) fn stylex_define_consts(
  constants: &EvaluateResultValue,
  state: &mut StateManager,
) -> (
  IndexMap<String, Rc<FlatCompiledStylesValue>>,
  IndexMap<String, Rc<InjectableStyleKind>>,
) {
  let Some(constants) = constants.as_expr().and_then(|expr| expr.as_object()) else {
    panic!("Values must be an object")
  };

  let class_name_prefix = state.options.class_name_prefix.clone();
  let debug = state.options.debug;
  let enable_debug_class_names = state.options.enable_debug_class_names;
  let theme_name = state.theme_name.clone().expect("Theme name must be set");

  let js_output = obj_map(
    ObjMapType::Object(constants.clone()),
    state,
    |item, _| -> Rc<FlatCompiledStylesValue> {
      let result = match item.as_ref() {
        FlatCompiledStylesValue::InjectableStyle(_) => {
          panic!("InjectableStyle is not supported")
        }
        FlatCompiledStylesValue::Tuple(key, value, _) => {
          if key.starts_with("--") {
            panic!("{}", messages::INVALID_CONST_KEY)
          }

          let serialized_value =
            serialize_value_to_json_string(EvaluateResultValue::Expr(*value.clone()));

          FlatCompiledStylesValue::String(serialized_value)
        }
        _ => unimplemented!(),
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

        let name_hash = if debug && enable_debug_class_names {
          format!(
            "{}-{}{}",
            var_safe_key,
            class_name_prefix,
            create_hash(&format!("{}.{}", theme_name, key))
          )
        } else {
          format!(
            "{}{}",
            class_name_prefix,
            create_hash(&format!("{}.{}", theme_name, key))
          )
        };

        Some((
          name_hash.to_owned(),
          Rc::new(InjectableStyleKind::Const(InjectableConstStyle {
            ltr: String::default(),
            rtl: None,
            priority: Some(0.0),
            const_key: key.to_owned(),
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
