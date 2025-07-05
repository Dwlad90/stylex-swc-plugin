use rustc_hash::FxHashMap;
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
  pub enable_font_size_px_to_rem: bool,
  pub class_name_prefix: String,
  pub enable_debug_class_names: bool,
  pub enable_debug_data_prop: bool,
  pub enable_dev_class_names: bool,
  pub enable_minified_keys: bool,
  pub enable_inlined_conditional_merge: bool,
  pub enable_logical_styles_polyfill: bool,
  // pub defined_stylex_css_variables: FxHashMap<String, String>,
  pub style_resolution: StyleResolution,
  pub import_sources: Vec<ImportSources>,
  pub runtime_injection: Option<RuntimeInjectionState>,
  pub treeshake_compensation: bool,
  pub aliases: Option<FxHashMap<String, Vec<String>>>,
  pub unstable_module_resolution: CheckModuleResolution,
}

impl Default for StyleXStateOptions {
  fn default() -> Self {
    StyleXStateOptions {
      style_resolution: StyleResolution::ApplicationOrder,
      dev: false,
      test: false,
      debug: false,
      enable_debug_class_names: true,
      enable_debug_data_prop: true,
      enable_dev_class_names: false,
      enable_logical_styles_polyfill: true,
      enable_inlined_conditional_merge: true,
      enable_font_size_px_to_rem: false,
      enable_minified_keys: true,
      class_name_prefix: "x".to_string(),
      import_sources: vec![],
      treeshake_compensation: false,
      runtime_injection: None,
      aliases: None,
      unstable_module_resolution: CheckModuleResolution::CommonJS(
        StyleXOptions::get_common_js_module_resolution(None),
      ),
    }
  }
}

impl Default for CheckModuleResolution {
  fn default() -> Self {
    CheckModuleResolution::CommonJS(StyleXOptions::get_common_js_module_resolution(None))
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
          None
        }
      }
      RuntimeInjection::Named(n) => Some(RuntimeInjectionState::Named(n)),
      RuntimeInjection::Regular(s) => Some(RuntimeInjectionState::Regular(s)),
    };

    StyleXStateOptions {
      style_resolution: options.style_resolution,
      enable_font_size_px_to_rem: options.enable_font_size_px_to_rem,
      runtime_injection,
      class_name_prefix: options.class_name_prefix,
      // defined_stylex_css_variables: options.defined_stylex_css_variables,
      import_sources: options.import_sources,
      dev: options.dev,
      test: options.test,
      debug: options.debug,
      enable_debug_class_names: options.enable_debug_class_names,
      enable_debug_data_prop: options.enable_debug_data_prop,
      enable_dev_class_names: options.enable_dev_class_names,
      enable_inlined_conditional_merge: options.enable_inlined_conditional_merge,
      enable_logical_styles_polyfill: options.enable_logical_styles_polyfill,
      enable_minified_keys: options.enable_minified_keys,
      treeshake_compensation: options.treeshake_compensation,
      aliases: options.aliases,
      unstable_module_resolution: options.unstable_module_resolution,
    }
  }
}
