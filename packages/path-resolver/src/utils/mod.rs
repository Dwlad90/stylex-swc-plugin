use std::path::{Path, PathBuf};

use path_clean::PathClean;

pub(crate) fn contains_subpath(path: &Path, sub_path: &Path) -> bool {
  path
    .display()
    .to_string()
    .split("/")
    .any(|part| part == sub_path.display().to_string())
}
pub fn relative_path(file_path: &Path, root: &Path) -> PathBuf {
  pathdiff::diff_paths(file_path, root)
    .expect("Path resolution failed")
    .clean()
}
