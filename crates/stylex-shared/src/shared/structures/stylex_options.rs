use rustc_hash::FxHashMap;
use serde::Deserialize;

use crate::shared::constants::common::DEFAULT_INJECT_PATH;

use super::named_import_source::{ImportSources, RuntimeInjection};

#[derive(Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct StyleXOptionsParams {
  pub style_resolution: Option<StyleResolution>,
  pub property_validation_mode: Option<PropertyValidationMode>,
  pub enable_font_size_px_to_rem: Option<bool>,
  pub runtime_injection: Option<RuntimeInjection>,
  pub class_name_prefix: Option<String>,
  pub defined_stylex_css_variables: Option<FxHashMap<String, String>>,
  pub import_sources: Option<Vec<ImportSources>>,
  pub treeshake_compensation: Option<bool>,
  pub enable_inlined_conditional_merge: Option<bool>,
  pub enable_media_query_order: Option<bool>,
  pub enable_logical_styles_polyfill: Option<bool>,
  pub enable_legacy_value_flipping: Option<bool>,
  pub enable_ltr_rtl_comments: Option<bool>,
  pub use_real_file_for_source: Option<bool>,
  pub dev: Option<bool>,
  pub test: Option<bool>,
  pub debug: Option<bool>,
  pub enable_debug_class_names: Option<bool>,
  pub enable_debug_data_prop: Option<bool>,
  pub enable_dev_class_names: Option<bool>,
  pub enable_minified_keys: Option<bool>,
  pub inject_stylex_side_effects: Option<bool>,
  pub aliases: Option<FxHashMap<String, Vec<String>>>,
  #[serde(rename = "unstable_moduleResolution")]
  pub unstable_module_resolution: Option<ModuleResolution>,
}

impl Default for StyleXOptionsParams {
  fn default() -> Self {
    StyleXOptionsParams {
      style_resolution: Some(StyleResolution::PropertySpecificity),
      property_validation_mode: Some(PropertyValidationMode::Silent),
      enable_font_size_px_to_rem: Some(false),
      runtime_injection: Some(RuntimeInjection::Boolean(false)),
      class_name_prefix: Some("x".to_string()),
      defined_stylex_css_variables: Some(FxHashMap::default()),
      import_sources: None,
      treeshake_compensation: Some(true),
      enable_inlined_conditional_merge: Some(true),
      enable_media_query_order: Some(true),
      enable_logical_styles_polyfill: Some(false),
      enable_ltr_rtl_comments: Some(false),
      enable_legacy_value_flipping: Some(false),
      dev: Some(false),
      test: Some(false),
      debug: None,
      enable_debug_class_names: Some(false),
      enable_debug_data_prop: Some(true),
      enable_dev_class_names: Some(false),
      enable_minified_keys: Some(true),
      inject_stylex_side_effects: Some(false),
      use_real_file_for_source: Some(true),
      aliases: None,
      unstable_module_resolution: None,
    }
  }
}

#[derive(Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all(deserialize = "kebab-case", serialize = "PascalCase"))]
pub enum StyleResolution {
  ApplicationOrder,
  PropertySpecificity,
  LegacyExpandShorthands,
}

#[derive(Deserialize, Debug, Clone, PartialEq, Default)]
#[serde(rename_all = "lowercase")]
pub enum PropertyValidationMode {
  Throw,
  Warn,
  #[default]
  Silent,
}

#[derive(Deserialize, Debug, Clone)]

pub enum Aliases {
  String(FxHashMap<String, String>),
  StringVec(FxHashMap<String, Vec<String>>),
}

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all(deserialize = "camelCase", serialize = "PascalCase"))]
pub struct ModuleResolution {
  pub r#type: String,
  pub root_dir: Option<String>,
  pub theme_file_extension: Option<String>,
}

#[derive(Deserialize, Debug, Clone)]

pub enum CheckModuleResolution {
  CommonJS(ModuleResolution),
  Haste(ModuleResolution),
  CrossFileParsing(ModuleResolution),
}

#[derive(Clone, Debug)]
pub struct StyleXOptions {
  pub dev: bool,
  pub test: bool,
  pub debug: bool,
  pub property_validation_mode: PropertyValidationMode,
  pub enable_debug_class_names: bool,
  pub enable_debug_data_prop: bool,
  pub enable_dev_class_names: bool,
  pub enable_inlined_conditional_merge: bool,
  pub enable_media_query_order: bool,
  pub enable_logical_styles_polyfill: bool,
  pub enable_legacy_value_flipping: bool,
  pub enable_ltr_rtl_comments: bool,
  pub enable_minified_keys: bool,
  pub enable_font_size_px_to_rem: bool,
  pub use_real_file_for_source: bool,
  pub class_name_prefix: String,
  // pub defined_stylex_css_variables: FxHashMap<String, String>,
  pub style_resolution: StyleResolution,
  pub runtime_injection: RuntimeInjection,
  pub import_sources: Vec<ImportSources>,
  pub treeshake_compensation: bool,
  pub inject_stylex_side_effects: bool,
  pub aliases: Option<FxHashMap<String, Vec<String>>>,
  pub unstable_module_resolution: CheckModuleResolution,
}

