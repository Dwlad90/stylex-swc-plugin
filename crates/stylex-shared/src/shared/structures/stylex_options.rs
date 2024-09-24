use std::collections::HashMap;

use serde::Deserialize;

use crate::shared::constants::common::DEFAULT_INJECT_PATH;

use super::named_import_source::{ImportSources, RuntimeInjection};

#[derive(Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct StyleXOptionsParams {
  pub style_resolution: Option<StyleResolution>,
  pub use_rem_for_font_size: Option<bool>,
  pub runtime_injection: Option<bool>,
  pub class_name_prefix: Option<String>,
  pub defined_stylex_css_variables: Option<HashMap<String, String>>,
  pub import_sources: Option<Vec<ImportSources>>,
  pub treeshake_compensation: Option<bool>,
  pub gen_conditional_classes: Option<bool>,
  pub dev: Option<bool>,
  pub test: Option<bool>,
  pub aliases: Option<HashMap<String, Vec<String>>>,
  #[serde(rename = "unstable_moduleResolution")]
  pub unstable_module_resolution: Option<ModuleResolution>,
}

impl Default for StyleXOptionsParams {
  fn default() -> Self {
    StyleXOptionsParams {
      style_resolution: Some(StyleResolution::ApplicationOrder),
      use_rem_for_font_size: Some(false),
      runtime_injection: Some(false),
      class_name_prefix: Some("x".to_string()),
      defined_stylex_css_variables: Some(HashMap::new()),
      import_sources: None,
      treeshake_compensation: Some(true),
      gen_conditional_classes: Some(false),
      dev: Some(false),
      test: Some(false),
      aliases: None,
      unstable_module_resolution: None,
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
  pub use_rem_for_font_size: bool,
  pub class_name_prefix: String,
  // pub defined_stylex_css_variables: HashMap<String, String>,
  pub style_resolution: StyleResolution,
  pub runtime_injection: RuntimeInjection,
  pub import_sources: Vec<ImportSources>,
  pub treeshake_compensation: Option<bool>,
  pub gen_conditional_classes: bool,
  pub aliases: Option<HashMap<String, Vec<String>>>,
  pub unstable_module_resolution: Option<CheckModuleResolution>,
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
      use_rem_for_font_size: false,
      runtime_injection: RuntimeInjection::Boolean(false),
      class_name_prefix: "x".to_string(),
      // defined_stylex_css_variables: HashMap::new(),
      import_sources: vec![],
      dev: false,
      test: false,
      treeshake_compensation: None,
      gen_conditional_classes: false,
      aliases: None,
      unstable_module_resolution: Some(CheckModuleResolution::Haste(
        StyleXOptions::get_haste_module_resolution(None),
      )),
    }
  }
}

impl From<StyleXOptionsParams> for StyleXOptions {
  fn from(options: StyleXOptionsParams) -> Self {
    let unstable_module_resolution = match options.unstable_module_resolution {
      Some(module_resolution) => match module_resolution.r#type.to_lowercase().as_str() {
        "haste" => Some(CheckModuleResolution::Haste(module_resolution)),
        "cross-file-parsing" => Some(CheckModuleResolution::CrossFileParsing(module_resolution)),
        _ => Some(CheckModuleResolution::CommonJS(module_resolution)),
      },
      None => None,
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
      use_rem_for_font_size: options.use_rem_for_font_size.unwrap_or(false),
      runtime_injection,
      class_name_prefix: options.class_name_prefix.unwrap_or("x".to_string()),
      // defined_stylex_css_variables: options.defined_stylex_css_variables.unwrap_or_default(),
      import_sources: options.import_sources.unwrap_or_default(),
      dev: options.dev.unwrap_or(false),
      test: options.test.unwrap_or(false),
      treeshake_compensation: options.treeshake_compensation,
      gen_conditional_classes: options.gen_conditional_classes.unwrap_or(false),
      aliases: options.aliases,
      unstable_module_resolution,
    }
  }
}
