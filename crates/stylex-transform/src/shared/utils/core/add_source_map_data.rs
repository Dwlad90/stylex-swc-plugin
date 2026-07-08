use indexmap::IndexMap;
use log::{debug, info, warn};
use rustc_hash::FxHashMap;
use std::{env, path::Path, rc::Rc, sync::LazyLock};
use stylex_macros::stylex_panic;
use stylex_path_resolver::package_json::PackageJsonExtended;

use swc_core::{
  common::{DUMMY_SP, Spanned},
  ecma::ast::{CallExpr, Expr, KeyValueProp},
};

use crate::shared::{
  enums::data_structures::flat_compiled_styles_value::FlatCompiledStylesValue,
  structures::{functions::FunctionMap, state_manager::StateManager, types::StylesObjectMap},
  utils::{
    ast::convertors::{convert_expr_to_str, create_string_expr},
    js::evaluate::evaluate_obj_key,
    log::build_code_frame_error::{get_key_span_from_source_code, get_span_from_source_code},
  },
};
use stylex_ast::ast::convertors::get_key_values_from_object;
use stylex_constants::constants::{
  common::COMPILED_KEY,
  messages::{EXPECTED_OBJECT_EXPRESSION, INVALID_UTF8, illegal_argument_length},
};
use stylex_structures::stylex_options::CheckModuleResolution;

static NEXTJS_HYDRATION_WARNING: LazyLock<String> = LazyLock::new(|| {
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
  functions: &FunctionMap,
) -> StylesObjectMap {
  let mut result: StylesObjectMap = IndexMap::new();
  let mut style_node_paths: FxHashMap<String, KeyValueProp> = FxHashMap::default();

  match call_expr.args.first() {
    Some(arg) => match arg.expr.as_ref() {
      Expr::Object(object) => {
        let key_values = get_key_values_from_object(object);

        for key_value in key_values {
          let key_string = evaluate_obj_key(&key_value, state, functions)
            .value
            .as_ref()
            .and_then(|value| value.as_expr())
            .and_then(|expr| convert_expr_to_str(expr, state, functions));

          if let Some(key_string) = key_string {
            style_node_paths.entry(key_string).or_insert(key_value);
          }
        }
      },
      _ => stylex_panic!("{}", EXPECTED_OBJECT_EXPRESSION),
    },
    _ => {
      stylex_panic!("{}", illegal_argument_length("add_source_map_data", 1));
    },
  };

  // The value-matching fallback wraps the same call for every namespace key,
  // so build it once instead of deep-cloning the call per key.
  let wrapped_call_expr = Expr::Call(call_expr.clone());

  for (key, value) in obj {
    let mut inner_map = IndexMap::new();

    inner_map.extend((**value).clone());

    match style_node_paths.remove(key) {
      Some(style_node_path) => {
        // Highest fidelity: resolve the key's own span against the compiler's
        // input and map it through the host-provided input source map back to
        // the original authored file. Exact even when earlier tooling (e.g.
        // macro loaders) already rewrote the code.
        if let Some(original_position) =
          original_position_from_input_source_map(&style_node_path, state)
        {
          insert_compiled_entry(
            &mut inner_map,
            &original_position.filename,
            original_position.line_number,
            state,
            package_json_seen,
            functions,
          );
          result.insert(key.clone(), Rc::new(inner_map));
          continue;
        }

        // Locate the namespace by its key next: keys are static strings that
        // survive value-level code transforms (e.g. macro expansion by an
        // earlier loader), so this finds the original source position even
        // when the compiled values no longer match the file content. Fall
        // back to matching the value expression when the key cannot be
        // located (e.g. computed keys).
        let source_code_frame_and_span = match get_key_span_from_source_code(call_expr, key, state)
        {
          Ok((code_frame, span)) if !span.eq(&DUMMY_SP) => Ok((code_frame, span)),
          _ => get_span_from_source_code(&wrapped_call_expr, &style_node_path.value, state),
        };

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
              // Panic-safe lookup: `None` leaves the map untouched and the
              // `contains_key` fallback below inserts the plain `true` marker.
              if let Some(original_line_number) = code_frame.try_get_span_line_number(span) {
                let filename = state.get_filename().to_string();
                insert_compiled_entry(
                  &mut inner_map,
                  &filename,
                  original_line_number,
                  state,
                  package_json_seen,
                  functions,
                );
              }
            }
          },
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
          },
        }

        if !inner_map.contains_key(&COMPILED_KEY.to_owned()) {
          inner_map.insert(
            COMPILED_KEY.to_owned(),
            Rc::new(FlatCompiledStylesValue::Bool(true)),
          );
        }

        result.insert(key.clone(), Rc::new(inner_map));
      },
      _ => {
        // Fallback in case no sourcemap data is found
        result.insert(key.clone(), Rc::new(inner_map));
      },
    };
  }

  result
}

