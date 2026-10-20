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
//!
//! `read` holds the rules and the config as the reference reads them; this
//! module holds the steps that follow.

use std::borrow::Cow;

use aho_corasick::AhoCorasick;
use indexmap::IndexMap;
use rustc_hash::{FxBuildHasher, FxHashMap};
use stylex_constants::constants::common::{LOGICAL_FLOAT_END_VAR, LOGICAL_FLOAT_START_VAR};
use stylex_js::array_sort::array_tim_sort;
use stylex_js::string::{js_replace_all, js_substitution};
use stylex_js::value::{JsValue, MapKey};
use stylex_regex::regex::{CONSTANT_REFERENCE, THEME_ROOT_SELECTOR};
use stylex_utils::number::is_js_whitespace;

use crate::collation::{Collated, locale_compare, locale_compare_collated};
mod read;

use read::has_no_length;
pub(crate) use read::read_rules;
pub use read::{
  ClassName, LayerOptions, ProcessStylexRulesConfig, ProcessStylexRulesError, Rule, RuleStyle,
  TextDirection,
};

use crate::width_sort_key::{
  WidthSortKey, at_rule_chain_end, at_rule_preludes, compare_width_sort_keys,
  declared_property_name, width_sort_key_for_chain,
};

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

/// Where `text.slice(text.lastIndexOf('{'))` begins: at the last `{`, or, when
/// there is none and `lastIndexOf` answers -1, at the last UTF-16 unit (0 for
/// an empty text). `None` when that unit is the second half of a character
/// outside the Basic Multilingual Plane (an emoji): JavaScript then cuts the
/// character in two, and a `str` cannot hold either half.
fn last_brace_or_last_unit(text: &str) -> Option<usize> {
  if let Some(brace) = text.rfind('{') {
    return Some(brace);
  }

  match text.char_indices().next_back() {
    Some((_, character)) if character.len_utf16() == 2 => None,
    Some((index, _)) => Some(index),
    None => Some(0),
  }
}

impl Rule {
  /// The refusal of a text of this rule that JavaScript would cut in two.
  fn cut_in_two(&self) -> ProcessStylexRulesError {
    ProcessStylexRulesError::CharacterCutInTwo {
      class_name: self.class_name().spelling().into_owned(),
    }
  }
}

// ==================== the constants ====================

/// The constants a sheet declares, by their reference text (`var(--key)`), in
/// declaration order: the reference walks `constsMap` in insertion order, and
/// a constant declared twice keeps its first position under its last value,
/// as `Map.prototype.set` keeps them.
pub type OrderedConstants = IndexMap<String, JsValue, FxBuildHasher>;

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

/// `for (const [key, value] of constsMap.entries()) constsMap.set(key,
/// resolveConstant(value))`: each constant resolved in declaration order. A
/// `set` of a key that is already there changes the value in place, so a
/// constant resolved later reads the resolved value of an earlier one.
fn resolve_constants(
  mut constants: OrderedConstants,
) -> Result<OrderedConstants, ProcessStylexRulesError> {
  for index in 0..constants.len() {
    constants[index] = resolve_constant(&constants[index], &constants, &mut Vec::new())?;
  }

  Ok(constants)
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
    let reference = name.as_str();
    let range = whole.range();

    if visited.iter().any(|seen| seen == reference) {
      return Err(ProcessStylexRulesError::CircularConstant(
        reference.to_string(),
      ));
    }

    let Some(referenced) = constants.get(format!("var({reference})").as_str()) else {
      position = range.end;
      continue;
    };

    let reference = reference.to_string();
    visited.push(reference.clone());
    let replacement = resolve_constant(referenced, constants, visited)?;
    // `result.replace(match[0], replacement.toString())`. The first
    // occurrence of the matched text is this match: the search starts from
    // the front, and an earlier occurrence would have matched first.
    let substitution = js_substitution(
      &replacement.to_js_string(),
      &result[range.clone()],
      &result[..range.start],
      &result[range.end..],
    );
    result.replace_range(range, &substitution);
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

impl<'c> Substitution<'c> {
  fn of(var_ref: &'c str, value: &JsValue) -> Self {
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
      var_ref,
      replacement,
      rename,
    }
  }

  /// `original.replaceAll(varRef, replacement)`, then, with a rename,
  /// `original.replaceAll(`${constName}:`, `${targetName}:`)`.
  fn apply<'t>(&self, text: Cow<'t, str>) -> Cow<'t, str> {
    let text = changed(text, |text| {
      js_replace_all(text, self.var_ref, &self.replacement)
    });

    match &self.rename {
      Some(rename) => changed(text, |text| js_replace_all(text, &rename.from, &rename.to)),
      None => text,
    }
  }
}

