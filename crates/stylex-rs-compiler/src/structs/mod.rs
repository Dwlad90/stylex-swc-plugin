use napi::{JsObject, UnknownRef};
use napi_derive::napi;
use rustc_hash::FxHashMap;
use stylex_enums::{
  property_validation_mode::PropertyValidationMode as StylexPropertyValidationMode,
  style_resolution::StyleResolution, sx_prop_name_param::SxPropNameParam,
};
use stylex_structures::{
  named_import_source::{ImportSources, NamedImportSource, RuntimeInjection},
  stylex_options::{ModuleResolution, StyleXOptionsParams},
};

use crate::enums::{
  ImportSourceUnion, PropertyValidationMode, RuntimeInjectionUnion, SourceMaps,
  StyleXModuleResolution, SxPropNameUnion,
};

#[napi(object)]
pub struct StyleXOptions {
  #[napi(ts_type = "'application-order' | 'property-specificity' | 'legacy-expand-shorthands'")]
  pub style_resolution: Option<String>,
  pub enable_font_size_px_to_rem: Option<bool>,
  #[napi(ts_type = "boolean | string")]
  pub runtime_injection: Option<RuntimeInjectionUnion>,
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
  pub inject_stylex_side_effects: Option<bool>,
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
  #[napi(ts_type = "'throw' | 'warn' | 'silent'")]
  pub property_validation_mode: Option<PropertyValidationMode>,
  /// Compile-time constants and functions accessible via `stylex.env`.
  #[napi(ts_type = "Record<string, any>")]
  pub env: Option<JsObject>,
  /// Optional function or string to transform file paths used in debug class names / source maps.
  #[napi(ts_type = "((filePath: string) => string) | string | undefined")]
  pub debug_file_path: Option<napi::UnknownRef>,
  /// The prop name to use as the `sx` shorthand (default: `"sx"`). Set to `false` to disable.
  #[napi(ts_type = "string | false")]
  pub sx_prop_name: Option<SxPropNameUnion>,
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

    let runtime_injection: Option<RuntimeInjection> = val.runtime_injection.map(|ri| match ri {
      RuntimeInjectionUnion::Boolean(b) => RuntimeInjection::Boolean(b),
      RuntimeInjectionUnion::Regular(s) => RuntimeInjection::Regular(s),
    });

    let property_validation_mode: Option<StylexPropertyValidationMode> =
      val.property_validation_mode.map(|pvm| match pvm {
        PropertyValidationMode::Throw => StylexPropertyValidationMode::Throw,
        PropertyValidationMode::Warn => StylexPropertyValidationMode::Warn,
        PropertyValidationMode::Silent => StylexPropertyValidationMode::Silent,
      });

    let sx_prop_name: Option<SxPropNameParam> = val.sx_prop_name.map(|spn| match spn {
      SxPropNameUnion::Disabled => SxPropNameParam::Disabled,
      SxPropNameUnion::Name(s) => SxPropNameParam::Enabled(s),
    });

    Ok(StyleXOptionsParams {
      style_resolution,
      enable_font_size_px_to_rem: val.enable_font_size_px_to_rem,
      runtime_injection,
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
      inject_stylex_side_effects: val.inject_stylex_side_effects,
      aliases: val.aliases,
      unstable_module_resolution,
      sx_prop_name,
      property_validation_mode,
      env: None, // Parsed separately via parse_env_object since it needs napi::Env
      debug_file_path: None, // Parsed separately via parse_debug_file_path since it needs napi::Env
    })
  }
}
