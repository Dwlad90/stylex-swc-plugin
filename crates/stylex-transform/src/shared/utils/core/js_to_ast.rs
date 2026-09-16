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
use stylex_structures::pair::Pair;

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
///
/// The properties a `props` merge answers are one such map too, and they hold
/// one value more: the inline style, which is an object of its own.
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
      FlatCompiledStylesValue::KeyValues(pairs) => {
        create_key_value_prop(key.as_str(), convert_inline_style_to_ast(pairs))
      },
      _ => stylex_unreachable!("Encountered an unsupported value type during AST conversion."),
    })
    .collect::<Vec<PropOrSpread>>();

  create_object_expression(props)
}

/// The object literal an inline style spells: `{ color: "blue", ... }`.
///
/// A `props` merge answers one of these under `style`, holding what the author
/// wrote beside the compiled styles. Each name keeps the spelling of the
/// source, because the runtime reads this object and not a stylesheet, and each
/// value stays text: `gridRow: '1'` is a string where the author wrote it, so
/// it is a string here.
fn convert_inline_style_to_ast(pairs: &[Pair]) -> Expr {
  let props = pairs
    .iter()
    .map(|pair| create_string_key_value_prop(pair.key.as_str(), pair.value.as_str()))
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
