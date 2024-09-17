use path_clean::PathClean;
use regex::Regex;
use std::path::{Path, PathBuf};
use std::{collections::HashMap, default::Default};
use swc_core::{
  common::FileName,
  ecma::loader::{resolve::Resolve, resolvers::node::NodeModulesResolver, TargetEnv},
};

use std::fs;

use crate::{
  package_json::get_package_json,
  utils::{contains_subpath, relative_path},
};

mod tests;

pub const EXTENSIONS: [&str; 8] = [".tsx", ".ts", ".jsx", ".js", ".mjs", ".cjs", ".mdx", ".md"];

pub fn resolve_path(processing_file: &Path, root_dir: &Path) -> String {
  let file_pattern = Regex::new(r"\.(jsx?|tsx?|mdx?|mjs|cjs)$").unwrap(); // Matches common file extensions

  if !file_pattern.is_match(processing_file.to_str().unwrap()) {
    let processing_path: PathBuf;

    #[cfg(test)]
    {
      processing_path = processing_file
        .strip_prefix(root_dir.parent().unwrap().parent().unwrap())
        .unwrap()
        .to_path_buf();
    }

    #[cfg(not(test))]
    {
      processing_path = processing_file.to_path_buf();
    }

    panic!(
      r#"Resolve path must be a file, but got: {}"#,
      processing_path.display()
    );
  }

  let cwd: PathBuf;

  #[cfg(test)]
  {
    cwd = root_dir.to_path_buf();
  }

  #[cfg(not(test))]
  {
    cwd = "cwd".into();
  }

  let mut stripped_path = match processing_file.strip_prefix(root_dir) {
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
          return resolved_path_from_node_modules;
        }
      }

      let resolver = NodeModulesResolver::new(TargetEnv::Node, Default::default(), true);

      let (package_json, _) = get_package_json(cwd.as_path());

      let relative_package_path = relative_path(processing_file, root_dir);

      let mut package_dependencies = package_json.dependencies.unwrap_or_default();
      let package_dev_dependencies = package_json.dev_dependencies.unwrap_or_default();

      package_dependencies.extend(package_dev_dependencies);

      let mut potential_package_path: String = Default::default();

      for (name, version) in package_dependencies.iter() {
        if version.starts_with("workspace") {
          let file_name = FileName::Real(cwd.to_path_buf());

          let potential_path_section = name.split("/").last().unwrap_or_default();

          if contains_subpath(&relative_package_path, Path::new(&potential_path_section)) {
            let relative_package_path_str = relative_package_path.display().to_string();

            let potential_file_path = relative_package_path_str
              .split(potential_path_section)
              .last()
              .unwrap_or_default();

            if !potential_file_path.is_empty()
              || relative_package_path_str
                .ends_with(format!("/{}", potential_path_section).as_str())
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
      }

      PathBuf::from(potential_package_path)
    }
  };

  if stripped_path.starts_with(&cwd) {
    stripped_path = stripped_path.strip_prefix(cwd).unwrap().to_path_buf();
  }

  let resolved_path = stripped_path.clean().display().to_string();

  #[cfg(test)]
  {
    let cwd_resolved_path = format!("{}/{}", root_dir.display(), resolved_path);

    assert!(
      fs::metadata(&cwd_resolved_path).is_ok(),
      "Path resolution failed: {}",
      resolved_path
    );
  }

  resolved_path
}

fn get_node_modules_path(
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
  potential_file_path: &str,
  exports: &HashMap<String, String>,
  potential_package_path: &mut String,
  real_resolved_node_modules_path: &Path,
) {
  let potential_file_path_without_extension = PathBuf::from(potential_file_path)
    .with_extension("")
    .display()
    .to_string();

  let mut values: Vec<&String> = exports.values().collect();

  values.sort_by_key(|k| -(k.len() as isize));

  let real_resolved_package_path = real_resolved_node_modules_path
    .parent()
    .expect("Path must have a parent");

  for value in values {
    if value.contains(&potential_file_path_without_extension) {
      *potential_package_path = real_resolved_package_path.join(value).display().to_string();

      break;
    }
  }

  if potential_package_path.is_empty() {
    let mut keys: Vec<&String> = exports.keys().collect();
    keys.sort_by_key(|k| -(k.len() as isize));

    for key in keys {
      if key.contains(&potential_file_path_without_extension) {
        *potential_package_path = real_resolved_package_path
          .join(exports.get(key).unwrap())
          .display()
          .to_string();

        break;
      }
    }
  }

  if potential_package_path.is_empty() {
    eprintln!("Unfortunatly, the exports field is not yet fully supported, so path resolving may work not as expected");
    // TODO: implement exports field resolution
  }
}

