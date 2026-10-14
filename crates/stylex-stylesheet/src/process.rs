//! `processStylexRules`: the rules a compile emits, assembled into one sheet.
//!
//! The reference is `processStylexRules` and the helpers around it in
//! `packages/@stylexjs/babel-plugin/src/index.js` at the commit `CONTEXT.md`
//! names, followed function by function and in its order:
//! `getLogicalFloatVars`, `processStylexRules` (with `resolveConstant` inside
//! it), `addAncestorSelector`, `addSpecificityLevel`. The reference's one
//! function body is split here along its own blocks (the constants, the
//! substitution, the sort, the groups, the output), so that a change upstream
//! maps onto one function. Where the reference leans on a JavaScript built-in
//! whose behaviour shapes the output -- `localeCompare`,
//! `String.prototype.replace` with a string replacement, `trim`,
//! `Array.prototype.sort`'s stability, `new Map(...)`'s keeping of the first
//! position under the last value, `slice(-1)` -- the port names the built-in
//! and does what it does.

use std::borrow::Cow;
use std::cmp::Ordering;
use std::fmt;

use indexmap::IndexMap;
use rustc_hash::FxHashMap;
use serde::Deserialize;
use serde::de::{self, Deserializer, IgnoredAny, MapAccess, SeqAccess, Visitor};
use stylex_constants::constants::common::{LOGICAL_FLOAT_END_VAR, LOGICAL_FLOAT_START_VAR};
use stylex_regex::regex::{CONSTANT_REFERENCE, THEME_ROOT_SELECTOR};
use stylex_utils::number;

use crate::collation::compare as locale_compare;
use crate::width_sort_key::{
  WidthSortKey, at_rule_preludes, compare_width_sort_keys, declared_property_name,
  width_sort_key_for_chain,
};

/// The value a `defineConsts` constant holds. The reference's type says
/// `string | number`; its runtime is wider, a boolean crosses too (a
/// `defineConsts({ d: true })` keeps the kind the author gave it), and
/// substitutes as `String(true)`. A `null` is no constant at all
/// (`constVal != null` fails), and deserializes as an absent one.
#[derive(Clone, Debug, PartialEq, Deserialize)]
#[serde(untagged)]
pub enum ConstantValue {
  Number(f64),
  String(String),
  Bool(bool),
}

impl ConstantValue {
  /// `String(constValue)`: the text a value substitutes as.
  pub(crate) fn to_js_string(&self) -> String {
    match self {
      ConstantValue::String(text) => text.clone(),
      ConstantValue::Bool(flag) => flag.to_string(),
      ConstantValue::Number(value) => number::to_js_string(*value),
    }
  }
}

/// What one rule injects: its LTR text, its RTL text where the two differ,
/// and, for a `defineConsts` rule, the constant it declares. A constant rule
/// may carry no text at all (the reference reads neither field of one), so
/// `ltr` defaults to empty.
#[derive(Clone, Debug, Default, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RuleStyle {
  #[serde(default)]
  pub ltr: String,
  #[serde(default)]
  pub rtl: Option<String>,
  #[serde(default)]
  pub const_key: Option<String>,
  #[serde(default)]
  pub const_val: Option<ConstantValue>,
}

impl RuleStyle {
  /// `ruleObj?.constKey != null && ruleObj?.constVal != null`: whether this
  /// rule declares a constant rather than a style.
  fn constant(&self) -> Option<(&str, &ConstantValue)> {
    match (&self.const_key, &self.const_val) {
      (Some(key), Some(value)) => Some((key.as_str(), value)),
      _ => None,
    }
  }

  /// Whether a text of this style can hold a reference to a constant or a
  /// declaration of one. Every pattern the substitution searches for,
  /// `var(--x)` and `--x:`, holds `--`.
  fn mentions_a_variable(&self) -> bool {
    self.ltr.contains("--") || self.rtl.as_deref().is_some_and(|rtl| rtl.contains("--"))
  }
}

/// One emitted rule: `[className, { ltr, rtl, constKey, constVal }, priority]`,
/// the reference's `Rule` tuple.
#[derive(Clone, Debug, PartialEq, Deserialize)]
pub struct Rule(pub String, pub RuleStyle, pub f64);

impl Rule {
  pub fn class_name(&self) -> &str {
    &self.0
  }

  pub fn style(&self) -> &RuleStyle {
    &self.1
  }

  pub fn priority(&self) -> f64 {
    self.2
  }
}

/// The `useLayers` object form: layers to declare before and after the
/// priority layers, and a prefix for their names. The reference reads each
/// property with `?? <default>`, so `null` is the default here too; a value
/// outside the declared type (a string for `before`, a number for `prefix`)
/// is refused, where the reference's loose reads would spread the string into
/// characters or print the number (`CONTEXT.md`, "Config").
#[derive(Clone, Debug, Default, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LayerOptions {
  #[serde(default, deserialize_with = "null_as_default")]
  pub before: Vec<String>,
  #[serde(default, deserialize_with = "null_as_default")]
  pub after: Vec<String>,
  #[serde(default, deserialize_with = "null_as_default")]
  pub prefix: String,
}

