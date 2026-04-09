use indexmap::IndexMap;
use rustc_hash::FxHashMap;
use serde::Deserialize;

use stylex_enums::property_validation_mode::PropertyValidationMode;
use stylex_enums::style_resolution::StyleResolution;

use crate::{
  named_import_source::ImportSources,
  stylex_env::{EnvEntry, JSFunction},
  stylex_options::CheckModuleResolution,
};

/// Shared configuration fields between `StyleXOptions` and `StyleXStateOptions`.
///
/// Both option structs embed this via a `core` field and implement `Deref`/`DerefMut`
/// so all fields are transparently accessible.
#[derive(Deserialize, Clone, Debug)]
pub struct CoreStyleXOptions {
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
  #[allow(dead_code)]
  pub enable_ltr_rtl_comments: bool,
  pub enable_minified_keys: bool,
  pub enable_font_size_px_to_rem: bool,
  pub use_real_file_for_source: bool,
  pub class_name_prefix: String,
  pub style_resolution: StyleResolution,
  pub import_sources: Vec<ImportSources>,
  pub treeshake_compensation: bool,
  pub inject_stylex_side_effects: bool,
  pub aliases: Option<FxHashMap<String, Vec<String>>>,
  pub unstable_module_resolution: CheckModuleResolution,
  pub sx_prop_name: Option<String>,
  #[serde(skip)]
  pub env: IndexMap<String, EnvEntry>,
  #[serde(skip)]
  pub debug_file_path: Option<JSFunction>,
}

impl Default for CoreStyleXOptions {
  fn default() -> Self {
    CoreStyleXOptions {
      dev: false,
      test: false,
      debug: false,
      property_validation_mode: PropertyValidationMode::Silent,
      enable_debug_class_names: false,
      enable_debug_data_prop: true,
      enable_dev_class_names: false,
      enable_inlined_conditional_merge: true,
      enable_media_query_order: true,
      enable_logical_styles_polyfill: false,
      enable_legacy_value_flipping: false,
      enable_ltr_rtl_comments: false,
      enable_minified_keys: true,
      enable_font_size_px_to_rem: false,
      use_real_file_for_source: true,
      class_name_prefix: "x".to_string(),
      style_resolution: StyleResolution::PropertySpecificity,
      import_sources: vec![],
      treeshake_compensation: false,
      inject_stylex_side_effects: false,
      aliases: None,
      unstable_module_resolution: CheckModuleResolution::default(),
      sx_prop_name: Some("sx".to_string()),
      env: IndexMap::new(),
      debug_file_path: None,
    }
  }
}
