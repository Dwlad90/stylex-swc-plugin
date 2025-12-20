use indexmap::IndexSet;
use rustc_hash::FxHashMap;
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, env};
use std::{default::Default, fs::read_to_string};

use package_json::{PackageDependencies, PackageJsonManager};
use std::path::{Path, PathBuf};

use crate::{enums::ExportsType, file_system::find_closest_path};

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
#[serde(rename_all = "camelCase")]
pub struct PackageJsonExtended {
  #[serde(skip_serializing_if = "Option::is_none")]
  pub name: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub main: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub module: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub exports: Option<FxHashMap<String, ExportsType>>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub dependencies: Option<PackageDependencies>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub dev_dependencies: Option<PackageDependencies>,
}

pub fn get_package_json(
  path: &Path,
  package_json_seen: &mut FxHashMap<String, PackageJsonExtended>,
) -> (PackageJsonExtended, PackageJsonManager) {
  let (package_json_path, manager) = get_package_json_path(path);

  match package_json_path {
    Some(file) => {
      let file_path_string = file.display().to_string();
      let file_path = file_path_string.as_str();
      let data = package_json_seen.get(file_path).cloned().or_else(|| {
        let data = read_to_string(file_path);

        data.ok().map(|package_json_raw| {
          let json = serde_json::from_str::<PackageJsonExtended>(package_json_raw.as_str())
            .unwrap_or_else(|error| {
              panic!(
                "Failed to parse `{}` file. Error: {}.\n\nPossible reasons:\n\
                  - The JSON structure might be incorrect.\n\
                  - There might be a syntax error in the JSON.\n\
                  - Ensure all required fields are present and correctly formatted.",
                file_path_string, error
              );
            });

          package_json_seen.insert(file_path.to_string(), json.clone());
          json
        })
      });

      match data {
        Some(json) => (json, manager),
        None => panic!(
          "Failed to read package.json file: {}/{}",
          env::current_dir().unwrap().display(),
          file.display()
        ),
      }
    }
    None => {
      panic!("No package.json found for path: {:?}", path.display());
    }
  }
}

pub(crate) fn get_package_json_path(path: &Path) -> (Option<PathBuf>, PackageJsonManager) {
  let mut manager = PackageJsonManager::new();

  match manager.locate_closest_from(path) {
    Ok(file) => (Option::Some(file), manager),
    Err(error) => {
      panic!("Error: {}, path: {}", error, path.display());
    }
  }
}

pub fn find_closest_package_json(path: &Path) -> Option<PathBuf> {
  find_closest_path(path, "package.json")
}

pub fn find_closest_package_json_folder(path: &Path) -> Option<PathBuf> {
  find_closest_package_json(path).map(|path| path.parent().unwrap().to_path_buf())
}

pub fn find_closest_node_modules(path: &Path) -> Option<PathBuf> {
  find_closest_path(path, "node_modules")
}

pub fn recursive_find_node_modules(
  path: &Path,
  known_git_dir: Option<PathBuf>,
) -> IndexSet<PathBuf> {
  let mut node_modules_paths = IndexSet::new();

  if path.eq(Path::new("/")) {
    let root_node_modules = path.join("node_modules");

    if root_node_modules.exists() {
      node_modules_paths.insert(root_node_modules);
    }

    return node_modules_paths;
  }

  let Some(closest_node_modules_path) = find_closest_node_modules(path) else {
    return node_modules_paths;
  };

  node_modules_paths.insert(closest_node_modules_path.clone());

  if let Some(repository_git_dir) = known_git_dir.or_else(|| find_closest_path(path, ".git")) {
    let repository_root = repository_git_dir.parent().unwrap_or(Path::new("/"));

    let closest_modules_parent = closest_node_modules_path
      .parent()
      .and_then(|p| p.parent())
      .unwrap_or(Path::new("/"));

    if repository_root.eq(closest_modules_parent) {
      let root_node_modules = repository_root.join("node_modules");

      if root_node_modules.exists() {
        node_modules_paths.insert(root_node_modules);
      }
    } else if closest_modules_parent.starts_with(repository_root) {
      node_modules_paths.extend(recursive_find_node_modules(
        closest_modules_parent,
        Some(repository_git_dir),
      ));
    }
  }

  node_modules_paths
}

pub(crate) fn get_package_json_deps(
  cwd: &Path,
  package_json_seen: &mut FxHashMap<String, PackageJsonExtended>,
) -> HashMap<String, String> {
  let (package_json, _) = get_package_json(cwd, package_json_seen);

  let mut package_dependencies = package_json.dependencies.unwrap_or_default();
  let package_dev_dependencies = package_json.dev_dependencies.unwrap_or_default();

  package_dependencies.extend(package_dev_dependencies);

  package_dependencies
}