/// The substitution of every constant, in declaration order, and one
/// automaton over all of their patterns.
///
/// The reference applies each substitution to each text in turn, and a
/// substitution whose patterns the text does not hold changes nothing. So only
/// the substitutions the automaton finds in a text are applied, in their
/// order; after each one, the text is searched again for the ones that follow
/// it, because a replacement can bring a pattern in or take one out. That is
/// one pass over a text per substitution that applies, where the reference
/// makes two per constant.
struct Substitutions<'c> {
  substitutions: Vec<Substitution<'c>>,
  /// `None` for a few patterns, and when the automaton cannot be built: every
  /// substitution is then tried in turn.
  patterns: Option<Patterns>,
}

/// One automaton over the patterns of every substitution, `var_ref` and
/// `rename.from`, and the substitution each pattern belongs to.
struct Patterns {
  automaton: AhoCorasick,
  owners: Vec<usize>,
}

/// The fewest patterns for which the automaton is built. Below it, a text is
/// searched once per substitution, which costs about as much as one pass of
/// the automaton: four constants with a rename each. The number is a
/// judgement, not a measurement; the bench's constants case has 2,000
/// patterns, where the automaton is five times faster.
const AUTOMATON_MIN_PATTERNS: usize = 8;

/// The most automaton searches for one text. Real rules read one to three
/// constants.
const AUTOMATON_MAX_ROUNDS: usize = 4;

impl<'c> Substitutions<'c> {
  fn of(constants: &'c OrderedConstants) -> Self {
    let substitutions: Vec<Substitution<'c>> = constants
      .iter()
      .map(|(var_ref, value)| Substitution::of(var_ref, value))
      .collect();

    let mut owners = Vec::new();
    let mut patterns = Vec::new();
    for (index, substitution) in substitutions.iter().enumerate() {
      patterns.push(substitution.var_ref);
      owners.push(index);
      if let Some(rename) = &substitution.rename {
        patterns.push(&rename.from);
        owners.push(index);
      }
    }

    let patterns = (patterns.len() >= AUTOMATON_MIN_PATTERNS)
      .then(|| AhoCorasick::new(&patterns).ok())
      .flatten()
      .map(|automaton| Patterns { automaton, owners });

    Substitutions {
      substitutions,
      patterns,
    }
  }

  /// The first substitution, at `from` or after it, that finds a pattern of
  /// its own in `text`.
  fn first_found(&self, text: &str, from: usize) -> Option<usize> {
    match &self.patterns {
      Some(Patterns { automaton, owners }) => automaton
        .find_overlapping_iter(text)
        .map(|found| owners[found.pattern().as_usize()])
        .filter(|&owner| owner >= from)
        .min(),
      None => (from < self.substitutions.len()).then_some(from),
    }
  }

  /// Every substitution applied to `text`, as the reference's loop applies
  /// them. `None` when the text does not change.
  fn apply(&self, text: &str) -> Option<String> {
    let mut text = Cow::Borrowed(text);
    let mut from = 0;
    let mut rounds = 0;

    while let Some(index) = self.first_found(&text, from) {
      text = self.substitutions[index].apply(text);
      from = index + 1;
      rounds += 1;

      // Each round searches the whole text again. A text that reads many
      // constants takes the rest one by one, which costs one search each and
      // writes the same text: a substitution with no pattern in the text
      // changes nothing.
      if rounds == AUTOMATON_MAX_ROUNDS {
        for substitution in &self.substitutions[from..] {
          text = substitution.apply(text);
        }
        break;
      }
    }

    match text {
      Cow::Owned(text) => Some(text),
      Cow::Borrowed(_) => None,
    }
  }
}