impl StyleXOptions {
  pub fn get_haste_module_resolution(root_dir: Option<String>) -> ModuleResolution {
    ModuleResolution {
      r#type: "haste".to_string(),
      root_dir,
      theme_file_extension: None,
    }
  }

  pub fn get_common_js_module_resolution(root_dir: Option<String>) -> ModuleResolution {
    ModuleResolution {
      r#type: "commonjs".to_string(),
      root_dir,
      theme_file_extension: None,
    }
  }
}

impl Default for StyleXOptions {
  fn default() -> Self {
    StyleXOptions {
      style_resolution: StyleResolution::PropertySpecificity,
      property_validation_mode: PropertyValidationMode::Silent,
      enable_font_size_px_to_rem: false,
      runtime_injection: RuntimeInjection::Boolean(false),
      class_name_prefix: "x".to_string(),
      // defined_stylex_css_variables: FxHashMap::default(),
      import_sources: vec![],
      dev: false,
      test: false,
      debug: false,
      enable_debug_class_names: false,
      enable_debug_data_prop: true,
      enable_dev_class_names: false,
      enable_inlined_conditional_merge: true,
      enable_media_query_order: true,
      enable_logical_styles_polyfill: false,
      enable_legacy_value_flipping: false,
      enable_ltr_rtl_comments: false,
      enable_minified_keys: true,
      inject_stylex_side_effects: false,
      use_real_file_for_source: true,
      treeshake_compensation: false,
      aliases: None,
      unstable_module_resolution: CheckModuleResolution::CommonJS(
        StyleXOptions::get_common_js_module_resolution(None),
      ),
    }
  }
}

impl From<StyleXOptionsParams> for StyleXOptions {
  fn from(options: StyleXOptionsParams) -> Self {
    let module_resolution = options
      .unstable_module_resolution
      .unwrap_or_else(|| StyleXOptions::get_common_js_module_resolution(None));

    let unstable_module_resolution = match module_resolution.r#type.to_lowercase().as_str() {
      "haste" => CheckModuleResolution::Haste(module_resolution),
      "cross-file-parsing" => CheckModuleResolution::CrossFileParsing(module_resolution),
      _ => CheckModuleResolution::CommonJS(module_resolution),
    };

    let runtime_injection = match options.runtime_injection {
      Some(value) => match value {
        RuntimeInjection::Boolean(true) => {
          RuntimeInjection::Regular(DEFAULT_INJECT_PATH.to_string())
        }
        RuntimeInjection::Boolean(false) => RuntimeInjection::Boolean(false),
        RuntimeInjection::Regular(path) => RuntimeInjection::Regular(path),
        RuntimeInjection::Named(named) => RuntimeInjection::Named(named),
      },
      None => RuntimeInjection::Boolean(false),
    };

    StyleXOptions {
      style_resolution: options
        .style_resolution
        .unwrap_or(StyleResolution::PropertySpecificity),
      property_validation_mode: options
        .property_validation_mode
        .unwrap_or(PropertyValidationMode::Silent),
      enable_font_size_px_to_rem: options.enable_font_size_px_to_rem.unwrap_or(false),
      runtime_injection,
      class_name_prefix: options.class_name_prefix.unwrap_or("x".to_string()),
      // defined_stylex_css_variables: options.defined_stylex_css_variables.unwrap_or_default(),
      import_sources: options.import_sources.unwrap_or_else(|| {
        vec![
          ImportSources::Regular("stylex".to_string()),
          ImportSources::Regular("@stylexjs/stylex".to_string()),
        ]
      }),
      dev: options.dev.unwrap_or(false),
      test: options.test.unwrap_or(false),
      debug: options.debug.or(options.dev).unwrap_or(false),
      enable_debug_class_names: options.enable_debug_class_names.unwrap_or(false),
      enable_debug_data_prop: options.enable_debug_data_prop.unwrap_or(true),
      enable_dev_class_names: options.enable_dev_class_names.unwrap_or(false),
      enable_minified_keys: options.enable_minified_keys.unwrap_or(true),
      inject_stylex_side_effects: options.inject_stylex_side_effects.unwrap_or(false),
      treeshake_compensation: options.treeshake_compensation.unwrap_or(false),
      enable_inlined_conditional_merge: options.enable_inlined_conditional_merge.unwrap_or(true),
      enable_media_query_order: options.enable_media_query_order.unwrap_or(true),
      enable_logical_styles_polyfill: options.enable_logical_styles_polyfill.unwrap_or(false),
      enable_legacy_value_flipping: options.enable_legacy_value_flipping.unwrap_or(false),
      enable_ltr_rtl_comments: options.enable_ltr_rtl_comments.unwrap_or(false),
      use_real_file_for_source: options.use_real_file_for_source.unwrap_or(true),
      aliases: options.aliases,
      unstable_module_resolution,
    }
  }
}
