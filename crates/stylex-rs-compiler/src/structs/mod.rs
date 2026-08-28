use napi::JsObject;
use napi_derive::napi;
use rustc_hash::FxHashMap;
use stylex_enums::{
  property_validation_mode::PropertyValidationMode as StylexPropertyValidationMode,
  style_resolution::StyleResolution, sx_prop_name_param::SxPropNameParam,
};
use stylex_structures::{
  ceiling::Ceiling,
  evaluation_depth::MAX_EVALUATION_DEPTH,
  fold_ceilings::{MAX_FOLDED_CHARACTERS, MAX_FOLDED_ENTRIES},
  named_import_source::{ImportSources, NamedImportSource, RuntimeInjection},
  stylex_options::{ModuleResolution, ModuleResolutionKind, StyleXOptionsParams},
};

use crate::enums::{
  ConfiguredCeiling, ImportSourceUnion, PropertyValidationMode, RuntimeInjectionUnion, SourceMaps,
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
  /// Embed the original source text in the emitted map's `sourcesContent`.
  /// Defaults to `true` so tooling that reads the map (Chrome DevTools via
  /// Next.js' `eval-source-map`, in particular) can render the authored file
  /// without re-fetching `sources[0]`. Set to `false` for production maps
  /// where size matters, or where the source shouldn't ship with the map.
  pub inline_sources_content: Option<bool>,
  /// Emit column positions in the source map's `mappings`. Defaults to `true`,
  /// which maps individual expressions rather than whole lines. Set to `false`
  /// for smaller, line-granularity maps (the `cheap-*` webpack devtools).
  pub emit_source_map_columns: Option<bool>,
  /// JSON source map for the incoming `code`, produced by earlier tooling
  /// (e.g. a macro loader). When provided, debug source-map annotations are
  /// mapped back to the original authored file and the emitted source map is
  /// chained onto it.
  pub input_source_map: Option<String>,
  #[napi(ts_type = "'throw' | 'warn' | 'silent'")]
  pub property_validation_mode: Option<PropertyValidationMode>,
  /// How many levels the compiler will descend into a nested expression before
  /// refusing to evaluate it, guarding against a generated file deep enough to
  /// exhaust the stack. Counted in evaluation steps rather than in levels of
  /// source nesting -- a member read costs two, a parenthesis costs none -- so
  /// raise it by measuring rather than by counting brackets. Absent means the
  /// `STYLEX_MAX_EVALUATION_DEPTH` environment variable decides, and failing
  /// that the built-in default. A number that is not a usable count -- a
  /// fraction, `NaN`, an infinity, a negative, or one past the cap -- is
  /// refused where it is written rather than quietly becoming something else.
  #[napi(ts_type = "number")]
  pub max_evaluation_depth: Option<ConfiguredCeiling>,
  /// How many UTF-16 code units of string one compile-time fold may build or
  /// carry, guarding against a mistyped repeat count that agrees with the
  /// language and reaches gigabytes of resident memory. Bounds a resolved value
  /// on the way into the fold and the answer on the way back. Absent means the
  /// `STYLEX_MAX_FOLDED_CHARACTERS` environment variable decides, and failing
  /// that the built-in default. A value that is not a usable count is refused
  /// for the reason `maxEvaluationDepth`'s is.
  #[napi(ts_type = "number")]
  pub max_folded_characters: Option<ConfiguredCeiling>,
  /// How many array elements and object properties one compile-time fold may
  /// build or carry. Separate from `maxFoldedCharacters` because a bounded
  /// string can still become one element per code unit, which costs far more as
  /// a syntax tree than it did as text. Absent means the
  /// `STYLEX_MAX_FOLDED_ENTRIES` environment variable decides, and failing that
  /// the built-in default. A value that is not a usable count is refused for
  /// the same reason.
  #[napi(ts_type = "number")]
  pub max_folded_entries: Option<ConfiguredCeiling>,
  /// Compile-time constants and functions accessible via `stylex.env`.
  #[napi(ts_type = "Record<string, any>")]
  pub env: Option<JsObject>,
  /// Optional function or string to transform file paths used in debug class
  /// names / source maps.
  #[napi(ts_type = "((filePath: string) => string) | string | undefined")]
  pub debug_file_path: Option<napi::UnknownRef>,
  /// The prop name to use as the `sx` shorthand (default: `"sx"`). Set to
  /// `false` to disable.
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

    let unstable_module_resolution = val
      .unstable_module_resolution
      .map(|res| {
        let kind = serde_plain::from_str::<ModuleResolutionKind>(&res.r#type).map_err(|e| {
          napi::Error::from_reason(format!("Failed to parse module resolution type: {}", e))
        })?;

        Ok::<ModuleResolution, napi::Error>(ModuleResolution {
          kind,
          root_dir: res.root_dir,
          theme_file_extension: res.theme_file_extension,
        })
      })
      .transpose()?;

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
      max_evaluation_depth: as_ceiling(
        val.max_evaluation_depth,
        &MAX_EVALUATION_DEPTH,
        "maxEvaluationDepth",
      )?,
      max_folded_characters: as_ceiling(
        val.max_folded_characters,
        &MAX_FOLDED_CHARACTERS,
        "maxFoldedCharacters",
      )?,
      max_folded_entries: as_ceiling(
        val.max_folded_entries,
        &MAX_FOLDED_ENTRIES,
        "maxFoldedEntries",
      )?,
      env: None, // Parsed separately via parse_env_object since it needs napi::Env
      debug_file_path: None, // Parsed separately via parse_debug_file_path since it needs napi::Env
    })
  }
}

