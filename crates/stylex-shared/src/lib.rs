pub mod shared;
pub(crate) mod transform;

use std::path::PathBuf;

use shared::{
  structures::{plugin_pass::PluginPass, stylex_options::StyleXOptionsParams},
  utils::log::log_formatter,
};
pub use transform::ModuleTransformVisitor;

use swc_core::{
  common::FileName,
  ecma::{ast::Program, visit::FoldWith},
  plugin::{
    metadata::TransformPluginMetadataContextKind,
    plugin_transform,
    proxies::{PluginCommentsProxy, TransformPluginProgramMetadata},
  },
};

#[cfg(test)]
mod tests {
  use ctor::ctor;

  #[ctor]
  fn init_color_backtrace() {
    pretty_env_logger::formatted_builder().init();
    color_backtrace::install();
  }
}
