pub(crate) mod transform;
pub mod shared {
    pub(crate) mod constants;
    pub(crate) mod enums;
    pub mod structures;
    pub mod utils;
}

use shared::structures::stylex_options::StyleXOptionsParams;
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

// #[derive(Deserialize, Clone, Debug)]
// #[serde(rename_all = "camelCase")]
// pub struct StylexConfigParams {
//     pub use_rem_for_font_size: Option<bool>,
//     pub runtime_injection: Option<bool>,
//     pub class_name_prefix: Option<String>,
//     pub defined_stylex_css_variables: Option<HashMap<String, String>>,
//     pub import_sources: Option<Vec<ImportSources>>,
// }

// impl From<StyleXOptionsParams> for StyleXOptions {
//     fn from(config: StyleXOptionsParams) -> Self {
//         StyleXOptions {
//             style_resolution: Option::Some("application-order".to_string()),
//             use_rem_for_font_size: config.use_rem_for_font_size.unwrap_or(false),
//             runtime_injection: config.runtime_injection.unwrap_or(false),
//             class_name_prefix: config.class_name_prefix.unwrap_or("x".to_string()),
//             defined_stylex_css_variables: config
//                 .defined_stylex_css_variables
//                 .unwrap_or(HashMap::new()),
//             import_sources: config.import_sources,
//         }
//     }
// }

#[plugin_transform]
pub(crate) fn process_transform(
    program: Program,
    metadata: TransformPluginProgramMetadata,
) -> Program {
    let config = serde_json::from_str::<StyleXOptionsParams>(
        &metadata
            .get_transform_plugin_config()
            .expect("failed to get plugin config for stylex"),
    )
    .expect("invalid config for stylex");

    let file_name: FileName =
        match metadata.get_context(&TransformPluginMetadataContextKind::Filename) {
            Some(s) => FileName::Real(s.into()),
            None => FileName::Anon,
        };

    let mut stylex: ModuleTransformVisitor<PluginCommentsProxy> =
        ModuleTransformVisitor::new(PluginCommentsProxy, file_name, config);

    let program = program.fold_with(&mut stylex);

    program
}
