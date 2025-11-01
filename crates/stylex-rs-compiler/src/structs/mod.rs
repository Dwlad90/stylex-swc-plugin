use napi::{JsObject, UnknownRef};
use napi_derive::napi;
use rustc_hash::FxHashMap;
use stylex_shared::shared::structures::{
  named_import_source::{ImportSources, NamedImportSource},
  stylex_options::{ModuleResolution, StyleResolution, StyleXOptionsParams},
};

use crate::enums::{ImportSourceUnion, SourceMaps, StyleXModuleResolution};

#[napi(object)]
pub struct StyleXOptions {
  #[napi(ts_type = "'application-order' | 'property-specificity' | 'legacy-expand-shorthands'")]
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
  pub enable_media_query_order: Option<bool>,
  pub enable_logical_styles_polyfill: Option<bool>,
  pub enable_legacy_value_flipping: Option<bool>,
  #[napi(js_name = "enableLTRRTLComments")]
  pub enable_ltr_rtl_comments: Option<bool>,
  pub legacy_disable_layers: Option<bool>,
  pub dev: Option<bool>,
  pub test: Option<bool>,
  pub debug: Option<bool>,
  pub enable_debug_class_names: Option<bool>,
  pub enable_debug_data_prop: Option<bool>,
  pub enable_dev_class_names: Option<bool>,
  pub enable_minified_keys: Option<bool>,
  pub use_real_file_for_source: Option<bool>,
  #[napi(ts_type = "Record<string, string[]>")]
  pub aliases: Option<FxHashMap<String, Vec<String>>>,
  #[napi(js_name = "unstable_moduleResolution")]
  pub unstable_module_resolution: Option<StyleXModuleResolution>,
  pub source_map: Option<SourceMaps>,
  #[napi(ts_type = "Array<string | RegExp>")]
  pub include: Option<Vec<UnknownRef>>,
  #[napi(ts_type = "Array<string | RegExp>")]
  pub exclude: Option<Vec<UnknownRef>>,
  #[napi(ts_type = "Array<[string, Record<string, any>]>")]
  pub swc_plugins: Option<Vec<UnknownRef>>,
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

impl TryFrom<StyleXOptions> for StyleXOptionsParams {
  type Error = napi::Error;
  fn try_from(val: StyleXOptions) -> Result<Self, Self::Error> {
    let style_resolution: Option<StyleResolution> = val
      .style_resolution
      .map(|sr| {
        serde_plain::from_str(&sr)
          .map_err(|e| napi::Error::from_reason(format!("Failed to parse style resolution: {}", e)))
      })
      .transpose()?;

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

    Ok(StyleXOptionsParams {
      style_resolution,
      enable_font_size_px_to_rem: val.enable_font_size_px_to_rem,
      runtime_injection: val.runtime_injection,
      class_name_prefix: val.class_name_prefix,
      defined_stylex_css_variables: val.defined_stylex_css_variables,
      import_sources,
      treeshake_compensation: val.treeshake_compensation,
      enable_inlined_conditional_merge: val.enable_inlined_conditional_merge,
      enable_media_query_order: val.enable_media_query_order,
      enable_logical_styles_polyfill: val.enable_logical_styles_polyfill,
      enable_legacy_value_flipping: val.enable_legacy_value_flipping,
      enable_ltr_rtl_comments: val.enable_ltr_rtl_comments,
      use_real_file_for_source: val.use_real_file_for_source,
      dev: val.dev,
      test: val.test,
      debug: val.debug.or(val.dev),
      enable_debug_class_names: val.enable_debug_class_names,
      enable_debug_data_prop: val.enable_debug_data_prop,
      enable_dev_class_names: val.enable_dev_class_names,
      enable_minified_keys: val.enable_minified_keys,
      aliases: val.aliases,
      unstable_module_resolution,
    })
  }
}