/// Inserts the `$$css` entry with a `file:line` annotation, or `true` when a
/// usable short filename cannot be produced.
fn insert_compiled_entry(
  inner_map: &mut IndexMap<String, Rc<FlatCompiledStylesValue>>,
  filename: &str,
  original_line_number: usize,
  state: &mut StateManager,
  package_json_seen: &mut FxHashMap<String, PackageJsonExtended>,
  functions: &FunctionMap,
) {
  let raw_short_filename = create_short_filename(filename, state, package_json_seen);
  let short_filename_expr = if let Some(ref f) = state.options.debug_file_path {
    f.call(vec![create_string_expr(&raw_short_filename)])
  } else {
    create_string_expr(&raw_short_filename)
  };

  let short_filename = convert_expr_to_str(&short_filename_expr, state, functions);

  if original_line_number > 0
    && let Some(short_filename) = short_filename
    && !short_filename.is_empty()
  {
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

struct OriginalSourcePosition {
  filename: String,
  line_number: usize,
}

/// Resolves a style namespace to its source position in the original authored
/// file by combining the namespace key's own span — exact in the compiler's
/// input — with the host-provided input source map, which maps the input back
/// to the original file when earlier tooling already transformed it.
///
/// Returns `None` when either piece is unavailable so callers can fall back
/// to locating the namespace in the source text.
fn original_position_from_input_source_map(
  style_node_path: &KeyValueProp,
  state: &StateManager,
) -> Option<OriginalSourcePosition> {
  let source_file = state.input_source_file.as_ref()?;
  let input_map = state.input_source_map.as_ref()?;

  let span = style_node_path.key.span();
  if span.is_dummy() {
    return None;
  }

  let pos = span.lo();
  if pos < source_file.start_pos || pos >= source_file.end_pos {
    return None;
  }

  let line = source_file.lookup_line(pos)?;
  let line_begin = source_file.line_begin_pos(pos);

  // Source map columns are counted in UTF-16 code units.
  let line_start_offset = (line_begin - source_file.start_pos).0 as usize;
  let pos_offset = (pos - source_file.start_pos).0 as usize;
  let col = source_file
    .src
    .get(line_start_offset..pos_offset)?
    .encode_utf16()
    .count();

  let token = input_map.lookup_token(line as u32, col as u32)?;

  // `lookup_token` returns the nearest preceding token, which a sparse map
  // (e.g. statement-level mappings only) can place on an earlier line. Only a
  // same-line token is trustworthy for a `file:line` annotation; otherwise
  // fall back to locating the key in the source text.
  if token.get_dst_line() != line as u32 {
    return None;
  }

  let filename = match token.get_source() {
    // Scheme-qualified sources (e.g. `webpack://app/src/x.ts`) are not
    // filesystem paths; `create_short_filename` would garble them — fall
    // back to locating the key in the source text instead.
    Some(source) if source.contains("://") => return None,
    Some(source) => source.to_string(),
    None => state.get_filename().to_string(),
  };

  Some(OriginalSourcePosition {
    filename,
    line_number: token.get_src_line() as usize + 1,
  })
}

#[cfg(test)]
#[path = "tests/add_source_map_data_tests.rs"]
mod tests;

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
  if let CheckModuleResolution::CommonJs {
    root_dir: Some(root_dir),
    ..
  } = &state.options.unstable_module_resolution
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
    CheckModuleResolution::Haste { .. }
  );

  let path = Path::new(absolute_path);
  let cwd = env::current_dir().unwrap_or_default();

  let cwd_str = match cwd.to_str() {
    Some(s) => s,
    None => stylex_panic!("{}", INVALID_UTF8),
  };
  let cwd_package = StateManager::get_package_name_and_path(cwd_str, package_json_seen);
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

    // Otherwise, return package:relativePath (or just relativePath if no package
    // name)
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
