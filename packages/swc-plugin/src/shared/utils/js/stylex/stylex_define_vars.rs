use indexmap::IndexMap;
use swc_core::ecma::ast::{KeyValueProp, PropName};

use crate::shared::{
  enums::{FlatCompiledStylesValue, ObjMapType},
  structures::{
    evaluate_result::EvaluateResultValue, injectable_style::InjectableStyle,
    state_manager::StateManager,
  },
  utils::{
    common::{create_hash, get_css_value},
    object::obj_map,
    stylex::define_vars_utils::construct_css_variables_string,
  },
};

pub(crate) fn stylex_define_vars(
  variables: &EvaluateResultValue,
  state: &mut StateManager,
) -> (
  IndexMap<String, Box<FlatCompiledStylesValue>>,
  IndexMap<String, Box<InjectableStyle>>,
) {
  // dbg!(&variables);

  let theme_name_hash = format!(
    "{}{}",
    state.options.class_name_prefix,
    create_hash(state.theme_name.clone().unwrap().as_str())
  );

  // dbg!(&theme_name_hash, state.theme_name.clone().unwrap().as_str());

  let mut typed_variables: IndexMap<String, Box<FlatCompiledStylesValue>> = IndexMap::new();

  let Some(variables) = variables.as_expr().and_then(|expr| expr.as_object()) else {
    panic!("Values must be an object")
  };

  let variables_map = obj_map(
    ObjMapType::Object(variables.clone()),
    |item| -> Box<FlatCompiledStylesValue> {
      let reuslt = match item.as_ref() {
        FlatCompiledStylesValue::String(_) => panic!("String is not supported"),
        FlatCompiledStylesValue::InjectableStyle(_) => {
          panic!("InjectableStyle is not supported")
        }
        FlatCompiledStylesValue::Tuple(key, value, _) => {
          // dbg!(&value);

          // Created hashed variable names with fileName//themeName//key
          let name_hash = format!(
            "{}{}",
            state.options.class_name_prefix,
            create_hash(format!("{}.{}", state.theme_name.clone().unwrap(), key).as_str())
          );

          let (css_value, css_type) = get_css_value(KeyValueProp {
            key: PropName::Str(key.clone().into()),
            value: value.clone(),
          });

          FlatCompiledStylesValue::Tuple(name_hash.clone(), css_value, css_type)
        }
        FlatCompiledStylesValue::Null => todo!("Null"),
        FlatCompiledStylesValue::IncludedStyle(_) => todo!("IncludedStyle"),
        FlatCompiledStylesValue::Bool(_) => todo!("Bool"),
        FlatCompiledStylesValue::KeyValue(_) => todo!("KeyValue"),
        FlatCompiledStylesValue::CSSType(_, _, _) => todo!("CSSType"),
      };

      Box::new(reuslt)
    },
  );

  // dbg!(&variables_map);

  let theme_variables_objects = obj_map(ObjMapType::Map(variables_map.clone()), |item| match item
    .as_ref()
  {
    FlatCompiledStylesValue::String(_) => panic!("String is not supported"),
    FlatCompiledStylesValue::InjectableStyle(_) => {
      panic!("InjectableStyle is not supported")
    }
    FlatCompiledStylesValue::Tuple(key, _, _) => {
      Box::new(FlatCompiledStylesValue::String(format!("var(--{})", key)))
    }
    FlatCompiledStylesValue::Null => todo!("Null"),
    FlatCompiledStylesValue::IncludedStyle(_) => todo!("IncludedStyle"),
    FlatCompiledStylesValue::Bool(_) => todo!("Bool"),
    FlatCompiledStylesValue::KeyValue(_) => todo!("KeyValue"),
    FlatCompiledStylesValue::CSSType(_, _, _) => todo!("CSSType"),
  });

  // dbg!(&variables_map, &theme_variables_objects,);

  let injectable_styles =
    construct_css_variables_string(&variables_map, &theme_name_hash, &mut typed_variables);

  // dbg!(&injectable_styles);
  // dbg!(&typed_variables);

  let injectable_types = obj_map(
    ObjMapType::Map(typed_variables),
    |item| -> Box<FlatCompiledStylesValue> {
      let result = match item.as_ref() {
        FlatCompiledStylesValue::String(_) => panic!("String is not supported"),
        FlatCompiledStylesValue::Null => todo!("Null"),
        FlatCompiledStylesValue::IncludedStyle(_) => todo!("IncludedStyle"),
        FlatCompiledStylesValue::Bool(_) => todo!("Bool"),
        FlatCompiledStylesValue::KeyValue(_) => todo!("KeyValue"),
        FlatCompiledStylesValue::CSSType(name_hash, syntax, initial_value) => {
          let property = format!(
            "@property --{} {{ syntax: \"{}\"; inherits: true; initial-value: {} }}",
            name_hash, syntax, initial_value
          );

          FlatCompiledStylesValue::InjectableStyle(InjectableStyle {
            ltr: property,
            rtl: Option::None,
            priority: Option::Some(0.0),
          })
        }
        FlatCompiledStylesValue::InjectableStyle(_) => todo!("InjectableStyle"),
        FlatCompiledStylesValue::Tuple(_, _, _) => todo!("Tuple"),
      };

      Box::new(result)
    },
  );

  let mut injectable_types: IndexMap<String, Box<InjectableStyle>> = injectable_types
    .iter()
    .filter_map(|(key, value)| {
      if let Some(inj_style) = value.as_injectable_style() {
        return Some((key.clone(), Box::new(inj_style.clone())));
      }

      Option::None
    })
    .collect();

  let mut theme_variables_objects = theme_variables_objects.clone();

  theme_variables_objects.insert(
    "__themeName__".to_string(),
    Box::new(FlatCompiledStylesValue::String(theme_name_hash)),
  );

  injectable_types.extend(injectable_styles);

  (theme_variables_objects, injectable_types)
}
