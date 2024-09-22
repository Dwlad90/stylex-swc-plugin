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

#[plugin_transform]
pub(crate) fn process_transform(
  program: Program,
  metadata: TransformPluginProgramMetadata,
) -> Program {
  pretty_env_logger::formatted_builder()
    .format(log_formatter)
    .init();
  color_backtrace::install();

  let mut config = serde_json::from_str::<StyleXOptionsParams>(
    &metadata
      .get_transform_plugin_config()
      .expect("failed to get plugin config for stylex"),
  )
  .expect("invalid config for stylex");

  let filename: FileName = match metadata.get_context(&TransformPluginMetadataContextKind::Filename)
  {
    Some(s) => FileName::Real(s.into()),
    None => FileName::Anon,
  };

  let cwd: Option<PathBuf> = metadata
    .get_context(&TransformPluginMetadataContextKind::Cwd)
    .map(PathBuf::from);

  let plugin_pass = Box::new(PluginPass { cwd, filename });

  let mut stylex: ModuleTransformVisitor<PluginCommentsProxy> =
    ModuleTransformVisitor::new(PluginCommentsProxy, plugin_pass, &mut config, true);

  program.fold_with(&mut stylex)
}
