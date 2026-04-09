use std::ops::{Deref, DerefMut};

use indexmap::IndexMap;
use rustc_hash::FxHashMap;
use serde::Deserialize;

use stylex_constants::constants::common::DEFAULT_INJECT_PATH;
use stylex_enums::property_validation_mode::PropertyValidationMode;
use stylex_enums::style_resolution::StyleResolution;
use stylex_enums::sx_prop_name_param::SxPropNameParam;

use crate::{
  core_stylex_options::CoreStyleXOptions,
  named_import_source::{ImportSources, RuntimeInjection},
  stylex_env::{EnvEntry, JSFunction},
};

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
  pub sx_prop_name: Option<SxPropNameParam>,
  #[serde(skip)]
  pub env: Option<IndexMap<String, EnvEntry>>,
  #[serde(skip)]
  pub debug_file_path: Option<JSFunction>,
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
      sx_prop_name: None,
      env: None,
      debug_file_path: None,
    }
  }
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

impl Default for CheckModuleResolution {
  fn default() -> Self {
    CheckModuleResolution::CommonJS(StyleXOptions::get_common_js_module_resolution(None))
  }
}

#[derive(Clone, Debug)]
pub struct StyleXOptions {
  pub core: CoreStyleXOptions,
  pub runtime_injection: RuntimeInjection,
}

impl Deref for StyleXOptions {
  type Target = CoreStyleXOptions;
  fn deref(&self) -> &Self::Target {
    &self.core
  }
}

impl DerefMut for StyleXOptions {
  fn deref_mut(&mut self) -> &mut Self::Target {
    &mut self.core
  }
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

  pub fn with_class_name_prefix(mut self, prefix: impl Into<String>) -> Self {
    self.core.class_name_prefix = prefix.into();
    self
  }

  pub fn with_style_resolution(mut self, resolution: StyleResolution) -> Self {
    self.core.style_resolution = resolution;
    self
  }

  pub fn with_debug(mut self, debug: bool) -> Self {
    self.core.debug = debug;
    self
  }

  pub fn with_dev(mut self, dev: bool) -> Self {
    self.core.dev = dev;
    self
  }

  pub fn with_test(mut self, test: bool) -> Self {
    self.core.test = test;
    self
  }

  pub fn with_enable_debug_class_names(mut self, enabled: bool) -> Self {
    self.core.enable_debug_class_names = enabled;
    self
  }

  pub fn with_enable_debug_data_prop(mut self, enabled: bool) -> Self {
    self.core.enable_debug_data_prop = enabled;
    self
  }

  pub fn with_enable_dev_class_names(mut self, enabled: bool) -> Self {
    self.core.enable_dev_class_names = enabled;
    self
  }

  pub fn with_enable_font_size_px_to_rem(mut self, enabled: bool) -> Self {
    self.core.enable_font_size_px_to_rem = enabled;
    self
  }

  pub fn with_enable_logical_styles_polyfill(mut self, enabled: bool) -> Self {
    self.core.enable_logical_styles_polyfill = enabled;
    self
  }

  pub fn with_enable_minified_keys(mut self, enabled: bool) -> Self {
    self.core.enable_minified_keys = enabled;
    self
  }

  pub fn with_runtime_injection(mut self, injection: RuntimeInjection) -> Self {
    self.runtime_injection = injection;
    self
  }

  pub fn with_unstable_module_resolution(
    mut self,
    resolution: CheckModuleResolution,
  ) -> Self {
    self.core.unstable_module_resolution = resolution;
    self
  }
}

impl Default for StyleXOptions {
  fn default() -> Self {
    StyleXOptions {
      core: CoreStyleXOptions::default(),
      runtime_injection: RuntimeInjection::Boolean(false),
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
        },
        RuntimeInjection::Boolean(false) => RuntimeInjection::Boolean(false),
        RuntimeInjection::Regular(path) => RuntimeInjection::Regular(path),
        RuntimeInjection::Named(named) => RuntimeInjection::Named(named),
      },
      None => RuntimeInjection::Boolean(false),
    };

    StyleXOptions {
      core: CoreStyleXOptions {
        style_resolution: options
          .style_resolution
          .unwrap_or(StyleResolution::PropertySpecificity),
        property_validation_mode: options
          .property_validation_mode
          .unwrap_or(PropertyValidationMode::Silent),
        enable_font_size_px_to_rem: options.enable_font_size_px_to_rem.unwrap_or(false),
        class_name_prefix: options.class_name_prefix.unwrap_or("x".to_string()),
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
        sx_prop_name: match options.sx_prop_name {
          None => Some("sx".to_string()),
          Some(SxPropNameParam::Disabled) => None,
          Some(SxPropNameParam::Enabled(name)) => Some(name),
        },
        env: options.env.unwrap_or_default(),
        debug_file_path: options.debug_file_path,
      },
      runtime_injection,
    }
  }
}