pub fn resolve_file_path(
  import_path_str: &str,
  source_file_path: &str,
  ext: &str,
  root_path: &str,
  aliases: &HashMap<String, Vec<String>>,
) -> std::io::Result<PathBuf> {
  let source_dir = Path::new(source_file_path).parent().unwrap();

  let resolved_file_paths: Vec<PathBuf> = if import_path_str.starts_with('.') {
    let root_path: &Path = Path::new(root_path);

    let resolved_import_path = PathBuf::from(resolve_path(
      source_dir.join(import_path_str).as_path(),
      root_path,
    ));

    vec![resolved_import_path]
  } else if import_path_str.starts_with('/') {
    vec![Path::new(root_path).join(import_path_str)]
  } else {
    let root_path: &Path = Path::new(root_path);

    let mut aliased_file_paths = possible_aliased_paths(import_path_str, aliases)
      .iter()
      .filter_map(|path| path.strip_prefix(root_path).ok())
      .map(PathBuf::from)
      .collect::<Vec<PathBuf>>();

    let node_modules_path = Path::new("node_modules").join(import_path_str);

    aliased_file_paths.push(node_modules_path);

    aliased_file_paths
  };

  for resolved_file_path in resolved_file_paths.iter() {
    let mut resolved_file_path = resolved_file_path.clean();

    if let Some(extension) = resolved_file_path.extension() {
      let subpath = extension.to_string_lossy();

      if EXTENSIONS.iter().all(|ext| {
        let res = !ext.ends_with(subpath.as_ref());
        res
      }) {
        resolved_file_path.set_extension(format!("{}{}", subpath, ext));
      }
    } else {
      resolved_file_path.set_extension(ext);
    }

    let resolved_file_path = resolved_file_path.clean();

    let cleaned_path_binding = resolved_file_path
      .to_str()
      .unwrap()
      .replace("..", ".")
      .to_string();

    let cleaned_path = cleaned_path_binding.as_str();

    let mut path_to_check = PathBuf::from(cleaned_path);

    let mut node_modules_path_to_check = path_to_check.clone();

    let cwd: &str;

    #[cfg(test)]
    {
      cwd = root_path;
    }

    #[cfg(not(test))]
    {
      cwd = "cwd";
    }

    if !cleaned_path.contains(cwd) {
      node_modules_path_to_check = Path::new(cwd)
        .join("node_modules")
        .join(path_to_check.clone());
      path_to_check = Path::new(cwd).join(path_to_check);
    }

    if fs::metadata(path_to_check.clone()).is_ok()
      || fs::metadata(node_modules_path_to_check.clone()).is_ok()
    {
      return Ok(resolved_file_path.to_path_buf());
    }
  }

  Err(std::io::Error::new(
    std::io::ErrorKind::NotFound,
    "File not found",
  ))
}

fn possible_aliased_paths(
  import_path_str: &str,
  aliases: &HashMap<String, Vec<String>>,
) -> Vec<PathBuf> {
  let mut result = vec![PathBuf::from(import_path_str)];

  if aliases.is_empty() {
    return result;
  }

  for (alias, values) in aliases.iter() {
    if alias.contains('*') {
      let parts: Vec<&str> = alias.split('*').collect();
      let before = parts[0];
      let after = parts[1];

      if import_path_str.starts_with(before) && import_path_str.ends_with(after) {
        let replacement_string =
          &import_path_str[before.len()..import_path_str.len() - after.len()];
        for value in values {
          result.push(PathBuf::from(value.replace('*', replacement_string)));
        }
      }
    } else if alias == import_path_str {
      for value in values {
        result.push(PathBuf::from(value.clone()));
      }
    }
  }

  result
}