/// Every rule with the constants substituted into its texts. A rule whose
/// texts do not change is handed on as it is rather than copied, and most
/// rules are such.
fn substitute_constants<'r>(
  rules: Vec<&'r Rule>,
  constants: &OrderedConstants,
) -> Vec<Cow<'r, Rule>> {
  if constants.is_empty() {
    return rules.into_iter().map(Cow::Borrowed).collect();
  }

  let substitutions = Substitutions::of(constants);

  rules
    .into_iter()
    .map(|rule| {
      // Every pattern the substitution searches for, `var(--x)` and `--x:`,
      // holds `--`.
      if !rule.style().mentions_a_variable() {
        return Cow::Borrowed(rule);
      }

      // `Object.keys(styleObj).forEach`: every text the style holds, `ltr` then
      // `rtl`; a value that is not a string (`rtl: null`) is left alone. No
      // other property is read after this, so none is substituted.
      let substituted = |text: &JsValue| match text {
        JsValue::String(text) => substitutions.apply(text).map(JsValue::String),
        _ => None,
      };
      let ltr = substituted(&rule.style().ltr);
      let rtl = substituted(&rule.style().rtl);

      if ltr.is_none() && rtl.is_none() {
        return Cow::Borrowed(rule);
      }

      let style = rule.style();
      let style = RuleStyle {
        ltr: ltr.unwrap_or_else(|| style.ltr.clone()),
        rtl: rtl.unwrap_or_else(|| style.rtl.clone()),
        const_key: style.const_key.clone(),
        const_val: style.const_val.clone(),
      };

      Cow::Owned(Rule::new(rule.class_name().clone(), style, rule.priority()))
    })
    .collect()
}

// ==================== the sort ====================

/// The media-query width sort keys of the rules: the distinct keys, and for
/// each rule the index of its key. Each key is read once, through a cache
/// keyed on the rule's text up to the end of its at-rule chain: parsing is
/// expensive, and many rules share a chain.
fn width_sort_keys_of<'r>(rules: &'r [Cow<'_, Rule>]) -> (Vec<WidthSortKey>, Vec<Option<usize>>) {
  let mut keys = Vec::new();
  let mut cache: FxHashMap<&'r str, Option<usize>> = FxHashMap::default();

  let of_rule = rules
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

      let chain = &text[..at_rule_chain_end(text)];
      *cache.entry(chain).or_insert_with(|| {
        let key = width_sort_key_for_chain(&at_rule_preludes(text))?;
        keys.push(key);
        Some(keys.len() - 1)
      })
    })
    .collect();

  (keys, of_rule)
}

/// What the comparator reads of a rule's `ltr` text, read once before the sort
/// rather than at each of its O(n log n) comparisons.
struct TextFacts<'r> {
  whole: Collated<'r>,
  property_name: Collated<'r>,
  /// `rule.slice(rule.lastIndexOf('{'))`, or `None` where JavaScript would cut
  /// a character in two, which the comparator refuses when it reaches it.
  block: Option<Collated<'r>>,
}

impl<'r> TextFacts<'r> {
  fn of(text: &'r str) -> Self {
    TextFacts {
      whole: Collated::new(text),
      property_name: Collated::new(declared_property_name(text)),
      block: last_brace_or_last_unit(text).map(|start| Collated::new(&text[start..])),
    }
  }
}

/// One rule as the comparator reads it.
struct SortFacts<'r> {
  rule: &'r Rule,
  width_key: Option<&'r WidthSortKey>,
  /// The refusal of an `ltr` that is not a string, which the comparator
  /// answers only when it reaches the text.
  text: Result<TextFacts<'r>, ProcessStylexRulesError>,
}

impl<'r> SortFacts<'r> {
  fn text(&self) -> Result<&TextFacts<'r>, ProcessStylexRulesError> {
    self.text.as_ref().map_err(Clone::clone)
  }
}