/// `value ?? default`: a `null` reads as the field's default.
fn null_as_default<'de, D: Deserializer<'de>, T: Default + Deserialize<'de>>(
  deserializer: D,
) -> Result<T, D::Error> {
  match Option::<T>::deserialize(deserializer)? {
    Some(value) => Ok(value),
    None => Ok(T::default()),
  }
}

/// The reference's `config` argument. `useLayers: true` is
/// `Some(LayerOptions::default())`; `false`, `null` and an absent key are
/// `None`.
///
/// It deserializes as the reference reads the argument:
/// `typeof config === 'boolean' ? { useLayers: config } : config ?? {}`, then
/// one property read each. A boolean stands for `useLayers`; `null` and every
/// value that is not an object have no properties, so every option is at its
/// default; an object is read property by property, `useLayers` by the
/// `!== false` rule ([`UseLayersVisitor`]) and the three flags by JavaScript
/// truthiness ([`TruthyVisitor`]).
#[derive(Clone, Debug, Default, PartialEq)]
pub struct ProcessStylexRulesConfig {
  pub use_layers: Option<LayerOptions>,
  pub enable_ltr_rtl_comments: bool,
  pub legacy_disable_layers: bool,
  pub use_legacy_classnames_sort: bool,
}

impl From<bool> for ProcessStylexRulesConfig {
  /// `processStylexRules(rules, true)`: a bare boolean is `useLayers`.
  fn from(use_layers: bool) -> Self {
    Self {
      use_layers: use_layers.then(LayerOptions::default),
      ..Self::default()
    }
  }
}

impl<'de> Deserialize<'de> for ProcessStylexRulesConfig {
  fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
    deserializer.deserialize_any(ConfigVisitor)
  }
}

/// The object form of `config`, property by property.
#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct ConfigOptions {
  #[serde(default, deserialize_with = "deserialize_use_layers")]
  use_layers: Option<LayerOptions>,
  // The reference spells the acronyms in capitals, which `camelCase` would
  // not.
  #[serde(
    default,
    rename = "enableLTRRTLComments",
    deserialize_with = "deserialize_truthy"
  )]
  enable_ltr_rtl_comments: bool,
  #[serde(default, deserialize_with = "deserialize_truthy")]
  legacy_disable_layers: bool,
  #[serde(default, deserialize_with = "deserialize_truthy")]
  use_legacy_classnames_sort: bool,
}

impl From<ConfigOptions> for ProcessStylexRulesConfig {
  fn from(options: ConfigOptions) -> Self {
    Self {
      use_layers: options.use_layers,
      enable_ltr_rtl_comments: options.enable_ltr_rtl_comments,
      legacy_disable_layers: options.legacy_disable_layers,
      use_legacy_classnames_sort: options.use_legacy_classnames_sort,
    }
  }
}

/// Drains a sequence the reference reads no element of.
fn ignore_sequence<'de, A: SeqAccess<'de>>(mut sequence: A) -> Result<(), A::Error> {
  while sequence.next_element::<IgnoredAny>()?.is_some() {}
  Ok(())
}

/// Drains a map the reference reads no property of.
fn ignore_map<'de, A: MapAccess<'de>>(mut map: A) -> Result<(), A::Error> {
  while map.next_entry::<IgnoredAny, IgnoredAny>()?.is_some() {}
  Ok(())
}

/// `typeof config === 'boolean' ? { useLayers: config } : config ?? {}`.
pub(crate) struct ConfigVisitor;

impl<'de> Visitor<'de> for ConfigVisitor {
  type Value = ProcessStylexRulesConfig;

  fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
    formatter.write_str("a boolean, null, or the options object")
  }

  fn visit_bool<E: de::Error>(self, use_layers: bool) -> Result<Self::Value, E> {
    Ok(ProcessStylexRulesConfig::from(use_layers))
  }

  fn visit_unit<E: de::Error>(self) -> Result<Self::Value, E> {
    Ok(ProcessStylexRulesConfig::default())
  }

  // A number, a string or an array has none of the properties the reference
  // reads: every option is at its default.
  fn visit_i64<E: de::Error>(self, _: i64) -> Result<Self::Value, E> {
    Ok(ProcessStylexRulesConfig::default())
  }

  fn visit_u64<E: de::Error>(self, _: u64) -> Result<Self::Value, E> {
    Ok(ProcessStylexRulesConfig::default())
  }

  fn visit_f64<E: de::Error>(self, _: f64) -> Result<Self::Value, E> {
    Ok(ProcessStylexRulesConfig::default())
  }

  fn visit_str<E: de::Error>(self, _: &str) -> Result<Self::Value, E> {
    Ok(ProcessStylexRulesConfig::default())
  }

  fn visit_seq<A: SeqAccess<'de>>(self, sequence: A) -> Result<Self::Value, A::Error> {
    ignore_sequence(sequence)?;
    Ok(ProcessStylexRulesConfig::default())
  }

  fn visit_map<A: MapAccess<'de>>(self, map: A) -> Result<Self::Value, A::Error> {
    let options = ConfigOptions::deserialize(de::value::MapAccessDeserializer::new(map))?;
    Ok(options.into())
  }
}

