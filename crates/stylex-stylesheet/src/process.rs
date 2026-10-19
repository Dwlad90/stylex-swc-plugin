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
//! `String.prototype.replace` with a string replacement, `trim`, V8's
//! `Array.prototype.sort`, `new Map(...)`'s keeping of the first position
//! under the last value, `slice(-1)`, and the coercions `String(...)` and
//! `Number(...)` -- the port names the built-in and does what it does.

use std::borrow::Cow;

use indexmap::IndexMap;
use rustc_hash::FxHashMap;
use stylex_constants::constants::common::{LOGICAL_FLOAT_END_VAR, LOGICAL_FLOAT_START_VAR};
use stylex_js::array_sort::array_tim_sort;
use stylex_js::value::{JsValue, MapKey};
use stylex_regex::regex::{CONSTANT_REFERENCE, THEME_ROOT_SELECTOR};
use stylex_utils::number::is_js_whitespace;

use crate::collation::locale_compare;
use crate::width_sort_key::{
  WidthSortKey, at_rule_preludes, compare_width_sort_keys, declared_property_name,
  width_sort_key_for_chain,
};

// ==================== the rules and the config as the reference reads them ====================
//
// The reference reads its arguments with plain JavaScript: destructuring, a
// spread, property reads, `String(...)` and `Number(...)`. The port reads a
// `JsValue` the same way, so a value outside the published types writes the
// sheet the reference writes. A read the reference throws on is refused here.
// Where the reference throws only when a value is used (a class name compared
// under the legacy sort, an `ltr` text rendered), the value is kept as it is
// and refused at that use.

/// A rule's class name, which the reference uses three ways: spelled with
/// `String(...)` in a constant's name, as a `Map` key in the dedupe
/// (`SameValueZero`, so `1` and `'1'` are two class names), and as the
/// receiver of `localeCompare` under the legacy sort.
#[derive(Clone, Debug, PartialEq)]
pub struct ClassName(pub JsValue);

impl ClassName {
  /// `String(className)`.
  pub fn spelling(&self) -> Cow<'_, str> {
    self.0.to_js_string()
  }

  /// The class name as a `Map` key, or `None` for an array or an object,
  /// which is a key of its own.
  fn map_key(&self) -> Option<MapKey<'_>> {
    self.0.same_value_zero_key()
  }

  /// The class name as the receiver of `localeCompare`, which only a string
  /// is.
  fn as_receiver(&self) -> Result<&str, ProcessStylexRulesError> {
    match &self.0 {
      JsValue::String(class_name) => Ok(class_name),
      other => Err(ProcessStylexRulesError::ClassNameNotAString {
        class_name: self.spelling().into_owned(),
        kind: other.describe(),
      }),
    }
  }
}

impl From<&str> for ClassName {
  fn from(class_name: &str) -> Self {
    ClassName(class_name.into())
  }
}

/// What one rule injects: its LTR text, its RTL text where the two differ,
/// and, for a `defineConsts` rule, the constant it declares. Each field holds
/// the value as it arrived; a missing property is `undefined`.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct RuleStyle {
  pub ltr: JsValue,
  pub rtl: JsValue,
  pub const_key: JsValue,
  pub const_val: JsValue,
}

impl RuleStyle {
  /// `[key, { ...styleObj }, priority]`: the spread copies an object's
  /// properties, and gives no property for a string, a number, a boolean or
  /// an array (an array's elements are not the properties this reads). It
  /// throws for `null` and `undefined`, which is the `Err` here.
  fn read(style: JsValue) -> Result<Self, &'static str> {
    match style {
      JsValue::Object(_) => {
        let mut style = style;

        Ok(RuleStyle {
          ltr: style.take_property("ltr"),
          rtl: style.take_property("rtl"),
          const_key: style.take_property("constKey"),
          const_val: style.take_property("constVal"),
        })
      },
      _ if style.is_nullish() => Err(style.describe()),
      _ => Ok(RuleStyle::default()),
    }
  }

  /// `ruleObj?.constKey != null && ruleObj?.constVal != null`: the value of
  /// the constant this rule declares, or `None` for a style rule.
  fn constant(&self) -> Option<&JsValue> {
    (!self.const_key.is_nullish() && !self.const_val.is_nullish()).then_some(&self.const_val)
  }

  /// Whether a text of this style can hold a reference to a constant or a
  /// declaration of one. Every pattern the substitution searches for,
  /// `var(--x)` and `--x:`, holds `--`.
  fn mentions_a_variable(&self) -> bool {
    [&self.ltr, &self.rtl]
      .into_iter()
      .any(|text| matches!(text, JsValue::String(text) if text.contains("--")))
  }
}

