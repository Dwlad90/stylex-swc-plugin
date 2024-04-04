use indexmap::IndexMap;

use crate::shared::{
    enums::{FlatCompiledStylesValue, ObjMapType},
    structures::{
        evaluate_result::EvaluateResultValue, flat_compiled_styles::FlatCompiledStyles,
        injectable_style::InjectableStyle, state_manager::StateManager,
    },
    utils::{
        common::create_hash, object::obj_map,
        stylex::define_vars_utils::construct_css_variables_string,
    },
};

pub(crate) fn stylex_define_vars(
    variables: &EvaluateResultValue,
    state: &mut StateManager,
) -> (
    IndexMap<String, FlatCompiledStylesValue>,
    IndexMap<String, InjectableStyle>,
) {
    dbg!(&variables);

    let theme_name_hash = format!(
        "{}{}",
        state.options.class_name_prefix,
        create_hash(&state.theme_name.clone().unwrap().as_str())
    );

    dbg!(theme_name_hash.clone());

    let typed_variables: IndexMap<String, FlatCompiledStylesValue> = IndexMap::new();

    let Some(variables) = variables.as_expr().and_then(|expr| expr.as_object()) else {
        panic!("Values must be an object")
    };

    let variables_map = obj_map(
        ObjMapType::Object(variables.clone()),
        |item| -> FlatCompiledStylesValue {
            match item {
                FlatCompiledStylesValue::String(_) => panic!("String is not supported"),
                FlatCompiledStylesValue::InjectableStyle(_) => {
                    panic!("InjectableStyle is not supported")
                }
                FlatCompiledStylesValue::Tuple(key, value) => {
                    // Created hashed variable names with fileName//themeName//key
                    let name_hash = format!(
                        "{}{}",
                        state.options.class_name_prefix,
                        create_hash(
                            &format!("{}.{}", state.theme_name.clone().unwrap(), key).as_str()
                        )
                    );
                    FlatCompiledStylesValue::Tuple(name_hash.clone(), value.clone())
                }
                FlatCompiledStylesValue::Null => todo!("Null"),
                FlatCompiledStylesValue::IncludedStyle(_) => todo!("IncludedStyle"),
                FlatCompiledStylesValue::Bool(_) => todo!("Bool"),
                FlatCompiledStylesValue::KeyValue(_) => todo!("KeyValue"),
            }
        },
    );

    let theme_variables_objects =
        obj_map(ObjMapType::Map(variables_map.clone()), |item| match item {
            FlatCompiledStylesValue::String(_) => panic!("String is not supported"),
            FlatCompiledStylesValue::InjectableStyle(_) => {
                panic!("InjectableStyle is not supported")
            }
            FlatCompiledStylesValue::Tuple(key, _) => {
                FlatCompiledStylesValue::String(format!("var(--{})", key))
            }
            FlatCompiledStylesValue::Null => todo!("Null"),
            FlatCompiledStylesValue::IncludedStyle(_) => todo!("IncludedStyle"),
            FlatCompiledStylesValue::Bool(_) => todo!("Bool"),
            FlatCompiledStylesValue::KeyValue(_) => todo!("KeyValue"),
        });

    dbg!(&variables_map, &theme_variables_objects,);

    let injectable_styles = construct_css_variables_string(&variables_map, &theme_name_hash);

    dbg!(&injectable_styles);

    let injectable_types = obj_map(ObjMapType::Map(typed_variables), |_| {
        todo!("Implement typed_variables mapper");
    });

    let injectable_types: IndexMap<String, InjectableStyle> = injectable_types
        .iter()
        .filter_map(|(key, value)| {
            if let Some(inj_style) = value.as_injectable_style() {
                return Some((key.clone(), inj_style.clone()));
            }

            Option::None
        })
        .collect();

    let mut theme_variables_objects = theme_variables_objects.clone();

    theme_variables_objects.insert(
        "__themeName__".to_string(),
        FlatCompiledStylesValue::String(theme_name_hash),
    );

    let mut injectable_types = injectable_types.clone();

    injectable_types.extend(injectable_styles);

    (theme_variables_objects, injectable_types)
}
