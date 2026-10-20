//! The native half of `processStylexRules`.
//!
//! `src/process-stylex-rules/index.ts` owns the public function. Its gate
//! (`fast-path-gate.ts`) proves that every read the reference would make is a
//! plain read, and packs what the reference reads into one string and three
//! typed arrays. This module unpacks them into rules and assembles the sheet.
//! It answers `null` wherever the reference throws, and wherever the sheet
//! would hold text that Rust cannot hold; the caller then runs the vendored
//! reference, which throws the reference's own error or writes its sheet.

use napi::bindgen_prelude::{Float64Array, Uint8Array, Uint32Array};
use napi_derive::napi;
use stylex_js::string::split_at_utf16_lengths;
use stylex_js::value::JsValue;
use stylex_stylesheet::collation::{CLDR_DATA_VERSION, ICU_DATA_VERSION, UNICODE_DATA_VERSION};
use stylex_stylesheet::process::{
  ClassName, LayerOptions, ProcessStylexRulesConfig, Rule, RuleStyle, process_stylex_rules,
};
use stylex_stylesheet::width_sort_key::media_query_width_sort_key;

/// What a packed rule is, one byte per rule. `fast-path-gate.ts` writes the
/// byte with the `RULE_KIND_*` constants below.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub(crate) enum RuleKind {
  /// A style rule with an LTR text only.
  Style = 0,
  /// A style rule with an LTR and an RTL text.
  StyleWithRtl = 1,
  /// A `defineConsts` rule: its class name, `String(constKey)`, `String(constVal)`.
  Constant = 2,
}

/// [`RuleKind::Style`], for `fast-path-gate.ts`: the two halves read one
/// definition.
#[napi]
pub const RULE_KIND_STYLE: u8 = RuleKind::Style as u8;
/// [`RuleKind::StyleWithRtl`], for `fast-path-gate.ts`.
#[napi]
pub const RULE_KIND_STYLE_WITH_RTL: u8 = RuleKind::StyleWithRtl as u8;
/// [`RuleKind::Constant`], for `fast-path-gate.ts`.
#[napi]
pub const RULE_KIND_CONSTANT: u8 = RuleKind::Constant as u8;

impl TryFrom<u8> for RuleKind {
  type Error = ();

  fn try_from(byte: u8) -> Result<Self, ()> {
    match byte {
      RULE_KIND_STYLE => Ok(RuleKind::Style),
      RULE_KIND_STYLE_WITH_RTL => Ok(RuleKind::StyleWithRtl),
      RULE_KIND_CONSTANT => Ok(RuleKind::Constant),
      _ => Err(()),
    }
  }
}

/// The config, as `fast-path-gate.ts` reads it from the reference's.
#[napi(object)]
pub struct NativeConfig {
  pub use_layers: bool,
  pub layers_before: Vec<String>,
  pub layers_after: Vec<String>,
  /// `String(prefix)` for a truthy prefix, else empty.
  pub layer_prefix: String,
  #[napi(js_name = "enableLTRRTLComments")]
  pub enable_ltr_rtl_comments: bool,
  pub legacy_disable_layers: bool,
  pub use_legacy_classnames_sort: bool,
}

impl From<NativeConfig> for ProcessStylexRulesConfig {
  fn from(config: NativeConfig) -> Self {
    ProcessStylexRulesConfig {
      use_layers: config.use_layers.then(|| LayerOptions {
        before: config.layers_before,
        after: config.layers_after,
        prefix: (!config.layer_prefix.is_empty()).then_some(config.layer_prefix),
      }),
      enable_ltr_rtl_comments: config.enable_ltr_rtl_comments,
      legacy_disable_layers: config.legacy_disable_layers,
      use_legacy_classnames_sort: config.use_legacy_classnames_sort,
    }
  }
}

/// The rules that `fast-path-gate.ts` packed, or `None` when the four parts
/// do not fit together. Only the gate packs them, so a misfit is a fault in
/// the gate; the caller then runs the reference rather than guess.
pub(crate) fn unpack_rules(
  texts: &str,
  lengths: &[u32],
  kinds: &[u8],
  priorities: &[f64],
) -> Option<Vec<Rule>> {
  if kinds.len() != priorities.len() {
    return None;
  }

  let mut texts = split_at_utf16_lengths(texts, lengths)?.into_iter();
  let mut next = || texts.next().map(JsValue::from);
  let mut rules = Vec::with_capacity(kinds.len());

  for (&kind, &priority) in kinds.iter().zip(priorities) {
    let class_name = ClassName::from(next()?);
    let style = match RuleKind::try_from(kind).ok()? {
      RuleKind::Style => RuleStyle {
        ltr: next()?,
        ..RuleStyle::default()
      },
      RuleKind::StyleWithRtl => RuleStyle {
        ltr: next()?,
        rtl: next()?,
        ..RuleStyle::default()
      },
      RuleKind::Constant => RuleStyle {
        const_key: next()?,
        const_val: next()?,
        ..RuleStyle::default()
      },
    };

    rules.push(Rule::new(class_name, style, priority));
  }

  // Every text is read.
  next().is_none().then_some(rules)
}

/// The sheet for the rules and config `fast-path-gate.ts` packed, or `null`
/// where the caller must run the reference.
#[napi(ts_return_type = "string | null")]
pub fn process_stylex_rules_native(
  texts: String,
  lengths: Uint32Array,
  kinds: Uint8Array,
  priorities: Float64Array,
  config: NativeConfig,
) -> Option<String> {
  let rules = unpack_rules(&texts, &lengths, &kinds, &priorities)?;

  process_stylex_rules(&rules, &config.into()).ok()
}

/// One `min-width` or `max-width` bound in px, as upstream's
/// `mediaQueryWidthSortKey` answers it.
#[napi(object)]
pub struct MediaQueryWidthBound {
  #[napi(ts_type = "'min' | 'max'")]
  pub kind: String,
  pub value: f64,
}

/// `mediaQueryWidthSortKey(prelude)`: the single width bound an `@media`
/// prelude sorts by, or `null` when it has none. The vendored reference calls
/// it in place of the parser that the plugin bundles.
#[napi]
pub fn media_query_width_bound(prelude: String) -> Option<MediaQueryWidthBound> {
  media_query_width_sort_key(&prelude).map(|bound| MediaQueryWidthBound {
    kind: bound.kind.as_str().to_string(),
    value: bound.value,
  })
}

/// The ICU data the native sort was built with.
#[napi(object)]
pub struct CollationDataVersion {
  /// The ICU major version, as `process.versions.icu` spells it before the dot.
  pub icu: String,
  /// The Unicode version, as `process.versions.unicode` spells it.
  pub unicode: String,
  /// The CLDR major version, as `process.versions.cldr` spells it before the dot.
  pub cldr: String,
}

/// The ICU data the native sort was built with, which `collation.ts` compares
/// with the data of this Node.
#[napi]
pub fn collation_data_version() -> CollationDataVersion {
  CollationDataVersion {
    icu: ICU_DATA_VERSION.to_string(),
    unicode: UNICODE_DATA_VERSION.to_string(),
    cldr: CLDR_DATA_VERSION.to_string(),
  }
}

#[cfg(test)]
#[path = "tests/process_stylex_rules_tests.rs"]
mod tests;