/// One emitted rule: `[className, { ltr, rtl, constKey, constVal }, priority]`,
/// the reference's `Rule` tuple, with the priority read as the reference's
/// arithmetic reads it (`Number(priority)`: a missing one is `NaN`).
#[derive(Clone, Debug, PartialEq)]
pub struct Rule(pub ClassName, pub RuleStyle, pub f64);

impl Rule {
  pub fn class_name(&self) -> &ClassName {
    &self.0
  }

  pub fn style(&self) -> &RuleStyle {
    &self.1
  }

  pub fn priority(&self) -> f64 {
    self.2
  }

  /// The `ltr` text, which the reference reads with string methods
  /// (`slice` in the sort, `replace` and `startsWith` in the output). Any
  /// other value throws there.
  fn ltr(&self) -> Result<&str, ProcessStylexRulesError> {
    match &self.style().ltr {
      JsValue::String(text) => Ok(text),
      other => Err(ProcessStylexRulesError::LtrNotAString {
        class_name: self.class_name().spelling().into_owned(),
        kind: other.describe(),
      }),
    }
  }

  /// The `rtl` text behind `if (rtlRule)`: a falsy value (the empty string
  /// included) is no RTL text, and a truthy value that is not a string throws
  /// at its string methods.
  fn rtl(&self) -> Result<Option<&str>, ProcessStylexRulesError> {
    match &self.style().rtl {
      rtl if !rtl.to_js_boolean() => Ok(None),
      JsValue::String(text) => Ok(Some(text)),
      other => Err(ProcessStylexRulesError::RtlNotAString {
        class_name: self.class_name().spelling().into_owned(),
        kind: other.describe(),
      }),
    }
  }
}

/// `rules.length === 0`, the one early return of the reference: an empty
/// array, an empty string, or an object whose `length` is 0. Every other
/// value goes on to `rules.filter`.
fn has_no_length(rules: &JsValue) -> bool {
  match rules {
    JsValue::Array(elements) => elements.is_empty(),
    JsValue::String(text) => text.is_empty(),
    JsValue::Object(_) => {
      matches!(rules.property("length"), JsValue::Number(length) if *length == 0.0)
    },
    _ => false,
  }
}

/// The rules as `rules.filter(([, ruleObj]) => ...)` reads them: only an
/// array has `filter`, a hole is skipped, and the callback destructures each
/// rule, so a rule that is not iterable is refused.
pub fn read_rules(rules: JsValue) -> Result<Vec<Rule>, ProcessStylexRulesError> {
  let JsValue::Array(elements) = rules else {
    return Err(ProcessStylexRulesError::RulesNotAList(rules.describe()));
  };

  elements
    .into_iter()
    .enumerate()
    .filter(|(_, rule)| *rule != JsValue::Hole)
    .map(|(index, rule)| read_rule(index, rule))
    .collect()
}

/// `([key, { ...styleObj }, priority])`: the first three items of the rule's
/// iterator, `undefined` past its end. An array iterates its elements and a
/// string its characters; any other value is not iterable.
fn read_rule(index: usize, rule: JsValue) -> Result<Rule, ProcessStylexRulesError> {
  let items: Vec<JsValue> = match rule {
    // Only the first three are read; the rest drop with the vector.
    JsValue::Array(elements) => elements,
    JsValue::String(text) => text
      .chars()
      .take(3)
      .map(|character| JsValue::String(character.to_string()))
      .collect(),
    other => {
      return Err(ProcessStylexRulesError::RuleNotAList {
        index,
        kind: other.describe(),
      });
    },
  };
  let mut items = items.into_iter();
  let mut next = || items.next().unwrap_or_default();

  let class_name = ClassName(next());
  let style = RuleStyle::read(next())
    .map_err(|kind| ProcessStylexRulesError::NoStyleObject { index, kind })?;
  let priority = next().to_js_number();

  Ok(Rule(class_name, style, priority))
}

