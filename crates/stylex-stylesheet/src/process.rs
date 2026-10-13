//! `processStylexRules`: the rules a compile emits, assembled into one sheet.
//!
//! The reference is `processStylexRules` and the helpers around it in
//! `packages/@stylexjs/babel-plugin/src/index.js` at the commit `CONTEXT.md`
//! names, followed function by function and in its order:
//! `getLogicalFloatVars`, `processStylexRules` (with `resolveConstant` inside
//! it), `addAncestorSelector`, `addSpecificityLevel`. Where the reference
//! leans on a JavaScript built-in whose behaviour shapes the output --
//! `localeCompare`, `String.prototype.replace` with a string replacement,
//! `Array.prototype.sort`'s stability, `new Map(...)`'s keeping of the first
//! position under the last value -- the port names the built-in and does what
//! it does.

use std::borrow::Cow;
use std::cmp::Ordering;

use rustc_hash::FxHashMap;
use serde::{Deserialize, Deserializer};
use stylex_constants::constants::common::{LOGICAL_FLOAT_END_VAR, LOGICAL_FLOAT_START_VAR};
use stylex_regex::regex::{CONSTANT_REFERENCE, THEME_ROOT_SELECTOR};
use stylex_utils::number::write_js_number;

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
      ConstantValue::Number(number) => {
        let mut text = String::new();
        // Writing into a `String` cannot fail; the result is dropped as the
        // reference's `String(n)` drops none.
        let _ = write_js_number(&mut text, *number);
        text
      },
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
/// priority layers, and a prefix for their names.
#[derive(Clone, Debug, Default, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LayerOptions {
  #[serde(default)]
  pub before: Vec<String>,
  #[serde(default)]
  pub after: Vec<String>,
  #[serde(default)]
  pub prefix: String,
}

/// The reference's `config` argument. `useLayers: true` is
/// `Some(LayerOptions::default())`; `false`, `null` and an absent key are
/// `None`. It deserializes from either spelling the reference accepts: the
/// options object, or a bare boolean standing for `useLayers`.
#[derive(Clone, Debug, Default, PartialEq, Deserialize)]
#[serde(from = "ConfigSpelling")]
pub struct ProcessStylexRulesConfig {
  pub use_layers: Option<LayerOptions>,
  pub enable_ltr_rtl_comments: bool,
  pub legacy_disable_layers: bool,
  pub use_legacy_classnames_sort: bool,
}