fn deserialize_use_layers<'de, D: Deserializer<'de>>(
  deserializer: D,
) -> Result<Option<LayerOptions>, D::Error> {
  deserializer.deserialize_any(UseLayersVisitor)
}

/// `const rawUseLayers = rawConfig.useLayers ?? false;` and
/// `const useLayers = rawUseLayers !== false;`: `null` and `false` are off;
/// an object is read property by property; every other value is on, with the
/// defaults, since `typeof rawUseLayers === 'object'` fails for it (an array
/// passes that test and has none of the properties, so it is the defaults
/// too).
pub(crate) struct UseLayersVisitor;

impl<'de> Visitor<'de> for UseLayersVisitor {
  type Value = Option<LayerOptions>;

  fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
    formatter.write_str("a boolean, null, or the layer options object")
  }

  fn visit_bool<E: de::Error>(self, flag: bool) -> Result<Self::Value, E> {
    Ok(flag.then(LayerOptions::default))
  }

  fn visit_unit<E: de::Error>(self) -> Result<Self::Value, E> {
    Ok(None)
  }

  fn visit_i64<E: de::Error>(self, _: i64) -> Result<Self::Value, E> {
    Ok(Some(LayerOptions::default()))
  }

  fn visit_u64<E: de::Error>(self, _: u64) -> Result<Self::Value, E> {
    Ok(Some(LayerOptions::default()))
  }

  fn visit_f64<E: de::Error>(self, _: f64) -> Result<Self::Value, E> {
    Ok(Some(LayerOptions::default()))
  }

  fn visit_str<E: de::Error>(self, _: &str) -> Result<Self::Value, E> {
    Ok(Some(LayerOptions::default()))
  }

  fn visit_seq<A: SeqAccess<'de>>(self, sequence: A) -> Result<Self::Value, A::Error> {
    ignore_sequence(sequence)?;
    Ok(Some(LayerOptions::default()))
  }

  fn visit_map<A: MapAccess<'de>>(self, map: A) -> Result<Self::Value, A::Error> {
    let options = LayerOptions::deserialize(de::value::MapAccessDeserializer::new(map))?;
    Ok(Some(options))
  }
}

fn deserialize_truthy<'de, D: Deserializer<'de>>(deserializer: D) -> Result<bool, D::Error> {
  deserializer.deserialize_any(TruthyVisitor)
}

/// A flag the reference reads with `if (flag)`: JavaScript truthiness. `null`,
/// `false`, `0` and `""` are false; every other value is true. (JSON has no
/// `NaN`.)
pub(crate) struct TruthyVisitor;

impl<'de> Visitor<'de> for TruthyVisitor {
  type Value = bool;

  fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
    formatter.write_str("a value read for its truthiness")
  }

  fn visit_bool<E: de::Error>(self, flag: bool) -> Result<Self::Value, E> {
    Ok(flag)
  }

  fn visit_unit<E: de::Error>(self) -> Result<Self::Value, E> {
    Ok(false)
  }

  fn visit_i64<E: de::Error>(self, value: i64) -> Result<Self::Value, E> {
    Ok(value != 0)
  }

  fn visit_u64<E: de::Error>(self, value: u64) -> Result<Self::Value, E> {
    Ok(value != 0)
  }

  fn visit_f64<E: de::Error>(self, value: f64) -> Result<Self::Value, E> {
    Ok(value != 0.0)
  }

  fn visit_str<E: de::Error>(self, value: &str) -> Result<Self::Value, E> {
    Ok(!value.is_empty())
  }

  fn visit_seq<A: SeqAccess<'de>>(self, sequence: A) -> Result<Self::Value, A::Error> {
    ignore_sequence(sequence)?;
    Ok(true)
  }

  fn visit_map<A: MapAccess<'de>>(self, map: A) -> Result<Self::Value, A::Error> {
    ignore_map(map)?;
    Ok(true)
  }
}

/// Why a sheet could not be assembled.
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum ProcessStylexRulesError {
  /// A constant's value refers, through any chain of constants, to itself.
  #[error("circular reference detected for constant {0}")]
  CircularConstant(String),
}

