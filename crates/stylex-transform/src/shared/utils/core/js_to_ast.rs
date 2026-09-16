use indexmap::IndexMap;
use stylex_ast::ast::convertors::{
  create_bool_expr, create_null_expr, create_number_expr, create_string_expr,
};
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

/// One property of an object this compiler wrote: the name it is written under,
/// and what it holds.
///
/// Every object the compiler writes here comes from a map whose keys are names,
/// so the list it spells can hold nothing else -- no spread, no method, no
/// computed key. Handing the list on instead of the object literal keeps that
/// in the type, so a reader below does not ask again what the producer has
/// already answered.
///
/// The name is borrowed from the map it was read from, because every reader
/// spells it straight into a key and none of them outlives the map.
pub(crate) struct NamedProp<'a, V> {
  pub(crate) key: &'a str,
  pub(crate) value: V,
}

/// The properties of one compiled object, in the order they were written.
pub(crate) type CompiledValues<'a> = Vec<NamedProp<'a, Expr>>;

/// The namespaces of a compiled style object, each holding its own properties.
pub(crate) type CompiledNamespaces<'a> = Vec<NamedProp<'a, CompiledValues<'a>>>;

pub(crate) fn remove_objects_with_spreads(obj: &StylesObjectMap) -> StylesObjectMap {
  let mut new_obj = IndexMap::with_capacity(obj.len());

  for (key, value) in obj.iter() {
    new_obj.insert(key.clone(), value.clone());
  }

  new_obj
}

/// The properties one flat map of compiled values spells.
///
/// A compiled namespace is one such map, and so are the variables `defineVars`
/// answers, the values `defineConsts` answers and a theme's overrides. Every
/// value in one is a string, a null or a boolean, because that is all the
/// compiler writes. A string that reads as a number is written back as a
/// number, the way the source spelled it.
///
/// The properties a `props` merge answers are one such map too, and they hold
/// one value more: the inline style, which is an object of its own.
///
/// Answered as an iterator, so a caller that wants the object literal builds
/// the one list it needs rather than a list of names and a list of properties.
fn compiled_value_props(values: &FlatCompiledStyles) -> impl Iterator<Item = NamedProp<'_, Expr>> {
  values.iter().map(|(key, value)| {
    let value = match value.as_ref() {
      FlatCompiledStylesValue::String(value) => match value.parse::<f64>() {
        Ok(number) => create_number_expr(number),
        Err(_) => create_string_expr(value.as_str()),
      },
      FlatCompiledStylesValue::Null => create_null_expr(),
      FlatCompiledStylesValue::Bool(value) => create_bool_expr(*value),
      FlatCompiledStylesValue::KeyValues(pairs) => convert_inline_style_to_ast(pairs),
      _ => stylex_unreachable!("Encountered an unsupported value type during AST conversion."),
    };

    NamedProp {
      key: key.as_str(),
      value,
    }
  })
}

/// The property one named value is written as.
fn named_prop_to_key_value(prop: NamedProp<'_, Expr>) -> PropOrSpread {
  create_key_value_prop(prop.key, prop.value)
}

/// The object literal a list of named properties spells:
/// `{ key: value, ... }`.
pub(crate) fn named_props_to_object(props: CompiledValues<'_>) -> Expr {
  create_object_expression(props.into_iter().map(named_prop_to_key_value).collect())
}

/// The object literal one flat map of compiled values spells:
/// `{ key: value, ... }`.
pub(crate) fn convert_values_to_ast(values: &FlatCompiledStyles) -> Expr {
  create_object_expression(
    compiled_value_props(values)
      .map(named_prop_to_key_value)
      .collect(),
  )
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

/// A whole compiled style object: one namespace per key, each namespace the
/// properties [`compiled_values`] spells.
pub(crate) fn compiled_namespaces(namespaces: &StylesObjectMap) -> CompiledNamespaces<'_> {
  namespaces
    .iter()
    .map(|(key, values)| NamedProp {
      key: key.as_str(),
      value: compiled_value_props(values).collect(),
    })
    .collect()
}

/// The object literal a whole compiled style object spells: one namespace per
/// key, each namespace the object [`convert_values_to_ast`] spells.
pub(crate) fn namespaces_to_object(namespaces: CompiledNamespaces<'_>) -> Expr {
  create_object_expression(
    namespaces
      .into_iter()
      .map(|NamedProp { key, value }| create_key_value_prop(key, named_props_to_object(value)))
      .collect::<Vec<PropOrSpread>>(),
  )
}

#[cfg(test)]
#[path = "tests/js_to_ast_tests.rs"]
mod tests;