/// One configured ceiling as the compiler reads it, or a refusal naming the
/// option that was written.
///
/// A ceiling is a count between one and the cap its own declaration sets, and
/// every other spelling used to be read as unset -- so a project that mistyped
/// one, or asked for more than the compiler will reserve, silently got the
/// default or the cap and was never told. Both are refused here instead, where
/// the option's name is still known and an author can find the line that wrote
/// it.
///
/// The environment variables are deliberately not held to this. They are an
/// escape hatch shared by every build on a machine, and one that failed a build
/// when mistyped would be a worse one, so a stray value there still falls back
/// to the default. Nor is a Rust caller that builds the options itself: it has
/// no written line to name, and `Ceiling::resolve` answers it as before.
fn as_ceiling(
  configured: Option<ConfiguredCeiling>,
  ceiling: &Ceiling,
  option: &str,
) -> Result<Option<usize>, napi::Error> {
  let Some(configured) = configured else {
    return Ok(None);
  };

  let usable = 1.0..=(ceiling.limit as f64);

  match configured {
    // `fract` answers `NaN` for `NaN` and for either infinity, so the whole-
    // number test refuses all three before the range is asked about.
    ConfiguredCeiling::Number(value) if value.fract() == 0.0 && usable.contains(&value) => {
      Ok(Some(value as usize))
    },
    ConfiguredCeiling::Number(value) => Err(napi::Error::from_reason(format!(
      "{option} must be a whole number between 1 and {}, but {} was configured.",
      ceiling.limit,
      as_javascript_writes_it(value)
    ))),
    ConfiguredCeiling::NotANumber => Err(napi::Error::from_reason(format!(
      "{option} must be a whole number between 1 and {}.",
      ceiling.limit
    ))),
  }
}

/// A refused number spelled the way the author wrote it.
///
/// Rust prints the two infinities as `inf` and `-inf`, which is not what a
/// reader typed or what the message should hand back to them.
fn as_javascript_writes_it(value: f64) -> String {
  // Guards rather than `f64::INFINITY` patterns: a float is not a pattern the
  // language wants matched on, and the two readings would drift apart.
  if value.is_nan() {
    "NaN".to_owned()
  } else if value.is_infinite() {
    if value.is_sign_positive() {
      "Infinity".to_owned()
    } else {
      "-Infinity".to_owned()
    }
  } else {
    value.to_string()
  }
}

#[cfg(test)]
#[path = "../tests/structs_tests.rs"]
mod tests;