/// The `:root, [dir="ltr"]` / `[dir="rtl"]` block that gives the logical float
/// variables a value, when any rule reads one.
pub(crate) fn get_logical_float_vars<'a>(rules: impl IntoIterator<Item = &'a Rule>) -> String {
  let has_logical_float = rules.into_iter().any(|rule| {
    let ltr = rule.style().ltr.as_str();
    let rtl = rule.style().rtl.as_deref().unwrap_or("");

    ltr.contains(LOGICAL_FLOAT_START_VAR)
      || ltr.contains(LOGICAL_FLOAT_END_VAR)
      || rtl.contains(LOGICAL_FLOAT_START_VAR)
      || rtl.contains(LOGICAL_FLOAT_END_VAR)
  });

  if has_logical_float {
    format!(
      ":root, [dir=\"ltr\"] {{\n  {LOGICAL_FLOAT_START_VAR}: left;\n  {LOGICAL_FLOAT_END_VAR}: right;\n}}\n[dir=\"rtl\"] {{\n  {LOGICAL_FLOAT_START_VAR}: right;\n  {LOGICAL_FLOAT_END_VAR}: left;\n}}\n"
    )
  } else {
    String::new()
  }
}

// ==================== the JavaScript built-ins ====================

/// `String.prototype.replace` and `replaceAll` with a string pattern and a
/// string replacement, as the reference calls them: the replacement is a
/// template in which `$$` is a dollar, `$&` the matched text, `` $` `` the text
/// before the match and `$'` the text after it (ECMA-262 GetSubstitution; a
/// string pattern has no capture groups, so `$1` and `$<name>` stay literal).
/// Upstream `main` has since passed a function instead, which writes the
/// replacement verbatim; that change arrives here with the version carrying it.
pub(crate) fn js_substitution(template: &str, matched: &str, before: &str, after: &str) -> String {
  let mut out = String::with_capacity(template.len());
  let mut characters = template.chars().peekable();

  while let Some(character) = characters.next() {
    if character != '$' {
      out.push(character);
      continue;
    }

    match characters.peek() {
      Some('$') => {
        characters.next();
        out.push('$');
      },
      Some('&') => {
        characters.next();
        out.push_str(matched);
      },
      Some('`') => {
        characters.next();
        out.push_str(before);
      },
      Some('\'') => {
        characters.next();
        out.push_str(after);
      },
      _ => out.push('$'),
    }
  }

  out
}

/// The substitution for the occurrence of `pattern` at `start` of `text`: the
/// template read against that occurrence's surroundings, the text before it
/// and the text after it.
fn substitution_at(text: &str, start: usize, pattern: &str, replacement: &str) -> String {
  js_substitution(
    replacement,
    pattern,
    &text[..start],
    &text[start + pattern.len()..],
  )
}

/// `text.replace(pattern, replacement)`: the first occurrence.
pub(crate) fn js_replace_first(text: &str, pattern: &str, replacement: &str) -> String {
  match text.find(pattern) {
    Some(start) => {
      let mut out = String::with_capacity(text.len() + replacement.len());
      out.push_str(&text[..start]);
      out.push_str(&substitution_at(text, start, pattern, replacement));
      out.push_str(&text[start + pattern.len()..]);
      out
    },
    None => text.to_string(),
  }
}

/// `text.replaceAll(pattern, replacement)`: every occurrence, left to right,
/// each substitution reading its own surroundings in the original text. A text
/// with no occurrence is handed back as it is: the loop below runs once per
/// rule and constant, and nearly every such pair has nothing to replace.
pub(crate) fn js_replace_all<'t>(text: &'t str, pattern: &str, replacement: &str) -> Cow<'t, str> {
  let Some(first) = text.find(pattern) else {
    return Cow::Borrowed(text);
  };

  let mut out = String::with_capacity(text.len());
  let mut rest = &text[first..];
  let mut consumed = first;
  out.push_str(&text[..first]);

  while let Some(offset) = rest.find(pattern) {
    let start = consumed + offset;
    out.push_str(&rest[..offset]);
    out.push_str(&substitution_at(text, start, pattern, replacement));
    rest = &rest[offset + pattern.len()..];
    consumed = start + pattern.len();
  }

  out.push_str(rest);
  Cow::Owned(out)
}

/// The characters `String.prototype.trim` removes: ECMA-262 WhiteSpace (TAB,
/// VT, FF, ZWNBSP and the `Zs` category) and LineTerminator (LF, CR, LS, PS).
/// This differs from Unicode `White_Space`, which `str::trim` removes, at two
/// points: U+FEFF is removed here and kept there, U+0085 is kept here and
/// removed there.
fn is_js_whitespace(character: char) -> bool {
  matches!(
    character,
    '\u{9}' | '\u{A}' | '\u{B}' | '\u{C}' | '\u{D}' | ' ' | '\u{A0}' | '\u{1680}' | '\u{2000}'
      ..='\u{200A}' | '\u{2028}' | '\u{2029}' | '\u{202F}' | '\u{205F}' | '\u{3000}' | '\u{FEFF}'
  )
}

/// `text.trim()`.
pub(crate) fn js_trim(text: &str) -> &str {
  text.trim_matches(is_js_whitespace)
}

/// Where `text.slice(-1)` begins: the start of the last character, 0 for an
/// empty text. JavaScript counts UTF-16 units, so for a character outside the
/// Basic Multilingual Plane (an emoji) it splits the surrogate pair; a `str`
/// cannot hold half a pair, so the port starts at the whole character
/// (`.a😀` splits before the emoji where the reference splits inside it).
fn last_character_start(text: &str) -> usize {
  match text.char_indices().next_back() {
    Some((index, _)) => index,
    None => 0,
  }
}

