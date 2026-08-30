//! Tests for reading the parts of a source file's name.

use std::path::PathBuf;

use swc_core::common::FileName;

use crate::ast::source_file::{
  extract_filename_from_path, extract_filename_with_ext_from_path, extract_path,
};

// ──────────────────────────────────────────────
// extract_filename_from_path
// ──────────────────────────────────────────────

mod extract_filename_from_path_tests {
  use super::*;

  #[test]
  fn returns_stem_for_simple_js_file() {
    let path = FileName::Real(PathBuf::from("/path/to/file.js"));
    assert_eq!(extract_filename_from_path(&path), "file");
  }

  #[test]
  fn returns_stem_for_dotted_extension() {
    let path = FileName::Real(PathBuf::from("/path/to/file.stylex.ts"));
    assert_eq!(extract_filename_from_path(&path), "file.stylex");
  }

  #[test]
  fn returns_stem_for_flat_path() {
    let path = FileName::Real(PathBuf::from("simple.js"));
    assert_eq!(extract_filename_from_path(&path), "simple");
  }

  #[test]
  fn returns_empty_string_for_anon() {
    let path = FileName::Anon;
    assert_eq!(extract_filename_from_path(&path), "");
  }

  #[test]
  fn returns_stem_for_tsx_file() {
    let path = FileName::Real(PathBuf::from("/src/components/Button.tsx"));
    assert_eq!(extract_filename_from_path(&path), "Button");
  }

  #[test]
  fn returns_stem_for_no_extension() {
    let path = FileName::Real(PathBuf::from("/path/to/Makefile"));
    assert_eq!(extract_filename_from_path(&path), "Makefile");
  }
}

// ──────────────────────────────────────────────
// extract_path
// ──────────────────────────────────────────────

mod extract_path_tests {
  use super::*;

  #[test]
  fn returns_full_path_for_real_file() {
    let path = FileName::Real(PathBuf::from("/path/to/file.js"));
    assert_eq!(extract_path(&path), "/path/to/file.js");
  }

  #[test]
  fn returns_empty_for_anon() {
    let path = FileName::Anon;
    assert_eq!(extract_path(&path), "");
  }

  #[test]
  fn returns_relative_path() {
    let path = FileName::Real(PathBuf::from("relative/file.ts"));
    assert_eq!(extract_path(&path), "relative/file.ts");
  }
}

// ──────────────────────────────────────────────
// extract_filename_with_ext_from_path
// ──────────────────────────────────────────────

mod extract_filename_with_ext_from_path_tests {
  use super::*;

  #[test]
  fn returns_filename_with_ext_for_js() {
    let path = FileName::Real(PathBuf::from("/path/to/file.js"));
    assert_eq!(extract_filename_with_ext_from_path(&path), Some("file.js"));
  }

  #[test]
  fn returns_filename_with_double_ext() {
    let path = FileName::Real(PathBuf::from("/path/to/file.stylex.ts"));
    assert_eq!(
      extract_filename_with_ext_from_path(&path),
      Some("file.stylex.ts")
    );
  }

  #[test]
  fn returns_none_for_anon() {
    let path = FileName::Anon;
    assert_eq!(extract_filename_with_ext_from_path(&path), None);
  }

  #[test]
  fn returns_filename_without_directory() {
    let path = FileName::Real(PathBuf::from("standalone.css"));
    assert_eq!(
      extract_filename_with_ext_from_path(&path),
      Some("standalone.css")
    );
  }

  #[test]
  fn returns_filename_without_extension() {
    let path = FileName::Real(PathBuf::from("/path/Makefile"));
    assert_eq!(extract_filename_with_ext_from_path(&path), Some("Makefile"));
  }
}
