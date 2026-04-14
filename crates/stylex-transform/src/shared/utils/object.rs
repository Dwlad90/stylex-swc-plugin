use std::rc::Rc;

use indexmap::IndexMap;

use stylex_macros::stylex_panic;
use swc_core::ecma::ast::{Expr, KeyValueProp};

use crate::shared::{
  enums::data_structures::{
    flat_compiled_styles_value::FlatCompiledStylesValue, obj_map_type::ObjMapType,
  },
  structures::{pre_rule::PreRuleValue, state_manager::StateManager, types::FlatCompiledStyles},
  utils::{
    common::get_key_values_from_object,
    core::flat_map_expanded_shorthands::flat_map_expanded_shorthands,
  },
};
use stylex_structures::{order_pair::OrderPair, pair::Pair};

use super::ast::convertors::convert_key_value_to_str;

pub(crate) fn obj_map<F>(
  prop_values: ObjMapType,
  state: &mut StateManager,
  mapper: F,
) -> FlatCompiledStyles
where
  F: Fn(Rc<FlatCompiledStylesValue>, &mut StateManager) -> Rc<FlatCompiledStylesValue>,
{
  let mut variables_map = IndexMap::new();

  match prop_values {
    ObjMapType::Object(obj) => {
      let key_values = get_key_values_from_object(&obj);

      for key_value in key_values.iter() {
        let key = convert_key_value_to_str(key_value);

        let value = Rc::new(FlatCompiledStylesValue::Tuple(
          key.clone(),
          key_value.value.clone(),
          None,
        ));

        let result = mapper(value, state);

        variables_map.insert(key, result);
      }
    },

    ObjMapType::Map(map) => {
      for (key, value) in map {
        // Created hashed variable names with fileName//themeName//key
        let result = mapper(value, state);

        variables_map.insert(key, result);
      }
    },
  }

  variables_map
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub(crate) struct Pipe<T> {
  value: T,
}

impl<T> Pipe<T> {
  pub fn new(value: T) -> Self {
    Self { value }
  }

  pub fn pipe<U, F>(self, mapper: F) -> Pipe<U>
  where
    F: FnOnce(T) -> U,
  {
    Pipe::new(mapper(self.value))
  }

  pub fn done(self) -> T {
    self.value
  }

  pub fn create(value: T) -> Self {
    Self::new(value)
  }
}

pub(crate) fn obj_entries(obj: &Expr) -> Vec<KeyValueProp> {
  let object = match obj.as_object() {
    Some(o) => o,
    #[cfg_attr(coverage_nightly, coverage(off))]
    None => stylex_panic!("Object expected"),
  };

  get_key_values_from_object(object)
}

pub(crate) fn obj_from_entries(entries: &[OrderPair]) -> IndexMap<String, String> {
  let mut map = IndexMap::with_capacity(entries.len());

  for OrderPair(key, value) in entries {
    map.insert(
      key.clone(),
      match value.as_ref() {
        Some(v) => v.clone(),
        #[cfg_attr(coverage_nightly, coverage(off))]
        None => stylex_panic!("Value is not a string"),
      },
    );
  }

  map
}

pub(crate) fn obj_map_keys_string(
  entries: &IndexMap<String, String>,
  mapper: impl Fn(&str) -> String,
) -> FlatCompiledStyles {
  let mut map = IndexMap::with_capacity(entries.len());

  for (key, value) in entries {
    let mapped_key = mapper(key);

    map.insert(
      mapped_key.clone(),
      Rc::new(FlatCompiledStylesValue::KeyValue(Pair::new(
        mapped_key,
        value.clone(),
      ))),
    );
  }

  map
}

pub(crate) fn obj_map_keys_key_value(
  entries: &FlatCompiledStyles,
  mapper: fn(&str) -> String,
) -> FlatCompiledStyles {
  let mut map = IndexMap::with_capacity(entries.len());

  for (key, value) in entries {
    let obejct_key = mapper(key);

    let key_values = match value.as_key_values() {
      Some(kv) => kv,
      #[cfg_attr(coverage_nightly, coverage(off))]
      None => stylex_panic!("Value must be a key-value pairs"),
    };

    let object_key_values = key_values
      .iter()
      .map(|pair| Pair::new(pair.key.clone(), pair.value.clone()))
      .collect::<Vec<Pair>>();

    map.insert(
      obejct_key.clone(),
      Rc::new(FlatCompiledStylesValue::KeyValues(object_key_values)),
    );
  }

  map
}

pub(crate) fn _obj_map_entries(
  entries: &IndexMap<String, String>,
  mapper: fn((&str, &str)) -> Pair,
) -> IndexMap<String, FlatCompiledStylesValue> {
  let mut map = IndexMap::with_capacity(entries.len());

  for (key, value) in entries {
    let result = mapper((key, value));
    map.insert(key.clone(), FlatCompiledStylesValue::KeyValue(result));
  }

  map
}

pub(crate) fn preprocess_object_properties(
  style: &Expr,
  state: &mut StateManager,
) -> IndexMap<String, String> {
  let res: Vec<OrderPair> = obj_entries(&style.clone())
    .iter()
    .flat_map(|pair| {
      let key = convert_key_value_to_str(pair);

      flat_map_expanded_shorthands(
        (key, PreRuleValue::Expr(*pair.value.clone())),
        &state.options,
      )
      .into_iter()
      .collect::<Vec<OrderPair>>()
    })
    .filter(|item| item.1.is_some())
    .collect::<Vec<OrderPair>>();

  obj_from_entries(&res)
}
