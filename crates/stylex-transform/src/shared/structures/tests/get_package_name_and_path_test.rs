#[cfg(test)]
mod get_package_name_and_path {
  use std::{env, path::PathBuf};

  use path_clean::PathClean;
  use rustc_hash::FxHashMap;

  use crate::shared::structures::state_manager::StateManager;

  fn get_fixture_path(test_path: &str) -> PathBuf {
    env::current_dir()
      .unwrap()
      .join("src/shared/structures/tests/fixtures")
      .join(test_path)
      .clean()
  }

  #[test]
  fn get_package_json_with_name() {
    let fixture_path = get_fixture_path("package_json_with_name");

    let (package_name, package_path) = StateManager::get_package_name_and_path(
      fixture_path.to_str().unwrap(),
      &mut FxHashMap::default(),
    )
    .unwrap();

    assert_eq!(package_name.unwrap(), "package_json_with_name");
    assert_eq!(package_path, fixture_path.to_string_lossy());
  }

  #[test]
  fn get_package_json_without_name() {
    let fixture_path = get_fixture_path("package_json_without_name");

    let (package_name, package_path) = StateManager::get_package_name_and_path(
      fixture_path.to_str().unwrap(),
      &mut FxHashMap::default(),
    )
    .unwrap();

    assert!(package_name.is_none(), "package_name should be None");
    assert_eq!(package_path, fixture_path.to_string_lossy());
  }
}
