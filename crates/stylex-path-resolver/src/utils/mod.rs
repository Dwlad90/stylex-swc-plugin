use std::{
  cmp::Ordering,
  path::{Path, PathBuf},
};

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
    .unwrap_or_else(|| {
      panic!(
        "Failed to get relative path for file {} based on root {}",
        file_path.display(),
        root.display()
      )
    })
    .clean()
}

// Sort by priority: .js files first, then .cjs files, then others, and then by length and string content
pub(crate) fn sort_export_paths_by_priority(a: &&String, b: &&String) -> Ordering {
  let ext_priority = |s: &str| {
    if s.ends_with(".js") {
      0
    } else if s.ends_with(".cjs") {
      1
    } else {
      2
    }
  };
  let ord = ext_priority(a).cmp(&ext_priority(b));
  if ord != Ordering::Equal {
    return ord;
  }
  a.len().cmp(&b.len()).then_with(|| a.cmp(b))
}
