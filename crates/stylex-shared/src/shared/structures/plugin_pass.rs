use std::path::PathBuf;

use swc_core::common::FileName;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PluginPass {
  pub cwd: Option<PathBuf>,
  pub filename: FileName,
}

impl Default for PluginPass {
  fn default() -> Self {
    Self {
      cwd: None,
      filename: FileName::Anon,
    }
  }
}

impl PluginPass {
  pub fn new(cwd: Option<PathBuf>, filename: Option<FileName>) -> Self {
    PluginPass {
      cwd: cwd.or_else(|| Some(PathBuf::from("/stylex/packages/"))),
      filename: filename
        .unwrap_or_else(|| FileName::Real("/stylex/packages/TestTheme.stylex.js".into())),
    }
  }
}
