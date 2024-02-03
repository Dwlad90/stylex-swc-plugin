pub(crate) mod transform;
pub mod shared {
    pub(crate) mod enums;
    pub mod structures;
    pub(crate) mod utils;
    pub(crate) mod constants {
        pub(crate) mod application_order;
        pub(crate) mod common;
        pub(crate) mod long_hand_logical;
        pub(crate) mod long_hand_physical;
        pub(crate) mod number_properties;
        pub(crate) mod priorities;
        pub(crate) mod shorthands_of_longhands;
        pub(crate) mod shorthands_of_shorthands;
        pub(crate) mod unitless_number_properties;
    }
}

use std::collections::HashMap;

use serde::Deserialize;
use shared::structures::named_import_source::ImportSources;
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
    pub import_sources: Option<Vec<ImportSources>>,
}

impl Default for StylexConfigParams {
    fn default() -> Self {
        StylexConfigParams {
            use_rem_for_font_size: Option::Some(false),
            runtime_injection: Option::Some(false),
            class_name_prefix: Option::Some("x".to_string()),
            defined_stylex_css_variables: Option::Some(HashMap::new()),
            import_sources: Option::None,
        }
    }
}
#[derive(Deserialize, Clone, Debug)]
pub struct StylexConfig {
    pub style_resolution: Option<String>,
    pub use_rem_for_font_size: bool,
    pub runtime_injection: bool,
    pub class_name_prefix: String,
    pub defined_stylex_css_variables: HashMap<String, String>,
    pub import_sources: Option<Vec<ImportSources>>,
}

impl Default for StylexConfig {
    fn default() -> Self {
        StylexConfig {
            style_resolution: Option::Some("application-order".to_string()),
            use_rem_for_font_size: false,
            runtime_injection: false,
            class_name_prefix: "x".to_string(),
            defined_stylex_css_variables: HashMap::new(),
            import_sources: Option::None,
        }
    }
}

impl From<StylexConfigParams> for StylexConfig {
    fn from(config: StylexConfigParams) -> Self {
        StylexConfig {
            style_resolution: Option::Some("application-order".to_string()),
            use_rem_for_font_size: config.use_rem_for_font_size.unwrap_or(false),
            runtime_injection: config.runtime_injection.unwrap_or(false),
            class_name_prefix: config.class_name_prefix.unwrap_or("x".to_string()),
            defined_stylex_css_variables: config
                .defined_stylex_css_variables
                .unwrap_or(HashMap::new()),
            import_sources: config.import_sources,
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