/// The comparator of the sort, answering a number as the reference's does: by
/// priority, then, within a priority, by declared property, width key,
/// declaration block and whole text. [`compare_class_names`] is the one
/// under `use_legacy_classnames_sort`.
///
/// `firstPriority - secondPriority` is `NaN` when either priority is `NaN` or
/// both are the same infinity, and `NaN !== 0` holds: the comparator answers
/// `NaN`, which the sort reads as 0, so the rules tie without the later keys
/// being read. That is not a consistent order, which is why the sort is V8's
/// own.
fn compare_rules(a: &SortFacts<'_>, b: &SortFacts<'_>) -> Result<f64, ProcessStylexRulesError> {
  let priority_comparison = a.rule.priority() - b.rule.priority();

  if priority_comparison != 0.0 {
    return Ok(priority_comparison);
  }

  let rule1 = a.text()?;
  let rule2 = b.text()?;

  let name_comparison = locale_compare_collated(rule1.property_name, rule2.property_name);

  if name_comparison != 0.0 {
    return Ok(name_comparison);
  }

  // Only rules for the same property compete in the cascade, so the
  // breakpoint order applies within a property, after it. Ordering by width
  // first would decide some pairs by width and others by declaration text,
  // which is not a consistent ordering.
  let mq_comparison = compare_width_sort_keys(a.width_key, b.width_key);

  if mq_comparison != 0.0 {
    return Ok(mq_comparison);
  }

  // `rule.slice(rule.lastIndexOf('{'))`: the declaration block, or, for a text
  // with no block, its last character.
  let block1 = rule1.block.ok_or_else(|| a.rule.cut_in_two())?;
  let block2 = rule2.block.ok_or_else(|| b.rule.cut_in_two())?;
  let property_comparison = locale_compare_collated(block1, block2);

  if property_comparison != 0.0 {
    return Ok(property_comparison);
  }

  Ok(locale_compare_collated(rule1.whole, rule2.whole))
}

/// The comparator under `use_legacy_classnames_sort`: by priority, then by
/// class name. It reads no text, so it needs no facts.
fn compare_class_names(a: &Rule, b: &Rule) -> Result<f64, ProcessStylexRulesError> {
  let priority_comparison = a.priority() - b.priority();

  if priority_comparison != 0.0 {
    return Ok(priority_comparison);
  }

  // `classname1.localeCompare(classname2)`: the argument is spelled with
  // `String(...)`.
  let classname1 = a.class_name().as_receiver()?;

  Ok(locale_compare(classname1, &b.class_name().spelling()))
}

/// `sortedRules`: the rules in the sheet's order, sorted as V8 sorts them.
fn sort_rules<'a>(
  rules: &'a [Cow<'_, Rule>],
  use_legacy_classnames_sort: bool,
) -> Result<Vec<&'a Rule>, ProcessStylexRulesError> {
  let mut order: Vec<usize> = (0..rules.len()).collect();

  if use_legacy_classnames_sort {
    array_tim_sort(&mut order, |&i, &j| {
      compare_class_names(&rules[i], &rules[j])
    })?;

    return Ok(order.into_iter().map(|i| rules[i].as_ref()).collect());
  }

  let (width_sort_keys, key_of_rule) = width_sort_keys_of(rules);
  let facts: Vec<SortFacts<'_>> = rules
    .iter()
    .zip(&key_of_rule)
    .map(|(rule, key)| SortFacts {
      rule,
      width_key: key.map(|index| &width_sort_keys[index]),
      text: rule.ltr().map(TextFacts::of),
    })
    .collect();

  array_tim_sort(&mut order, |&i, &j| compare_rules(&facts[i], &facts[j]))?;

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

/// `text` changed by `change`, or `text` itself when `change` leaves it as it
/// is: most texts are written as they arrive.
fn changed<'a>(text: Cow<'a, str>, change: impl FnOnce(&str) -> Cow<'_, str>) -> Cow<'a, str> {
  let owned = match change(&text) {
    Cow::Owned(changed) => Some(changed),
    Cow::Borrowed(_) => None,
  };
  owned.map_or(text, Cow::Owned)
}

/// [`insert_specificity`] on a text of `rule`, refused where JavaScript would
/// cut a character in two.
fn with_specificity<'a>(
  text: Cow<'a, str>,
  pseudo: &str,
  rule: &Rule,
) -> Result<Cow<'a, str>, ProcessStylexRulesError> {
  let owned = match insert_specificity(&text, pseudo) {
    Some(Cow::Owned(changed)) => Some(changed),
    Some(Cow::Borrowed(_)) => None,
    None => return Err(rule.cut_in_two()),
  };
  Ok(owned.map_or(text, Cow::Owned))
}