/// Where `text.slice(text.lastIndexOf('{'))` begins: at the last `{`, or, when
/// there is none and `lastIndexOf` answers -1, at the last character.
fn last_brace_or_last_character(text: &str) -> usize {
  match text.rfind('{') {
    Some(brace) => brace,
    None => last_character_start(text),
  }
}

// ==================== the constants ====================

/// The constants a sheet declares, by their reference text (`var(--key)`), in
/// declaration order: the reference walks `constsMap` in insertion order, and
/// a constant declared twice keeps its first position under its last value,
/// as `Map.prototype.set` keeps them.
pub type OrderedConstants = IndexMap<String, ConstantValue>;

/// `constsMap` and `nonConstantRules`: the constants a sheet declares and the
/// rules that are styles.
fn collect_constants(rules: &[Rule]) -> (OrderedConstants, Vec<&Rule>) {
  let mut constants = OrderedConstants::default();
  let mut non_constant_rules = Vec::new();

  for rule in rules {
    match rule.style().constant() {
      Some((_, value)) => {
        constants.insert(format!("var(--{})", rule.class_name()), value.clone());
      },
      None => non_constant_rules.push(rule),
    }
  }

  (constants, non_constant_rules)
}

/// Every constant resolved against the constants as declared.
fn resolve_constants(
  constants: &OrderedConstants,
) -> Result<OrderedConstants, ProcessStylexRulesError> {
  let mut resolved = OrderedConstants::with_capacity(constants.len());

  for (name, value) in constants {
    resolved.insert(
      name.clone(),
      resolve_constant(value, constants, &mut Vec::new())?,
    );
  }

  Ok(resolved)
}

/// `resolveConstant`: a constant's value with every reference to another
/// constant replaced by that constant's resolved value. A reference to a
/// constant the sheet does not declare stays as written. The search restarts
/// from the front after each replacement (`regex.lastIndex = 0`), so a value a
/// replacement brings in is resolved in turn; a chain that returns to a
/// constant already on it is a circular reference.
pub(crate) fn resolve_constant(
  value: &ConstantValue,
  constants: &OrderedConstants,
  visited: &mut Vec<String>,
) -> Result<ConstantValue, ProcessStylexRulesError> {
  let ConstantValue::String(text) = value else {
    return Ok(value.clone());
  };

  let mut result = text.clone();
  let mut position = 0;

  // Both groups are set in every match (group 0 is the match itself, group 1
  // is not optional), so the chain ends only when no match is left.
  while let Some(captures) = CONSTANT_REFERENCE.captures_at(&result, position)
    && let (Some(whole), Some(name)) = (captures.get(0), captures.get(1))
  {
    let reference = name.as_str().to_string();
    let matched = whole.as_str().to_string();
    let end = whole.end();

    if visited.contains(&reference) {
      return Err(ProcessStylexRulesError::CircularConstant(reference));
    }

    let Some(referenced) = constants.get(format!("var({reference})").as_str()) else {
      position = end;
      continue;
    };

    visited.push(reference.clone());
    let replacement = resolve_constant(referenced, constants, visited)?;
    result = js_replace_first(&result, &matched, &replacement.to_js_string());
    visited.retain(|seen| *seen != reference);
    position = 0;
  }

  Ok(ConstantValue::String(result))
}

// ==================== the substitution ====================

/// The `--<class name>:` to `--<target>:` rename of a declaration of a
/// constant whose value is itself a `var(...)`.
struct Rename {
  from: String,
  to: String,
}

/// What one constant substitutes as, spelled once rather than once per rule.
struct Substitution<'c> {
  /// The reference text a declaration reads: `var(--<class name>)`.
  var_ref: &'c str,
  /// `String(constValue)`: what replaces each reference.
  replacement: String,
  /// The rename that follows the replacement when the value is a `var(...)`.
  rename: Option<Rename>,
}

impl Substitution<'_> {
  /// `original.replaceAll(varRef, replacement)`, then, with a rename,
  /// `original.replaceAll(`${constName}:`, `${targetName}:`)`.
  fn apply(&self, text: &mut String) {
    if let Cow::Owned(replaced) = js_replace_all(text, self.var_ref, &self.replacement) {
      *text = replaced;
    }

    if let Some(rename) = &self.rename
      && let Cow::Owned(renamed) = js_replace_all(text, &rename.from, &rename.to)
    {
      *text = renamed;
    }
  }
}

/// The substitution of every constant, in declaration order.
fn substitutions_of(constants: &OrderedConstants) -> Vec<Substitution<'_>> {
  constants
    .iter()
    .map(|(var_ref, value)| {
      let replacement = value.to_js_string();
      let rename = if replacement.starts_with("var(") && replacement.ends_with(')') {
        let inside = js_trim(&replacement[4..replacement.len() - 1]);
        let target_name = match inside.find(',') {
          Some(comma) => js_trim(&inside[..comma]),
          None => inside,
        };
        let const_name = &var_ref[4..var_ref.len() - 1];

        Some(Rename {
          from: format!("{const_name}:"),
          to: format!("{target_name}:"),
        })
      } else {
        None
      };

      Substitution {
        var_ref: var_ref.as_str(),
        replacement,
        rename,
      }
    })
    .collect()
}

