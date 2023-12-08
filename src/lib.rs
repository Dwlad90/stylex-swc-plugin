pub mod transform;
pub mod shared {
    pub mod enums;
    pub mod utils;
}

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
pub fn process_transform(program: Program, metadata: TransformPluginProgramMetadata) -> Program {
    let file_name: FileName = match metadata.get_context(&TransformPluginMetadataContextKind::Cwd) {
        Some(s) => FileName::Real(s.into()),
        None => FileName::Anon,
    };
    let mut stylex: ModuleTransformVisitor<PluginCommentsProxy> =
        ModuleTransformVisitor::new(PluginCommentsProxy, file_name);

    let program = program.fold_with(&mut stylex);

    program
}
