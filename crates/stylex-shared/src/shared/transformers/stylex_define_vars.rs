use std::rc::Rc;

use indexmap::IndexMap;
use swc_core::ecma::ast::{KeyValueProp, PropName};

use crate::shared::{
  enums::data_structures::{
    evaluate_result_value::EvaluateResultValue,
    flat_compiled_styles_value::FlatCompiledStylesValue, injectable_style::InjectableStyleKind,
    obj_map_type::ObjMapType,
  },
  structures::{injectable_style::InjectableStyle, state_manager::StateManager},
  utils::{
    common::{create_hash, get_css_value},
    core::define_vars_utils::construct_css_variables_string,
    object::obj_map,
  },
};

pub(crate) fn stylex_define_vars(
  variables: &EvaluateResultValue,
  state: &mut StateManager,
) -> (
  IndexMap<String, Rc<FlatCompiledStylesValue>>,
  IndexMap<String, Rc<InjectableStyleKind>>,
) {
  let var_group_hash = format!(
    "{}{}",
    state.options.class_name_prefix,
    create_hash(state.export_id.as_ref().unwrap())
  );

  let mut typed_variables: IndexMap<String, Rc<FlatCompiledStylesValue>> = IndexMap::new();

  let Some(variables) = variables.as_expr().and_then(|expr| expr.as_object()) else {
    panic!("Values must be an object")
  };

  let variables_map = obj_map(
    ObjMapType::Object(variables.clone()),
    state,
    |item, state| -> Rc<FlatCompiledStylesValue> {
      let result = match item.as_ref() {
        FlatCompiledStylesValue::InjectableStyle(_) => {
          panic!("InjectableStyle is not supported")
        }
        FlatCompiledStylesValue::Tuple(key, value, _) => {
          let str_to_hash = format!("{}.{}", state.export_id.as_ref().unwrap(), key);

          let debug = state.options.debug;
          let enable_debug_class_names = state.options.enable_debug_class_names;

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

          // Created hashed variable names with fileName//themeName//key
          let name_hash = if key.starts_with("--") {
            key.get(2..).unwrap_or_default()
          } else if debug && enable_debug_class_names {
            &format!(
              "{}-{}{}",
              var_safe_key,
              &state.options.class_name_prefix,
              create_hash(str_to_hash.as_str())
            )
          } else {
            &format!(
              "{}{}",
              &state.options.class_name_prefix,
              create_hash(str_to_hash.as_str())
            )
          };

          let (css_value, css_type) = get_css_value(KeyValueProp {
            key: PropName::Str(key.clone().into()),
            value: value.clone(),
          });

          FlatCompiledStylesValue::Tuple(name_hash.to_string(), css_value, css_type)
        }
        _ => unimplemented!(),
      };

      Rc::new(result)
    },
  );

  let mut theme_variables_objects = obj_map(
    ObjMapType::Map(variables_map.clone()),
    state,
    |item, _| match item.as_ref() {
      FlatCompiledStylesValue::InjectableStyle(_) => {
        panic!("InjectableStyle is not supported")
      }
      FlatCompiledStylesValue::Tuple(key, _, _) => {
        Rc::new(FlatCompiledStylesValue::String(format!("var(--{})", key)))
      }
      _ => unreachable!("Unsupported value type"),
    },
  );

  let injectable_styles =
    construct_css_variables_string(&variables_map, &var_group_hash, &mut typed_variables);

  let injectable_types = obj_map(
    ObjMapType::Map(typed_variables),
    state,
    |item, _| -> Rc<FlatCompiledStylesValue> {
      let result = match item.as_ref() {
        FlatCompiledStylesValue::CSSType(name_hash, syntax, initial_value) => {
          let property = format!(
            "@property --{} {{ syntax: \"{}\"; inherits: true; initial-value: {} }}",
            name_hash, syntax, initial_value
          );

          FlatCompiledStylesValue::InjectableStyle(InjectableStyle {
            ltr: property,
            ..Default::default()
          })
        }
        _ => unreachable!("Unsupported value type"),
      };

      Rc::new(result)
    },
  );

  let mut injectable_types: IndexMap<String, Rc<InjectableStyleKind>> = injectable_types
    .iter()
    .filter_map(|(key, value)| {
      value.as_injectable_style().map(|inj_style| {
        (
          key.to_owned(),
          Rc::new(InjectableStyleKind::Regular(inj_style.clone())),
        )
      })
    })
    .collect();

  theme_variables_objects.insert(
    "__varGroupHash__".to_string(),
    Rc::new(FlatCompiledStylesValue::String(var_group_hash)),
  );

  injectable_types.extend(injectable_styles);

  (theme_variables_objects, injectable_types)
}
