use std::collections::HashMap;

use serde::Deserialize;

use crate::shared::constants::constants::DEFAULT_INJECT_PATH;

use super::{
    named_import_source::{ImportSources, RuntimeInjection, RuntimeInjectionState},
    stylex_options::{self, CheckModuleResolution, StyleResolution, StyleXOptions},
};

#[derive(Deserialize, Clone, Debug)]
pub struct StyleXStateOptions {
    pub dev: bool,
    pub test: bool,
    pub use_rem_for_font_size: bool,
    pub class_name_prefix: String,
    pub defined_stylex_css_variables: HashMap<String, String>, // Assuming the values are strings
    pub style_resolution: StyleResolution,
    pub import_sources: Vec<ImportSources>,
    pub runtime_injection: Option<RuntimeInjectionState>,
    pub treeshake_compensation: Option<bool>,
    pub gen_conditional_classes: bool,
    pub aliases: Option<HashMap<String, Vec<String>>>,
    pub unstable_module_resolution: Option<CheckModuleResolution>,
}

impl StyleXStateOptions {
    pub(crate) fn _new() -> Self {
        StyleXStateOptions {
            style_resolution: StyleResolution::ApplicationOrder,
            use_rem_for_font_size: false,
            runtime_injection: Option::None, // HERERE
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

impl From<StyleXOptions> for StyleXStateOptions {
    fn from(options: StyleXOptions) -> Self {
        let runtime_injection = match options.runtime_injection {
            RuntimeInjection::Boolean(b) => {
                if b {
                    Some(RuntimeInjectionState::Regular(
                        DEFAULT_INJECT_PATH.to_string(),
                    ))
                } else {
                    Option::None
                }
            }
            RuntimeInjection::Named(n) => Some(RuntimeInjectionState::Named(n)),
            RuntimeInjection::Regular(s) => Some(RuntimeInjectionState::Regular(s)),
        };

        let aliases = match options.aliases {
            Some(aliases) => match aliases {
                stylex_options::Aliases::String(aliases) => {
                    let mut aliases_map = HashMap::new();
                    for (key, value) in aliases {
                        let mut vec = Vec::new();
                        vec.push(value);
                        aliases_map.insert(key, vec);
                    }

                    Option::Some(aliases_map)
                }
                stylex_options::Aliases::StringVec(aliases) => Option::Some(aliases),
            },
            None => Option::None,
        };

        StyleXStateOptions {
            style_resolution: StyleResolution::ApplicationOrder,
            use_rem_for_font_size: options.use_rem_for_font_size,
            runtime_injection,
            class_name_prefix: options.class_name_prefix,
            defined_stylex_css_variables: options.defined_stylex_css_variables,
            import_sources: options.import_sources,
            dev: options.dev,
            test: options.test,
            treeshake_compensation: options.treeshake_compensation,
            gen_conditional_classes: options.gen_conditional_classes,
            aliases,
            unstable_module_resolution: options.unstable_module_resolution,
        }
    }
}
