use indexmap::IndexMap;
use std::{env, path::Path, rc::Rc};
use swc_core::ecma::ast::{CallExpr, Expr, KeyValueProp};

use crate::shared::{
  constants::{common::COMPILED_KEY, messages::ILLEGAL_ARGUMENT_LENGTH},
  enums::data_structures::flat_compiled_styles_value::FlatCompiledStylesValue,
  structures::{
    state_manager::StateManager, stylex_options::CheckModuleResolution, types::StylesObjectMap,
  },
  utils::{
    ast::convertors::key_value_to_str, common::get_key_values_from_object,
    log::build_code_frame_error::get_span_from_source_code,
  },
};

pub(crate) fn add_source_map_data(
  obj: &StylesObjectMap,
  call_expr: &CallExpr,
  state: &StateManager,
) -> StylesObjectMap {
  let mut result: StylesObjectMap = IndexMap::new();

  let current_filename = state.get_filename();

  for (key, value) in obj {
    let mut style_node_path: Option<KeyValueProp> = None;

    match call_expr.args.first() {
      Some(arg) => {
        match arg.expr.as_ref() {
          Expr::Object(object) => {
            let key_values = get_key_values_from_object(object);

            for key_value in key_values {
              let key_string = key_value_to_str(&key_value);

              if key == &key_string {
                style_node_path = Some(key_value.clone());

                break;
              }
            }
          }
          _ => panic!("Expected object expression"),
        };
        let mut inner_map = IndexMap::new();

        match style_node_path {
          Some(style_node_path) => {
            let (code_frame, span) = get_span_from_source_code(
              &Expr::Call(call_expr.clone()),
              &style_node_path.value,
              state,
            );

            let original_line_number = code_frame.get_span_line_number(span);

            let short_filename = create_short_filename(current_filename, state);

            let css_value = if !short_filename.is_empty() && original_line_number > 0 {
              FlatCompiledStylesValue::String(format!(
                "{}:{}",
                short_filename, original_line_number
              ))
            } else {
              FlatCompiledStylesValue::Bool(true)
            };

            inner_map.extend((**value).clone());

            inner_map.insert(COMPILED_KEY.to_string(), Rc::new(css_value));

            result.insert(key.clone(), Rc::new(inner_map));
          }
          _ => {
            // fallback in case no sourcemap data is found

            inner_map.extend((**value).clone());

            result.insert(key.clone(), Rc::new(inner_map));
          }
        };
      }
      _ => {
        panic!("{}", ILLEGAL_ARGUMENT_LENGTH)
      }
    };
  }

  result
}

fn get_package_prefix(absolute_path: &str) -> Option<String> {
  const NODE_MODULES: &str = "node_modules";

  let node_modules_index = absolute_path.find(NODE_MODULES)?;
  let start_pos = node_modules_index + NODE_MODULES.len() + 1;

  if start_pos >= absolute_path.len() {
    return None;
  }

  absolute_path[start_pos..]
    .split(std::path::MAIN_SEPARATOR)
    .next()
    .map(String::from)
}

fn get_short_path(relative_path: &str) -> String {
  let path_parts: Vec<&str> = relative_path.split(std::path::MAIN_SEPARATOR).collect();

  let path_segments = if path_parts.len() >= 2 {
    &path_parts[path_parts.len() - 2..]
  } else {
    &path_parts[..]
  };

  path_segments.join("/")
}

fn create_short_filename(absolute_path: &str, state: &StateManager) -> String {
  let is_haste = matches!(
    state.options.unstable_module_resolution,
    CheckModuleResolution::Haste(_)
  );

  let path = Path::new(absolute_path);

  let cwd = env::current_dir().unwrap_or_default();
  let relative_path = path
    .strip_prefix(&cwd)
    .map_or(absolute_path.to_string(), |p| {
      p.to_string_lossy().into_owned()
    });

  if let Some(package_prefix) = get_package_prefix(absolute_path) {
    let short_path = get_short_path(&relative_path);
    return format!("{}:{}", package_prefix, short_path);
  }

  if is_haste {
    return path
      .file_name()
      .map_or_else(String::new, |f| f.to_string_lossy().into_owned());
  }

  get_short_path(&relative_path)
}
