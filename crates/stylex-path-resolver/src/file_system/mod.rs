use std::{
  fs,
  path::{Path, PathBuf},
};

pub(crate) fn _check_directory(path: &Path) -> bool {
  match fs::metadata(path) {
    Ok(metadata) => metadata.is_dir(),
    Err(_) => false,
  }
}

pub(crate) fn get_directories(path: &Path) -> Result<Vec<PathBuf>, std::io::Error> {
  Ok(
    fs::read_dir(path)?
      .filter_map(|entry| {
        entry.ok().and_then(|e| {
          let path = e.path();
          if path.is_dir() {
            Some(path)
          } else {
            None
          }
        })
      })
      .collect::<Vec<_>>(),
  )
}