/// Writes the lines one style writes: its LTR text, and, when it has an RTL
/// text, both texts under the two direction selectors or between the `@ltr`
/// and `@rtl` comments. `specificity` is the polyfill of the style's group,
/// when the sheet has one.
fn write_style(
  out: &mut String,
  rule: &Rule,
  specificity: Option<&str>,
  enable_ltr_rtl_comments: bool,
) -> Result<(), ProcessStylexRulesError> {
  let mut ltr_rule = Cow::Borrowed(rule.ltr()?);
  // An empty RTL text is as absent as a missing one (`rtlRule &&`,
  // `if (rtlRule)`, `rtlRule ?` all read it as false).
  let mut rtl_rule = rule.rtl()?.map(Cow::Borrowed);

  if let Some(pseudo) = specificity {
    ltr_rule = with_specificity(ltr_rule, pseudo, rule)?;
    rtl_rule = match rtl_rule {
      Some(rtl) => Some(with_specificity(rtl, pseudo, rule)?),
      None => None,
    };
  }

  // check if the selector looks like .xtrlmmh, .xtrlmmh:root
  // if so, turn it into .xtrlmmh.xtrlmmh, .xtrlmmh.xtrlmmh:root
  // This is to ensure the themes always have precedence over the
  // default variable values
  let ltr_rule = changed(ltr_rule, double_theme_root_selector);
  let rtl_rule = rtl_rule.map(|rtl| changed(rtl, double_theme_root_selector));

  match rtl_rule {
    Some(rtl_rule) if enable_ltr_rtl_comments => {
      out.push_str("/* @ltr begin */");
      out.push_str(&ltr_rule);
      out.push_str("/* @ltr end */\n/* @rtl begin */");
      out.push_str(&rtl_rule);
      out.push_str("/* @rtl end */");
    },
    Some(rtl_rule) => {
      write_with_ancestor(out, &ltr_rule, "html:not([dir='rtl'])");
      out.push('\n');
      write_with_ancestor(out, &rtl_rule, "html[dir='rtl']");
    },
    None => out.push_str(&ltr_rule),
  }

  Ok(())
}

/// Writes one priority group: its styles' lines, inside a `@layer` block when
/// layers are on and the group holds styles (a group of priority 0,
/// `@property`, `@keyframes`, `@position-try`, goes in no layer).
fn write_group(
  out: &mut String,
  group: &[&Rule],
  index: usize,
  config: &ProcessStylexRulesConfig,
  layers: &Layers<'_>,
) -> Result<(), ProcessStylexRulesError> {
  // A group is never empty: `group_by_priority_level` opens one with a rule.
  let pri = group[0].priority();
  // Don't put @property, @keyframe, @position-try in layers
  let in_layer = layers.on && pri > 0.0;
  // The polyfill: `index` copies of `:not(#\#)`, the same for every style of
  // the group.
  let specificity =
    (!layers.on && !config.legacy_disable_layers).then(|| SPECIFICITY_LEVEL.repeat(index));

  if in_layer {
    out.push_str("@layer ");
    out.push_str(&layers.name(index));
    out.push_str("{\n");
  }

  for (position, rule) in deduplicate_by_class_name(group).into_iter().enumerate() {
    if position > 0 {
      out.push('\n');
    }
    write_style(
      out,
      rule,
      specificity.as_deref(),
      config.enable_ltr_rtl_comments,
    )?;
  }

  if in_layer {
    out.push_str("\n}");
  }

  Ok(())
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
///
/// This is the entry the addon calls. A slice has no holes, so an empty slice
/// is the reference's `rules.length === 0`, and the sheet is empty.
/// [`process_stylex_rule_values`] reads JavaScript values into rules first,
/// and then shares every step after the early return.
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
  let constants = resolve_constants(constants)?;
  let substituted = substitute_constants(non_constant_rules, &constants);
  let sorted_rules = sort_rules(&substituted, config.use_legacy_classnames_sort)?;
  let grouped = group_by_priority_level(&sorted_rules)?;
  let layers = Layers::of(config);

  // Each rule is written once, or twice with an RTL text, with a few
  // characters added.
  let size: usize = sorted_rules
    .iter()
    .map(|rule| text_length(&rule.style().ltr) + 2 * text_length(&rule.style().rtl))
    .sum();
  let mut sheet = String::with_capacity(size + size / 4);

  sheet.push_str(&get_logical_float_vars(sorted_rules.iter().copied()));
  sheet.push_str(&layers.statement(grouped.len()));

  for (index, group) in grouped.iter().enumerate() {
    if index > 0 {
      sheet.push('\n');
    }
    write_group(&mut sheet, group, index, config, &layers)?;
  }

  Ok(sheet)
}

/// The length of a text in bytes, 0 for a value that is not a string.
fn text_length(text: &JsValue) -> usize {
  match text {
    JsValue::String(text) => text.len(),
    _ => 0,
  }
}

// ==================== the selectors ====================

