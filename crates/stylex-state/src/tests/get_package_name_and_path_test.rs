#[cfg(test)]
mod get_package_name_and_path {
  use rustc_hash::FxHashMap;

  use crate::state_manager::StateManager;
  use crate::tests::prelude::fixture_path;

  #[test]
  fn get_package_json_with_name() {
    let fixture_path = fixture_path("package_json_with_name");
    // A file, as a caller passes. A folder is read as an entry of its parent.
    let file_path = fixture_path.join("app.js");

    let (package_name, package_path) = StateManager::get_package_name_and_path(
      file_path.to_str().unwrap(),
      &mut FxHashMap::default(),
    )
    .unwrap();

    assert_eq!(package_name.unwrap(), "package_json_with_name");
    assert_eq!(package_path, fixture_path.to_string_lossy());
  }

  #[test]
  fn get_package_json_without_name() {
    let fixture_path = fixture_path("package_json_without_name");
    // A file, as a caller passes. A folder is read as an entry of its parent.
    let file_path = fixture_path.join("app.js");

    let (package_name, package_path) = StateManager::get_package_name_and_path(
      file_path.to_str().unwrap(),
      &mut FxHashMap::default(),
    )
    .unwrap();

    assert!(package_name.is_none(), "package_name should be None");
    assert_eq!(package_path, fixture_path.to_string_lossy());
  }
}
