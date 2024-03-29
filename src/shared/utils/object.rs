use indexmap::IndexMap;

use crate::shared::{
    enums::{FlatCompiledStylesValue, ObjMapType},
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
