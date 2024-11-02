use rustc_hash::FxHashMap;
use serde::{Deserialize, Serialize};
use std::env;
use std::{default::Default, fs::read_to_string};

use package_json::{PackageDependencies, PackageJsonManager};
use std::path::{Path, PathBuf};

#[derive(Serialize, Deserialize, Debug, Default)]
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

pub(crate) fn get_package_json(path: &Path) -> (PackageJsonExtended, PackageJsonManager) {
  let (package_json_content, manager) = get_package_json_path(path);

  match package_json_content {
    Some(file) => {
      let data = read_to_string(file.display().to_string().as_str());

      match data {
        Ok(package_json_raw) => {
          let json =
            serde_json::from_str::<PackageJsonExtended>(package_json_raw.as_str()).unwrap();

          (json, manager)
        }
        Err(_) => panic!(
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
