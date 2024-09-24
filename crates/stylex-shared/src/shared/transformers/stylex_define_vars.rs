use indexmap::IndexMap;
use swc_core::ecma::ast::{KeyValueProp, PropName};

use crate::shared::{
  enums::data_structures::{
    evaluate_result_value::EvaluateResultValue,
    flat_compiled_styles_value::FlatCompiledStylesValue, obj_map_type::ObjMapType,
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
  IndexMap<String, Box<FlatCompiledStylesValue>>,
  IndexMap<String, Box<InjectableStyle>>,
) {
  let theme_name_hash = format!(
    "{}{}",
    state.options.class_name_prefix,
    create_hash(state.theme_name.as_ref().unwrap())
  );

  let mut typed_variables: IndexMap<String, Box<FlatCompiledStylesValue>> = IndexMap::new();

  let Some(variables) = variables.as_expr().and_then(|expr| expr.as_object()) else {
    panic!("Values must be an object")
  };

  let variables_map = obj_map(
    ObjMapType::Object(variables.clone()),
    state,
    |item, state| -> Box<FlatCompiledStylesValue> {
      let reuslt = match item.as_ref() {
        FlatCompiledStylesValue::InjectableStyle(_) => {
          panic!("InjectableStyle is not supported")
        }
        FlatCompiledStylesValue::Tuple(key, value, _) => {
          let str_to_hash = format!("{}.{}", state.theme_name.clone().unwrap(), key);

          // Created hashed variable names with fileName//themeName//key
          let name_hash = if key.starts_with("--") {
            key.get(2..).unwrap_or_default()
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

      Box::new(reuslt)
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
        Box::new(FlatCompiledStylesValue::String(format!("var(--{})", key)))
      }

      _ => unreachable!("Unsupported value type"),
    },
  );

  let injectable_styles =
    construct_css_variables_string(&variables_map, &theme_name_hash, &mut typed_variables);

  let injectable_types = obj_map(
    ObjMapType::Map(typed_variables),
    state,
    |item, _| -> Box<FlatCompiledStylesValue> {
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

      Box::new(result)
    },
  );

  let mut injectable_types: IndexMap<String, Box<InjectableStyle>> = injectable_types
    .iter()
    .filter_map(|(key, value)| {
      value
        .as_injectable_style()
        .map(|inj_style| (key.to_owned(), Box::new(inj_style.clone())))
    })
    .collect();

  theme_variables_objects.insert(
    "__themeName__".to_string(),
    Box::new(FlatCompiledStylesValue::String(theme_name_hash)),
  );

  injectable_types.extend(injectable_styles);

  (theme_variables_objects, injectable_types)
}
