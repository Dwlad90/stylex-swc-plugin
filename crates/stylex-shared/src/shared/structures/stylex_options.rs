use rustc_hash::FxHashMap;
use serde::Deserialize;

use crate::shared::constants::common::DEFAULT_INJECT_PATH;

use super::named_import_source::{ImportSources, RuntimeInjection};

#[derive(Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct StyleXOptionsParams {
  pub style_resolution: Option<StyleResolution>,
  pub enable_font_size_px_to_rem: Option<bool>,
  pub runtime_injection: Option<bool>,
  pub class_name_prefix: Option<String>,
  pub defined_stylex_css_variables: Option<FxHashMap<String, String>>,
  pub import_sources: Option<Vec<ImportSources>>,
  pub treeshake_compensation: Option<bool>,
  pub enable_inlined_conditional_merge: Option<bool>,
  pub dev: Option<bool>,
  pub test: Option<bool>,
  pub debug: Option<bool>,
  pub enable_debug_class_names: Option<bool>,
  pub enable_debug_data_prop: Option<bool>,
  pub enable_dev_class_names: Option<bool>,
  pub enable_minified_keys: Option<bool>,
  pub aliases: Option<FxHashMap<String, Vec<String>>>,
  #[serde(rename = "unstable_moduleResolution")]
  pub unstable_module_resolution: Option<ModuleResolution>,
}

impl Default for StyleXOptionsParams {
  fn default() -> Self {
    StyleXOptionsParams {
      style_resolution: Some(StyleResolution::ApplicationOrder),
      enable_font_size_px_to_rem: Some(false),
      runtime_injection: Some(false),
      class_name_prefix: Some("x".to_string()),
      defined_stylex_css_variables: Some(FxHashMap::default()),
      import_sources: None,
      treeshake_compensation: Some(true),
      enable_inlined_conditional_merge: Some(true),
      dev: Some(false),
      test: Some(false),
      debug: None,
      enable_debug_class_names: Some(true),
      enable_debug_data_prop: Some(true),
      enable_dev_class_names: Some(false),
      enable_minified_keys: Some(true),
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
  pub enable_debug_class_names: bool,
  pub enable_debug_data_prop: bool,
  pub enable_dev_class_names: bool,
  pub enable_minified_keys: bool,
  pub enable_font_size_px_to_rem: bool,
  pub class_name_prefix: String,
  // pub defined_stylex_css_variables: FxHashMap<String, String>,
  pub style_resolution: StyleResolution,
  pub runtime_injection: RuntimeInjection,
  pub import_sources: Vec<ImportSources>,
  pub treeshake_compensation: Option<bool>,
  pub enable_inlined_conditional_merge: bool,
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
      style_resolution: StyleResolution::ApplicationOrder,
      enable_font_size_px_to_rem: false,
      runtime_injection: RuntimeInjection::Boolean(false),
      class_name_prefix: "x".to_string(),
      // defined_stylex_css_variables: FxHashMap::default(),
      import_sources: vec![],
      dev: false,
      test: false,
      debug: false,
      enable_debug_class_names: true,
      enable_debug_data_prop: true,
      enable_dev_class_names: false,
      enable_minified_keys: true,
      treeshake_compensation: None,
      enable_inlined_conditional_merge: true,
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
        true => RuntimeInjection::Regular(DEFAULT_INJECT_PATH.to_string()),
        false => RuntimeInjection::Boolean(options.dev.unwrap_or(false)),
      },
      None => RuntimeInjection::Boolean(options.dev.unwrap_or(false)),
    };

    StyleXOptions {
      style_resolution: options
        .style_resolution
        .unwrap_or(StyleResolution::ApplicationOrder),
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
      enable_debug_class_names: options.enable_debug_class_names.unwrap_or(true),
      enable_debug_data_prop: options.enable_debug_data_prop.unwrap_or(true),
      enable_dev_class_names: options.enable_dev_class_names.unwrap_or(false),
      enable_minified_keys: options.enable_minified_keys.unwrap_or(true),
      treeshake_compensation: options.treeshake_compensation,
      enable_inlined_conditional_merge: options.enable_inlined_conditional_merge.unwrap_or(true),
      aliases: options.aliases,
      unstable_module_resolution,
    }
  }
}
