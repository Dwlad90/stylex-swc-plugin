pub mod shared;
pub(crate) mod transform;

use std::env;

use shared::structures::{plugin_pass::PluginPass, stylex_options::StyleXOptionsParams};
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

#[plugin_transform]
pub(crate) fn process_transform(
  program: Program,
  metadata: TransformPluginProgramMetadata,
) -> Program {
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

  let cwd = match env::current_dir() {
    Ok(cwd) => Some(cwd),
    Err(e) => panic!("Error getting current directory: {}", e),
  };

  let plugin_pass = Box::new(PluginPass { cwd, filename });

  let mut stylex: ModuleTransformVisitor<PluginCommentsProxy> =
    ModuleTransformVisitor::new(PluginCommentsProxy, plugin_pass, &mut config);

  program.fold_with(&mut stylex)
}
