use indexmap::IndexMap;
use swc_core::ecma::ast::{Expr, KeyValueProp, Lit, PropName};

use crate::shared::{
    enums::{FlatCompiledStylesValue, ObjMapType},
    structures::{order_pair::OrderPair, pair::Pair},
    utils::common::{get_key_str, get_key_values_from_object},
};

pub(crate) fn obj_map<F>(
    prop_values: ObjMapType,
    mapper: F,
) -> IndexMap<String, FlatCompiledStylesValue>
where
    F: Fn(FlatCompiledStylesValue) -> FlatCompiledStylesValue,
{
    let mut variables_map = IndexMap::new();

    match prop_values {
        ObjMapType::Object(obj) => {
            let key_values = get_key_values_from_object(&obj);

            for key_value in key_values.iter() {
                let key = get_key_str(key_value);

                let value = key_value.value.clone();

                let result = mapper(FlatCompiledStylesValue::Tuple(key.clone(), value));

                variables_map.insert(key, result);
            }
        }

        ObjMapType::Map(map) => {
            for (key, value) in map {
                // Created hashed variable names with fileName//themeName//key
                let result = mapper(value);

                variables_map.insert(key, result);
            }
        }
    }

    dbg!(&variables_map);
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
    let mut ret_val = Vec::new();
    let object = obj.as_object().expect("Object expected");

    let key_values = get_key_values_from_object(object);

    for key_value in key_values.iter() {
        ret_val.push(key_value.clone());
    }

    ret_val
}

// fn obj_from_entries<K: Eq + std::hash::Hash + Clone, V: Clone>(entries: &Vec<(K, V)>) -> HashMap<K, V> {
pub(crate) fn obj_from_entries(entries: &Vec<OrderPair>) -> IndexMap<String, String> {
    let mut map = IndexMap::new();

    for OrderPair(key, value) in entries {
        map.insert(key.clone(), value.clone().expect("Value is not a string"));
    }

    map
}

pub(crate) fn obj_map_keys(
    entries: &IndexMap<String, String>,
    mapper: fn(&str) -> String,
) -> IndexMap<String, FlatCompiledStylesValue> {
    let mut map = IndexMap::new();

    for (key, value) in entries {
        let key = mapper(key);
        map.insert(
            key.clone(),
            FlatCompiledStylesValue::KeyValue(Pair {
                key,
                value: value.clone(),
            }),
        );
    }

    map
}

pub(crate) fn obj_map_entries(
    entries: &IndexMap<String, String>,
    mapper: fn((&str, &str)) -> Pair,
) -> IndexMap<String, FlatCompiledStylesValue> {
    let mut map = IndexMap::new();

    for (key, value) in entries {
        let result = mapper((key, value));
        map.insert(key.clone(), FlatCompiledStylesValue::KeyValue(result));
    }

    map
}
