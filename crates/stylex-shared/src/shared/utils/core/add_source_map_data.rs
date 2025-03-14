use indexmap::IndexMap;
use log::{debug, warn};
use std::{env, path::Path, rc::Rc};
use swc_core::{
  common::DUMMY_SP,
  ecma::ast::{CallExpr, Expr, KeyValueProp},
};

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
  state: &mut StateManager,
) -> StylesObjectMap {
  let mut result: StylesObjectMap = IndexMap::new();

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

        inner_map.extend((**value).clone());

        let compiled_key = COMPILED_KEY.to_string();

        match style_node_path {
          Some(style_node_path) => {
            let source_code_frame_and_span = get_span_from_source_code(
              &Expr::Call(call_expr.clone()),
              &style_node_path.value,
              state,
            );

            match source_code_frame_and_span {
              Ok((code_frame, span)) => {
                if span.eq(&DUMMY_SP) {
                  if log::log_enabled!(log::Level::Debug) {
                    debug!(
                      "Could not find span for style node path. File: {}, Style node path: {:?}",
                      state.get_filename(),
                      style_node_path
                    );
                  } else {
                    warn!(
                      "Could not find span for style node path. File: {}. For more information enable debug logging.",
                      state.get_filename()
                    );
                  };
                } else {
                  let original_line_number = code_frame.get_span_line_number(span);
                  let filename = state.get_filename().to_string();
                  let short_filename = create_short_filename(filename.as_ref(), state);

                  if !short_filename.is_empty() && original_line_number > 0 {
                    let source_map = format!("{}:{}", short_filename, original_line_number);
                    inner_map.insert(
                      compiled_key.clone(),
                      Rc::new(FlatCompiledStylesValue::String(source_map)),
                    );
                  } else {
                    inner_map.insert(
                      compiled_key.clone(),
                      Rc::new(FlatCompiledStylesValue::Bool(true)),
                    );
                  }
                }
              }
              Err(e) => {
                warn!("Could not retrieve source code frame: {}", e);
                if log::log_enabled!(log::Level::Debug) {
                  debug!(
                    "Could not retrieve source code frame: {}. File: {}. Style node path: {:?}",
                    e,
                    state.get_filename(),
                    style_node_path
                  );
                } else {
                  warn!(
                    "Could not retrieve source code frame: {}. File: {}. For more information enable debug logging.",
                    e,
                    state.get_filename()
                  );
                };
              }
            }

            if !inner_map.contains_key(&compiled_key) {
              inner_map.insert(compiled_key, Rc::new(FlatCompiledStylesValue::Bool(true)));
            }

            result.insert(key.clone(), Rc::new(inner_map));
          }
          _ => {
            // Fallback in case no sourcemap data is found

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