/// The `useLayers` object form: layers to declare before and after the
/// priority layers, and a prefix for their names, each as the header spells
/// it.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct LayerOptions {
  pub before: Vec<String>,
  pub after: Vec<String>,
  /// `None` when the prefix is falsy; else `String(prefix)`, which can be
  /// empty (an empty array is truthy and spells as nothing).
  pub prefix: Option<String>,
}

impl LayerOptions {
  /// `rawUseLayers.before ?? []`, `rawUseLayers.after ?? []` and
  /// `rawUseLayers.prefix ?? ''`, read off an object. Any other value has none
  /// of the properties, which gives the defaults.
  fn read(use_layers: &JsValue) -> Result<Self, ProcessStylexRulesError> {
    let prefix = use_layers.property("prefix");

    Ok(LayerOptions {
      before: read_layer_names("before", use_layers.property("before"))?,
      after: read_layer_names("after", use_layers.property("after"))?,
      prefix: prefix
        .to_js_boolean()
        .then(|| prefix.to_js_string().into_owned()),
    })
  }
}

/// `[...(names ?? [])]` in the header, joined with `', '`: an array gives its
/// elements, spelled as `join` spells them (`null` and `undefined` as
/// nothing), and a string gives its characters. Any other value is not
/// iterable, and the spread throws.
fn read_layer_names(
  option: &'static str,
  names: &JsValue,
) -> Result<Vec<String>, ProcessStylexRulesError> {
  match names {
    JsValue::Array(elements) => Ok(
      elements
        .iter()
        .map(|name| name.to_js_join_element().into_owned())
        .collect(),
    ),
    JsValue::String(text) => Ok(text.chars().map(String::from).collect()),
    _ if names.is_nullish() => Ok(Vec::new()),
    other => Err(ProcessStylexRulesError::LayerOptionNotAList {
      option,
      kind: other.describe(),
    }),
  }
}

/// The reference's `config` argument. `useLayers: true` is
/// `Some(LayerOptions::default())`; `false`, `null` and an absent key are
/// `None`.
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

impl ProcessStylexRulesConfig {
  /// `typeof config === 'boolean' ? { useLayers: config } : config ?? {}`,
  /// then one property read each. Only an object has the properties, so for
  /// any other value every option is at its default. `useLayers` is on for
  /// every value but `false` and a nullish one (`(useLayers ?? false) !==
  /// false`), and the three flags are read by truthiness.
  pub fn read(config: &JsValue) -> Result<Self, ProcessStylexRulesError> {
    if let JsValue::Boolean(use_layers) = config {
      return Ok(Self::from(*use_layers));
    }

    let use_layers = match config.property("useLayers") {
      JsValue::Boolean(false) => None,
      use_layers if use_layers.is_nullish() => None,
      use_layers => Some(LayerOptions::read(use_layers)?),
    };

    Ok(Self {
      use_layers,
      enable_ltr_rtl_comments: config.property("enableLTRRTLComments").to_js_boolean(),
      legacy_disable_layers: config.property("legacyDisableLayers").to_js_boolean(),
      use_legacy_classnames_sort: config.property("useLegacyClassnamesSort").to_js_boolean(),
    })
  }
}

