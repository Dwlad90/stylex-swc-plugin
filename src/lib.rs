pub(crate) mod transform;
pub(crate) mod shared {
    pub(crate) mod enums;
    pub(crate) mod structures;
    pub(crate) mod utils;
    pub(crate) mod consts;
}

use std::collections::HashMap;

use serde::Deserialize;
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

#[derive(Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct StylexConfigParams {
    pub use_rem_for_font_size: Option<bool>,
    pub runtime_injection: Option<bool>,
    pub class_name_prefix: Option<String>,
    pub defined_stylex_css_variables: Option<HashMap<String, String>>,
}

impl Default for StylexConfigParams {
    fn default() -> Self {
        StylexConfigParams {
            use_rem_for_font_size: Option::Some(false),
            runtime_injection: Option::Some(false),
            class_name_prefix: Option::Some("x".to_string()),
            defined_stylex_css_variables: Option::Some(HashMap::new()),
        }
    }
}
#[derive(Deserialize, Clone, Debug)]
pub struct StylexConfig {
    pub use_rem_for_font_size: bool,
    pub runtime_injection: bool,
    pub class_name_prefix: String,
    pub defined_stylex_css_variables: HashMap<String, String>,
}

impl Default for StylexConfig {
    fn default() -> Self {
        StylexConfig {
            use_rem_for_font_size: false,
            runtime_injection: false,
            class_name_prefix: "x".to_string(),
            defined_stylex_css_variables: HashMap::new(),
        }
    }
}

impl From<StylexConfigParams> for StylexConfig {
    fn from(config: StylexConfigParams) -> Self {
        StylexConfig {
            use_rem_for_font_size: config.use_rem_for_font_size.unwrap_or(false),
            runtime_injection: config.runtime_injection.unwrap_or(false),
            class_name_prefix: config.class_name_prefix.unwrap_or("x".to_string()),
            defined_stylex_css_variables: config
                .defined_stylex_css_variables
                .unwrap_or(HashMap::new()),
        }
    }
}

#[plugin_transform]
pub(crate) fn process_transform(
    program: Program,
    metadata: TransformPluginProgramMetadata,
) -> Program {
    let config = serde_json::from_str::<StylexConfigParams>(
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