/// Every rule with the constants substituted into its texts. A rule whose
/// texts hold no `--` has nothing to substitute, and most rules are such: it
/// is handed on as it is rather than copied.
fn substitute_constants<'r>(
  rules: Vec<&'r Rule>,
  constants: &OrderedConstants,
) -> Vec<Cow<'r, Rule>> {
  let substitutions = substitutions_of(constants);

  rules
    .into_iter()
    .map(|rule| {
      if !rule.style().mentions_a_variable() {
        return Cow::Borrowed(rule);
      }

      let mut style = rule.style().clone();

      // `Object.keys(styleObj).forEach`: every text the style holds, `ltr` then
      // `rtl`; a value that is not a string (`rtl: null`) is left alone.
      for text in [Some(&mut style.ltr), style.rtl.as_mut()]
        .into_iter()
        .flatten()
      {
        for substitution in &substitutions {
          substitution.apply(text);
        }
      }

      Cow::Owned(Rule(rule.class_name().to_string(), style, rule.priority()))
    })
    .collect()
}

// ==================== the sort ====================

/// The media-query width sort key of every rule, read once through a cache
/// keyed on the at-rule chain rather than at every comparison: parsing is
/// expensive and the comparator runs O(n log n) times, and every rule has a
/// distinct class name, so a key on the whole rule would never hit.
fn width_sort_keys_of(rules: &[Cow<'_, Rule>]) -> Vec<Option<WidthSortKey>> {
  let mut cache: FxHashMap<String, Option<WidthSortKey>> = FxHashMap::default();

  rules
    .iter()
    .map(|rule| {
      let text = rule.style().ltr.as_str();

      if !text.starts_with('@') {
        return None;
      }

      let preludes = at_rule_preludes(text);
      let chain = preludes.join("{");

      if let Some(cached) = cache.get(&chain) {
        return cached.clone();
      }

      let key = width_sort_key_for_chain(&preludes);
      cache.insert(chain, key.clone());
      key
    })
    .collect()
}

/// The comparator of the sort: by priority, then, within a priority, by class
/// name under `use_legacy_classnames_sort`, else by declared property, width
/// key, declaration block and whole text.
fn compare_rules(
  (a, key_a): (&Rule, Option<&WidthSortKey>),
  (b, key_b): (&Rule, Option<&WidthSortKey>),
  use_legacy_classnames_sort: bool,
) -> Ordering {
  let priority_comparison = a
    .priority()
    .partial_cmp(&b.priority())
    .unwrap_or(Ordering::Equal);

  if priority_comparison != Ordering::Equal {
    return priority_comparison;
  }

  if use_legacy_classnames_sort {
    return locale_compare(a.class_name(), b.class_name());
  }

  let rule1 = a.style().ltr.as_str();
  let rule2 = b.style().ltr.as_str();

  let name_comparison =
    locale_compare(declared_property_name(rule1), declared_property_name(rule2));

  if name_comparison != Ordering::Equal {
    return name_comparison;
  }

  // Only rules for the same property compete in the cascade, so the
  // breakpoint order applies within a property, after it. Ordering by width
  // first would decide some pairs by width and others by declaration text,
  // which is not a consistent ordering.
  let mq_comparison = compare_width_sort_keys(key_a, key_b);

  if mq_comparison != Ordering::Equal {
    return mq_comparison;
  }

  // `rule.slice(rule.lastIndexOf('{'))`: the declaration block, or, for a text
  // with no block, its last character.
  let property1 = &rule1[last_brace_or_last_character(rule1)..];
  let property2 = &rule2[last_brace_or_last_character(rule2)..];
  let property_comparison = locale_compare(property1, property2);

  if property_comparison != Ordering::Equal {
    return property_comparison;
  }

  locale_compare(rule1, rule2)
}

/// `sortedRules`: the rules in the sheet's order. `Array.prototype.sort` is
/// stable, and so is `sort_by`.
fn sort_rules<'a>(rules: &'a [Cow<'_, Rule>], use_legacy_classnames_sort: bool) -> Vec<&'a Rule> {
  let width_sort_keys = width_sort_keys_of(rules);
  let mut order: Vec<usize> = (0..rules.len()).collect();

  order.sort_by(|&i, &j| {
    compare_rules(
      (&rules[i], width_sort_keys[i].as_ref()),
      (&rules[j], width_sort_keys[j].as_ref()),
      use_legacy_classnames_sort,
    )
  });

  order.into_iter().map(|i| rules[i].as_ref()).collect()
}

// ==================== the groups ====================

/// The sorted rules grouped by thousand of priority (`Math.floor(priority /
/// 1000)`), each group in the order the sort gave its rules.
fn group_by_priority_level<'a>(sorted_rules: &[&'a Rule]) -> Vec<Vec<&'a Rule>> {
  let mut last_k_pri = -1.0;
  let mut grouped: Vec<Vec<&Rule>> = Vec::new();

  for &rule in sorted_rules {
    let priority_level = (rule.priority() / 1000.0).floor();

    match grouped.last_mut() {
      Some(group) if priority_level == last_k_pri => group.push(rule),
      _ => {
        last_k_pri = priority_level;
        grouped.push(vec![rule]);
      },
    }
  }

  grouped
}

