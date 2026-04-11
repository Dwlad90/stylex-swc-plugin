//! Unit tests for path resolver internal helpers: `file_not_found_error`
//! and `possible_aliased_paths`.

use std::path::PathBuf;

use rustc_hash::FxHashMap;

use crate::resolvers::{file_not_found_error, possible_aliased_paths};

/// file_not_found_error should create a NotFound IO error containing the import path.
#[test]
fn file_not_found_error_creates_not_found() {
  let err = file_not_found_error("./foo/bar");
  assert_eq!(err.kind(), std::io::ErrorKind::NotFound);
  assert!(err.to_string().contains("./foo/bar"));
}

/// With empty aliases map, possible_aliased_paths should return only the original path.
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

/// Non-matching alias should return only the original path.
#[test]
fn possible_aliased_paths_no_match() {
  let mut aliases = FxHashMap::default();
  aliases.insert("@other/*".to_string(), vec!["./other/*".to_string()]);
  let paths = possible_aliased_paths("@pkg/foo", &aliases);
  assert_eq!(paths, vec![PathBuf::from("@pkg/foo")]);
}
