use indexmap::IndexMap;
use stylex_macros::stylex_unreachable;
use swc_core::ecma::ast::{Expr, PropOrSpread};

use stylex_ast::ast::factories::{
  create_key_value_prop,
  create_object_expression,
  create_string_key_value_prop,
};
use crate::shared::enums::data_structures::flat_compiled_styles_value::FlatCompiledStylesValue;
use crate::shared::structures::types::{FlatCompiledStyles, StylesObjectMap};
use crate::shared::utils::ast::convertors::{create_bool_expr, create_null_expr, create_number_expr};

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

        let prop = create_key_value_prop(key.as_str(), expr);

        props.push(prop);
      }
    }
    NestedStringObject::FlatCompiledStylesValues(obj) => {
      for (key, value) in obj.iter() {
        let prop = match value.as_ref() {
          FlatCompiledStylesValue::String(value) => {
            if let Ok(num) = value.parse::<f64>() {
              create_key_value_prop(key.as_str(), create_number_expr(num))
            } else {
              create_string_key_value_prop(key.as_str(), value.as_str())
            }
          }
          FlatCompiledStylesValue::Null => {
            create_key_value_prop(key.as_str(), create_null_expr())
          }
          FlatCompiledStylesValue::Bool(value) => {
            create_key_value_prop(key.as_str(), create_bool_expr(*value))
          }
          _ => {
            stylex_unreachable!("Encountered an unsupported value type during AST conversion.")
          }
        };

        props.push(prop);
      }
    }
  }

  create_object_expression(props)
}
