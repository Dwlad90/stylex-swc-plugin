use std::borrow::Cow;

use indexmap::IndexMap;

use stylex_macros::stylex_panic;
use stylex_structures::pre_rule_value::PreRuleValue;
use swc_core::ecma::ast::{Expr, KeyValueProp};

use crate::shared::utils::core::flat_map_expanded_shorthands::flat_map_expanded_shorthands;
use stylex_ast::ast::convertors::{convert_key_value_to_str, get_key_values_from_object};
use stylex_state::state_manager::StateManager;
use stylex_structures::{
  order_pair::OrderPair,
  pair::{Pair, PairCow},
  raw_value::TRawValue,
};
use stylex_utils::collections::{FxBuildHasher, FxIndexMap};

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

/// Maps each declaration with `mapper`, and keeps one per property name.
///
/// `mapper` can rename a property, so two declarations can map to one name.
/// Then the later value wins and stays in the place of the first, as for any
/// object key.
pub(crate) fn obj_map_entry<'a>(
  pairs: &'a [Pair],
  mapper: impl Fn(&'a Pair) -> PairCow<'a>,
) -> Vec<PairCow<'a>> {
  let mut entries = FxIndexMap::with_capacity_and_hasher(pairs.len(), FxBuildHasher);

  for pair in pairs {
    let mapped = mapper(pair);
    entries.insert(mapped.key.clone(), mapped);
  }

  entries.into_values().collect()
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
  map_keys_and_build_declarations(entries, state, mapper, ValueKey::Mapped)
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
  map_keys_and_build_declarations(entries, state, mapper, ValueKey::Original)
}

/// The key that [`map_keys_and_build_declarations`] converts a value with.
enum ValueKey {
  /// The key as the source gives it.
  Original,
  /// The key after the mapper.
  Mapped,
}

/// Maps each key and converts each raw value to CSS text, with the key that
/// `value_key` selects.
///
/// Two keys can map to the same name. They make one declaration: it keeps the
/// position of the first key and the value of the last key.
fn map_keys_and_build_declarations(
  entries: &IndexMap<String, TRawValue>,
  state: &mut StateManager,
  mapper: impl Fn(&str) -> String,
  value_key: ValueKey,
) -> Vec<Pair> {
  let mut map: IndexMap<String, String> = IndexMap::with_capacity(entries.len());

  for (key, value) in entries {
    let mapped_key = mapper(key);
    let transform_key = match value_key {
      ValueKey::Original => key.as_str(),
      ValueKey::Mapped => mapped_key.as_str(),
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