// ==================== the output ====================

/// The layers of the sheet as the reference reads `useLayers`: on or off, the
/// layers declared before and after the priority layers, and their prefix.
struct Layers<'c> {
  on: bool,
  before: &'c [String],
  after: &'c [String],
  prefix: &'c str,
}

impl<'c> Layers<'c> {
  fn of(config: &'c ProcessStylexRulesConfig) -> Self {
    match &config.use_layers {
      Some(options) => Layers {
        on: true,
        before: &options.before,
        after: &options.after,
        prefix: &options.prefix,
      },
      None => Layers {
        on: false,
        before: &[],
        after: &[],
        prefix: "",
      },
    }
  }

  /// `layerName(index)`: the priority layers are named by ordinal, under the
  /// prefix when there is one.
  fn name(&self, index: usize) -> String {
    if self.prefix.is_empty() {
      format!("priority{}", index + 1)
    } else {
      format!("{}.priority{}", self.prefix, index + 1)
    }
  }

  /// The `@layer` statement that opens a sheet written with layers: every
  /// layer in cascade order, the ones before, one per group, the ones after.
  fn statement(&self, group_count: usize) -> String {
    if !self.on {
      return String::new();
    }

    let names: Vec<String> = self
      .before
      .iter()
      .cloned()
      .chain((0..group_count).map(|index| self.name(index)))
      .chain(self.after.iter().cloned())
      .collect();

    format!("\n@layer {};\n", names.join(", "))
  }
}

/// `new Map(group.map(([a, b]) => [a, b])).values()`: one style per class
/// name, at its first position, holding its last style.
fn deduplicate_by_class_name<'a>(group: &[&'a Rule]) -> Vec<&'a RuleStyle> {
  let mut deduplicated: Vec<&RuleStyle> = Vec::with_capacity(group.len());
  let mut position_of: FxHashMap<&str, usize> = FxHashMap::default();

  for rule in group {
    match position_of.get(rule.class_name()) {
      Some(&position) => deduplicated[position] = rule.style(),
      None => {
        position_of.insert(rule.class_name(), deduplicated.len());
        deduplicated.push(rule.style());
      },
    }
  }

  deduplicated
}

/// The lines one style writes: its LTR text, and, when it has an RTL text,
/// both texts under the two direction selectors or between the `@ltr` and
/// `@rtl` comments.
fn style_lines(
  style: &RuleStyle,
  index: usize,
  config: &ProcessStylexRulesConfig,
  layers: &Layers<'_>,
) -> Vec<String> {
  let mut ltr_rule = style.ltr.clone();
  // An empty RTL text is as absent as a missing one (`rtlRule &&`,
  // `if (rtlRule)`, `rtlRule ?` all read it as false).
  let mut rtl_rule = style.rtl.clone().filter(|rtl| !rtl.is_empty());

  if !layers.on && !config.legacy_disable_layers {
    ltr_rule = add_specificity_level(&ltr_rule, index);
    rtl_rule = rtl_rule.map(|rtl| add_specificity_level(&rtl, index));
  }

  // check if the selector looks like .xtrlmmh, .xtrlmmh:root
  // if so, turn it into .xtrlmmh.xtrlmmh, .xtrlmmh.xtrlmmh:root
  // This is to ensure the themes always have precedence over the
  // default variable values
  ltr_rule = double_theme_root_selector(&ltr_rule);
  rtl_rule = rtl_rule.map(|rtl| double_theme_root_selector(&rtl));

  match rtl_rule {
    Some(rtl_rule) if config.enable_ltr_rtl_comments => vec![
      format!("/* @ltr begin */{ltr_rule}/* @ltr end */"),
      format!("/* @rtl begin */{rtl_rule}/* @rtl end */"),
    ],
    Some(rtl_rule) => vec![
      add_ancestor_selector(&ltr_rule, "html:not([dir='rtl'])"),
      add_ancestor_selector(&rtl_rule, "html[dir='rtl']"),
    ],
    None => vec![ltr_rule],
  }
}

/// One priority group written out: its styles' lines, inside a `@layer` block
/// when layers are on and the group holds styles (a group of priority 0,
/// `@property`, `@keyframes`, `@position-try`, goes in no layer).
fn render_group(
  group: &[&Rule],
  index: usize,
  config: &ProcessStylexRulesConfig,
  layers: &Layers<'_>,
) -> String {
  let pri = group[0].priority();

  let collected_css = deduplicate_by_class_name(group)
    .into_iter()
    .flat_map(|style| style_lines(style, index, config, layers))
    .collect::<Vec<String>>()
    .join("\n");

  // Don't put @property, @keyframe, @position-try in layers
  if layers.on && pri > 0.0 {
    format!("@layer {}{{\n{collected_css}\n}}", layers.name(index))
  } else {
    collected_css
  }
}

