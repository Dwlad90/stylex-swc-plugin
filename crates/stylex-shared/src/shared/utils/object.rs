use std::rc::Rc;

use indexmap::IndexMap;
use swc_core::ecma::ast::{Expr, KeyValueProp};

use crate::shared::{
  enums::data_structures::{
    flat_compiled_styles_value::FlatCompiledStylesValue, obj_map_type::ObjMapType,
  },
  structures::{order_pair::OrderPair, pair::Pair, state_manager::StateManager},
  utils::common::{get_key_str, get_key_values_from_object},
};

pub(crate) fn obj_map<F>(
  prop_values: ObjMapType,
  state: &mut StateManager,
  mapper: F,
) -> IndexMap<String, Rc<FlatCompiledStylesValue>>
where
  F: Fn(Rc<FlatCompiledStylesValue>, &mut StateManager) -> Rc<FlatCompiledStylesValue>,
{
  let mut variables_map = IndexMap::new();

  match prop_values {
    ObjMapType::Object(obj) => {
      let key_values = get_key_values_from_object(&obj);

      for key_value in key_values.iter() {
        let key = get_key_str(key_value);

        let value = Rc::new(FlatCompiledStylesValue::Tuple(
          key.clone(),
          key_value.value.clone(),
          None,
        ));

        let result = mapper(value, state);

        variables_map.insert(key, result);
      }
    }

    ObjMapType::Map(map) => {
      for (key, value) in map {
        // Created hashed variable names with fileName//themeName//key
        let result = mapper(value, state);

        variables_map.insert(key, result);
      }
    }
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
  let object = obj.as_object().expect("Object expected");

  get_key_values_from_object(object)
}

pub(crate) fn obj_from_entries(entries: &[OrderPair]) -> IndexMap<String, String> {
  let mut map = IndexMap::with_capacity(entries.len());

  for OrderPair(key, value) in entries {
    map.insert(
      key.clone(),
      value.as_ref().expect("Value is not a string").clone(),
    );
  }

  map
}

pub(crate) fn obj_map_keys(
  entries: &IndexMap<String, String>,
  mapper: fn(&str) -> String,
) -> IndexMap<String, Rc<FlatCompiledStylesValue>> {
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
