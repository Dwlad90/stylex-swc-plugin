use core::panic;
use log::warn;
use once_cell::sync::Lazy;
use path_clean::PathClean;
use regex::Regex;
use rustc_hash::FxHashMap;
use std::path::{Path, PathBuf};
use swc_core::{
  common::FileName,
  ecma::loader::{resolve::Resolve, resolvers::node::NodeModulesResolver},
};

use std::fs;

use crate::{
  package_json::{
    find_nearest_package_json, get_package_json, get_package_json_with_deps,
    resolve_package_from_package_json,
  },
  utils::{contains_subpath, relative_path},
};

mod tests;

pub const EXTENSIONS: [&str; 8] = [".tsx", ".ts", ".jsx", ".js", ".mjs", ".cjs", ".mdx", ".md"];

pub static FILE_PATTERN: Lazy<Regex> =
  Lazy::new(|| Regex::new(r#"\.(jsx?|tsx?|mdx?|mjs|cjs)$"#).unwrap());

pub fn resolve_path(processing_file: &Path, root_dir: &Path) -> String {
  if !FILE_PATTERN.is_match(processing_file.to_str().unwrap()) {
    let processing_path = if !cfg!(feature = "wasm") || cfg!(test) {
      processing_file
        .strip_prefix(root_dir.parent().unwrap().parent().unwrap())
        .unwrap()
        .to_path_buf()
    } else {
      processing_file.to_path_buf()
    };

    panic!(
      r#"Resolve path must be a file, but got: {}"#,
      processing_path.display()
    );
  }

  let cwd = if !cfg!(feature = "wasm") || cfg!(test) {
    root_dir.to_path_buf()
  } else {
    "cwd".into()
  };

  let mut path_by_package_json = match resolve_from_package_json(processing_file, root_dir, &cwd) {
    Ok(value) => value,
    Err(value) => return value,
  };

  if path_by_package_json.starts_with(&cwd) {
    path_by_package_json = path_by_package_json
      .strip_prefix(cwd)
      .unwrap()
      .to_path_buf();
  }

  let resolved_path_by_package_name = path_by_package_json.clean().display().to_string();

  if cfg!(test) {
    let cwd_resolved_path = format!("{}/{}", root_dir.display(), resolved_path_by_package_name);

    assert!(
      fs::metadata(&cwd_resolved_path).is_ok(),
      "Path resolution failed: {}",
      resolved_path_by_package_name
    );
  }

  resolved_path_by_package_name
}

fn resolve_from_package_json(
  processing_file: &Path,
  root_dir: &Path,
  cwd: &Path,
) -> Result<PathBuf, String> {
  let resolved_path = match processing_file.strip_prefix(root_dir) {
    Ok(stripped) => stripped.to_path_buf(),
    Err(_) => {
      let processing_file_str = processing_file.to_string_lossy();

      if let Some(node_modules_index) = processing_file_str.rfind("node_modules") {
        // NOTE: This is a workaround for the case when the file is located in the node_modules directory and pnpm is package manager

        let resolved_path_from_node_modules = processing_file_str
          .split_at(node_modules_index)
          .1
          .to_string();

        if !resolved_path_from_node_modules.is_empty() {
          return Err(resolved_path_from_node_modules);
        }
      }

      let relative_package_path = relative_path(processing_file, root_dir);

      get_package_path_by_package_json(cwd, &relative_package_path)
    }
  };

  Ok(resolved_path)
}

fn get_package_path_by_package_json(cwd: &Path, relative_package_path: &Path) -> PathBuf {
  let (resolver, package_dependencies) = get_package_json_with_deps(cwd);

  let mut potential_package_path: String = Default::default();

  for (name, _) in package_dependencies.iter() {
    let file_name = FileName::Real(cwd.to_path_buf());

    let potential_path_section = name.split("/").last().unwrap_or_default();

    if contains_subpath(relative_package_path, Path::new(&potential_path_section)) {
      let relative_package_path_str = relative_package_path.display().to_string();

      let potential_file_path = relative_package_path_str
        .split(potential_path_section)
        .last()
        .unwrap_or_default();

      if !potential_file_path.is_empty()
        || relative_package_path_str.ends_with(format!("/{}", potential_path_section).as_str())
      {
        let resolved_node_modules_path = get_node_modules_path(&resolver, &file_name, name);

        if let Some(resolved_node_modules_path) = resolved_node_modules_path {
          if let FileName::Real(real_resolved_node_modules_path) =
            resolved_node_modules_path.filename
          {
            let (potential_package_json, _) =
              get_package_json(real_resolved_node_modules_path.as_path());

            match &potential_package_json.exports {
              Some(exports) => resolve_package_json_exports(
                potential_file_path,
                exports,
                &mut potential_package_path,
                &real_resolved_node_modules_path,
              ),
              None => {
                let node_modules_regex = Regex::new(r".*node_modules").unwrap();

                potential_package_path = node_modules_regex
                  .replace(
                    real_resolved_node_modules_path
                      .display()
                      .to_string()
                      .as_str(),
                    "node_modules",
                  )
                  .to_string();
              }
            }
          }
        }

        if potential_package_path.is_empty() {
          potential_package_path = format!("node_modules/{}{}", name, potential_file_path);
        }

        break;
      }
    }
  }

  PathBuf::from(potential_package_path)
}

pub(crate) fn get_node_modules_path(
  resolver: &NodeModulesResolver,
  file_name: &FileName,
  name: &str,
) -> Option<swc_core::ecma::loader::resolve::Resolution> {
  {
    match resolver.resolve(file_name, name) {
      Ok(resolution) => {
        if let FileName::Real(real_filename) = &resolution.filename {
          if real_filename.starts_with("node_modules") {
            return Some(resolution);
          }
        }
        None
      }
      Err(_) => None,
    }
  }
}

fn resolve_package_json_exports(
  potential_import_segment_path: &str,
  exports: &FxHashMap<String, String>,
  potential_package_path: &mut String,
  resolved_node_modules_path: &Path,
) {
  let import_segment_path_without_extension = PathBuf::from(potential_import_segment_path)
    .with_extension("")
    .display()
    .to_string();

  let mut exports_values: Vec<&String> = exports.values().collect();

  exports_values.sort_by_key(|k| -(k.len() as isize));

  let resolved_package_path =
    find_nearest_package_json(resolved_node_modules_path).unwrap_or_else(|| {
      panic!(
        "package.json not found near: {}",
        resolved_node_modules_path.display()
      )
    });

  for export_value in exports_values {
    if export_value.contains(&import_segment_path_without_extension) {
      *potential_package_path = resolved_package_path
        .join(export_value)
        .display()
        .to_string();

      break;
    }
  }

  if potential_package_path.is_empty() {
    let mut keys: Vec<&String> = exports.keys().collect();
    keys.sort_by_key(|k| -(k.len() as isize));

    for key in keys {
      if key.contains(&import_segment_path_without_extension) {
        *potential_package_path = resolved_package_path
          .join(exports.get(key).unwrap())
          .display()
          .to_string();

        break;
      }
    }
  }

  if potential_package_path.is_empty() {
    warn!("Unfortunatly, the exports field is not yet fully supported, so path resolving may work not as expected");
    // TODO: implement exports field resolution
  }
}

pub fn resolve_file_path(
  import_path_str: &str,
  source_file_path: &str,
  ext: &str,
  root_path: &str,
  aliases: &FxHashMap<String, Vec<String>>,
) -> std::io::Result<PathBuf> {
  let source_dir = Path::new(source_file_path).parent().unwrap();
  let root_path = Path::new(root_path);

  let cwd = if cfg!(feature = "wasm") {
    "cwd"
  } else {
    root_path.to_str().unwrap()
  };

  let cwd_path = Path::new(cwd);

  let resolved_file_paths: Vec<PathBuf> = if import_path_str.starts_with('.') {
    vec![resolve_path(&source_dir.join(import_path_str), root_path).into()]
  } else if import_path_str.starts_with('/') {
    vec![root_path.join(import_path_str)]
  } else {
    let mut aliased_file_paths = possible_aliased_paths(import_path_str, aliases)
      .iter()
      .filter_map(|path| path.strip_prefix(root_path).ok())
      .map(PathBuf::from)
      .collect::<Vec<PathBuf>>();

    if let Some((package_resolution, name)) =
      find_node_modules_resolution(cwd_path, import_path_str)
    {
      if let FileName::Real(resolved_node_modules_path_buf) = package_resolution.filename {
        let (package_json, _) = get_package_json(&resolved_node_modules_path_buf);

        let potential_import_path_segment = import_path_str.split(&name).last().unwrap_or_default();

        let import_path_segment = if potential_import_path_segment.is_empty() {
          name
        } else {
          potential_import_path_segment.to_string()
        };

        let mut potential_package_path = String::default();

        if let Some(exports) = &package_json.exports {
          resolve_package_json_exports(
            &import_path_segment,
            exports,
            &mut potential_package_path,
            &resolved_node_modules_path_buf,
          );
        }

        if !potential_package_path.is_empty() {
          aliased_file_paths.push(Path::new(&potential_package_path).to_path_buf().clean());
        }

        aliased_file_paths.push(resolved_node_modules_path_buf.clean());
      }
    }

    aliased_file_paths.push(Path::new("node_modules").join(import_path_str));
    aliased_file_paths
  };

  for resolved_file_path in resolved_file_paths.iter() {
    let mut resolved_file_path = resolved_file_path.clean();

    if let Some(extension) = resolved_file_path.extension() {
      let subpath = extension.to_string_lossy();
      if EXTENSIONS
        .iter()
        .all(|ext| !ext.ends_with(subpath.as_ref()))
      {
        resolved_file_path.set_extension(format!("{}{}", subpath, ext));
      }
    } else {
      resolved_file_path.set_extension(ext);
    }

    let cleaned_path = resolved_file_path
      .to_str()
      .unwrap()
      .replace("..", ".")
      .to_string();

    let path_to_check: PathBuf;
    let node_modules_path_to_check: PathBuf;

    if !cleaned_path.contains(cwd) {
      if !cleaned_path.starts_with("node_modules") {
        node_modules_path_to_check = cwd_path.join("node_modules").join(&cleaned_path);
      } else {
        node_modules_path_to_check = cwd_path.join(&cleaned_path);
      }
      path_to_check = cwd_path.join(cleaned_path);
    } else {
      path_to_check = PathBuf::from(&cleaned_path);
      node_modules_path_to_check = path_to_check.clone();
    }

    if fs::metadata(&path_to_check).is_ok() || fs::metadata(&node_modules_path_to_check).is_ok() {
      return Ok(resolved_file_path.to_path_buf());
    }
  }

  Err(std::io::Error::new(
    std::io::ErrorKind::NotFound,
    "File not found",
  ))
}

fn find_node_modules_resolution(
  cwd: &Path,
  import_path_str: &str,
) -> Option<(swc_core::ecma::loader::resolve::Resolution, String)> {
  let (resolver, _) = get_package_json_with_deps(cwd);

  let file_name = FileName::Real(cwd.to_path_buf());

  resolve_package_from_package_json(&resolver, &file_name, import_path_str)
}

fn possible_aliased_paths(
  import_path_str: &str,
  aliases: &FxHashMap<String, Vec<String>>,
) -> Vec<PathBuf> {
  let mut result = vec![PathBuf::from(import_path_str)];

  if aliases.is_empty() {
    return result;
  }

  for (alias, values) in aliases.iter() {
    if let Some((before, after)) = alias.split_once('*') {
      if import_path_str.starts_with(before) && import_path_str.ends_with(after) {
        let replacement_string =
          &import_path_str[before.len()..import_path_str.len() - after.len()];
        for value in values {
          result.push(PathBuf::from(value.replace('*', replacement_string)));
        }
      }
    } else if alias == import_path_str {
      result.extend(values.iter().map(PathBuf::from));
    }
  }

  result
}
