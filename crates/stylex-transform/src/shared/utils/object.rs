use std::borrow::Cow;

use indexmap::IndexMap;

use stylex_macros::stylex_panic;
use stylex_structures::pre_rule_value::PreRuleValue;
use swc_core::ecma::ast::{Expr, KeyValueProp};

use crate::shared::utils::core::flat_map_expanded_shorthands::flat_map_expanded_shorthands;
use stylex_ast::ast::convertors::{convert_key_value_to_str, get_key_values_from_object};
use stylex_state::state_manager::StateManager;
use stylex_structures::{order_pair::OrderPair, pair::Pair, raw_value::TRawValue};

use super::css::common::transform_value_cached;

pub(crate) fn obj_entries(obj: &Expr) -> Vec<KeyValueProp> {
  let object = match obj.as_object() {
    Some(o) => o,
    None => stylex_panic!("Object expected"),
  };

  get_key_values_from_object(object)
}

pub(crate) fn obj_from_entries(entries: &[OrderPair]) -> IndexMap<String, TRawValue> {
  let mut map = IndexMap::with_capacity(entries.len());

  for OrderPair(key, value) in entries {
    map.insert(
      key.to_string(),
      match value.as_ref() {
        Some(v) => v.clone(),
        None => stylex_panic!("Value is not a string"),
      },
    );
  }

  map
}

/// Maps each key, then converts each raw value with the mapped key.
///
/// The mapped key sets the unit suffix. A dashed key is not in the camelCase
/// lookup tables, so `font-weight: 700` gets `px`. `stylex.positionTry` uses
/// this order. Its properties give the same value with each key.
pub(crate) fn obj_map_keys_and_transform_values(
  entries: &IndexMap<String, TRawValue>,
  state: &mut StateManager,
  mapper: impl Fn(&str) -> String,
) -> Vec<Pair> {
  map_keys_and_build_declarations(entries, state, mapper, false)
}

/// Converts each raw value with its original key, then maps each key.
///
/// `stylex.create` uses this order. The camelCase key finds the unitless and
/// time properties, so `fontWeight: 700` stays `700`.
pub(crate) fn obj_transform_values_and_map_keys(
  entries: &IndexMap<String, TRawValue>,
  state: &mut StateManager,
  mapper: impl Fn(&str) -> String,
) -> Vec<Pair> {
  map_keys_and_build_declarations(entries, state, mapper, true)
}

/// Maps each key and converts each raw value to CSS text. When
/// `transform_with_original_key` is set, the value is converted with the
/// original key. When it is not set, the value is converted with the mapped key.
///
/// Two keys can map to the same name. They make one declaration: it keeps the
/// position of the first key and the value of the last key.
fn map_keys_and_build_declarations(
  entries: &IndexMap<String, TRawValue>,
  state: &mut StateManager,
  mapper: impl Fn(&str) -> String,
  transform_with_original_key: bool,
) -> Vec<Pair> {
  let mut map: IndexMap<String, String> = IndexMap::with_capacity(entries.len());

  for (key, value) in entries {
    let mapped_key = mapper(key);
    let transform_key = if transform_with_original_key {
      key.as_str()
    } else {
      mapped_key.as_str()
    };
    let value = transform_value_cached(transform_key, value, state);

    map.insert(mapped_key, value);
  }

  map
    .into_iter()
    .map(|(key, value)| Pair { key, value })
    .collect()
}

pub(crate) fn preprocess_object_properties(
  style: &Expr,
  state: &mut StateManager,
) -> IndexMap<String, TRawValue> {
  let res: Vec<OrderPair> = obj_entries(style)
    .iter()
    .flat_map(|pair| {
      let key = convert_key_value_to_str(pair);

      flat_map_expanded_shorthands(
        (Cow::Owned(key), PreRuleValue::Expr(*pair.value.clone())),
        &state.options,
      )
      .into_iter()
      .collect::<Vec<OrderPair>>()
    })
    .filter(|item| item.1.is_some())
    .collect::<Vec<OrderPair>>();

  obj_from_entries(&res)
}

#[cfg(test)]
#[path = "tests/object_tests.rs"]
mod tests;
