#[cfg(test)]
mod get_canonical_file_path {
  use std::{env, path::PathBuf};

  use path_clean::PathClean;
  use rustc_hash::FxHashMap;

  use crate::shared::structures::{
    state_manager::StateManager,
    stylex_options::{CheckModuleResolution, ModuleResolution},
  };

  #[allow(dead_code)]
  fn get_fixture_path(test_path: &str) -> PathBuf {
    env::current_dir()
      .unwrap()
      .join("src/shared/structures/tests/fixtures")
      .join(test_path)
      .clean()
  }

  #[test]
  fn get_canonical_file_path_with_name() {
    let fixture_path = get_fixture_path("package_json_with_name");

    let stage_manager = StateManager::default();

    let canonical_path = stage_manager
      .get_canonical_file_path(fixture_path.to_str().unwrap(), &mut FxHashMap::default());

    assert_eq!(canonical_path, "package_json_with_name:.");
  }

  #[test]
  fn get_canonical_file_path_without_name() {
    let fixture_path = get_fixture_path("package_json_without_name");

    let stage_manager = StateManager::default();

    let canonical_path = stage_manager
      .get_canonical_file_path(fixture_path.to_str().unwrap(), &mut FxHashMap::default());

    assert_eq!(canonical_path, "_unknown_name_:.");
  }

  #[test]
  fn get_canonical_file_unknown_path() {
    let fixture_path = PathBuf::from("/unknown/path");

    let stage_manager = StateManager::default();

    let canonical_path = stage_manager
      .get_canonical_file_path(fixture_path.to_str().unwrap(), &mut FxHashMap::default());

    assert_eq!(canonical_path, "_unknown_path_:path");
  }

  #[test]
  fn get_canonical_file_from_unknown_root_dir() {
    let fixture_path = PathBuf::from("/unknown/path");

    let mut stage_manager = StateManager::default();

    stage_manager.options.unstable_module_resolution =
      Some(CheckModuleResolution::CommonJS(ModuleResolution {
        r#type: "commonjs".to_string(),
        root_dir: Some(fixture_path.parent().unwrap().to_string_lossy().into()),
        theme_file_extension: None,
      }));

    let canonical_path = stage_manager
      .get_canonical_file_path(fixture_path.to_str().unwrap(), &mut FxHashMap::default());

    assert_eq!(canonical_path, "path");
  }

  #[test]
  fn get_canonical_file_from_root_dir() {
    let fixture_path = get_fixture_path("package_json_with_name");

    let root_dir = fixture_path.parent().unwrap();

    let mut stage_manager = StateManager::default();

    stage_manager.options.unstable_module_resolution =
      Some(CheckModuleResolution::CommonJS(ModuleResolution {
        r#type: "commonjs".to_string(),
        root_dir: Some(root_dir.to_string_lossy().into()),
        theme_file_extension: None,
      }));

    let canonical_path = stage_manager.get_canonical_file_path(
      root_dir.join("src/components").to_str().unwrap(),
      &mut FxHashMap::default(),
    );

    assert_eq!(
      canonical_path,
      "@stylexswc/shared:src/shared/structures/tests/fixtures/src/components"
    );
  }
}
