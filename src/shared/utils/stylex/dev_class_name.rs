use std::path::Path;

use indexmap::IndexMap;

use crate::shared::{
    constants::constants::COMPILED_KEY,
    regex::SANITIZE_CLASS_NAME_REGEX,
    structures::{flat_compiled_styles::FlatCompiledStylesValue, state_manager::StateManager},
};

pub(crate) fn inject_dev_class_names(
    obj: &IndexMap<String, IndexMap<String, FlatCompiledStylesValue>>,
    var_name: &Option<String>,
    state: &StateManager,
) -> IndexMap<String, IndexMap<String, FlatCompiledStylesValue>> {
    let mut result: IndexMap<String, IndexMap<String, FlatCompiledStylesValue>> = IndexMap::new();

    for (key, value) in obj.iter() {
        let dev_class_name = namespace_to_dev_class_name(&key, &var_name, state.get_filename());

        let mut dev_class = IndexMap::new();

        dev_class.insert(
            dev_class_name.clone(),
            FlatCompiledStylesValue::String(dev_class_name),
        );

        dev_class.extend(value.clone());

        result.insert(key.clone(), dev_class);
    }

    result
}

pub(crate) fn convert_to_test_styles(
    obj: &IndexMap<String, IndexMap<String, FlatCompiledStylesValue>>,
    var_name: &Option<String>,
    state: &StateManager,
) -> IndexMap<String, IndexMap<String, FlatCompiledStylesValue>> {
    let mut result: IndexMap<String, IndexMap<String, FlatCompiledStylesValue>> = IndexMap::new();

    for (key, _value) in obj.iter() {
        let dev_class_name = namespace_to_dev_class_name(&key, &var_name, state.get_filename());

        let mut dev_class = IndexMap::new();

        dev_class.insert(
            dev_class_name.clone(),
            FlatCompiledStylesValue::String(dev_class_name),
        );

        dev_class.insert(
            COMPILED_KEY.to_string(),
            FlatCompiledStylesValue::Bool(true),
        );

        result.insert(key.clone(), dev_class);
    }

    result
}

fn namespace_to_dev_class_name(
    namespace: &str,
    var_name: &Option<String>,
    filename: String,
) -> String {
    // Get the basename of the file without the extension
    let basename = Path::new(filename.as_str())
        .file_stem()
        .and_then(|os_str| os_str.to_str())
        .unwrap_or("");

    // Build up the class name, and sanitize it of disallowed characters
    let class_name = format!(
        "{}__{}{}",
        basename,
        var_name
            .clone()
            .and_then(|var_name| Some(format!("{}.", var_name)))
            .unwrap_or("".to_string()),
        namespace
    );
    let sanitized_class_name = SANITIZE_CLASS_NAME_REGEX
        .replace_all(&class_name, "$1 $2")
        .to_string();

    sanitized_class_name
}
