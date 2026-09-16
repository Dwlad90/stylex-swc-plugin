use indexmap::IndexMap;
use stylex_ast::ast::convertors::{create_bool_expr, create_null_expr, create_number_expr};
use stylex_macros::stylex_unreachable;
use swc_core::ecma::ast::{Expr, PropOrSpread};

use stylex_ast::ast::factories::{
  create_key_value_prop, create_object_expression, create_string_key_value_prop,
};
use stylex_state::{
  flat_compiled_styles_value::FlatCompiledStylesValue,
  types::{FlatCompiledStyles, StylesObjectMap},
};

pub(crate) fn remove_objects_with_spreads(obj: &StylesObjectMap) -> StylesObjectMap {
  let mut new_obj = IndexMap::with_capacity(obj.len());

  for (key, value) in obj.iter() {
    new_obj.insert(key.clone(), value.clone());
  }

  new_obj
}

/// The object literal one flat map of compiled values spells:
/// `{ key: value, ... }`.
///
/// A compiled namespace is one such map, and so are the variables `defineVars`
/// answers, the values `defineConsts` answers and a theme's overrides. Every
/// value in one is a string, a null or a boolean, because that is all the
/// compiler writes. A string that reads as a number is written back as a
/// number, the way the source spelled it.
pub(crate) fn convert_values_to_ast(values: &FlatCompiledStyles) -> Expr {
  let props = values
    .iter()
    .map(|(key, value)| match value.as_ref() {
      FlatCompiledStylesValue::String(value) => match value.parse::<f64>() {
        Ok(number) => create_key_value_prop(key.as_str(), create_number_expr(number)),
        Err(_) => create_string_key_value_prop(key.as_str(), value.as_str()),
      },
      FlatCompiledStylesValue::Null => create_key_value_prop(key.as_str(), create_null_expr()),
      FlatCompiledStylesValue::Bool(value) => {
        create_key_value_prop(key.as_str(), create_bool_expr(*value))
      },
      _ => stylex_unreachable!("Encountered an unsupported value type during AST conversion."),
    })
    .collect::<Vec<PropOrSpread>>();

  create_object_expression(props)
}

/// The object literal a whole compiled style object spells: one namespace per
/// key, each namespace the object [`convert_values_to_ast`] spells.
pub(crate) fn convert_namespaces_to_ast(namespaces: &StylesObjectMap) -> Expr {
  let props = namespaces
    .iter()
    .map(|(key, values)| create_key_value_prop(key.as_str(), convert_values_to_ast(values)))
    .collect::<Vec<PropOrSpread>>();

  create_object_expression(props)
}

#[cfg(test)]
#[path = "tests/js_to_ast_tests.rs"]
mod tests;
