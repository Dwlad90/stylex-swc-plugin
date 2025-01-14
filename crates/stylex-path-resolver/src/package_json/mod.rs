use rustc_hash::FxHashMap;
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, env};
use std::{default::Default, fs::read_to_string};
use swc_core::{
  common::FileName,
  ecma::loader::{resolvers::node::NodeModulesResolver, TargetEnv},
};

use package_json::{PackageDependencies, PackageJsonManager};
use std::path::{Path, PathBuf};

use crate::resolvers::get_node_modules_path;

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
#[serde(rename_all = "camelCase")]
pub struct PackageJsonExtended {
  pub name: String,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub main: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub module: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub exports: Option<FxHashMap<String, String>>,
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
                  - The required field `name` might be missing.\n\
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

pub fn find_nearest_package_json(path: &Path) -> Option<PathBuf> {
  let package_json_path: PathBuf = path.join("package.json");

  if package_json_path.exists() {
    return package_json_path.parent().map(|p| p.to_path_buf());
  }

  match path.parent() {
    Some(parent) => find_nearest_package_json(parent),
    None => None,
  }
}

pub(crate) fn resolve_package_from_package_json(
  resolver: &NodeModulesResolver,
  file_name: &FileName,
  import_path_str: &str,
  package_json_seen: &mut FxHashMap<String, PackageJsonExtended>,
) -> Option<swc_core::ecma::loader::resolve::Resolution> {
  const PATH_SEPARATOR: char = '/';

  if let Some(resolution) =
    get_node_modules_path(resolver, file_name, import_path_str, package_json_seen)
  {
    if let FileName::Real(_) = &resolution.filename {
      return Some(resolution);
    }
  }

  let parts: Vec<&str> = import_path_str.split(PATH_SEPARATOR).collect();

  if parts.len() <= 1 {
    return None;
  }

  let parent_path = parts[..parts.len() - 1].join(&PATH_SEPARATOR.to_string());

  resolve_package_from_package_json(resolver, file_name, &parent_path, package_json_seen)
}

pub(crate) fn get_package_json_with_deps(
  cwd: &Path,
  package_json_seen: &mut FxHashMap<String, PackageJsonExtended>,
) -> (NodeModulesResolver, HashMap<String, String>) {
  let node_modules_resolver = NodeModulesResolver::new(TargetEnv::Node, Default::default(), true);
  let resolver = node_modules_resolver;

  let (package_json, _) = get_package_json(cwd, package_json_seen);

  let mut package_dependencies = package_json.dependencies.unwrap_or_default();
  let package_dev_dependencies = package_json.dev_dependencies.unwrap_or_default();

  package_dependencies.extend(package_dev_dependencies);

  (resolver, package_dependencies)
}
