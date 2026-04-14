//! Unit tests for path resolver internal helpers: `file_not_found_error`
//! and `possible_aliased_paths`.

use std::{
  fs,
  path::PathBuf,
  time::{SystemTime, UNIX_EPOCH},
};

use rustc_hash::FxHashMap;

use crate::{
  package_json::PackageJsonExtended,
  resolvers::{file_not_found_error, possible_aliased_paths, resolve_file_path},
};

fn fixture_root(name: &str) -> PathBuf {
  PathBuf::from(env!("CARGO_MANIFEST_DIR"))
    .join("fixtures")
    .join(name)
}

fn temp_dir(prefix: &str) -> PathBuf {
  let nanos = SystemTime::now()
    .duration_since(UNIX_EPOCH)
    .unwrap()
    .as_nanos();
  let dir = std::env::temp_dir().join(format!("stylex-path-resolver-{prefix}-{nanos}"));
  fs::create_dir_all(&dir).unwrap();
  dir
}

/// file_not_found_error should create a NotFound IO error containing the import
/// path.
#[test]
fn file_not_found_error_creates_not_found() {
  let err = file_not_found_error("./foo/bar");
  assert_eq!(err.kind(), std::io::ErrorKind::NotFound);
  assert!(err.to_string().contains("./foo/bar"));
}

/// With empty aliases map, possible_aliased_paths should return only the
/// original path.
#[test]
fn possible_aliased_paths_empty_aliases() {
  let aliases = FxHashMap::default();
  let paths = possible_aliased_paths("@pkg/foo", &aliases);
  assert_eq!(paths, vec![PathBuf::from("@pkg/foo")]);
}

/// Exact alias match should add the aliased path after the original.
#[test]
fn possible_aliased_paths_exact_match() {
  let mut aliases = FxHashMap::default();
  aliases.insert("@pkg/foo".to_string(), vec!["/abs/path/foo".to_string()]);
  let paths = possible_aliased_paths("@pkg/foo", &aliases);
  assert_eq!(paths.len(), 2);
  assert_eq!(paths[0], PathBuf::from("@pkg/foo"));
  assert_eq!(paths[1], PathBuf::from("/abs/path/foo"));
}

/// Wildcard alias should expand the matched portion.
#[test]
fn possible_aliased_paths_wildcard_match() {
  let mut aliases = FxHashMap::default();
  aliases.insert("@src/*".to_string(), vec!["./src/*".to_string()]);
  let paths = possible_aliased_paths("@src/components/Button", &aliases);
  assert_eq!(paths.len(), 2);
  assert_eq!(paths[0], PathBuf::from("@src/components/Button"));
  assert_eq!(paths[1], PathBuf::from("./src/components/Button"));
}

/// Non-matching wildcard alias should return only the original path.
#[test]
fn possible_aliased_paths_no_match() {
  let mut aliases = FxHashMap::default();
  aliases.insert("@other/*".to_string(), vec!["./other/*".to_string()]);
  let paths = possible_aliased_paths("@pkg/foo", &aliases);
  assert_eq!(paths, vec![PathBuf::from("@pkg/foo")]);
}

/// Non-matching exact alias (no wildcard) should return only the original path.
/// Exercises the false branch of `else if alias == import_path_str`.
#[test]
fn possible_aliased_paths_exact_alias_no_match() {
  let mut aliases = FxHashMap::default();
  aliases.insert("@other/bar".to_string(), vec!["/abs/other".to_string()]);
  let paths = possible_aliased_paths("@pkg/foo", &aliases);
  assert_eq!(paths, vec![PathBuf::from("@pkg/foo")]);
}

/// Relative imports without extension should try known JS/TS extensions.
#[test]
fn resolve_file_path_resolves_relative_import_without_extension() {
  let root = fixture_root("application-pnpm");
  let source_file = root.join("src/pages/home.js");

  let mut package_json_seen = FxHashMap::<String, PackageJsonExtended>::default();
  let aliases = FxHashMap::<String, Vec<String>>::default();

  let resolved = resolve_file_path(
    "../components/button",
    source_file.to_str().unwrap(),
    root.to_str().unwrap(),
    &aliases,
    &mut package_json_seen,
  )
  .unwrap();

  assert_eq!(resolved, root.join("src/components/button.js"));
}

/// Missing relative import should return a NotFound error instead of panicking.
#[test]
fn resolve_file_path_returns_not_found_for_missing_relative_import() {
  let root = fixture_root("application-pnpm");
  let source_file = root.join("src/pages/home.js");

  let mut package_json_seen = FxHashMap::<String, PackageJsonExtended>::default();
  let aliases = FxHashMap::<String, Vec<String>>::default();

  let err = resolve_file_path(
    "../components/does-not-exist",
    source_file.to_str().unwrap(),
    root.to_str().unwrap(),
    &aliases,
    &mut package_json_seen,
  )
  .unwrap_err();

  assert_eq!(err.kind(), std::io::ErrorKind::NotFound);
  assert!(err.to_string().contains("../components/does-not-exist"));
}

/// Source file path that has no parent should be rejected early.
#[test]
fn resolve_file_path_rejects_source_path_without_parent() {
  let root = fixture_root("application-pnpm");
  let mut package_json_seen = FxHashMap::<String, PackageJsonExtended>::default();
  let aliases = FxHashMap::<String, Vec<String>>::default();

  let err = resolve_file_path(
    "stylex-lib-dist-main",
    "/",
    root.to_str().unwrap(),
    &aliases,
    &mut package_json_seen,
  )
  .unwrap_err();

  assert_eq!(err.kind(), std::io::ErrorKind::InvalidInput);
}

/// If resolution fails from the source directory, resolver should retry from
/// root path.
#[test]
fn resolve_file_path_falls_back_to_root_resolver() {
  let root = fixture_root("application-pnpm");
  let outside_dir = temp_dir("resolve-from-root-fallback");
  let source_file = outside_dir.join("entry.js");

  fs::write(&source_file, "export {};").unwrap();

  let mut package_json_seen = FxHashMap::<String, PackageJsonExtended>::default();
  let aliases = FxHashMap::<String, Vec<String>>::default();

  let resolved = resolve_file_path(
    "stylex-lib-dist-main",
    source_file.to_str().unwrap(),
    root.to_str().unwrap(),
    &aliases,
    &mut package_json_seen,
  )
  .unwrap();

  assert_eq!(
    resolved,
    root.join("node_modules/stylex-lib-dist-main/dist/index.jsx")
  );

  fs::remove_dir_all(outside_dir).unwrap();
}

/// Unknown bare package imports should return a NotFound error after all
/// fallbacks.
#[test]
fn resolve_file_path_returns_not_found_for_unknown_package() {
  let root = fixture_root("application-pnpm");
  let source_file = root.join("src/pages/home.js");

  let mut package_json_seen = FxHashMap::<String, PackageJsonExtended>::default();
  let aliases = FxHashMap::<String, Vec<String>>::default();

  let err = resolve_file_path(
    "stylex-lib-package-that-does-not-exist",
    source_file.to_str().unwrap(),
    root.to_str().unwrap(),
    &aliases,
    &mut package_json_seen,
  )
  .unwrap_err();

  assert_eq!(err.kind(), std::io::ErrorKind::NotFound);
  assert!(
    err
      .to_string()
      .contains("stylex-lib-package-that-does-not-exist")
  );
}
