use indexmap::IndexMap;
use swc_core::ecma::ast::{Expr, PropOrSpread};

use crate::shared::{
  enums::data_structures::flat_compiled_styles_value::FlatCompiledStylesValue,
  structures::types::{FlatCompiledStyles, StylesObjectMap},
  utils::ast::{
    convertors::{bool_to_expression, null_to_expression},
    factories::{
      object_expression_factory, prop_or_spread_expression_factory, prop_or_spread_string_factory,
    },
  },
};

pub(crate) fn remove_objects_with_spreads(obj: &StylesObjectMap) -> StylesObjectMap {
  let mut new_obj = IndexMap::with_capacity(obj.len());

  for (key, value) in obj.iter() {
    new_obj.insert(key.clone(), value.clone());
  }

  new_obj
}

#[derive(Debug, PartialEq, Clone)]
pub(crate) enum NestedStringObject {
  FlatCompiledStyles(StylesObjectMap),
  FlatCompiledStylesValues(FlatCompiledStyles),
}

impl NestedStringObject {
  pub(crate) fn _as_styles(&self) -> Option<&StylesObjectMap> {
    match self {
      NestedStringObject::FlatCompiledStyles(obj) => Some(obj),
      _ => None,
    }
  }

  pub(crate) fn as_values(&self) -> Option<&FlatCompiledStyles> {
    match self {
      NestedStringObject::FlatCompiledStylesValues(obj) => Some(obj),
      _ => None,
    }
  }
}

pub(crate) fn convert_object_to_ast(obj: &NestedStringObject) -> Expr {
  let mut props: Vec<PropOrSpread> = vec![];

  match obj {
    NestedStringObject::FlatCompiledStyles(obj) => {
      for (key, value) in obj.iter() {
        let expr = convert_object_to_ast(&NestedStringObject::FlatCompiledStylesValues(
          (**value).clone(),
        ));

        let prop = prop_or_spread_expression_factory(key.as_str(), expr);

        props.push(prop);
      }
    }
    NestedStringObject::FlatCompiledStylesValues(obj) => {
      for (key, value) in obj.iter() {
        let prop = match value.as_ref() {
          FlatCompiledStylesValue::String(value) => {
            prop_or_spread_string_factory(key.as_str(), value.as_str())
          }
          FlatCompiledStylesValue::Null => {
            prop_or_spread_expression_factory(key.as_str(), null_to_expression())
          }
          FlatCompiledStylesValue::Bool(value) => {
            prop_or_spread_expression_factory(key.as_str(), bool_to_expression(*value))
          }
          _ => unreachable!("Unsupported value type"),
        };

        props.push(prop);
      }
    }
  }

  object_expression_factory(props)
}
