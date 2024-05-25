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
      cwd: Option::None,
      filename: FileName::Anon,
    }
  }
}
