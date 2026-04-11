use std::path::{Path, PathBuf};

use path_clean::PathClean;
use stylex_macros::stylex_panic;

pub(crate) fn contains_subpath(path: &Path, sub_path: &Path) -> bool {
  path
    .display()
    .to_string()
    .split("/")
    .any(|part| part == sub_path.display().to_string())
}

pub fn relative_path(file_path: &Path, root: &Path) -> PathBuf {
  pathdiff::diff_paths(file_path, root)
    .unwrap_or_else(|| {
      stylex_panic!(
        "Failed to get relative path for file {} based on root {}",
        file_path.display(),
        root.display()
      )
    })
    .clean()
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn contains_subpath_found() {
    assert!(contains_subpath(
      Path::new("a/b/c"),
      Path::new("b")
    ));
  }

  #[test]
  fn contains_subpath_not_found() {
    assert!(!contains_subpath(
      Path::new("a/b/c"),
      Path::new("d")
    ));
  }

  #[test]
  fn contains_subpath_first_segment() {
    assert!(contains_subpath(
      Path::new("node_modules/pkg/index.js"),
      Path::new("pkg")
    ));
  }

  #[test]
  fn contains_subpath_exact_match() {
    assert!(contains_subpath(
      Path::new("a"),
      Path::new("a")
    ));
  }

  #[test]
  fn relative_path_basic() {
    let result = relative_path(
      Path::new("/root/src/file.ts"),
      Path::new("/root"),
    );
    assert_eq!(result, PathBuf::from("src/file.ts"));
  }

  #[test]
  fn relative_path_with_parent() {
    let result = relative_path(
      Path::new("/other/file.ts"),
      Path::new("/root"),
    );
    assert_eq!(result, PathBuf::from("../other/file.ts"));
  }
}