/// `processStylexRules`: the sheet for `rules` under `config`.
///
/// The constants (`defineConsts` rules) are resolved against one another and
/// substituted into every other rule; the rules are sorted by priority, then
/// within a priority by declared property, breakpoint and text (or, under
/// `use_legacy_classnames_sort`, by class name); grouped by thousand of
/// priority; and written out, each group as a `@layer` when layers are on,
/// else with the specificity polyfill unless `legacy_disable_layers` asks for
/// neither. A rule with an RTL text is written twice, under `html[dir='rtl']`
/// and its negation, or between `@ltr`/`@rtl` comments.
pub fn process_stylex_rules(
  rules: &[Rule],
  config: &ProcessStylexRulesConfig,
) -> Result<String, ProcessStylexRulesError> {
  if rules.is_empty() {
    return Ok(String::new());
  }

  let (constants, non_constant_rules) = collect_constants(rules);
  let constants = resolve_constants(&constants)?;
  let substituted = substitute_constants(non_constant_rules, &constants);
  let sorted_rules = sort_rules(&substituted, config.use_legacy_classnames_sort);
  let grouped = group_by_priority_level(&sorted_rules);
  let layers = Layers::of(config);

  let collected_css = grouped
    .iter()
    .enumerate()
    .map(|(index, group)| render_group(group, index, config, &layers))
    .collect::<Vec<String>>()
    .join("\n");

  let mut sheet = get_logical_float_vars(sorted_rules.iter().copied());
  sheet.push_str(&layers.statement(grouped.len()));
  sheet.push_str(&collected_css);

  Ok(sheet)
}

// ==================== the selectors ====================

/// `addAncestorSelector`: `ancestor_selector` prefixed to the rule's own
/// selector, inside the innermost at-rule when the rule sits in one. A
/// `@keyframes` block holds no selector to prefix.
pub(crate) fn add_ancestor_selector(selector: &str, ancestor_selector: &str) -> String {
  if selector.starts_with("@keyframes") {
    return selector.to_string();
  }

  // `rfind` on a text that opens with `@` answers at or after 0; the `None`
  // arm is folded into the "no at-rule" return above it rather than left as a
  // branch nothing can reach.
  let last_at_rule = match (selector.starts_with('@'), selector.rfind('@')) {
    (true, Some(last_at_rule)) => last_at_rule,
    _ => return format!("{ancestor_selector} {selector}"),
  };
  // `indexOf('{', lastAtRule)` is -1 when no brace follows, and the two slices
  // then split at 0: the ancestor goes in front of the whole text.
  let split = selector[last_at_rule..]
    .find('{')
    .map_or(0, |brace| last_at_rule + brace + 1);
  let media_query_part = &selector[..split];
  let rest = &selector[split..];

  format!("{media_query_part}{ancestor_selector} {rest}")
}

/// `addSpecificityLevel`: `index` copies of `:not(#\#)` on the rule's selector,
/// the `@layer` polyfill for a sheet written without layers. They go before a
/// pseudo-element, else before the declaration block, else (a text with no
/// block) before its last character.
pub(crate) fn add_specificity_level(selector: &str, index: usize) -> String {
  if selector.starts_with("@keyframes") {
    return selector.to_string();
  }

  let pseudo = ":not(#\\#)".repeat(index);

  let split = match selector.find("::") {
    Some(pseudo_element) => pseudo_element,
    None => last_brace_or_last_character(selector),
  };
  let before_curly = &selector[..split];
  let after_curly = &selector[split..];

  format!("{before_curly}{pseudo}{after_curly}")
}

/// `.x, .x:root` -> `.x.x, .x.x:root`, by the first class name, as the
/// reference's `'.$1.$1, .$1.$1:root'` replacement reads it.
pub(crate) fn double_theme_root_selector(rule: &str) -> String {
  let mut out = String::with_capacity(rule.len());
  let mut copied = 0;
  let mut matches = THEME_ROOT_SELECTOR.captures_iter(rule);

  // Both groups are set in every match (group 0 is the match itself, group 1
  // is not optional), so the chain ends only when no match is left.
  while let Some(captures) = matches.next()
    && let (Some(whole), Some(class_name)) = (captures.get(0), captures.get(1))
  {
    let class_name = class_name.as_str();
    out.push_str(&rule[copied..whole.start()]);
    out.push_str(&format!(
      ".{class_name}.{class_name}, .{class_name}.{class_name}:root"
    ));
    copied = whole.end();
  }

  out.push_str(&rule[copied..]);
  out
}

#[cfg(test)]
#[path = "tests/process_test.rs"]
mod process_test;
