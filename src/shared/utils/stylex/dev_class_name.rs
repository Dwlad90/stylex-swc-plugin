use std::path::Path;

use indexmap::IndexMap;

use crate::shared::{
  constants::common::COMPILED_KEY, enums::FlatCompiledStylesValue,
  regex::SANITIZE_CLASS_NAME_REGEX, structures::state_manager::StateManager,
};

pub(crate) fn inject_dev_class_names(
  obj: &IndexMap<String, IndexMap<String, FlatCompiledStylesValue>>,
  var_name: &Option<String>,
  state: &StateManager,
) -> IndexMap<String, IndexMap<String, FlatCompiledStylesValue>> {
  let mut result: IndexMap<String, IndexMap<String, FlatCompiledStylesValue>> = IndexMap::new();

  for (key, value) in obj.iter() {
    let dev_class_name =
      namespace_to_dev_class_name(key, var_name, state.get_short_filename().as_str());

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
    let dev_class_name =
      namespace_to_dev_class_name(key, var_name, state.get_short_filename().as_str());

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
  filename: &str,
) -> String {
  // Get the basename of the file without the extension
  let basename = Path::new(filename)
    .file_stem()
    .and_then(|os_str| os_str.to_str())
    .unwrap_or("");

  // Build up the class name, and sanitize it of disallowed characters
  let class_name = format!(
    "{}__{}{}",
    basename,
    var_name
      .clone()
      .map(|var_name| format!("{}.", var_name))
      .unwrap_or("".to_string()),
    namespace
  );
  let sanitized_class_name = SANITIZE_CLASS_NAME_REGEX
    .replace_all(&class_name, "$1 $2")
    .to_string();

  sanitized_class_name
}

fn convert_theme_to_base_styles(
  variable_name: &str,
  filename: &str,
) -> IndexMap<String, FlatCompiledStylesValue> {
  let mut overrides_obj_extended = IndexMap::new();

  // Get the basename of the file without the extension
  let basename = Path::new(filename)
    .file_stem()
    .and_then(|os_str| os_str.to_str())
    .unwrap_or("")
    .split('.')
    .next()
    .expect("basename is empty");

  // Build up the class name, and sanitize it of disallowed characters
  let dev_class_name = format!("{}__{}", basename, variable_name);

  overrides_obj_extended.insert(
    dev_class_name.clone(),
    FlatCompiledStylesValue::String(dev_class_name),
  );

  overrides_obj_extended
}

pub(crate) fn convert_theme_to_dev_styles(
  variable_name: &Option<String>,
  overrides_obj: &IndexMap<String, FlatCompiledStylesValue>,
  filename: &str,
) -> IndexMap<String, FlatCompiledStylesValue> {
  let mut overrides_obj_extended = convert_theme_to_base_styles(
    variable_name
      .clone()
      .expect("Variable name not found.")
      .as_str(),
    filename,
  );

  overrides_obj_extended.extend(overrides_obj.clone());

  overrides_obj_extended
}

pub(crate) fn convert_theme_to_test_styles(
  variable_name: &Option<String>,
  overrides_obj: &IndexMap<String, FlatCompiledStylesValue>,
  filename: &str,
) -> IndexMap<String, FlatCompiledStylesValue> {
  let mut overrides_obj_extended = convert_theme_to_base_styles(
    variable_name
      .clone()
      .expect("Variable name not found.")
      .as_str(),
    filename,
  );
  overrides_obj_extended.extend(overrides_obj.clone());

  overrides_obj_extended.insert(
    COMPILED_KEY.to_string(),
    FlatCompiledStylesValue::Bool(true),
  );

  overrides_obj_extended
}