/// `config` as JavaScript spells it: `boolean | { useLayers?, ... }`.
#[derive(Deserialize)]
#[serde(untagged)]
enum ConfigSpelling {
  Flag(bool),
  Options(ConfigOptions),
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct ConfigOptions {
  #[serde(default, deserialize_with = "deserialize_use_layers")]
  use_layers: Option<LayerOptions>,
  // The reference spells the acronyms in capitals, which `camelCase` would
  // not.
  #[serde(default, rename = "enableLTRRTLComments")]
  enable_ltr_rtl_comments: bool,
  #[serde(default)]
  legacy_disable_layers: bool,
  #[serde(default)]
  use_legacy_classnames_sort: bool,
}

impl From<ConfigSpelling> for ProcessStylexRulesConfig {
  fn from(spelling: ConfigSpelling) -> Self {
    match spelling {
      ConfigSpelling::Flag(use_layers) => Self::from(use_layers),
      ConfigSpelling::Options(options) => Self {
        use_layers: options.use_layers,
        enable_ltr_rtl_comments: options.enable_ltr_rtl_comments,
        legacy_disable_layers: options.legacy_disable_layers,
        use_legacy_classnames_sort: options.use_legacy_classnames_sort,
      },
    }
  }
}

/// `useLayers` as JavaScript spells it: a boolean, the options object, or
/// `null`.
#[derive(Deserialize)]
#[serde(untagged)]
enum UseLayersSpelling {
  Flag(bool),
  Options(LayerOptions),
}

fn deserialize_use_layers<'de, D: Deserializer<'de>>(
  deserializer: D,
) -> Result<Option<LayerOptions>, D::Error> {
  Ok(
    match Option::<UseLayersSpelling>::deserialize(deserializer)? {
      Some(UseLayersSpelling::Flag(flag)) => flag.then(LayerOptions::default),
      Some(UseLayersSpelling::Options(options)) => Some(options),
      None => None,
    },
  )
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

/// Why a sheet could not be assembled.
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum ProcessStylexRulesError {
  /// A constant's value refers, through any chain of constants, to itself.
  #[error("circular reference detected for constant {0}")]
  CircularConstant(String),
}

/// The `:root, [dir="ltr"]` / `[dir="rtl"]` block that gives the logical float
/// variables a value, when any rule reads one.
pub(crate) fn get_logical_float_vars(rules: &[Rule]) -> String {
  let has_logical_float = rules.iter().any(|rule| {
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

/// `text.replace(pattern, replacement)`: the first occurrence.
pub(crate) fn js_replace_first(text: &str, pattern: &str, replacement: &str) -> String {
  match text.find(pattern) {
    Some(start) => {
      let end = start + pattern.len();
      let mut out = String::with_capacity(text.len() + replacement.len());
      out.push_str(&text[..start]);
      out.push_str(&js_substitution(
        replacement,
        pattern,
        &text[..start],
        &text[end..],
      ));
      out.push_str(&text[end..]);
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
    let end = start + pattern.len();
    out.push_str(&rest[..offset]);
    out.push_str(&js_substitution(
      replacement,
      pattern,
      &text[..start],
      &text[end..],
    ));
    rest = &rest[offset + pattern.len()..];
    consumed = end;
  }

  out.push_str(rest);
  Cow::Owned(out)
}

/// The constants a sheet declares, by their reference text (`var(--key)`).
type ConstantsMap = Vec<(String, ConstantValue)>;

fn constant_of<'a>(constants: &'a ConstantsMap, reference: &str) -> Option<&'a ConstantValue> {
  constants
    .iter()
    .find(|(key, _)| key == reference)
    .map(|(_, value)| value)
}

/// `resolveConstant`: a constant's value with every reference to another
/// constant replaced by that constant's resolved value. A reference to a
/// constant the sheet does not declare stays as written. The search restarts
/// from the front after each replacement (`regex.lastIndex = 0`), so a value a
/// replacement brings in is resolved in turn; a chain that returns to a
/// constant already on it is a circular reference.
pub(crate) fn resolve_constant(
  value: &ConstantValue,
  constants: &ConstantsMap,
  visited: &mut Vec<String>,
) -> Result<ConstantValue, ProcessStylexRulesError> {
  let ConstantValue::String(text) = value else {
    return Ok(value.clone());
  };

  let mut result = text.clone();
  let mut position = 0;

  // Both groups are set whenever the expression matches (group 1 is not
  // optional), so the chain ends only when there is no match left.
  while let Ok(Some(captures)) = CONSTANT_REFERENCE.captures_from_pos(&result, position)
    && let (Some(whole), Some(name)) = (captures.get(0), captures.get(1))
  {
    let reference = name.as_str().to_string();
    let matched = whole.as_str().to_string();
    let end = whole.end();

    if visited.contains(&reference) {
      return Err(ProcessStylexRulesError::CircularConstant(reference));
    }

    let Some(referenced) = constant_of(constants, &format!("var({reference})")) else {
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
/// pseudo-element, else before the declaration block.
pub(crate) fn add_specificity_level(selector: &str, index: usize) -> String {
  if selector.starts_with("@keyframes") {
    return selector.to_string();
  }

  let pseudo = ":not(#\\#)".repeat(index);

  // `lastIndexOf('{')` is -1 for a text with no block, and `slice(0, -1)` then
  // keeps all but the last character: the split sits one before the end.
  let split = match selector.find("::") {
    Some(pseudo_element) => pseudo_element,
    None => selector
      .rfind('{')
      .unwrap_or(selector.len().saturating_sub(1)),
  };
  let before_curly = &selector[..split];
  let after_curly = &selector[split..];

  format!("{before_curly}{pseudo}{after_curly}")
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
  let enable_ltr_rtl_comments = config.enable_ltr_rtl_comments;
  let legacy_disable_layers = config.legacy_disable_layers;
  let use_legacy_classnames_sort = config.use_legacy_classnames_sort;

  let use_layers = config.use_layers.is_some();
  let layer_options = config.use_layers.clone().unwrap_or_default();
  let layers_before = layer_options.before;
  let layers_after = layer_options.after;
  let layer_prefix = layer_options.prefix;

  if rules.is_empty() {
    return Ok(String::new());
  }

  // `constsMap`: a `Map` keyed by the reference text (`var(--<class name>)`),
  // so a constant declared twice keeps its first position and its last value.
  let mut constants: ConstantsMap = Vec::new();
  let mut non_constant_rules: Vec<&Rule> = Vec::new();

  for rule in rules {
    match rule.style().constant() {
      Some((_, value)) => {
        let constant_name = format!("var(--{})", rule.class_name());

        match constants.iter_mut().find(|(key, _)| *key == constant_name) {
          Some(entry) => entry.1 = value.clone(),
          None => constants.push((constant_name, value.clone())),
        }
      },
      None => non_constant_rules.push(rule),
    }
  }

  let unresolved = constants.clone();

  for (_, value) in &mut constants {
    *value = resolve_constant(value, &unresolved, &mut Vec::new())?;
  }

  // Parsing is expensive and the comparator runs O(n log n) times. Key on the
  // at-rule chain, not the whole rule -- every rule has a distinct class name,
  // so a full-rule key would never hit.
  let mut width_sort_keys: FxHashMap<String, Option<WidthSortKey>> = FxHashMap::default();
  let mut get_width_sort_key = |rule: &str| -> Option<WidthSortKey> {
    if !rule.starts_with('@') {
      return None;
    }

    let preludes = at_rule_preludes(rule);
    let chain = preludes.join("{");

    if let Some(cached) = width_sort_keys.get(&chain) {
      return cached.clone();
    }

    let key = width_sort_key_for_chain(&preludes);
    width_sort_keys.insert(chain, key.clone());
    key
  };

  // What each constant substitutes as, spelled once rather than once per rule:
  // `String(constValue)`, and, when that is itself a `var(...)`, the
  // `--const:` -> `--target:` rename of a declaration of the constant.
  let substitutions: Vec<(&str, String, Option<(String, String)>)> = constants
    .iter()
    .map(|(var_ref, const_value)| {
      let replacement = const_value.to_js_string();
      let rename = if replacement.starts_with("var(") && replacement.ends_with(')') {
        let inside = replacement[4..replacement.len() - 1].trim();
        let target_name = match inside.find(',') {
          Some(comma) => inside[..comma].trim(),
          None => inside,
        };
        let const_name = &var_ref[4..var_ref.len() - 1];

        Some((format!("{const_name}:"), format!("{target_name}:")))
      } else {
        None
      };

      (var_ref.as_str(), replacement, rename)
    })
    .collect();

  let sorted_rules: Vec<Rule> = non_constant_rules
    .into_iter()
    .map(|rule| {
      let mut style = rule.style().clone();

      // `Object.keys(styleObj).forEach`: every text the style holds, `ltr` then
      // `rtl`; a value that is not a string (`rtl: null`) is left alone.
      for original in [Some(&mut style.ltr), style.rtl.as_mut()]
        .into_iter()
        .flatten()
      {
        // Every pattern the loop searches for, `var(--x)` and `--x:`, holds
        // `--`, so a text without it has nothing to replace and never changes:
        // the loop is skipped as a whole. Most rules are such a text.
        if !original.contains("--") {
          continue;
        }

        for (var_ref, replacement, rename) in &substitutions {
          if let Cow::Owned(replaced) = js_replace_all(original, var_ref, replacement) {
            *original = replaced;
          }

          if let Some((const_declaration, target_declaration)) = rename
            && let Cow::Owned(renamed) =
              js_replace_all(original, const_declaration, target_declaration)
          {
            *original = renamed;
          }
        }
      }

      Rule(rule.class_name().to_string(), style, rule.priority())
    })
    .collect();

  // The key of every rule, read once through the chain cache rather than at
  // every comparison; the comparator then indexes.
  let width_sort_keys_of: Vec<Option<WidthSortKey>> = sorted_rules
    .iter()
    .map(|rule| get_width_sort_key(&rule.style().ltr))
    .collect();
  let mut order: Vec<usize> = (0..sorted_rules.len()).collect();

  // `Array.prototype.sort` is stable, and so is `sort_by`.
  order.sort_by(|&i, &j| {
    let a = &sorted_rules[i];
    let b = &sorted_rules[j];
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
    let mq_comparison = compare_width_sort_keys(
      width_sort_keys_of[i].as_ref(),
      width_sort_keys_of[j].as_ref(),
    );

    if mq_comparison != Ordering::Equal {
      return mq_comparison;
    }

    let property1 = &rule1[rule1.rfind('{').unwrap_or(0)..];
    let property2 = &rule2[rule2.rfind('{').unwrap_or(0)..];
    let property_comparison = locale_compare(property1, property2);

    if property_comparison != Ordering::Equal {
      return property_comparison;
    }

    locale_compare(rule1, rule2)
  });

  let sorted_rules: Vec<Rule> = order.into_iter().map(|i| sorted_rules[i].clone()).collect();

  let mut last_k_pri = -1.0;
  let mut grouped: Vec<Vec<&Rule>> = Vec::new();

  for rule in &sorted_rules {
    let priority_level = (rule.priority() / 1000.0).floor();

    match grouped.last_mut() {
      Some(group) if priority_level == last_k_pri => group.push(rule),
      _ => {
        last_k_pri = priority_level;
        grouped.push(vec![rule]);
      },
    }
  }

  let logical_float_vars = get_logical_float_vars(&sorted_rules);

  let layer_name = |index: usize| -> String {
    if layer_prefix.is_empty() {
      format!("priority{}", index + 1)
    } else {
      format!("{layer_prefix}.priority{}", index + 1)
    }
  };

  let header = if use_layers {
    let names: Vec<String> = layers_before
      .iter()
      .cloned()
      .chain((0..grouped.len()).map(layer_name))
      .chain(layers_after.iter().cloned())
      .collect();

    format!("\n@layer {};\n", names.join(", "))
  } else {
    String::new()
  };

  let collected_css = grouped
    .iter()
    .enumerate()
    .map(|(index, group)| {
      let pri = group[0].priority();

      // `new Map(group.map(([a, b]) => [a, b])).values()`: one entry per class
      // name, at its first position, holding its last style.
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

      let collected_css = deduplicated
        .into_iter()
        .flat_map(|style| {
          let mut ltr_rule = style.ltr.clone();
          // An empty RTL text is as absent as a missing one (`rtlRule &&`,
          // `if (rtlRule)`, `rtlRule ?` all read it as false).
          let mut rtl_rule = style.rtl.clone().filter(|rtl| !rtl.is_empty());

          if !use_layers && !legacy_disable_layers {
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
            Some(rtl_rule) if enable_ltr_rtl_comments => vec![
              format!("/* @ltr begin */{ltr_rule}/* @ltr end */"),
              format!("/* @rtl begin */{rtl_rule}/* @rtl end */"),
            ],
            Some(rtl_rule) => vec![
              add_ancestor_selector(&ltr_rule, "html:not([dir='rtl'])"),
              add_ancestor_selector(&rtl_rule, "html[dir='rtl']"),
            ],
            None => vec![ltr_rule],
          }
        })
        .collect::<Vec<String>>()
        .join("\n");

      // Don't put @property, @keyframe, @position-try in layers
      if use_layers && pri > 0.0 {
        format!("@layer {}{{\n{collected_css}\n}}", layer_name(index))
      } else {
        collected_css
      }
    })
    .collect::<Vec<String>>()
    .join("\n");

  let mut sheet = logical_float_vars;
  sheet.push_str(&header);
  sheet.push_str(&collected_css);

  Ok(sheet)
}

/// `.x, .x:root` -> `.x.x, .x.x:root`, by the first class name, as the
/// reference's `'.$1.$1, .$1.$1:root'` replacement reads it.
pub(crate) fn double_theme_root_selector(rule: &str) -> String {
  let mut out = String::with_capacity(rule.len());
  let mut copied = 0;

  // Both groups are set whenever the expression matches (neither is
  // optional), so the chain ends only when there is no match left; a match
  // the engine cannot finish (`Err`) ends it the same way, as `replace_all`
  // would stop there.
  let mut matches = THEME_ROOT_SELECTOR.captures_iter(rule);

  while let Some(Ok(captures)) = matches.next()
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
