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

#[cfg(not(tarpaulin_include))]
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

#[cfg(tarpaulin_include)]
pub fn relative_path(file_path: &Path, root: &Path) -> PathBuf {
  pathdiff::diff_paths(file_path, root).unwrap().clean()
}

#[cfg(test)]
mod tests;
