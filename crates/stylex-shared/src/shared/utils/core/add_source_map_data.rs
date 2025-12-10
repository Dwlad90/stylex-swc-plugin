use indexmap::IndexMap;
use log::{debug, info, warn};
use once_cell::sync::Lazy;
use rustc_hash::FxHashMap;
use std::{env, path::Path, rc::Rc};
use stylex_path_resolver::package_json::PackageJsonExtended;
use swc_core::{
  common::DUMMY_SP,
  ecma::ast::{CallExpr, Expr, KeyValueProp},
};

use crate::shared::{
  constants::{common::COMPILED_KEY, messages::illegal_argument_length},
  enums::data_structures::flat_compiled_styles_value::FlatCompiledStylesValue,
  structures::{
    state_manager::StateManager, stylex_options::CheckModuleResolution, types::StylesObjectMap,
  },
  utils::{
    ast::convertors::key_value_to_str, common::get_key_values_from_object,
    log::build_code_frame_error::get_span_from_source_code,
  },
};

static NEXTJS_HYDRATION_WARNING: Lazy<String> = Lazy::new(|| {
  "\n\nNote: If you are using Next.js, you may encounter hydration mismatches between server and client.\n\
    This occurs because Next.js uses precompiled and transformed source code on the client side,\n\
    which can lead to different AST generation and cause the following error:\n\
    'A tree hydrated but some attributes of the server rendered HTML didn't match the client properties'\n\
    Please verify the expression that caused this error.".to_string()
});

pub(crate) fn add_source_map_data(
  obj: &StylesObjectMap,
  call_expr: &CallExpr,
  state: &mut StateManager,
  package_json_seen: &mut FxHashMap<String, PackageJsonExtended>,
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
                      "Could not find span for style node path. File: {}, Style node path: {:?}.{}",
                      state.get_filename(),
                      style_node_path,
                      &*NEXTJS_HYDRATION_WARNING
                    );
                  } else {
                    info!(
                      "Could not find span for style node path. File: {}. For more information enable debug logging.{}",
                      state.get_filename(),
                      &*NEXTJS_HYDRATION_WARNING
                    );
                  };
                } else {
                  let original_line_number = code_frame.get_span_line_number(span);
                  let filename = state.get_filename().to_string();
                  let short_filename =
                    create_short_filename(filename.as_ref(), state, package_json_seen);

                  if !short_filename.is_empty() && original_line_number > 0 {
                    let source_map = format!("{}:{}", short_filename, original_line_number);
                    inner_map.insert(
                      COMPILED_KEY.to_owned(),
                      Rc::new(FlatCompiledStylesValue::String(source_map)),
                    );
                  } else {
                    inner_map.insert(
                      COMPILED_KEY.to_owned(),
                      Rc::new(FlatCompiledStylesValue::Bool(true)),
                    );
                  }
                }
              }
              Err(e) => {
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

            if !inner_map.contains_key(&COMPILED_KEY.to_owned()) {
              inner_map.insert(
                COMPILED_KEY.to_owned(),
                Rc::new(FlatCompiledStylesValue::Bool(true)),
              );
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
        panic!("{}", illegal_argument_length("add_source_map_data", 1));
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

fn get_short_path(relative_path: &str, state: &StateManager) -> String {
  // Check if commonJS module resolution with rootDir is configured
  if let CheckModuleResolution::CommonJS(ref config) = state.options.unstable_module_resolution
    && let Some(ref root_dir) = config.root_dir
  {
    let relative_path_obj = Path::new(relative_path);
    let root_dir_path = Path::new(root_dir);

    if let Ok(rel) = relative_path_obj.strip_prefix(root_dir_path) {
      return rel.to_string_lossy().into_owned();
    }
  }

  // Normalize slashes in the path and truncate to last 2 segments
  let path_parts: Vec<&str> = relative_path.split(std::path::MAIN_SEPARATOR).collect();

  let path_segments = if path_parts.len() >= 2 {
    &path_parts[path_parts.len() - 2..]
  } else {
    &path_parts[..]
  };

  path_segments.join("/")
}

fn create_short_filename(
  absolute_path: &str,
  state: &StateManager,
  package_json_seen: &mut FxHashMap<String, PackageJsonExtended>,
) -> String {
  let is_haste = matches!(
    state.options.unstable_module_resolution,
    CheckModuleResolution::Haste(_)
  );

  let path = Path::new(absolute_path);
  let cwd = env::current_dir().unwrap_or_default();

  let cwd_package =
    StateManager::get_package_name_and_path(cwd.to_str().unwrap(), package_json_seen);
  let package_details = StateManager::get_package_name_and_path(absolute_path, package_json_seen);

  // If package details exist, use package-relative path
  if let Some((package_name_opt, package_root_path)) = package_details {
    let package_root = Path::new(&package_root_path);
    let relative_path = path
      .strip_prefix(package_root)
      .map_or(absolute_path.to_string(), |p| {
        p.to_string_lossy().into_owned()
      });

    // If the file is in the same package as cwd, return just the relative path
    if let Some((cwd_package_name, _)) = cwd_package
      && cwd_package_name == package_name_opt
    {
      return relative_path;
    }

    // Otherwise, return package:relativePath (or just relativePath if no package name)
    if let Some(package_name) = package_name_opt {
      return format!("{}:{}", package_name, relative_path);
    } else {
      return relative_path;
    }
  }

  // Fallback: construct a path based on package prefix, module type, and file
  if let Some(package_prefix) = get_package_prefix(absolute_path) {
    let short_path = get_short_path(absolute_path, state);

    return format!("{}:{}", package_prefix, short_path);
  }

  // If haste mode, return just the filename
  if is_haste {
    return path
      .file_name()
      .map_or_else(String::new, |f| f.to_string_lossy().into_owned());
  }

  // Otherwise, return short path relative to cwd
  let relative_path = path
    .strip_prefix(&cwd)
    .map_or(absolute_path.to_string(), |p| {
      p.to_string_lossy().into_owned()
    });

  get_short_path(&relative_path, state)
}
