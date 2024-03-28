use std::collections::HashMap;

use serde::Deserialize;

use super::named_import_source::{ImportSources, RuntimeInjection};

#[derive(Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct StyleXOptionsParams {
    pub use_rem_for_font_size: Option<bool>,
    pub runtime_injection: Option<RuntimeInjection>,
    pub class_name_prefix: Option<String>,
    pub defined_stylex_css_variables: Option<HashMap<String, String>>,
    pub import_sources: Option<Vec<ImportSources>>,
    pub treeshake_compensation: Option<bool>,
    pub gen_conditional_classes: Option<bool>,
    pub dev: Option<bool>,
    pub test: Option<bool>,
    pub aliases: Option<Aliases>,
    pub unstable_module_resolution: Option<ModuleResolution>,
}

impl Default for StyleXOptionsParams {
    fn default() -> Self {
        StyleXOptionsParams {
            use_rem_for_font_size: Option::Some(false),
            runtime_injection: Option::Some(RuntimeInjection::Boolean(true)),
            class_name_prefix: Option::Some("x".to_string()),
            defined_stylex_css_variables: Option::Some(HashMap::new()),
            import_sources: Option::None,
            treeshake_compensation: Option::Some(true),
            gen_conditional_classes: Option::Some(false),
            dev: Option::Some(false),
            test: Option::Some(false),
            aliases: Option::None,
            unstable_module_resolution: Option::None,
        }
    }
}

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all(deserialize = "kebab-case", serialize = "PascalCase"))]

pub enum StyleResolution {
    ApplicationOrder,
    PropertySpecificity,
    LegacyExpandShorthands,
}

#[derive(Deserialize, Debug, Clone)]

pub enum Aliases {
    String(HashMap<String, String>),
    StringVec(HashMap<String, Vec<String>>),
}

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all(deserialize = "camelCase", serialize = "PascalCase"))]
pub struct ModuleResolution {
    r#type: String,
    _root_dir: String,
    _theme_file_extension: Option<String>,
}

#[derive(Deserialize, Debug, Clone)]

pub enum CheckModuleResolution {
    CommonJS(ModuleResolution),
    Haste(ModuleResolution),
    CrossFileParsing(ModuleResolution),
}

#[derive(Deserialize, Clone, Debug)]
pub struct StyleXOptions {
    pub dev: bool,
    pub test: bool,
    pub use_rem_for_font_size: bool,
    pub class_name_prefix: String,
    pub defined_stylex_css_variables: HashMap<String, String>, // Assuming the values are strings
    pub style_resolution: StyleResolution,
    pub runtime_injection: RuntimeInjection,
    pub import_sources: Vec<ImportSources>,
    pub treeshake_compensation: Option<bool>,
    pub gen_conditional_classes: bool,
    pub aliases: Option<Aliases>,
    pub unstable_module_resolution: Option<CheckModuleResolution>,
}

impl Default for StyleXOptions {
    fn default() -> Self {
        StyleXOptions {
            style_resolution: StyleResolution::ApplicationOrder,
            use_rem_for_font_size: false,
            runtime_injection: RuntimeInjection::Boolean(false),
            class_name_prefix: "x".to_string(),
            defined_stylex_css_variables: HashMap::new(),
            import_sources: vec![],
            dev: false,
            test: false,
            treeshake_compensation: Option::None,
            gen_conditional_classes: false,
            aliases: Option::None,
            unstable_module_resolution: Option::None,
        }
    }
}

impl From<StyleXOptionsParams> for StyleXOptions {
    fn from(options: StyleXOptionsParams) -> Self {
        // let aliases: Option<HashMap<String, Vec<String>>> = Option::None;

        let unstable_module_resolution = match options.unstable_module_resolution {
            Some(module_resolution) => match module_resolution.r#type.to_lowercase().as_str() {
                "haste" => Option::Some(CheckModuleResolution::Haste(module_resolution)),
                "cross-file-parsing" => {
                    Option::Some(CheckModuleResolution::CrossFileParsing(module_resolution))
                }
                _ => Option::Some(CheckModuleResolution::CommonJS(module_resolution)),
            },
            None => Option::None,
        };

        StyleXOptions {
            style_resolution: StyleResolution::ApplicationOrder,
            use_rem_for_font_size: options.use_rem_for_font_size.unwrap_or(false),
            runtime_injection: options
                .runtime_injection
                .unwrap_or(RuntimeInjection::Boolean(false)),
            class_name_prefix: options.class_name_prefix.unwrap_or("x".to_string()),
            defined_stylex_css_variables: options
                .defined_stylex_css_variables
                .unwrap_or(HashMap::new()),
            import_sources: options.import_sources.unwrap_or(vec![]),
            dev: options.dev.unwrap_or(false),
            test: options.test.unwrap_or(false),
            treeshake_compensation: options.treeshake_compensation,
            gen_conditional_classes: options.gen_conditional_classes.unwrap_or(false),
            aliases: options.aliases,
            unstable_module_resolution,
        }
    }
}