/// Why a sheet could not be assembled. Each one is a point where the
/// reference throws.
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum ProcessStylexRulesError {
  /// A constant's value refers, through any chain of constants, to itself.
  #[error("circular reference detected for constant {0}")]
  CircularConstant(String),
  /// The rules are not an array, so they have no `filter`.
  #[error("the rules are {0}, not a list")]
  RulesNotAList(&'static str),
  /// A rule is not iterable, so it cannot be destructured.
  #[error("rule {index} is {kind}, not a [className, style, priority] list")]
  RuleNotAList { index: usize, kind: &'static str },
  /// A style rule's style is `null` or `undefined`, which a spread cannot
  /// read.
  #[error("rule {index} has {kind} in place of its style object")]
  NoStyleObject { index: usize, kind: &'static str },
  /// A style rule's `ltr` text is not a string where it is read as one.
  #[error("the ltr text of class {class_name} is {kind}, not a string")]
  LtrNotAString {
    class_name: String,
    kind: &'static str,
  },
  /// A style rule's `rtl` text is truthy and not a string where it is read
  /// as one.
  #[error("the rtl text of class {class_name} is {kind}, not a string")]
  RtlNotAString {
    class_name: String,
    kind: &'static str,
  },
  /// Under `useLegacyClassnamesSort`, the class name that `localeCompare` is
  /// called on is not a string.
  #[error("the legacy sort cannot compare class {class_name}: it is {kind}, not a string")]
  ClassNameNotAString {
    class_name: String,
    kind: &'static str,
  },
  /// A layer option is not iterable, so the header cannot spread it.
  #[error("the layer option {option} is {kind}, not a list or a string")]
  LayerOptionNotAList {
    option: &'static str,
    kind: &'static str,
  },
  /// The first rule after the sort is in priority level -1 (a priority from
  /// -1000 to less than 0). The reference's grouping starts at level -1 with
  /// no group, so it fails on this rule.
  #[error(
    "class {class_name} sorts first with a priority from -1000 to less than 0, which cannot be \
     grouped; use a priority of 0 or more"
  )]
  FirstRuleBelowZero { class_name: String },
}

/// The `:root, [dir="ltr"]` / `[dir="rtl"]` block that gives the logical float
/// variables a value, when any rule reads one.
pub(crate) fn get_logical_float_vars<'a>(rules: impl IntoIterator<Item = &'a Rule>) -> String {
  let has_logical_float = rules.into_iter().any(|rule| {
    // `String(ltr)`, and `rtl ? String(rtl) : ''`.
    let ltr = rule.style().ltr.to_js_string();
    let rtl = match &rule.style().rtl {
      rtl if rtl.to_js_boolean() => rtl.to_js_string(),
      _ => Cow::Borrowed(""),
    };

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
pub type OrderedConstants = IndexMap<String, JsValue>;

/// `constsMap` and `nonConstantRules`: the constants a sheet declares and the
/// rules that are styles.
fn collect_constants(rules: &[Rule]) -> (OrderedConstants, Vec<&Rule>) {
  let mut constants = OrderedConstants::default();
  let mut non_constant_rules = Vec::new();

  for rule in rules {
    match rule.style().constant() {
      Some(value) => {
        constants.insert(
          format!("var(--{})", rule.class_name().spelling()),
          value.clone(),
        );
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
  value: &JsValue,
  constants: &OrderedConstants,
  visited: &mut Vec<String>,
) -> Result<JsValue, ProcessStylexRulesError> {
  let JsValue::String(text) = value else {
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

  Ok(JsValue::String(result))
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
      let replacement = value.to_js_string().into_owned();
      let rename = if replacement.starts_with("var(") && replacement.ends_with(')') {
        // `trim()` removes the ECMAScript white space, not Unicode's.
        let inside = replacement[4..replacement.len() - 1].trim_matches(is_js_whitespace);
        let target_name = match inside.find(',') {
          Some(comma) => inside[..comma].trim_matches(is_js_whitespace),
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
      // `rtl`; a value that is not a string (`rtl: null`) is left alone. No
      // other property is read after this, so none is substituted.
      for text in [&mut style.ltr, &mut style.rtl] {
        if let JsValue::String(text) = text {
          for substitution in &substitutions {
            substitution.apply(text);
          }
        }
      }

      Cow::Owned(Rule(rule.class_name().clone(), style, rule.priority()))
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
      // A text that is not a string is refused in the comparator before its
      // key is read.
      let JsValue::String(text) = &rule.style().ltr else {
        return None;
      };

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

/// The comparator of the sort, answering a number as the reference's does: by
/// priority, then, within a priority, by class name under
/// `use_legacy_classnames_sort`, else by declared property, width key,
/// declaration block and whole text.
///
/// `firstPriority - secondPriority` is `NaN` when either priority is `NaN` or
/// both are the same infinity, and `NaN !== 0` holds: the comparator answers
/// `NaN`, which the sort reads as 0, so the rules tie without the later keys
/// being read. That is not a consistent order, which is why the sort is V8's
/// own.
fn compare_rules(
  (a, key_a): (&Rule, Option<&WidthSortKey>),
  (b, key_b): (&Rule, Option<&WidthSortKey>),
  use_legacy_classnames_sort: bool,
) -> Result<f64, ProcessStylexRulesError> {
  let priority_comparison = a.priority() - b.priority();

  if priority_comparison != 0.0 {
    return Ok(priority_comparison);
  }

  if use_legacy_classnames_sort {
    // `classname1.localeCompare(classname2)`: the argument is spelled with
    // `String(...)`.
    let classname1 = a.class_name().as_receiver()?;

    return Ok(locale_compare(classname1, &b.class_name().spelling()));
  }

  let rule1 = a.ltr()?;
  let rule2 = b.ltr()?;

  let name_comparison =
    locale_compare(declared_property_name(rule1), declared_property_name(rule2));

  if name_comparison != 0.0 {
    return Ok(name_comparison);
  }

  // Only rules for the same property compete in the cascade, so the
  // breakpoint order applies within a property, after it. Ordering by width
  // first would decide some pairs by width and others by declaration text,
  // which is not a consistent ordering.
  let mq_comparison = compare_width_sort_keys(key_a, key_b);

  if mq_comparison != 0.0 {
    return Ok(mq_comparison);
  }

  // `rule.slice(rule.lastIndexOf('{'))`: the declaration block, or, for a text
  // with no block, its last character.
  let property1 = &rule1[last_brace_or_last_character(rule1)..];
  let property2 = &rule2[last_brace_or_last_character(rule2)..];
  let property_comparison = locale_compare(property1, property2);

  if property_comparison != 0.0 {
    return Ok(property_comparison);
  }

  Ok(locale_compare(rule1, rule2))
}

/// `sortedRules`: the rules in the sheet's order, sorted as V8 sorts them.
fn sort_rules<'a>(
  rules: &'a [Cow<'_, Rule>],
  use_legacy_classnames_sort: bool,
) -> Result<Vec<&'a Rule>, ProcessStylexRulesError> {
  let width_sort_keys = width_sort_keys_of(rules);
  let mut order: Vec<usize> = (0..rules.len()).collect();

  array_tim_sort(&mut order, |&i, &j| {
    compare_rules(
      (&rules[i], width_sort_keys[i].as_ref()),
      (&rules[j], width_sort_keys[j].as_ref()),
      use_legacy_classnames_sort,
    )
  })?;

  Ok(order.into_iter().map(|i| rules[i].as_ref()).collect())
}

// ==================== the groups ====================

/// The sorted rules grouped by thousand of priority (`Math.floor(priority /
/// 1000)`), each group in the order the sort gave its rules. A `NaN` level
/// equals no level, so each such rule is a group of its own.
///
/// The reference starts with `lastKPri = -1` and no group, and a rule whose
/// level equals `lastKPri` goes into the last group: when the first rule is
/// at level -1, there is no last group, and the reference throws.
fn group_by_priority_level<'a>(
  sorted_rules: &[&'a Rule],
) -> Result<Vec<Vec<&'a Rule>>, ProcessStylexRulesError> {
  let mut last_k_pri = -1.0;
  let mut grouped: Vec<Vec<&Rule>> = Vec::new();

  for &rule in sorted_rules {
    let priority_level = (rule.priority() / 1000.0).floor();

    if priority_level == last_k_pri {
      let Some(group) = grouped.last_mut() else {
        return Err(ProcessStylexRulesError::FirstRuleBelowZero {
          class_name: rule.class_name().spelling().into_owned(),
        });
      };
      group.push(rule);
    } else {
      last_k_pri = priority_level;
      grouped.push(vec![rule]);
    }
  }

  Ok(grouped)
}

// ==================== the output ====================

/// The layers of the sheet as the reference reads `useLayers`: on or off, the
/// layers declared before and after the priority layers, and their prefix.
struct Layers<'c> {
  on: bool,
  before: &'c [String],
  after: &'c [String],
  prefix: Option<&'c str>,
}

impl<'c> Layers<'c> {
  fn of(config: &'c ProcessStylexRulesConfig) -> Self {
    match &config.use_layers {
      Some(options) => Layers {
        on: true,
        before: &options.before,
        after: &options.after,
        prefix: options.prefix.as_deref(),
      },
      None => Layers {
        on: false,
        before: &[],
        after: &[],
        prefix: None,
      },
    }
  }

  /// `layerName(index)`: the priority layers are named by ordinal, under the
  /// prefix when there is one.
  fn name(&self, index: usize) -> String {
    match self.prefix {
      Some(prefix) => format!("{prefix}.priority{}", index + 1),
      None => format!("priority{}", index + 1),
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
/// name, at its first position, holding its last style. The rule is kept
/// rather than the style alone, so a refusal can name its class.
fn deduplicate_by_class_name<'a>(group: &[&'a Rule]) -> Vec<&'a Rule> {
  let mut deduplicated: Vec<&Rule> = Vec::with_capacity(group.len());
  let mut position_of: FxHashMap<MapKey<'_>, usize> = FxHashMap::default();

  for &rule in group {
    // An array or an object is a key of its own.
    let Some(key) = rule.class_name().map_key() else {
      deduplicated.push(rule);
      continue;
    };

    match position_of.get(&key) {
      Some(&position) => deduplicated[position] = rule,
      None => {
        position_of.insert(key, deduplicated.len());
        deduplicated.push(rule);
      },
    }
  }

  deduplicated
}

/// The lines one style writes: its LTR text, and, when it has an RTL text,
/// both texts under the two direction selectors or between the `@ltr` and
/// `@rtl` comments.
fn style_lines(
  rule: &Rule,
  index: usize,
  config: &ProcessStylexRulesConfig,
  layers: &Layers<'_>,
) -> Result<Vec<String>, ProcessStylexRulesError> {
  let mut ltr_rule = rule.ltr()?.to_string();
  // An empty RTL text is as absent as a missing one (`rtlRule &&`,
  // `if (rtlRule)`, `rtlRule ?` all read it as false).
  let mut rtl_rule = rule.rtl()?.map(str::to_string);

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

  Ok(match rtl_rule {
    Some(rtl_rule) if config.enable_ltr_rtl_comments => vec![
      format!("/* @ltr begin */{ltr_rule}/* @ltr end */"),
      format!("/* @rtl begin */{rtl_rule}/* @rtl end */"),
    ],
    Some(rtl_rule) => vec![
      add_ancestor_selector(&ltr_rule, "html:not([dir='rtl'])"),
      add_ancestor_selector(&rtl_rule, "html[dir='rtl']"),
    ],
    None => vec![ltr_rule],
  })
}

/// One priority group written out: its styles' lines, inside a `@layer` block
/// when layers are on and the group holds styles (a group of priority 0,
/// `@property`, `@keyframes`, `@position-try`, goes in no layer).
fn render_group(
  group: &[&Rule],
  index: usize,
  config: &ProcessStylexRulesConfig,
  layers: &Layers<'_>,
) -> Result<String, ProcessStylexRulesError> {
  let pri = group[0].priority();

  let mut lines = Vec::with_capacity(group.len());
  for rule in deduplicate_by_class_name(group) {
    lines.extend(style_lines(rule, index, config, layers)?);
  }
  let collected_css = lines.join("\n");

  // Don't put @property, @keyframe, @position-try in layers
  Ok(if layers.on && pri > 0.0 {
    format!("@layer {}{{\n{collected_css}\n}}", layers.name(index))
  } else {
    collected_css
  })
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

  assemble(rules, config)
}

/// `processStylexRules(rules, config)` over the two arguments as JavaScript
/// values, read as the reference reads them: the early return on
/// `rules.length === 0` first, then the config and the rules.
pub fn process_stylex_rule_values(
  rules: JsValue,
  config: &JsValue,
) -> Result<String, ProcessStylexRulesError> {
  if has_no_length(&rules) {
    return Ok(String::new());
  }

  let config = ProcessStylexRulesConfig::read(config)?;
  let rules = read_rules(rules)?;

  // The list can be empty here (`[, ,]` has a length, and only holes): the
  // reference goes on and writes the `@layer` header alone.
  assemble(&rules, &config)
}

/// The steps of `processStylexRules` after its early return.
fn assemble(
  rules: &[Rule],
  config: &ProcessStylexRulesConfig,
) -> Result<String, ProcessStylexRulesError> {
  let (constants, non_constant_rules) = collect_constants(rules);
  let constants = resolve_constants(&constants)?;
  let substituted = substitute_constants(non_constant_rules, &constants);
  let sorted_rules = sort_rules(&substituted, config.use_legacy_classnames_sort)?;
  let grouped = group_by_priority_level(&sorted_rules)?;
  let layers = Layers::of(config);

  let collected_css = grouped
    .iter()
    .enumerate()
    .map(|(index, group)| render_group(group, index, config, &layers))
    .collect::<Result<Vec<String>, _>>()?
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
