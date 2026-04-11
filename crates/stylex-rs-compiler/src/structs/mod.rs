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

#[cfg(test)]
mod tests {
  use super::*;

  fn empty_options() -> StyleXOptions {
    StyleXOptions {
      style_resolution: None,
      enable_font_size_px_to_rem: None,
      runtime_injection: None,
      class_name_prefix: None,
      defined_stylex_css_variables: None,
      import_sources: None,
      treeshake_compensation: None,
      enable_inlined_conditional_merge: None,
      enable_media_query_order: None,
      enable_logical_styles_polyfill: None,
      enable_legacy_value_flipping: None,
      enable_ltr_rtl_comments: None,
      legacy_disable_layers: None,
      dev: None,
      test: None,
      debug: None,
      enable_debug_class_names: None,
      enable_debug_data_prop: None,
      enable_dev_class_names: None,
      enable_minified_keys: None,
      inject_stylex_side_effects: None,
      use_real_file_for_source: None,
      aliases: None,
      unstable_module_resolution: None,
      source_map: None,
      include: None,
      exclude: None,
      swc_plugins: None,
      property_validation_mode: None,
      env: None,
      debug_file_path: None,
      sx_prop_name: None,
    }
  }

  #[test]
  fn try_from_maps_supported_fields() {
    let options = StyleXOptions {
      style_resolution: Some("application-order".to_string()),
      runtime_injection: Some(RuntimeInjectionUnion::Regular("inject-path".to_string())),
      import_sources: Some(vec![
        ImportSourceUnion::Regular("@stylexjs/stylex".to_string()),
        ImportSourceUnion::Named(NamedImportSource {
          r#as: "sx".to_string(),
          from: "stylex".to_string(),
        }),
      ]),
      unstable_module_resolution: Some(StyleXModuleResolution {
        r#type: "commonJS".to_string(),
        root_dir: Some("/root".to_string()),
        theme_file_extension: Some(".stylex".to_string()),
      }),
      property_validation_mode: Some(PropertyValidationMode::Warn),
      sx_prop_name: Some(SxPropNameUnion::Name("sx".to_string())),
      ..empty_options()
    };

    let parsed = StyleXOptionsParams::try_from(options).unwrap();

    assert!(parsed.style_resolution.is_some());
    assert_eq!(
      parsed.runtime_injection,
      Some(RuntimeInjection::Regular("inject-path".to_string()))
    );
    assert_eq!(parsed.import_sources.as_ref().map(Vec::len), Some(2));
    assert_eq!(
      parsed
        .unstable_module_resolution
        .as_ref()
        .map(|m| m.r#type.as_str()),
      Some("commonJS")
    );
    assert_eq!(
      parsed.property_validation_mode,
      Some(StylexPropertyValidationMode::Warn)
    );
    assert!(matches!(
      parsed.sx_prop_name,
      Some(SxPropNameParam::Enabled(ref value)) if value == "sx"
    ));
  }

  #[test]
  fn try_from_returns_error_for_invalid_style_resolution() {
    let options = StyleXOptions {
      style_resolution: Some("not-a-style-resolution".to_string()),
      ..empty_options()
    };

    match StyleXOptionsParams::try_from(options) {
      Ok(_) => panic!("expected style resolution parsing to fail"),
      Err(error) => assert!(
        error
          .to_string()
          .contains("Failed to parse style resolution")
      ),
    }
  }

  #[test]
  fn try_from_uses_dev_for_debug_when_debug_not_set() {
    let options = StyleXOptions {
      dev: Some(true),
      debug: None,
      ..empty_options()
    };

    let parsed = StyleXOptionsParams::try_from(options).unwrap();
    assert_eq!(parsed.debug, Some(true));
  }

  #[test]
  fn try_from_maps_runtime_injection_bool_and_sx_disabled() {
    let options = StyleXOptions {
      runtime_injection: Some(RuntimeInjectionUnion::Boolean(true)),
      sx_prop_name: Some(SxPropNameUnion::Disabled),
      ..empty_options()
    };

    let parsed = StyleXOptionsParams::try_from(options).unwrap();
    assert_eq!(
      parsed.runtime_injection,
      Some(RuntimeInjection::Boolean(true))
    );
    assert!(matches!(
      parsed.sx_prop_name,
      Some(SxPropNameParam::Disabled)
    ));
  }
}
