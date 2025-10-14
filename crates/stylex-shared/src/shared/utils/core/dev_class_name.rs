use std::{path::Path, rc::Rc};

use indexmap::IndexMap;

use crate::shared::{
  constants::common::COMPILED_KEY,
  enums::data_structures::flat_compiled_styles_value::FlatCompiledStylesValue,
  regex::SANITIZE_CLASS_NAME_REGEX,
  structures::{
    state_manager::StateManager,
    types::{FlatCompiledStyles, StylesObjectMap},
  },
};

pub(crate) fn inject_dev_class_names(
  obj: &StylesObjectMap,
  var_name: &Option<String>,
  state: &StateManager,
) -> StylesObjectMap {
  let mut result: StylesObjectMap = IndexMap::new();

  for (key, value) in obj.iter() {
    let dev_class_name =
      namespace_to_dev_class_name(key, var_name, state.get_short_filename().as_str());

    let mut dev_class = IndexMap::new();

    dev_class.insert(
      dev_class_name.clone(),
      Rc::new(FlatCompiledStylesValue::String(dev_class_name)),
    );

    dev_class.extend((**value).clone());

    result.insert(key.clone(), Rc::new(dev_class));
  }

  result
}

pub(crate) fn convert_to_test_styles(
  obj: &StylesObjectMap,
  var_name: &Option<String>,
  state: &StateManager,
) -> StylesObjectMap {
  let mut result: StylesObjectMap = IndexMap::new();

  for (key, _value) in obj.iter() {
    let dev_class_name =
      namespace_to_dev_class_name(key, var_name, state.get_short_filename().as_str());

    let mut dev_class = IndexMap::new();

    dev_class.insert(
      dev_class_name.clone(),
      Rc::new(FlatCompiledStylesValue::String(dev_class_name)),
    );

    dev_class.insert(
      COMPILED_KEY.to_string(),
      Rc::new(FlatCompiledStylesValue::Bool(true)),
    );

    result.insert(key.clone(), Rc::new(dev_class));
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
    .unwrap_or_default();

  // Build up the class name, and sanitize it of disallowed characters
  let class_name = format!(
    "{}__{}{}",
    basename,
    var_name
      .as_ref()
      .map(|var_name| format!("{}.", var_name))
      .unwrap_or_default(),
    namespace
  );

  SANITIZE_CLASS_NAME_REGEX
    .replace_all(&class_name, "$1 $2")
    .to_string()
}

fn convert_theme_to_base_styles(
  variable_name: &str,
  filename: &str,
) -> FlatCompiledStyles {
  let mut overrides_obj_extended = IndexMap::new();

  // Get the basename of the file without the extension
  let basename = Path::new(filename)
    .file_stem()
    .and_then(|os_str| os_str.to_str())
    .unwrap_or_default()
    .split('.')
    .next()
    .expect("basename is empty");

  // Build up the class name, and sanitize it of disallowed characters
  let dev_class_name = format!("{}__{}", basename, variable_name);

  overrides_obj_extended.insert(
    dev_class_name.clone(),
    Rc::new(FlatCompiledStylesValue::String(dev_class_name)),
  );

  overrides_obj_extended
}

pub(crate) fn convert_theme_to_dev_styles(
  variable_name: &Option<String>,
  overrides_obj: &FlatCompiledStyles,
  filename: &str,
) -> FlatCompiledStyles {
  let variable_name_str = variable_name
    .as_ref()
    .expect("Variable name not found.")
    .as_str();

  let mut overrides_obj_extended = convert_theme_to_base_styles(variable_name_str, filename);

  overrides_obj_extended.extend(overrides_obj.clone());

  overrides_obj_extended
}

pub(crate) fn convert_theme_to_test_styles(
  variable_name: &Option<String>,
  overrides_obj: &FlatCompiledStyles,
  filename: &str,
) -> FlatCompiledStyles {
  let mut overrides_obj_extended =
    convert_theme_to_dev_styles(variable_name, overrides_obj, filename);

  overrides_obj_extended.insert(
    COMPILED_KEY.to_string(),
    Rc::new(FlatCompiledStylesValue::Bool(true)),
  );

  overrides_obj_extended
}
