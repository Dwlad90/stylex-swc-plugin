// Tests for parser file traversal, output creation, and package processing.
// Source: crates/stylex-test-parser/src/main.rs

use std::{
  fs,
  sync::Mutex,
  time::{SystemTime, UNIX_EPOCH},
};

use super::*;
use ctor::ctor;

static CWD_LOCK: Mutex<()> = Mutex::new(());

#[ctor]
fn init_color_backtrace() {
  let _ = pretty_env_logger::formatted_builder().try_init();
  color_backtrace::install();
}

fn unique_temp_dir(prefix: &str) -> PathBuf {
  let unique = SystemTime::now()
    .duration_since(UNIX_EPOCH)
    .unwrap()
    .as_nanos();
  std::env::temp_dir().join(format!("{prefix}-{unique}"))
}

#[test]
fn read_all_files_collects_nested_files() {
  let temp_dir = unique_temp_dir("stylex-test-parser-read");
  fs::create_dir_all(temp_dir.join("nested")).unwrap();
  fs::write(temp_dir.join("root.js"), "const a = 1;").unwrap();
  fs::write(temp_dir.join("nested").join("child.js"), "const b = 2;").unwrap();

  let files = read_all_files(&temp_dir).unwrap();
  assert_eq!(files.len(), 2);

  fs::remove_dir_all(temp_dir).unwrap();
}

#[test]
fn write_and_recreate_output_directory() {
  let temp_dir = unique_temp_dir("stylex-test-parser-write");
  create_dir_if_not_exists(temp_dir.to_str().unwrap()).unwrap();
  write_file(
    temp_dir.display().to_string(),
    "sample.js".to_string(),
    "const x = 1;",
  )
  .unwrap();
  assert!(temp_dir.join("sample.js").exists());

  // Recreating should clear previous contents.
  create_dir_if_not_exists(temp_dir.to_str().unwrap()).unwrap();
  assert!(!temp_dir.join("sample.js").exists());

  fs::remove_dir_all(temp_dir).unwrap();
}

#[test]
fn transform_file_handles_snapshot_and_regular_js() {
  let temp_dir = unique_temp_dir("stylex-test-parser-transform");
  let output_dir = temp_dir.join("out");
  fs::create_dir_all(&output_dir).unwrap();

  let regular_file = temp_dir.join("sample.test.js");
  fs::write(&regular_file, "const value = 1; value;").unwrap();
  transform_file(&regular_file, output_dir.to_str().unwrap()).unwrap();
  assert!(output_dir.join("sample.test.js").exists());

  let snap_file = temp_dir.join("sample.tests.js.snap");
  fs::write(&snap_file, "SNAPSHOT_CONTENT").unwrap();
  transform_file(&snap_file, output_dir.to_str().unwrap()).unwrap();
  assert_eq!(
    fs::read_to_string(output_dir.join("sample.tests.js.snap")).unwrap(),
    "SNAPSHOT_CONTENT"
  );

  fs::remove_dir_all(temp_dir).unwrap();
}

#[test]
fn run_processes_all_expected_packages() {
  let _guard = CWD_LOCK.lock().unwrap();
  let temp_workspace = unique_temp_dir("stylex-test-parser-run");
  let stylex_root = temp_workspace.join("stylex");

  for package in [
    "babel-plugin",
    "shared",
    "stylex",
    "open-props",
    "benchmarks",
  ] {
    let package_dir = stylex_root.join(package);
    fs::create_dir_all(package_dir.join("__tests__")).unwrap();
    fs::write(
      package_dir.join("__tests__").join("fixture.test.js"),
      "const fixture = 1; fixture;",
    )
    .unwrap();
  }

  let original_cwd = std::env::current_dir().unwrap();
  std::env::set_current_dir(&temp_workspace).unwrap();
  run(stylex_root.to_str().unwrap());
  std::env::set_current_dir(&original_cwd).unwrap();

  for package in [
    "babel-plugin",
    "shared",
    "stylex",
    "open-props",
    "benchmarks",
  ] {
    let generated = temp_workspace
      .join("output")
      .join("__tests__")
      .join(package)
      .join("fixture.test.js");
    assert!(
      generated.exists(),
      "missing generated file for package: {package}"
    );
  }

  fs::remove_dir_all(temp_workspace).unwrap();
}
