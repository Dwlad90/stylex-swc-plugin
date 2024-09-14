#[cfg(test)]
mod resolve_path_tests {
  use crate::resolvers::resolve_path;
  use path_clean::PathClean;
  use std::{
    env,
    path::{Path, PathBuf},
  };

  fn fixture(test_path: &PathBuf, part: &str) -> PathBuf {
    PathBuf::from(
      env::var("original_root_dir").unwrap_or(env::current_dir().unwrap().display().to_string()),
    )
    .join("fixtures")
    .join(test_path)
    .join(part)
    .clean()
  }

  fn get_root_dir(test_path: &Path) -> PathBuf {
    if env::var("original_root_dir").is_err() {
      env::set_var("original_root_dir", env::current_dir().unwrap());
    }

    let new_cwd = PathBuf::from(env::var("original_root_dir").unwrap())
      .join("fixtures")
      .join(test_path)
      .clean();

    env::set_current_dir(&new_cwd).expect("Failed to set current directory");

    new_cwd
  }

  #[test]
  fn resolve_work_dir_packages() {
    let test_path = PathBuf::from("workspace");

    assert_eq!(
      resolve_path(
        fixture(&test_path, "test/index.js").as_path(),
        get_root_dir(&test_path).as_path()
      ),
      "test/index.js"
    );

    assert_eq!(
      resolve_path(
        fixture(&test_path, "index.js").as_path(),
        get_root_dir(&test_path).as_path()
      ),
      "index.js"
    );
  }

  #[test]
  #[should_panic(expected = "Path resolution failed: index.jsx")]
  fn resolve_work_dir_not_existed_packages() {
    let test_path = PathBuf::from("workspace");

    resolve_path(
      fixture(&test_path, "index.jsx").as_path(),
      get_root_dir(&test_path).as_path(),
    );
  }

  #[test]
  fn external_package_with_namespace() {
    let test_path = PathBuf::from("workspace");

    assert_eq!(
      resolve_path(
        fixture(
          &test_path,
          "node_modules/@stylex/open-props/lib/colors.stylex.js"
        )
        .as_path(),
        get_root_dir(&test_path).as_path()
      ),
      "node_modules/@stylex/open-props/lib/colors.stylex.js"
    );
  }

