use napi::JsObject;
use napi_derive::napi;
use rustc_hash::FxHashMap;
use stylex_shared::shared::structures::{
  named_import_source::{ImportSources, NamedImportSource},
  stylex_options::{ModuleResolution, StyleResolution, StyleXOptionsParams},
};

use crate::enums::{ImportSourceUnion, SourceMaps, StyleXModuleResolution};

#[derive(Debug)]
#[napi(object)]
pub struct StyleXOptions {
  pub style_resolution: Option<String>,
  pub enable_font_size_px_to_rem: Option<bool>,
  pub runtime_injection: Option<bool>,
  pub class_name_prefix: Option<String>,
  #[napi(ts_type = "Record<string, string>")]
  pub defined_stylex_css_variables: Option<FxHashMap<String, String>>,
  #[napi(ts_type = "(string | { as: string, from: string })[]")]
  pub import_sources: Option<Vec<ImportSourceUnion>>,
  pub treeshake_compensation: Option<bool>,
  pub enable_inlined_conditional_merge: Option<bool>,
  pub enable_logical_styles_polyfill: Option<bool>,
  pub dev: Option<bool>,
  pub test: Option<bool>,
  pub debug: Option<bool>,
  pub enable_debug_class_names: Option<bool>,
  pub enable_debug_data_prop: Option<bool>,
  pub enable_dev_class_names: Option<bool>,
  pub enable_minified_keys: Option<bool>,
  #[napi(ts_type = "Record<string, string[]>")]
  pub aliases: Option<FxHashMap<String, Vec<String>>>,
  #[napi(js_name = "unstable_moduleResolution")]
  pub unstable_module_resolution: Option<StyleXModuleResolution>,
  pub source_map: Option<SourceMaps>,
}

#[napi(object)]
pub struct StyleXMetadata {
  #[napi(ts_type = "([string, { ltr: string; rtl?: null | string }, number])[]")]
  pub stylex: Vec<JsObject>,
}

#[napi(object)]
pub struct StyleXTransformResult {
  pub code: String,
  pub metadata: StyleXMetadata,
  pub map: Option<String>,
}

impl From<StyleXOptions> for StyleXOptionsParams {
  fn from(val: StyleXOptions) -> Self {
    let style_resolution: Option<StyleResolution> = match val.style_resolution {
      Some(style_resolution) => serde_json::from_str(&style_resolution).ok(),
      None => None,
    };

    let import_sources: Option<Vec<ImportSources>> = val.import_sources.map(|import_sources| {
      import_sources
        .into_iter()
        .map(|source| match source {
          ImportSourceUnion::Regular(s) => ImportSources::Regular(s),
          ImportSourceUnion::Named(named) => ImportSources::Named(NamedImportSource {
            r#as: named.r#as,
            from: named.from,
          }),
        })
        .collect()
    });

    let unstable_module_resolution = val.unstable_module_resolution.map(|res| ModuleResolution {
      r#type: res.r#type,
      root_dir: res.root_dir,
      theme_file_extension: res.theme_file_extension,
    });

    StyleXOptionsParams {
      style_resolution,
      enable_font_size_px_to_rem: val.enable_font_size_px_to_rem,
      runtime_injection: val.runtime_injection,
      class_name_prefix: val.class_name_prefix,
      defined_stylex_css_variables: val.defined_stylex_css_variables,
      import_sources,
      treeshake_compensation: val.treeshake_compensation,
      enable_inlined_conditional_merge: val.enable_inlined_conditional_merge,
      enable_logical_styles_polyfill: val.enable_logical_styles_polyfill,
      dev: val.dev,
      test: val.test,
      debug: val.debug.or(val.dev),
      enable_debug_class_names: val.enable_debug_class_names,
      enable_debug_data_prop: val.enable_debug_data_prop,
      enable_dev_class_names: val.enable_dev_class_names,
      enable_minified_keys: val.enable_minified_keys,
      aliases: val.aliases,
      unstable_module_resolution,
    }
  }
}
