use std::rc::Rc;

use indexmap::IndexMap;
use stylex_macros::{stylex_panic, stylex_unimplemented, stylex_unreachable};
use swc_core::ecma::ast::{KeyValueProp, PropName};

use crate::shared::enums::data_structures::evaluate_result_value::EvaluateResultValue;
use crate::shared::enums::data_structures::flat_compiled_styles_value::FlatCompiledStylesValue;
use crate::shared::enums::data_structures::obj_map_type::ObjMapType;
use crate::shared::structures::state_manager::StateManager;
use crate::shared::structures::types::{FlatCompiledStyles, InjectableStylesMap};
use crate::shared::utils::common::get_css_value;
use crate::shared::utils::core::define_vars_utils::construct_css_variables_string;
use stylex_utils::hash::create_hash;
use crate::shared::utils::object::obj_map;
use stylex_constants::constants::common::VAR_GROUP_HASH_KEY;
use stylex_constants::constants::messages::{
  EXPORT_ID_NOT_SET, INJECTABLE_STYLE_NOT_SUPPORTED, VALUES_MUST_BE_OBJECT,
};
use stylex_types::enums::data_structures::injectable_style::InjectableStyleKind;
use stylex_types::structures::injectable_style::InjectableStyle;

pub(crate) fn stylex_define_vars(
  variables: &EvaluateResultValue,
  state: &mut StateManager,
) -> (FlatCompiledStyles, InjectableStylesMap) {
  let var_group_hash = format!(
    "{}{}",
    state.options.class_name_prefix,
    create_hash(match state.export_id.as_ref() {
      Some(id) => id,
      None => stylex_panic!("{}", EXPORT_ID_NOT_SET),
    })
  );

  let mut typed_variables: FlatCompiledStyles = IndexMap::new();

  let Some(variables) = variables.as_expr().and_then(|expr| expr.as_object()) else {
    stylex_panic!("{}", VALUES_MUST_BE_OBJECT)
  };

  let variables_map = obj_map(
    ObjMapType::Object(variables.clone()),
    state,
    |item, state| -> Rc<FlatCompiledStylesValue> {
      let result = match item.as_ref() {
        FlatCompiledStylesValue::InjectableStyle(_) => {
          stylex_panic!("{}", INJECTABLE_STYLE_NOT_SUPPORTED)
        },
        FlatCompiledStylesValue::Tuple(key, value, _) => {
          let str_to_hash = format!(
            "{}.{}",
            match state.export_id.as_ref() {
              Some(id) => id,
              None => stylex_panic!("{}", EXPORT_ID_NOT_SET),
            },
            key
          );

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
        },
        _ => stylex_unimplemented!("Unsupported value type in define vars"),
      };

      Rc::new(result)
    },
  );

  let mut theme_variables_objects = obj_map(
    ObjMapType::Map(variables_map.clone()),
    state,
    |item, _| match item.as_ref() {
      FlatCompiledStylesValue::InjectableStyle(_) => {
        stylex_panic!("{}", INJECTABLE_STYLE_NOT_SUPPORTED)
      },
      FlatCompiledStylesValue::Tuple(key, _, _) => {
        Rc::new(FlatCompiledStylesValue::String(format!("var(--{})", key)))
      },
      _ => stylex_unreachable!("Unsupported value type"),
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
        },
        _ => stylex_unreachable!("Unsupported value type"),
      };

      Rc::new(result)
    },
  );

  let mut injectable_types: InjectableStylesMap = injectable_types
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
    VAR_GROUP_HASH_KEY.to_owned(),
    Rc::new(FlatCompiledStylesValue::String(var_group_hash)),
  );

  injectable_types.extend(injectable_styles);

  (theme_variables_objects, injectable_types)
}
