// Tests for node package resolution error messages from basedir helper.
// Source: crates/stylex-evaluator/src/common.rs

use std::path::PathBuf;

use super::resolve_node_package_path_from_basedir;

#[test]
fn resolve_node_package_path_from_basedir_includes_package_in_error() {
  let result =
    resolve_node_package_path_from_basedir("definitely-not-a-real-package-123", PathBuf::from("/"));

  match result {
    Ok(_) => panic!("expected resolution failure"),
    Err(message) => assert!(message.contains("definitely-not-a-real-package-123")),
  }
}