/// One level of the specificity polyfill.
const SPECIFICITY_LEVEL: &str = ":not(#\\#)";

/// `addAncestorSelector`: `ancestor_selector` prefixed to the rule's own
/// selector, inside the innermost at-rule when the rule sits in one. A
/// `@keyframes` block holds no selector to prefix.
#[cfg(test)]
pub(crate) fn add_ancestor_selector(selector: &str, ancestor_selector: &str) -> String {
  let mut out = String::new();
  write_with_ancestor(&mut out, selector, ancestor_selector);
  out
}

/// [`add_ancestor_selector`], written into `out`.
fn write_with_ancestor(out: &mut String, selector: &str, ancestor_selector: &str) {
  if selector.starts_with("@keyframes") {
    out.push_str(selector);
    return;
  }

  // `rfind` on a text that opens with `@` answers at or after 0; the `None`
  // arm is folded into the "no at-rule" arm rather than left as a branch
  // nothing can reach.
  let split = match (selector.starts_with('@'), selector.rfind('@')) {
    // `indexOf('{', lastAtRule)` is -1 when no brace follows, and the two
    // slices then split at 0: the ancestor goes in front of the whole text.
    (true, Some(last_at_rule)) => selector[last_at_rule..]
      .find('{')
      .map_or(0, |brace| last_at_rule + brace + 1),
    _ => 0,
  };

  out.push_str(&selector[..split]);
  out.push_str(ancestor_selector);
  out.push(' ');
  out.push_str(&selector[split..]);
}

/// `addSpecificityLevel`: `index` copies of `:not(#\#)` on the rule's selector,
/// the `@layer` polyfill for a sheet written without layers. They go before a
/// pseudo-element, else before the declaration block, else (a text with no
/// block) before its last character. `None` where JavaScript would cut that
/// character in two ([`last_brace_or_last_unit`]).
#[cfg(test)]
pub(crate) fn add_specificity_level(selector: &str, index: usize) -> Option<String> {
  insert_specificity(selector, &SPECIFICITY_LEVEL.repeat(index)).map(Cow::into_owned)
}

/// [`add_specificity_level`] with the copies already spelled. With no copy, as
/// in the first group, the two halves join back into the text as it was, even
/// where JavaScript cuts a character in two.
fn insert_specificity<'s>(selector: &'s str, pseudo: &str) -> Option<Cow<'s, str>> {
  if pseudo.is_empty() || selector.starts_with("@keyframes") {
    return Some(Cow::Borrowed(selector));
  }

  let split = match selector.find("::") {
    Some(pseudo_element) => pseudo_element,
    None => last_brace_or_last_unit(selector)?,
  };
  let mut out = String::with_capacity(selector.len() + pseudo.len());
  out.push_str(&selector[..split]);
  out.push_str(pseudo);
  out.push_str(&selector[split..]);

  Some(Cow::Owned(out))
}

/// `.x, .x:root` -> `.x.x, .x.x:root`, by the first class name, as the
/// reference's `'.$1.$1, .$1.$1:root'` replacement reads it. A text with no
/// such selector, nearly every one, is handed back as it is.
pub(crate) fn double_theme_root_selector(rule: &str) -> Cow<'_, str> {
  // Every match holds `:root`, and nearly no text does.
  if !rule.contains(":root") {
    return Cow::Borrowed(rule);
  }

  let mut out: Option<String> = None;
  let mut copied = 0;
  let mut matches = THEME_ROOT_SELECTOR.captures_iter(rule);

  // Both groups are set in every match (group 0 is the match itself, group 1
  // is not optional), so the chain ends only when no match is left.
  while let Some(captures) = matches.next()
    && let (Some(whole), Some(class_name)) = (captures.get(0), captures.get(1))
  {
    let class_name = class_name.as_str();
    let out = out.get_or_insert_with(|| String::with_capacity(rule.len() * 2));
    out.push_str(&rule[copied..whole.start()]);
    for piece in [
      ".", class_name, ".", class_name, ", .", class_name, ".", class_name, ":root",
    ] {
      out.push_str(piece);
    }
    copied = whole.end();
  }

  match out {
    Some(mut out) => {
      out.push_str(&rule[copied..]);
      Cow::Owned(out)
    },
    None => Cow::Borrowed(rule),
  }
}

#[cfg(test)]
#[path = "tests/process_test.rs"]
mod process_test;
