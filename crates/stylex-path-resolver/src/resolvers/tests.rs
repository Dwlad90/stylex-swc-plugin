use std::{
  env,
  path::{Path, PathBuf},
};

use path_clean::PathClean;

#[allow(dead_code)]
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

#[allow(dead_code)]
fn fixture(test_path: &PathBuf, part: &str) -> PathBuf {
  PathBuf::from(
    env::var("original_root_dir").unwrap_or(env::current_dir().unwrap().display().to_string()),
  )
  .join("fixtures")
  .join(test_path)
  .join(part)
  .clean()
}

#[cfg(test)]
mod resolve_path_tests {
  use crate::resolvers::{
    possible_aliased_paths, resolve_file_path, resolve_path,
    tests::{fixture, get_root_dir},
  };

  use std::{collections::HashMap, path::PathBuf};

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
  fn external_pnpm_package_file() {
    assert_eq!(
      resolve_path(
        fixture(& PathBuf::from("workspace"), "../../node_modules/.pnpm/@stylexjs+open-props@0.7.5/node_modules/@stylexjs/open-props/lib/colors.stylex.js").as_path(),
        get_root_dir(& PathBuf::from("workspace")).as_path()
      ),
      "node_modules/@stylexjs/open-props/lib/colors.stylex.js"
    );
  }

  #[test]
  fn external_npm_package_file() {
    assert_eq!(
      resolve_path(
        fixture(
          &PathBuf::from("workspace"),
          "../../node_modules/@stylexjs/open-props/lib/colors.stylex.js"
        )
        .as_path(),
        get_root_dir(&PathBuf::from("workspace")).as_path()
      ),
      "node_modules/@stylexjs/open-props/lib/colors.stylex.js"
    );
  }

  #[test]
  fn external_yarn_pnp_package_file() {
    assert_eq!(
      resolve_path(
        fixture(& PathBuf::from("workspace"), "../../app/node_modules/.yarn/__virtual__/swc-virtual-123123/node_modules/@stylexjs/open-props/lib/colors.stylex.js").as_path(),
        get_root_dir(& PathBuf::from("workspace")).as_path()
      ),
      "node_modules/@stylexjs/open-props/lib/colors.stylex.js"
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

  #[test]
  fn resolve_regular_local_import_from_src() {
    let test_path = PathBuf::from("application");

    let import_path_str = "../colors.stylex.js";
    let source_file_path = format!(
      "{}/src/pages/home.js",
      get_root_dir(&test_path).as_path().display()
    );
    let ext = ".js";
    let root_path = get_root_dir(&test_path).display().to_string();
    let aliases = Default::default();

    let expected_result = "src/colors.stylex.js";

    assert_eq!(
      resolve_file_path(
        import_path_str,
        source_file_path.as_str(),
        ext,
        root_path.as_str(),
        &aliases,
      )
      .unwrap_or_default()
      .display()
      .to_string(),
      expected_result
    );
  }

  #[test]
  fn resolve_regular_local_import_from_same_level_directory() {
    let test_path = PathBuf::from("application");

    let import_path_str = "../components/button.js";
    let source_file_path = format!(
      "{}/src/pages/home.js",
      get_root_dir(&test_path).as_path().display()
    );
    let ext = ".js";
    let root_path = get_root_dir(&test_path).display().to_string();
    let aliases = Default::default();

    let expected_result = "src/components/button.js";

    assert_eq!(
      resolve_file_path(
        import_path_str,
        source_file_path.as_str(),
        ext,
        root_path.as_str(),
        &aliases,
      )
      .unwrap_or_default()
      .display()
      .to_string(),
      expected_result
    );
  }

  #[test]
  fn resolve_regular_local_import_from_alias() {
    let test_path = PathBuf::from("application");

    let import_path_str = "@/components/button.js";
    let source_file_path = format!(
      "{}/src/pages/home.js",
      get_root_dir(&test_path).as_path().display()
    );
    let ext = ".js";
    let root_path = get_root_dir(&test_path).display().to_string();
    let aliases = HashMap::from([("@/*".to_string(), vec![format!("{}/src/*", root_path)])]);

    let expected_result = "src/components/button.js";

    assert_eq!(
      resolve_file_path(
        import_path_str,
        source_file_path.as_str(),
        ext,
        root_path.as_str(),
        &aliases,
      )
      .unwrap_or_default()
      .display()
      .to_string(),
      expected_result
    );
  }

  #[test]
  fn get_import_path_when_no_aliases() {
    assert_eq!(
      possible_aliased_paths("@stylexjs/stylex", &HashMap::new()),
      vec![PathBuf::from("@stylexjs/stylex")]
    );
  }

  #[test]
  fn get_import_path_when_right_aliase() {
    assert_eq!(
      possible_aliased_paths(
        "@/components/button",
        &HashMap::from([("#/app/*".to_string(), vec![format!("{}/src/*", "root")])])
      ),
      vec![PathBuf::from("@/components/button")]
    );
  }

  #[test]
  fn get_import_path_with_aliases() {
    assert_eq!(
      possible_aliased_paths(
        "@/components/button",
        &HashMap::from([("@/*".to_string(), vec!["/src/*".to_string()])])
      ),
      vec![
        PathBuf::from("@/components/button"),
        PathBuf::from("/src/components/button"),
      ]
    );
  }

  #[test]
  fn get_import_path_with_aliases_with_ext() {
    assert_eq!(
      possible_aliased_paths(
        "@/components/button.js",
        &HashMap::from([("@/*".to_string(), vec!["/src/*".to_string()])])
      ),
      vec![
        PathBuf::from("@/components/button.js"),
        PathBuf::from("/src/components/button.js"),
      ]
    );
  }
}
