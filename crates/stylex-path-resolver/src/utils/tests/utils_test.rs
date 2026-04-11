//! Tests for path utility functions: `contains_subpath` and `relative_path`.

use std::path::{Path, PathBuf};

use crate::utils::{contains_subpath, relative_path};

/// Verifies subpath is found in the middle of the path.
#[test]
fn contains_subpath_found() {
  assert!(contains_subpath(Path::new("a/b/c"), Path::new("b")));
}

/// Verifies non-existent subpath returns false.
#[test]
fn contains_subpath_not_found() {
  assert!(!contains_subpath(Path::new("a/b/c"), Path::new("d")));
}

/// Verifies subpath matching works for first segment.
#[test]
fn contains_subpath_first_segment() {
  assert!(contains_subpath(
    Path::new("node_modules/pkg/index.js"),
    Path::new("pkg")
  ));
}

/// Verifies exact single-segment path matches.
#[test]
fn contains_subpath_exact_match() {
  assert!(contains_subpath(Path::new("a"), Path::new("a")));
}

/// Verifies relative path computation from a root.
#[test]
fn relative_path_basic() {
  let result = relative_path(Path::new("/root/src/file.ts"), Path::new("/root"));
  assert_eq!(result, PathBuf::from("src/file.ts"));
}

/// Verifies relative path when file is outside root (produces `../`).
#[test]
fn relative_path_with_parent() {
  let result = relative_path(Path::new("/other/file.ts"), Path::new("/root"));
  assert_eq!(result, PathBuf::from("../other/file.ts"));
}