  #[test]
  #[should_panic(
    expected = "Path resolution failed: node_modules/@stylex/open-props/lib/spaces.stylex.js"
  )]
  fn resolve_work_dir_not_existed_external_package_file_with_namespace() {
    let test_path = PathBuf::from("workspace");

    resolve_path(
      fixture(
        &test_path,
        "node_modules/@stylex/open-props/lib/spaces.stylex.js",
      )
      .as_path(),
      get_root_dir(&test_path).as_path(),
    );
  }

  #[test]
  #[should_panic(
    expected = "Path resolution failed: node_modules/@stylex/close-props/lib/colors.stylex.js"
  )]
  fn resolve_work_dir_not_existed_external_package_with_namespace() {
    let test_path = PathBuf::from("workspace");

    resolve_path(
      fixture(
        &test_path,
        "node_modules/@stylex/close-props/lib/colors.stylex.js",
      )
      .as_path(),
      get_root_dir(&test_path).as_path(),
    );
  }

  #[test]
  fn external_package_without_namespace() {
    let test_path = PathBuf::from("workspace");

    assert_eq!(
      resolve_path(
        fixture(&test_path, "node_modules/stylex-lib/colors.stylex.js").as_path(),
        get_root_dir(&test_path).as_path()
      ),
      "node_modules/stylex-lib/colors.stylex.js"
    );
  }

  #[test]
  fn workspace_package_without_namespace() {
    let test_path = PathBuf::from("workspace");
    let local_package_test_path = PathBuf::from("");

    assert_eq!(
      resolve_path(
        fixture(
          &local_package_test_path,
          "packages/stylex-lib/colors.stylex.js"
        )
        .as_path(),
        get_root_dir(&test_path).as_path()
      ),
      "node_modules/stylex-lib/colors.stylex.js"
    );
  }

  #[test]
  fn workspace_package_with_namespace() {
    let test_path = PathBuf::from("workspace");
    let local_package_test_path = PathBuf::from("");

    assert_eq!(
      resolve_path(
        fixture(
          &local_package_test_path,
          "packages/@stylex/theme-lib/colors.stylex.js"
        )
        .as_path(),
        get_root_dir(&test_path).as_path()
      ),
      "node_modules/@stylex/theme-lib/colors.stylex.js"
    );
  }

  #[test]
  fn external_package_main_exports() {
    let test_path = PathBuf::from("exports");

    assert_eq!(
      resolve_path(
        fixture(
          &test_path,
          "node_modules/stylex-lib-dist-main/dist/index.jsx"
        )
        .as_path(),
        get_root_dir(&test_path).as_path()
      ),
      "node_modules/stylex-lib-dist-main/dist/index.jsx"
    );
  }

  #[test]
  fn external_package_module_exports() {
    let test_path = PathBuf::from("exports");
    assert_eq!(
      resolve_path(
        fixture(
          &test_path,
          "node_modules/stylex-lib-dist-module/dist/index.jsx"
        )
        .as_path(),
        get_root_dir(&test_path).as_path()
      ),
      "node_modules/stylex-lib-dist-module/dist/index.jsx"
    );
  }

  #[test]
  fn external_package_exports() {
    let test_path = PathBuf::from("exports");

    assert_eq!(
      resolve_path(
        fixture(
          &test_path,
          "node_modules/stylex-lib-dist-exports/dist/index.js"
        )
        .as_path(),
        get_root_dir(&test_path).as_path()
      ),
      "node_modules/stylex-lib-dist-exports/dist/index.js"
    );
  }

  #[test]
  fn workspace_package_main_exports() {
    let test_path = PathBuf::from("exports");
    let local_package_test_path = PathBuf::from("");

    assert_eq!(
      resolve_path(
        fixture(
          &local_package_test_path,
          "packages/stylex-lib-dist-main-local/dist/index.jsx"
        )
        .as_path(),
        get_root_dir(&test_path).as_path()
      ),
      "node_modules/stylex-lib-dist-main-local/dist/index.jsx"
    );
  }

  #[test]
  #[should_panic(
    expected = "Resolve path must be a file, but got: fixtures/packages/stylex-lib-dist-main-local"
  )]
  fn resolve_work_dir_not_existed_workspace_package_main_exports() {
    let test_path = PathBuf::from("exports");
    let local_package_test_path = PathBuf::from("");

    resolve_path(
      fixture(
        &local_package_test_path,
        "packages/stylex-lib-dist-main-local",
      )
      .as_path(),
      get_root_dir(&test_path).as_path(),
    );
  }

  #[test]
  fn workspace_package_module_exports() {
    let test_path = PathBuf::from("exports");
    let local_package_test_path = PathBuf::from("");

    assert_eq!(
      resolve_path(
        fixture(
          &local_package_test_path,
          "packages/stylex-lib-dist-module-local/dist/index.jsx"
        )
        .as_path(),
        get_root_dir(&test_path).as_path()
      ),
      "node_modules/stylex-lib-dist-module-local/dist/index.jsx"
    );
  }

  #[test]
  fn workspace_package_exports() {
    let test_path = PathBuf::from("exports");
    let local_package_test_path = PathBuf::from("");

    assert_eq!(
      resolve_path(
        fixture(
          &test_path,
          "node_modules/stylex-lib-dist-exports/dist/index.js"
        )
        .as_path(),
        get_root_dir(&test_path).as_path()
      ),
      "node_modules/stylex-lib-dist-exports/dist/index.js"
    );

    assert_eq!(
      resolve_path(
        fixture(
          &local_package_test_path,
          "packages/stylex-lib-dist-exports-local/dist/index.js"
        )
        .as_path(),
        get_root_dir(&test_path).as_path()
      ),
      "node_modules/stylex-lib-dist-exports-local/dist/index.js"
    );

    assert_eq!(
      resolve_path(
        fixture(
          &local_package_test_path,
          "packages/stylex-lib-dist-exports-local/dist/colors.stylex.js"
        )
        .as_path(),
        get_root_dir(&test_path).as_path()
      ),
      "node_modules/stylex-lib-dist-exports-local/dist/colors.stylex.js"
    );
  }

  #[test]
  #[should_panic(
    expected = "Resolve path must be a file, but got: fixtures/packages/stylex-lib-dist-module-local"
  )]
  fn resolve_path_not_a_file_workspace_package_module_exports() {
    let test_path = PathBuf::from("exports");
    let local_package_test_path = PathBuf::from("");

    resolve_path(
      fixture(
        &local_package_test_path,
        "packages/stylex-lib-dist-module-local",
      )
      .as_path(),
      get_root_dir(&test_path).as_path(),
    );
  }

  #[test]
  #[should_panic(
    expected = "Resolve path must be a file, but got: fixtures/packages/stylex-lib-dist-exports/colors.stylex"
  )]
  fn resolve_work_dir_not_existed_workspace_package_exports() {
    let test_path = PathBuf::from("exports");
    let local_package_test_path = PathBuf::from("");

    resolve_path(
      fixture(
        &local_package_test_path,
        "packages/stylex-lib-dist-exports/colors.stylex",
      )
      .as_path(),
      get_root_dir(&test_path).as_path(),
    );
  }

  #[test]
  #[should_panic(
    expected = "Resolve path must be a file, but got: fixtures/exports/node_modules/stylex-lib-dist-exports/colors.stylex"
  )]
  fn resolve_work_dir_not_existed_external_package_exports() {
    let test_path = PathBuf::from("exports");

    resolve_path(
      fixture(
        &test_path,
        "node_modules/stylex-lib-dist-exports/colors.stylex",
      )
      .as_path(),
      get_root_dir(&test_path).as_path(),
    );
  }

  #[test]
  #[should_panic(
    expected = "Resolve path must be a file, but got: fixtures/exports/node_modules/stylex-lib-dist-exports"
  )]
  fn failed_resolve_root_package_path() {
    let test_path = PathBuf::from("exports");

    assert_eq!(
      resolve_path(
        fixture(&test_path, "node_modules/stylex-lib-dist-exports").as_path(),
        get_root_dir(&test_path).as_path()
      ),
      "node_modules/stylex-lib-dist-exports/dist/index.js"
    );
  }

  #[test]
  #[should_panic(
    expected = "Resolve path must be a file, but got: fixtures/exports/node_modules/stylex-lib-dist-exports/colors.stylex"
  )]
  fn failed_resolve_package_exports_dir_path() {
    let test_path = PathBuf::from("exports");

    resolve_path(
      fixture(
        &test_path,
        "node_modules/stylex-lib-dist-exports/colors.stylex",
      )
      .as_path(),
      get_root_dir(&test_path).as_path(),
    );
  }

  #[test]
  #[should_panic(
    expected = "Resolve path must be a file, but got: fixtures/packages/stylex-lib-dist-exports-local"
  )]
  fn failed_resolve_local_package_root_dir_path() {
    let test_path = PathBuf::from("exports");
    let local_package_test_path = PathBuf::from("");

    resolve_path(
      fixture(
        &local_package_test_path,
        "packages/stylex-lib-dist-exports-local",
      )
      .as_path(),
      get_root_dir(&test_path).as_path(),
    );
  }

  #[test]
  #[should_panic(
    expected = "Path resolution failed: node_modules/stylex-lib-dist-exports-local/colors.stylex.js"
  )]
  fn resolve_work_dir_not_existed_local_package_exports_path() {
    let test_path = PathBuf::from("exports");
    let local_package_test_path = PathBuf::from("");

    resolve_path(
      fixture(
        &local_package_test_path,
        "packages/stylex-lib-dist-exports-local/colors.stylex.js",
      )
      .as_path(),
      get_root_dir(&test_path).as_path(),
    );
  }
}
