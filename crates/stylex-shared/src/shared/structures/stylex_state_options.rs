use std::collections::HashMap;

use serde::Deserialize;

use crate::shared::constants::common::DEFAULT_INJECT_PATH;

use super::{
  named_import_source::{ImportSources, RuntimeInjection, RuntimeInjectionState},
  stylex_options::{CheckModuleResolution, StyleResolution, StyleXOptions},
};

#[derive(Deserialize, Clone, Debug)]
pub struct StyleXStateOptions {
  pub dev: bool,
  pub test: bool,
  pub debug: bool,
  pub use_rem_for_font_size: bool,
  pub class_name_prefix: String,
  // pub defined_stylex_css_variables: HashMap<String, String>,
  pub style_resolution: StyleResolution,
  pub import_sources: Vec<ImportSources>,
  pub runtime_injection: Option<RuntimeInjectionState>,
  pub treeshake_compensation: Option<bool>,
  pub gen_conditional_classes: bool,
  pub aliases: Option<HashMap<String, Vec<String>>>,
  pub unstable_module_resolution: Option<CheckModuleResolution>,
}

impl Default for StyleXStateOptions {
  fn default() -> Self {
    StyleXStateOptions {
      style_resolution: StyleResolution::ApplicationOrder,
      dev: false,
      test: false,
      debug: false,
      use_rem_for_font_size: false,
      class_name_prefix: "x".to_string(),
      import_sources: vec![],
      runtime_injection: None,
      treeshake_compensation: None,
      gen_conditional_classes: false,
      aliases: None,
      unstable_module_resolution: None,
    }
  }
}

impl Default for CheckModuleResolution {
  fn default() -> Self {
    CheckModuleResolution::Haste(StyleXOptions::get_haste_module_resolution(None))
  }
}
impl From<StyleXOptions> for StyleXStateOptions {
  fn from(options: StyleXOptions) -> Self {
    let runtime_injection = match options.runtime_injection {
      RuntimeInjection::Boolean(b) => {
        if b || options.dev {
          Some(RuntimeInjectionState::Regular(
            DEFAULT_INJECT_PATH.to_string(),
          ))
        } else {
          None
        }
      }
      RuntimeInjection::Named(n) => Some(RuntimeInjectionState::Named(n)),
      RuntimeInjection::Regular(s) => Some(RuntimeInjectionState::Regular(s)),
    };

    StyleXStateOptions {
      style_resolution: options.style_resolution,
      use_rem_for_font_size: options.use_rem_for_font_size,
      runtime_injection,
      class_name_prefix: options.class_name_prefix,
      // defined_stylex_css_variables: options.defined_stylex_css_variables,
      import_sources: options.import_sources,
      dev: options.dev,
      test: options.test,
      debug: options.debug,
      treeshake_compensation: options.treeshake_compensation,
      gen_conditional_classes: options.gen_conditional_classes,
      aliases: options.aliases,
      unstable_module_resolution: options.unstable_module_resolution,
    }
  }
}
