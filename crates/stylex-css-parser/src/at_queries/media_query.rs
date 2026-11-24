/*!
Media query parsing and representation.

Core functionality for parsing and representing CSS media queries.
*/

use stylex_utils::number::{to_js_string, write_js_number};

use crate::{
  CssParseError,
  css_types::{Length, calc::Calc},
  token_parser::{TokenParser, tokens},
  token_types::{SimpleToken, TokenList},
};
use std::fmt::{self, Display};

/// Fraction type for media query values like (aspect-ratio: 16/9)
///
/// Held as doubles, not integers. A media fraction is a *ratio* rather than a
/// count: CSS admits `(aspect-ratio: 16.5/9)`, and the reference compiler's
/// `mediaRuleValueParser` keeps both halves as `number`. An `i32` truncated the
/// numerator -- `16.5/9` printed as `16 / 9` -- and saturated anything past
/// `i32::MAX`, so `1e30/1` printed as `2147483647 / 1`.
///
/// Reachable, and that is the point: every `@media` key nested one level down is
/// re-parsed and reprinted through [`super::media_query_transform`], including
/// the case where there is nothing to negate and the query comes back
/// unchanged. So the truncation was not confined to a parser nobody calls; it
/// reached the stylesheet.
#[derive(Debug, Clone, PartialEq)]
pub struct Fraction {
  pub numerator: f64,
  pub denominator: f64,
}

#[cfg_attr(coverage_nightly, coverage(off))]
impl Display for Fraction {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    // Format with spaces for consistent output, and through `to_js_string` for
    // the same reason every other number in this crate goes through it: a
    // fraction's halves are authored numbers and must be spelled the way
    // JavaScript spells them.
    write!(
      f,
      "{} / {}",
      to_js_string(self.numerator),
      to_js_string(self.denominator)
    )
  }
}

/// Word rule types for media queries
#[derive(Debug, Clone, PartialEq)]
pub enum WordRule {
  Color,
  Monochrome,
  Grid,
  ColorIndex,
}

#[cfg_attr(coverage_nightly, coverage(off))]
impl Display for WordRule {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    match self {
      WordRule::Color => write!(f, "color"),
      WordRule::Monochrome => write!(f, "monochrome"),
      WordRule::Grid => write!(f, "grid"),
      WordRule::ColorIndex => write!(f, "color-index"),
    }
  }
}

/// Media rule values that can appear in media queries
#[derive(Debug, Clone, PartialEq)]
pub enum MediaRuleValue {
  Number(f64),
  Length(Length),
  String(String),
  Fraction(Fraction),
}

#[cfg_attr(coverage_nightly, coverage(off))]
impl Display for MediaRuleValue {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    match self {
      MediaRuleValue::Number(n) => write_js_number(f, *n),
      MediaRuleValue::Length(l) => write!(f, "{}", l),
      MediaRuleValue::String(s) => write!(f, "{}", s),
      MediaRuleValue::Fraction(frac) => write!(f, "{}", frac),
    }
  }
}

/// Media keyword for CSS media queries
#[derive(Debug, Clone, PartialEq)]
pub struct MediaKeyword {
  pub r#type: String, // Always "media-keyword"
  pub key: String,    // 'screen' | 'print' | 'all'
  pub not: bool,
  pub only: bool, // Boolean field for CSS media queries
}

impl MediaKeyword {
  pub fn new(key: impl Into<String>, not: bool, only: bool) -> Self {
    Self {
      r#type: "media-keyword".to_string(),
      key: key.into(),
      not,
      only,
    }
  }
}

#[cfg_attr(coverage_nightly, coverage(off))]
impl Display for MediaKeyword {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    let mut parts = Vec::new();

    if self.not {
      parts.push("not".to_string());
    }

    if self.only {
      parts.push("only".to_string());
    }

    parts.push(self.key.clone());
    write!(f, "{}", parts.join(" "))
  }
}

/// Media word rule for CSS media queries
#[derive(Debug, Clone, PartialEq)]
pub struct MediaWordRule {
  pub r#type: String,    // Always "word-rule"
  pub key_value: String, // The word rule value
}

impl MediaWordRule {
  pub fn new(key_value: impl Into<String>) -> Self {
    Self {
      r#type: "word-rule".to_string(),
      key_value: key_value.into(),
    }
  }
}

#[cfg_attr(coverage_nightly, coverage(off))]
impl Display for MediaWordRule {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "({})", self.key_value)
  }
}

/// Media rule pair for CSS media queries
#[derive(Debug, Clone, PartialEq)]
pub struct MediaRulePair {
  #[allow(dead_code)]
  pub r#type: String, // Always "pair"
  pub key: String,           // Property name
  pub value: MediaRuleValue, // Property value
}

impl MediaRulePair {
  pub fn new(key: impl Into<String>, value: MediaRuleValue) -> Self {
    Self {
      r#type: "pair".to_string(),
      key: key.into(),
      value,
    }
  }
}

#[cfg_attr(coverage_nightly, coverage(off))]
impl Display for MediaRulePair {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "({}: {})", self.key, self.value)
  }
}

/// Media NOT rule for CSS media queries
#[derive(Debug, Clone, PartialEq)]
pub struct MediaNotRule {
  #[allow(dead_code)]
  pub r#type: String, // Always "not"
  pub rule: Box<MediaQueryRule>, // Nested rule
}

impl MediaNotRule {
  pub fn new(rule: MediaQueryRule) -> Self {
    Self {
      r#type: "not".to_string(),
      rule: Box::new(rule),
    }
  }
}

#[cfg_attr(coverage_nightly, coverage(off))]
impl Display for MediaNotRule {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    match self.rule.as_ref() {
      MediaQueryRule::And(_) | MediaQueryRule::Or(_) => {
        write!(f, "(not ({}))", self.rule)
      },
      _ => {
        write!(f, "(not {})", self.rule)
      },
    }
  }
}

/// Media AND rules for CSS media queries
#[derive(Debug, Clone, PartialEq)]
pub struct MediaAndRules {
  pub r#type: String,             // Always "and"
  pub rules: Vec<MediaQueryRule>, // Array of rules
}

impl MediaAndRules {
  pub fn new(rules: Vec<MediaQueryRule>) -> Self {
    Self {
      r#type: "and".to_string(),
      rules,
    }
  }
}

#[cfg_attr(coverage_nightly, coverage(off))]
impl Display for MediaAndRules {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    let rule_strings: Vec<String> = self.rules.iter().map(|rule| rule.to_string()).collect();
    write!(f, "{}", rule_strings.join(" and "))
  }
}

/// Media OR rules for CSS media queries
#[derive(Debug, Clone, PartialEq)]
pub struct MediaOrRules {
  pub r#type: String,             // Always "or"
  pub rules: Vec<MediaQueryRule>, // Array of rules
}

impl MediaOrRules {
  pub fn new(rules: Vec<MediaQueryRule>) -> Self {
    Self {
      r#type: "or".to_string(),
      rules,
    }
  }
}

#[cfg_attr(coverage_nightly, coverage(off))]
impl Display for MediaOrRules {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    let rule_strings: Vec<String> = self.rules.iter().map(|rule| rule.to_string()).collect();
    write!(f, "{}", rule_strings.join(" or "))
  }
}

/// All media query rules for CSS media queries
#[derive(Debug, Clone, PartialEq)]
#[allow(dead_code)]
pub enum MediaQueryRule {
  MediaKeyword(MediaKeyword),
  WordRule(MediaWordRule),
  Pair(MediaRulePair),
  Not(MediaNotRule),
  And(MediaAndRules),
  Or(MediaOrRules),
}

/// Main MediaQuery struct for CSS media queries
#[derive(Debug, Clone, PartialEq)]
pub struct MediaQuery {
  pub queries: MediaQueryRule,
}

impl MediaQuery {
  pub fn new(queries: MediaQueryRule) -> Self {
    Self {
      queries: resolve_media_types_within_budget(Self::normalize(queries)),
    }
  }

  /// Takes `rule` by value and consumes each combinator's children, so a tree
  /// is normalized without being deep-cloned once per level of its own depth.
  pub fn normalize(rule: MediaQueryRule) -> MediaQueryRule {
    match rule {
      MediaQueryRule::And(and_rules) => {
        let mut flattened: Vec<MediaQueryRule> = Vec::with_capacity(and_rules.rules.len());
        for r in and_rules.rules {
          match Self::normalize(r) {
            MediaQueryRule::And(inner_and) => {
              flattened.extend(inner_and.rules);
            },
            norm => {
              flattened.push(norm);
            },
          }
        }

        let merged = merge_and_simplify_ranges(flattened);
        if merged.is_empty() {
          return MediaQueryRule::MediaKeyword(MediaKeyword::new("all".to_string(), true, false));
        }
        MediaQueryRule::And(MediaAndRules::new(merged))
      },
      MediaQueryRule::Or(or_rules) => {
        let normalized_rules: Vec<MediaQueryRule> =
          or_rules.rules.into_iter().map(Self::normalize).collect();
        MediaQueryRule::Or(MediaOrRules::new(normalized_rules))
      },
      MediaQueryRule::Not(not_rule) => {
        let normalized_operand = Self::normalize(*not_rule.rule);

        match normalized_operand {
          MediaQueryRule::MediaKeyword(ref keyword) if keyword.key == "all" && keyword.not => {
            return MediaQueryRule::MediaKeyword(MediaKeyword::new(
              "all".to_string(),
              false,
              false,
            ));
          },
          MediaQueryRule::Not(inner_not) => {
            return Self::normalize(*inner_not.rule);
          },
          _ => {},
        }

        MediaQueryRule::Not(MediaNotRule::new(normalized_operand))
      },
      other => other,
    }
  }
}

// StyleX knows the `screen`, `print` and `all` media types. A media type is
// not a boolean feature: browsers do not match `(print)` or `(not (print))`.
// The helpers below change negated and combined media types into one positive
// media type for each query. For example, `(max-width: 900px) and (not print)`
// becomes `screen and (max-width: 900px)`.

/// The media type that `not <key>` matches, or `None` when it matches nothing
/// (`not all`).
fn complement_media_type(key: &str) -> Option<&'static str> {
  match key {
    "print" => Some("screen"),
    "screen" => Some("print"),
    _ => None,
  }
}

/// A query that never matches.
fn never_rule() -> MediaQueryRule {
  MediaQueryRule::MediaKeyword(MediaKeyword::new("all", true, false))
}

/// A query that always matches.
fn always_rule() -> MediaQueryRule {
  MediaQueryRule::MediaKeyword(MediaKeyword::new("all", false, false))
}

fn is_never_rule(rule: &MediaQueryRule) -> bool {
  matches!(rule, MediaQueryRule::MediaKeyword(keyword) if keyword.key == "all" && keyword.not)
}

fn is_always_rule(rule: &MediaQueryRule) -> bool {
  matches!(
    rule,
    MediaQueryRule::MediaKeyword(keyword) if keyword.key == "all" && !keyword.not && !keyword.only
  )
}

fn negate_rule(rule: MediaQueryRule) -> MediaQueryRule {
  match rule {
    MediaQueryRule::MediaKeyword(keyword) => MediaQueryRule::MediaKeyword(MediaKeyword {
      not: !keyword.not,
      ..keyword
    }),
    MediaQueryRule::Not(not_rule) => *not_rule.rule,
    MediaQueryRule::And(and_rules) => MediaQueryRule::Or(MediaOrRules::new(
      and_rules.rules.into_iter().map(negate_rule).collect(),
    )),
    MediaQueryRule::Or(or_rules) => MediaQueryRule::And(MediaAndRules::new(
      or_rules.rules.into_iter().map(negate_rule).collect(),
    )),
    other => MediaQueryRule::Not(MediaNotRule::new(other)),
  }
}

/// Whether a subtree refers to a media type. A subtree without one has nothing
/// to resolve, so it keeps its shape (for example, a plain width range).
fn contains_media_keyword(node: &MediaQueryRule) -> bool {
  match node {
    MediaQueryRule::MediaKeyword(_) => true,
    MediaQueryRule::Not(not_rule) => contains_media_keyword(&not_rule.rule),
    MediaQueryRule::And(MediaAndRules { rules, .. })
    | MediaQueryRule::Or(MediaOrRules { rules, .. }) => rules.iter().any(contains_media_keyword),
    _ => false,
  }
}

/// Whether a rule is a query with a leading `not`, such as
/// `not screen and (min-width: 1px)`: a negated `and` list that starts with a
/// positive media type and has no other media type.
fn is_leading_not_query(node: &MediaQueryRule) -> bool {
  if let MediaQueryRule::Not(not_rule) = node
    && let MediaQueryRule::And(and_rules) = not_rule.rule.as_ref()
    && let Some((MediaQueryRule::MediaKeyword(first), rest)) = and_rules.rules.split_first()
  {
    !first.not && !rest.iter().any(contains_media_keyword)
  } else {
    false
  }
}

/// The media type that a media keyword requires, or `None` when the keyword
/// never matches (`not all`).
fn media_type_requirement(keyword: &MediaKeyword) -> Option<&str> {
  if keyword.not {
    complement_media_type(&keyword.key)
  } else {
    Some(&keyword.key)
  }
}

/// The branches of an `or`, or the rule itself.
fn flatten_or_rule(rule: MediaQueryRule) -> Vec<MediaQueryRule> {
  match rule {
    MediaQueryRule::Or(or_rules) => or_rules.rules,
    other => vec![other],
  }
}

/// The media type that the keywords of an `and` list require.
enum AndMediaType<'a> {
  /// The keywords contradict each other, so the list never matches.
  Never,
  /// The keywords accept any media type.
  Any,
  /// The keywords accept this media type only.
  Only(&'a str),
}

/// The media type that the keywords of an `and` list require.
fn and_media_type(rules: &[MediaQueryRule]) -> AndMediaType<'_> {
  let mut media_type = None;
  for rule in rules {
    let MediaQueryRule::MediaKeyword(keyword) = rule else {
      continue;
    };
    let Some(required) = media_type_requirement(keyword) else {
      return AndMediaType::Never;
    };
    if required == "all" {
      continue;
    }
    match media_type {
      Some(current) if current != required => return AndMediaType::Never,
      Some(_) => {},
      None => media_type = Some(required),
    }
  }
  media_type.map_or(AndMediaType::Any, AndMediaType::Only)
}

/// Whether a branch is a keyword that cannot match together with
/// `media_type`.
fn contradicts_media_type(branch: &MediaQueryRule, media_type: &str) -> bool {
  matches!(
    branch,
    MediaQueryRule::MediaKeyword(keyword) if media_type_requirement(keyword)
      .is_none_or(|required| required != "all" && required != media_type)
  )
}

/// The number of nodes in a rule tree.
fn node_count(rule: &MediaQueryRule) -> u64 {
  match rule {
    MediaQueryRule::Not(not_rule) => node_count(&not_rule.rule).saturating_add(1),
    MediaQueryRule::And(MediaAndRules { rules, .. })
    | MediaQueryRule::Or(MediaOrRules { rules, .. }) => {
      rules.iter().map(node_count).fold(1, u64::saturating_add)
    },
    _ => 1,
  }
}

/// Returns `empty` for no rules, the rule itself for one rule, and `wrap` of
/// the list for more.
fn single_or_wrap(
  mut rules: Vec<MediaQueryRule>,
  empty: fn() -> MediaQueryRule,
  wrap: fn(Vec<MediaQueryRule>) -> MediaQueryRule,
) -> MediaQueryRule {
  if rules.len() > 1 {
    return wrap(rules);
  }
  rules.pop().unwrap_or_else(empty)
}

/// The work that is left for [`resolve_media_types`], in copied nodes. When
/// a cost goes past what is left, the budget is gone and stays gone.
struct WorkBudget {
  left: Option<u64>,
}

impl WorkBudget {
  fn new(left: u64) -> Self {
    Self { left: Some(left) }
  }

  /// Takes `cost` from the budget. Returns `false` when the budget is gone.
  fn spend(&mut self, cost: u64) -> bool {
    self.left = self.left.and_then(|left| left.checked_sub(cost));
    self.left.is_some()
  }

  fn is_gone(&self) -> bool {
    self.left.is_none()
  }
}

/// The number of nodes in a query that cannot split, or `None` when it can.
///
/// A split happens only for an `or` in an `and` list, and a `not` over an
/// `and` or an `or` can become such an `or`. Without a split, each `and` list
/// is resolved in one pass that pays once for each of its items, so the work
/// is not more than the number of nodes.
fn split_free_node_count(rule: &MediaQueryRule, in_and: bool) -> Option<u64> {
  let count_all = |rules: &[MediaQueryRule], in_and: bool| {
    rules.iter().try_fold(1u64, |count, rule| {
      count.checked_add(split_free_node_count(rule, in_and)?)
    })
  };
  match rule {
    MediaQueryRule::Or(MediaOrRules { rules, .. }) if !in_and => count_all(rules, false),
    MediaQueryRule::And(MediaAndRules { rules, .. }) => count_all(rules, true),
    MediaQueryRule::Not(not_rule) => match not_rule.rule.as_ref() {
      MediaQueryRule::And(_) | MediaQueryRule::Or(_) => None,
      inner => split_free_node_count(inner, in_and)?.checked_add(1),
    },
    MediaQueryRule::Or(_) => None,
    _ => Some(1),
  }
}

/// Resolves the media types of a normalized query, with a limit on the work.
///
/// A split can double the query for each `or` in an `and` list, and the
/// reference implementation does not stop. Each split pays for the nodes
/// that it copies, from one budget of `MAX_DISTRIBUTION_NODES` for the query.
/// If the budget is gone, the query keeps its normalized rules, as the range
/// merge does. The query is then not partly resolved. A split that is small in
/// fact (for example, when most branches contradict each other) stays in the
/// budget, though the number of `or`s is large.
fn resolve_media_types_within_budget(node: MediaQueryRule) -> MediaQueryRule {
  if !contains_media_keyword(&node) {
    return node;
  }
  let mut budget = WorkBudget::new(MAX_DISTRIBUTION_NODES);
  // The work of a query that cannot split fits in the budget, so the query
  // needs no copy to fall back to.
  if split_free_node_count(&node, false).is_some_and(|count| count <= MAX_DISTRIBUTION_NODES) {
    return resolve_media_types(node, true, &mut budget);
  }
  let resolved = resolve_media_types(node.clone(), true, &mut budget);
  if budget.is_gone() { node } else { resolved }
}

/// Rewrites a normalized media query so that its media types are valid CSS:
/// one positive media type for each query. It splits an `and` that holds an
/// `or` into one `and` for each branch, and removes the branches that never
/// match.
///
/// `is_query_list` is true for the queries of the list itself. A query there
/// with a leading `not` is already valid CSS, so it stays as written.
fn resolve_media_types(
  node: MediaQueryRule,
  is_query_list: bool,
  budget: &mut WorkBudget,
) -> MediaQueryRule {
  // Past the budget, the caller discards the result, so stop the work.
  if budget.is_gone()
    || (is_query_list && is_leading_not_query(&node))
    || !contains_media_keyword(&node)
  {
    return node;
  }
  match node {
    MediaQueryRule::Or(or_rules) => {
      let mut rules: Vec<MediaQueryRule> = Vec::with_capacity(or_rules.rules.len());
      for child in or_rules.rules {
        rules.extend(
          flatten_or_rule(resolve_media_types(child, is_query_list, budget))
            .into_iter()
            .filter(|rule| !is_never_rule(rule)),
        );
      }
      if rules.iter().any(is_always_rule) {
        return always_rule();
      }
      single_or_wrap(rules, never_rule, |rules| {
        MediaQueryRule::Or(MediaOrRules::new(rules))
      })
    },
    MediaQueryRule::And(and_rules) => resolve_and_rules(and_rules.rules, budget),
    // The `not` moves inward before the operand is resolved. Resolving first
    // splits the operand over each `or` it holds, and the negation of that
    // split is much larger than the split itself.
    MediaQueryRule::Not(not_rule) => match *not_rule.rule {
      inner
      @ (MediaQueryRule::MediaKeyword(_) | MediaQueryRule::And(_) | MediaQueryRule::Or(_)) => {
        resolve_media_types(negate_rule(inner), false, budget)
      },
      inner => MediaQueryRule::Not(MediaNotRule::new(resolve_media_types(inner, false, budget))),
    },
    other => other,
  }
}

/// Resolves the media types of an `and` list. See [`resolve_media_types`].
fn resolve_and_rules(mut rules: Vec<MediaQueryRule>, budget: &mut WorkBudget) -> MediaQueryRule {
  // Each pass resolves the list once. A split over a one-branch `or` starts
  // the next pass and does not call this function again, so a long ladder
  // does not deepen the stack.
  loop {
    // A pass resolves each item again, so it pays for each item.
    if !budget.spend(rules.len() as u64) {
      return never_rule();
    }
    // A nested `and` is flattened, so that a media type in it can move to the
    // front of the list.
    let mut resolved: Vec<MediaQueryRule> = Vec::with_capacity(rules.len());
    for rule in rules {
      match resolve_media_types(rule, false, budget) {
        MediaQueryRule::And(and_rules) => resolved.extend(and_rules.rules),
        other => resolved.push(other),
      }
    }

    // The list is split over its first `or`. The later `or`s stay whole: each
    // one is split in a later pass, with its branch first.
    let mut nested_or: Option<MediaOrRules> = None;
    let mut rest: Vec<MediaQueryRule> = Vec::with_capacity(resolved.len());
    for rule in resolved {
      match rule {
        MediaQueryRule::Or(or_rules) if nested_or.is_none() => nested_or = Some(or_rules),
        rule => rest.push(rule),
      }
    }

    // Contradicting keywords put a contradiction into every branch of the
    // split below, so the list never matches. Stop before the split. All the
    // keywords are in `rest`, because the `or` is not a keyword.
    let media_type = match and_media_type(&rest) {
      AndMediaType::Never => return never_rule(),
      AndMediaType::Any => None,
      AndMediaType::Only(media_type) => Some(media_type),
    };

    if let Some(mut or_rules) = nested_or {
      // Each branch gets its own media type. A bare keyword branch that
      // contradicts the list never matches, so it is removed before the
      // split. In a breakpoint ladder, this leaves one branch.
      if let Some(media_type) = media_type {
        or_rules
          .rules
          .retain(|branch| !contradicts_media_type(branch, media_type));
        if or_rules.rules.is_empty() {
          return never_rule();
        }
      }

      // Each branch goes first in its own copy of the rest of the list.
      match <[MediaQueryRule; 1]>::try_from(or_rules.rules) {
        Ok([branch]) => {
          rest.insert(0, branch);
          // The reference resolves the new list as `or(and(list))`. A list
          // without a media type comes back unchanged, still in that `or`.
          if !rest.iter().any(contains_media_keyword) {
            return MediaQueryRule::Or(MediaOrRules::new(vec![MediaQueryRule::And(
              MediaAndRules::new(rest),
            )]));
          }
          rules = rest;
          continue;
        },
        Err(branches) => {
          let copied_nodes = rest.iter().map(node_count).fold(1, u64::saturating_add);
          if !budget.spend(copied_nodes.saturating_mul(branches.len() as u64)) {
            // The caller discards the result.
            return never_rule();
          }
          let branches = branches
            .into_iter()
            .map(|branch| {
              let mut rules = Vec::with_capacity(rest.len() + 1);
              rules.push(branch);
              rules.extend(rest.iter().cloned());
              MediaQueryRule::And(MediaAndRules::new(rules))
            })
            .collect();
          return resolve_media_types(
            MediaQueryRule::Or(MediaOrRules::new(branches)),
            false,
            budget,
          );
        },
      }
    }

    // The first positive keyword starts the query (the anchor). It is kept
    // as written, so that `only` stays. A list without an anchor gets a new
    // keyword for its media type. That keyword is made before `rest` moves,
    // because the media type is a part of `rest`.
    let anchor_index = rest.iter().position(|rule| {
      matches!(rule, MediaQueryRule::MediaKeyword(keyword) if !keyword.not && keyword.key != "all")
    });
    let mut anchor_node = match anchor_index {
      Some(_) => None,
      None => media_type.map(|media_type| MediaKeyword::new(media_type, false, false)),
    };
    // One more slot for a new anchor, which `rest` does not hold.
    let mut rules: Vec<MediaQueryRule> = Vec::with_capacity(rest.len() + 1);
    for (index, rule) in rest.into_iter().enumerate() {
      match rule {
        MediaQueryRule::MediaKeyword(keyword) => {
          if anchor_index == Some(index) {
            anchor_node = Some(keyword);
          }
        },
        other => rules.push(other),
      }
    }
    if let Some(anchor_node) = anchor_node {
      rules.insert(0, MediaQueryRule::MediaKeyword(anchor_node));
    }
    return single_or_wrap(rules, always_rule, |rules| {
      MediaQueryRule::And(MediaAndRules::new(rules))
    });
  }
}

/// Add Display implementation for MediaQueryRule
#[cfg_attr(coverage_nightly, coverage(off))]
impl Display for MediaQueryRule {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    // Use the format_queries logic instead of the individual Display
    // implementations
    write!(f, "{}", MediaQuery::format_queries(self, false))
  }
}

#[cfg_attr(coverage_nightly, coverage(off))]
impl Display for MediaQuery {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(
      f,
      "@media {}",
      MediaQuery::format_queries(&self.queries, true)
    )
  }
}

impl MediaQuery {
  fn format_queries(queries: &MediaQueryRule, is_top_level: bool) -> String {
    match queries {
      MediaQueryRule::MediaKeyword(keyword) => {
        let prefix = if keyword.not {
          "not "
        } else if keyword.only {
          "only "
        } else {
          ""
        };
        // A media type is always a bare keyword. Browsers read `(screen)` as
        // an unknown boolean feature, and then the query never matches.
        format!("{}{}", prefix, keyword.key)
      },
      MediaQueryRule::WordRule(word_rule) => {
        format!("({})", word_rule.key_value)
      },
      MediaQueryRule::Pair(pair) => match &pair.value {
        MediaRuleValue::Fraction(frac) => {
          // Through the shared formatter, like every other numeric arm here.
          // The reference implementation interpolates a JavaScript `number`, so
          // the two spellings part company outside `[1e-6, 1e21)` and at `-0`:
          // `1e30/1` prints `1e+30 / 1` there and would print thirty-one digits
          // under Rust's `{}`, and `-0/1` prints `0 / 1` rather than `-0 / 1`.
          // `Fraction`'s own `Display` already does this, but nothing reaches
          // it -- `to_string` routes through here.
          format!(
            "({}: {} / {})",
            pair.key,
            to_js_string(frac.numerator),
            to_js_string(frac.denominator)
          )
        },
        MediaRuleValue::Length(len) => {
          format!("({}: {})", pair.key, len)
        },
        MediaRuleValue::String(s) => {
          format!("({}: {})", pair.key, s)
        },
        MediaRuleValue::Number(n) => {
          format!("({}: {})", pair.key, to_js_string(*n))
        },
      },
      // A query with a leading `not` prints as written: the `not` negates
      // the whole query.
      MediaQueryRule::Not(not_rule) if is_top_level && is_leading_not_query(queries) => {
        format!(
          "not {}",
          MediaQuery::format_queries(not_rule.rule.as_ref(), false)
        )
      },
      MediaQueryRule::Not(not_rule) => match not_rule.rule.as_ref() {
        MediaQueryRule::And(_) | MediaQueryRule::Or(_) | MediaQueryRule::Not(_) => {
          format!(
            "(not ({}))",
            MediaQuery::format_queries(not_rule.rule.as_ref(), false)
          )
        },
        _ => {
          format!(
            "(not {})",
            MediaQuery::format_queries(not_rule.rule.as_ref(), false)
          )
        },
      },
      MediaQueryRule::And(and_rules) => {
        // An `or` inside an `and` keeps its parentheses, or the query would
        // mix `and` and `or` at one level.
        let rule_strings: Vec<String> = and_rules
          .rules
          .iter()
          .map(|rule| match rule {
            MediaQueryRule::Or(or_rules)
              if and_rules.rules.len() > 1 && or_rules.rules.len() > 1 =>
            {
              format!("({})", MediaQuery::format_queries(rule, false))
            },
            _ => MediaQuery::format_queries(rule, false),
          })
          .collect();
        rule_strings.join(" and ")
      },
      MediaQueryRule::Or(or_rules) => {
        // Filter out invalid rules (like empty or rules)
        let valid_rules: Vec<&MediaQueryRule> = or_rules
          .rules
          .iter()
          .filter(|r| !matches!(r, MediaQueryRule::Or(or) if or.rules.is_empty()))
          .collect();

        if valid_rules.is_empty() {
          return "not all".to_string();
        }

        if valid_rules.len() == 1 {
          return MediaQuery::format_queries(valid_rules[0], is_top_level);
        }

        let formatted_rules: Vec<String> = valid_rules
          .iter()
          .map(|rule| match rule {
            MediaQueryRule::And(_) | MediaQueryRule::Or(_) => {
              let rule_string = MediaQuery::format_queries(rule, false);
              if !is_top_level {
                format!("({})", rule_string)
              } else {
                rule_string
              }
            },
            _ => MediaQuery::format_queries(rule, is_top_level),
          })
          .collect();

        if is_top_level {
          formatted_rules.join(", ")
        } else {
          formatted_rules.join(" or ")
        }
      },
    }
  }
}

impl MediaQuery {
  pub fn parser() -> TokenParser<MediaQuery> {
    TokenParser::new(
      |tokens| {
        if let Ok(Some(SimpleToken::AtKeyword(keyword))) = tokens.peek() {
          if keyword == "media" {
            let _ = tokens.consume_next_token(); // consume "@media"

            // Skip mandatory whitespace after "@media"
            if let Ok(Some(SimpleToken::Whitespace)) = tokens.peek() {
              let _ = tokens.consume_next_token();
            } else {
              // "@media" without space or content should be a syntax error
              return Err(CssParseError::ParseError {
                message: "Expected whitespace or content after @media".to_string(),
              });
            }
          } else {
            return Err(CssParseError::ParseError {
              message: "Expected @media at-keyword".to_string(),
            });
          }
        } else {
          // If no @media prefix, assume we're parsing just the query part (for
          // backwards compatibility) This maintains compatibility
          // with existing tests
        }

        let rule = (media_query_rule_parser().run)(tokens)?;
        Ok(MediaQuery::new(rule))
      },
      "media_query_parser",
    )
  }

  /// Whether every parenthesis in `input` closes, counting only the ones that
  /// are syntax. Mirrors the reference implementation's `_hasBalancedParens`.
  ///
  /// The check exists because the tokenizer synthesizes a closing parenthesis
  /// at end of input, so `(min-width: 100px` parses cleanly and would reach the
  /// stylesheet as a query nobody wrote. The reference implementation's
  /// tokenizer synthesizes nothing and its parse fails on the same input, so
  /// this is how the two arrive at the same refusal.
  ///
  /// A parenthesis inside a string, or one written as the escape `\(`, is a
  /// character rather than syntax and is skipped -- which the reference
  /// implementation's own counter does not do. That makes this scanner *more*
  /// permissive than upstream's, not less: upstream counts every parenthesis
  /// and so refuses `(foo: "(")`, where this reaches the parser. It is inert in
  /// practice, because the tokenizer handles neither strings nor escapes, so
  /// such a query fails the parse instead and both refusals become the same
  /// invalid-syntax error. An unterminated string is unbalanced in its own
  /// right: it swallows the rest of the query, including whatever would have
  /// closed the parenthesis it sits in.
  ///
  /// Kept as public API and as the name this check is asked for by. Inside the
  /// crate `validate_media_query` reads the scan directly, because it wants the
  /// nesting depth from the same walk.
  pub fn has_balanced_parens(input: &str) -> bool {
    scan_query_structure(input).parens_balanced
  }
}

/// How deeply a media query may nest before it is rejected.
///
/// The number and the reasoning behind it are
/// [`stylex_utils::nesting::MAX_NESTING_DEPTH`], shared with the value guard in
/// `stylex-css` because both enforce one decision about this compiler's stack.
/// Measured for this syntax specifically: two thousand levels of parentheses
/// take 10 ms and five thousand abort the process.
const MAX_QUERY_NESTING_DEPTH: usize = stylex_utils::nesting::MAX_NESTING_DEPTH;

/// Validate media query string
///
/// Two guards enforce the one nesting budget, because there are two recursions
/// and neither guard can see the other's. The scan below bounds the
/// **tokenizer**, which recurses once per nested block while building the token
/// list -- so it has to run before tokenizing, and a counter inside the parser
/// is reached too late to help. [`TokenList::with_depth`] bounds the **parser**,
/// which recurses once for each list in parentheses, each `(not ...)` and the
/// operand of a bare `not`. It counts the frames that the parser enters, so it
/// does not depend on how the text is spelled.
pub fn validate_media_query(input: &str) -> Result<MediaQuery, String> {
  let structure = scan_query_structure(input);

  if !structure.parens_balanced {
    return Err(crate::at_queries::messages::MediaQueryErrors::UNBALANCED_PARENS.to_string());
  }

  // Before the parse rather than during it: tokenizing is what would abort.
  if structure.max_paren_depth > MAX_QUERY_NESTING_DEPTH {
    return Err(crate::at_queries::messages::MediaQueryErrors::SYNTAX_ERROR.to_string());
  }

  match MediaQuery::parser().parse_to_end(input) {
    Ok(media_query) => Ok(media_query),
    Err(_) => Err(crate::at_queries::messages::MediaQueryErrors::SYNTAX_ERROR.to_string()),
  }
}

/// What one walk over the raw query text can answer.
///
/// Both questions are asked before the query is tokenized, and both have to be:
/// an unbalanced parenthesis is a query the tokenizer would silently close, and
/// parentheses nested past the budget are what the *tokenizer* aborts on, since
/// it recurses once per nested block while building the token list. One walk
/// answers both rather than two that could disagree about where a string ends.
struct QueryStructure {
  /// Every parenthesis that is syntax closes, and none closes too early.
  parens_balanced: bool,
  /// The deepest the query nests parentheses, counted outside strings and
  /// escapes.
  max_paren_depth: usize,
}

fn scan_query_structure(input: &str) -> QueryStructure {
  let mut depth: usize = 0;
  let mut max_paren_depth: usize = 0;
  let mut chars = input.chars();

  // One exit rather than four. Every way this walk can decide the parentheses
  // do not balance breaks out of the same loop with the depth seen so far,
  // which is what the caller needs either way.
  let parens_balanced = loop {
    let Some(ch) = chars.next() else {
      break depth == 0;
    };

    match ch {
      // A backslash escapes whatever follows, including a quote or a paren.
      '\\' => {
        if chars.next().is_none() {
          break false;
        }
      },
      '"' | '\'' => {
        if !skip_string(&mut chars, ch) {
          break false;
        }
      },
      '(' => {
        depth += 1;
        max_paren_depth = max_paren_depth.max(depth);
      },
      // A close with nothing open is unbalanced, which `checked_sub` is how
      // an unsigned depth says.
      ')' => match depth.checked_sub(1) {
        Some(remaining) => depth = remaining,
        None => break false,
      },
      _ => {},
    }
  };

  QueryStructure {
    parens_balanced,
    max_paren_depth,
  }
}

/// Consume up to and including the `quote` that ends a string, reporting
/// whether one was found before the input ran out.
fn skip_string(chars: &mut std::str::Chars<'_>, quote: char) -> bool {
  while let Some(ch) = chars.next() {
    match ch {
      '\\' => {
        if chars.next().is_none() {
          return false;
        }
      },
      c if c == quote => return true,
      _ => {},
    }
  }

  false
}

/// The dimensions whose `min-`/`max-` bounds merge into a single interval.
const DIMENSIONS: [&str; 2] = ["width", "height"];

/// A fresh, empty interval accumulator per entry of [`DIMENSIONS`].
fn new_dimension_intervals() -> [(&'static str, DimensionIntervals); DIMENSIONS.len()] {
  DIMENSIONS.map(|dim| (dim, DimensionIntervals::default()))
}

/// Which side of a dimension a rule bounds: `min-width` bounds it from below,
/// `max-width` from above.
#[derive(Clone, Copy)]
enum Bound {
  Min,
  Max,
}

/// The numeric `min-`/`max-` constraint a rule places on `dim`, if any, plus
/// whether it arrived negated. `None` means the rule says nothing about `dim`,
/// which for every dimension means it is not an interval-mergeable rule at all.
fn dimension_constraint<'a>(
  rule: &'a MediaQueryRule,
  dim: &str,
) -> Option<(Bound, &'a Length, bool)> {
  let (inner, negated) = match rule {
    MediaQueryRule::Not(not_rule) => (not_rule.rule.as_ref(), true),
    other => (other, false),
  };

  let MediaQueryRule::Pair(pair) = inner else {
    return None;
  };

  let bound = match (pair.key.strip_prefix("min-"), pair.key.strip_prefix("max-")) {
    (Some(rest), _) if rest == dim => Bound::Min,
    (_, Some(rest)) if rest == dim => Bound::Max,
    _ => return None,
  };

  match &pair.value {
    MediaRuleValue::Length(length) => Some((bound, length, negated)),
    _ => None,
  }
}

/// How far an exclusive bound is nudged past the value it excludes.
///
/// Two callers, one number.
///
/// A negated bound is the first: `not (min-height: 600px)` is
/// `max-height: 599.99px`. A strict inequality is the second: a `width` greater
/// than `400px` is `min-width: 400.01px`, and `(400px < width <= 700px)` nudges
/// its lower bound the same way.
///
/// A negated `min-width` in pixels is the one bound that does not read this
/// number. It reads [`MAX_WIDTH_EPSILON`], which is wider.
///
/// Both ask how to spell "not this value" in a syntax that has only inclusive
/// bounds, so they read one constant rather than three copies of it.
const EPSILON: f64 = 0.01;

/// The nudge a negated `min-width` in pixels takes instead of [`EPSILON`].
///
/// One case and one number, and the reference implementation carries the same
/// pair. `not (min-width: 100px)` becomes `max-width: 99.98px` where every
/// other negated bound moves by a hundredth. The wider step clears the device
/// pixel ratios a browser rounds a `width` to, which a hundredth does not: at a
/// ratio of 1.5 a viewport reports `99.99px`, which `max-width: 99.99px` still
/// matches and the author asked it not to.
///
/// Only width, and only pixels. A `height` is not rounded the same way, and a
/// relative unit is resolved before it is compared.
const MAX_WIDTH_EPSILON: f64 = 0.02;

/// The interval a single constraint imposes on its dimension.
///
/// The bounds this returns are not a private comparison aid: `merge_dimension`
/// intersects them and emits the survivors as the query's own `min-` and
/// `max-` values, so whatever arithmetic happens here is read back out as
/// text and hashed into a class name. That is why the width matters twice
/// over -- once for which comparisons the merge sees, and once for the digits
/// an author reads in the stylesheet.
///
/// The nudge is what makes a negated bound exclusive, and it is the half that
/// is easiest to lose: at widths past roughly 2^24, single precision cannot
/// represent `value - 0.01` as anything but `value`, so the exclusion
/// disappears and a contradiction such as
/// `(min-width: 1e7px) and (not (min-width: 1e7px))` reads back as the
/// satisfiable `width == 1e7px`. The other half is ordinary fractional
/// authoring: `28.81 - 0.01` is `28.799999999999997`, which is the number a
/// double holds and the number the official compiler emits.
///
/// `Length` holds an `f64`, so both halves come from the type rather than
/// from a conversion here. This function is where the reasoning lives, not
/// where the width is established.
fn constraint_interval(
  bound: Bound,
  dimension: &str,
  length: &Length,
  negated: bool,
) -> (f64, f64) {
  let value = length.value;

  // A negated `min-width` in pixels steps by the wider nudge. The dimension and
  // the unit both have to match, so the name is read here rather than left to
  // the caller.
  let negated_min_step = if dimension == "width" && length.unit.eq_ignore_ascii_case("px") {
    MAX_WIDTH_EPSILON
  } else {
    EPSILON
  };

  // A negated `min-` bound is a `max-` bound just below it, and vice versa.
  match (bound, negated) {
    (Bound::Min, false) => (value, f64::INFINITY),
    (Bound::Max, false) => (f64::NEG_INFINITY, value),
    (Bound::Min, true) => (f64::NEG_INFINITY, value - negated_min_step),
    (Bound::Max, true) => (value + EPSILON, f64::INFINITY),
  }
}

/// The accumulated numeric constraints on a single dimension. `unit` is set by
/// the first interval pushed; a later interval in a different unit sets
/// `unit_conflict`, which makes the whole merge bail out.
#[derive(Default)]
struct DimensionIntervals {
  intervals: Vec<(f64, f64)>,
  unit: String,
  unit_conflict: bool,
}

impl DimensionIntervals {
  /// Add `interval` to this dimension, flagging a unit that disagrees with the
  /// one the first interval established.
  fn push(&mut self, interval: (f64, f64), unit: &str) {
    match self.intervals.is_empty() {
      true => self.unit = unit.to_string(),
      false => self.unit_conflict |= self.unit != unit,
    }

    self.intervals.push(interval);
  }

  /// Intersect every recorded interval. `None` means the constraints
  /// contradict each other.
  fn intersect(&self) -> Option<(f64, f64)> {
    let mut lower = f64::NEG_INFINITY;
    let mut upper = f64::INFINITY;

    for (l, u) in &self.intervals {
      if *l > lower {
        lower = *l;
      }
      if *u < upper {
        upper = *u;
      }
    }

    match lower > upper {
      true => None,
      false => Some((lower, upper)),
    }
  }
}

/// How much distribution work is allowed before the merge is abandoned.
///
/// Stated in branch *nodes* rather than in levels, because depth is only one
/// factor of the cost. Each `not (A and B)` clause splits the rule list in two,
/// so a list carrying `d` of them expands into `2^d` branches -- but every one
/// of those branches carries a copy of the whole list, and every surviving
/// branch prints it, so both the clones and the emitted text are
/// `2^d * rules.len()`.
///
/// The second factor is not bounded by the first, which is what a depth bound
/// cannot see. `negation_depth` charges nothing for a clause it cannot split, so
/// a negated non-range condition -- `not (orientation: portrait)`,
/// `not (min-resolution: 200dpi)` -- adds width and no depth at all. Twenty of
/// those beside a fourteen-rung ladder is depth 12, comfortably under any depth
/// bound ever proposed here, and emitted 255 MB of query text in 6.7 seconds
/// from 1.7 KB of authored input; sixteen rungs made it 1.1 GB and 30 seconds.
/// Nor is the shape exotic, because the transform builds it: each key is
/// rewritten against a negation of every later sibling, so one `and` list is as
/// wide as the conditional value map is long.
///
/// `2^18` nodes leaves every shape in this repo untouched -- the deepest is the
/// benchmark fixture's `2^6 * 9` -- and caps a single list in the low tens of
/// kilobytes.
///
/// The number is this compiler's own choice and not a length read off the
/// reference implementation, because there is no such length to read.
/// `@stylexjs/babel-plugin` 0.19.0 never gives up: its recursion depth is
/// linear in ladder length while its branch count doubles, so the call stack it
/// wraps in a `try`/`catch` is never what gives out. What gives out is the heap
/// or V8's string-length limit -- the first is a fatal abort no `catch` sees,
/// the second is raised while the text is built, outside the `try` that would
/// have caught it. Measured at 28 rungs: 435 seconds, 252 MB of query text,
/// about 7.4 GB resident.
///
/// So past this bound we deliberately stop matching rather than reproduce a
/// build that dies. Under the node budget a pure ladder expands to about
/// fourteen rungs where the old depth bound reached twenty; no single budget
/// does both, because the shape that blows up sits under any depth that keeps
/// twenty. Real ladders are a handful of rungs. See [ADR 0001] for the
/// measurements and for what is still not capped.
///
/// What this caps is one `and` list, which is what the boundary is crossed once
/// for -- so a query holding several, such as a comma-separated disjunction,
/// costs that many times as much.
///
/// [ADR 0001]: https://github.com/Dwlad90/stylex-swc-plugin/blob/develop/crates/stylex-css-parser/docs/adr/0001-the-official-compilers-output-wins.md
const MAX_DISTRIBUTION_NODES: u64 = 1 << 18;

/// How many times distributing `rules` can split before it runs out of clauses.
///
/// Every `not (A and B)` in the list is peeled at its own level, and each branch
/// keeps the rest, so the longest path down is the sum over the list rather than
/// its maximum. Measured before the expansion starts, because measuring it
/// during would mean already having paid for it.
fn distribution_depth(rules: &[MediaQueryRule]) -> u32 {
  rules
    .iter()
    .map(|rule| match rule {
      MediaQueryRule::Not(not_rule) => negation_depth(not_rule.rule.as_ref()),
      _ => 0,
    })
    .sum()
}

/// How many splits `not operand` costs.
///
/// Negating an `and` of two operands is one split, and each operand is negated
/// in turn -- so an operand that is itself an `and` of two splits again. The
/// walk is as deep as the tree, and no deeper than `normalize`'s own walk over
/// the same tree, which runs first.
fn negation_depth(operand: &MediaQueryRule) -> u32 {
  match as_binary_and(operand) {
    Some((left, right)) => 1 + negation_depth(left).max(negation_depth(right)),
    None => 0,
  }
}

/// The two operands of an `and` of exactly two, or `None` for anything else.
///
/// This is the one shape distribution acts on: DeMorgan splits it into two
/// branches, and the depth walk counts one split for it. Both readers ask the
/// same question, so they ask it in one place -- a second spelling of "an `and`
/// of exactly two" is a second thing to keep in step with the first.
fn as_binary_and(rule: &MediaQueryRule) -> Option<(&MediaQueryRule, &MediaQueryRule)> {
  match rule {
    MediaQueryRule::And(and_rules) => match and_rules.rules.as_slice() {
      [left, right] => Some((left, right)),
      _ => None,
    },
    _ => None,
  }
}

/// The single boundary canonicalization crosses to simplify an `and` list's
/// ranges, mirroring the reference implementation's `mergeAndSimplifyRanges`.
///
/// There the merge is wrapped in a `try`/`catch` handing the input rules back
/// on any throw. The recursion re-enters the merge directly rather than
/// crossing the wrapper again, which is what makes the wrapper the one place a
/// give-up can live. This is that place, and the give-up living here is the
/// depth bound: the expansion is measured before it starts, and a list too deep
/// to expand is handed straight back.
///
/// The two failure modes of this pass are deliberately kept apart. The inner
/// recovery gives up merging and emits the author's rules as written, and it
/// never propagates. The outer refusal turns a query the parser cannot read
/// into the invalid-media-query-syntax error and rejects the declaration.
/// Conflating them would turn a query too deep to merge into an error the
/// author cannot act on.
///
/// One correction worth leaving here, because it is easy to assume otherwise:
/// the reference implementation's own recovery is not reachable by a deep
/// breakpoint ladder. Its recursion depth grows with ladder length while its
/// branch count doubles per rung, so what gives out is the string-length limit
/// or the heap -- whichever binds first -- and not the call stack. Neither
/// reaches its recovery: the heap aborts where no `catch` runs, and the string
/// limit is raised outside the one that would have caught it.
fn merge_and_simplify_ranges(rules: Vec<MediaQueryRule>) -> Vec<MediaQueryRule> {
  // Giving up here rather than partway down is what makes the outcome the
  // author's own rules: a bound checked inside the recursion would leave some
  // branches merged and some not, which is neither compiler's answer.
  //
  // Branches times the list each one carries, because that product is what the
  // expansion costs to build and to print -- see [`MAX_DISTRIBUTION_NODES`].
  // The saturating arithmetic is how a list too deep to shift stays a refusal
  // rather than becoming an overflow: `checked_shl` gives up at 64 levels and
  // `saturating_mul` cannot wrap, so an absurd input lands on "too big" by the
  // same path a merely large one does.
  let branches = 1u64
    .checked_shl(distribution_depth(&rules))
    .unwrap_or(u64::MAX);

  if branches.saturating_mul(rules.len() as u64) > MAX_DISTRIBUTION_NODES {
    return rules;
  }

  merge_intervals_for_and(rules)
}

/// Merge the numeric width/height constraints of an `and` list into a single
/// interval per dimension.
///
/// The recursive interior of `merge_and_simplify_ranges`, and named separately
/// because each of the two names its own counterpart in the reference
/// implementation -- collapsing them would cost the property that lets either
/// be checked against it. Callers outside this pair want the boundary, not this.
///
/// The returned `Vec` carries three outcomes, and callers must read all three:
/// empty means the constraints contradict each other, which the caller turns
/// into `not all`; `rules` handed back unchanged means the list was not
/// interval-mergeable, whether from a non-numeric rule or from units that
/// disagree; anything else is the merged interval pairs. Collapsing the three
/// into one `Vec` is the shape the canonicalization pipeline is specified
/// against, so it stays.
fn merge_intervals_for_and(rules: Vec<MediaQueryRule>) -> Vec<MediaQueryRule> {
  let mut dimensions = new_dimension_intervals();

  // Handle DeMorgan's law: not (A and B) = (not A) or (not B)
  for (index, rule) in rules.iter().enumerate() {
    if let MediaQueryRule::Not(not_rule) = rule
      && let Some((left, right)) = as_binary_and(not_rule.rule.as_ref())
    {
      // Each branch is every rule except this one, plus one negated operand --
      // so it ends up exactly as long as `rules`, and is built at that capacity
      // rather than grown into. This runs once per node of a 2^d expansion, so
      // the reallocations it saves are not incidental. The current rule is left
      // out by its position, which the loop already knows, rather than by
      // comparing addresses inside the slice.
      let branch_without_current = |negated: &MediaQueryRule| {
        let mut branch = Vec::with_capacity(rules.len());
        branch.extend_from_slice(&rules[..index]);
        branch.extend_from_slice(&rules[index + 1..]);
        branch.push(MediaQueryRule::Not(MediaNotRule::new(negated.clone())));
        branch
      };

      let left_branch_rules = branch_without_current(left);
      let right_branch_rules = branch_without_current(right);

      // Recursively process each branch
      let left_branch = merge_intervals_for_and(left_branch_rules);
      let right_branch = merge_intervals_for_and(right_branch_rules);

      // An *empty* branch is dropped; a contradictory one is not. A
      // contradiction recurses to the bottom and comes back as a one-element
      // result holding an empty `or`, which survives this filter and is kept
      // as-is, then collapsed to `not all` by serialization -- along with the
      // nesting built around it. That retention is the contract; see
      // [ADR 0001](../../docs/adr/0001-the-official-compilers-output-wins.md).
      // A branch of several rules is re-wrapped in `and`.
      let or_rules: Vec<MediaQueryRule> = [left_branch, right_branch]
        .into_iter()
        .filter(|branch| !branch.is_empty())
        .map(|mut branch| match branch.len() {
          1 => branch.remove(0),
          _ => MediaQueryRule::And(MediaAndRules::new(branch)),
        })
        .collect();

      return vec![MediaQueryRule::Or(MediaOrRules::new(or_rules))];
    }
  }

  for rule in &rules {
    let mut mergeable = false;

    for (dim, state) in dimensions.iter_mut() {
      let Some((bound, length, negated)) = dimension_constraint(rule, dim) else {
        continue;
      };

      state.push(
        constraint_interval(bound, dim, length, negated),
        &length.unit,
      );

      mergeable = true;
      break;
    }

    // Anything that is not a numeric width/height constraint blocks the merge.
    if !mergeable {
      return rules;
    }
  }

  // Mixed units cannot be intersected numerically; leave the rules untouched.
  if dimensions.iter().any(|(_, state)| state.unit_conflict) {
    return rules;
  }

  let mut result = Vec::new();

  for (dim, state) in &dimensions {
    if state.intervals.is_empty() {
      continue;
    }

    let Some((lower, upper)) = state.intersect() else {
      return Vec::new();
    };

    // `!= -Infinity` and `!= Infinity` rather than `is_finite`, which is what
    // the reference compiler asks. An overflowed bound is kept, not dropped,
    // and dropping it was not a spelling difference: at
    // `(min-width: 1e400px) and (min-height: 10px)` upstream emits
    // `(min-width: Infinitypx) and (min-height: 10px)`, which is invalid CSS a
    // browser discards whole, where dropping the bound left
    // `(min-height: 10px)` -- a rule that *applies*, at every viewport ten
    // pixels tall or more. Emitting a spelling nobody can use is the faithful
    // answer here, because the alternative is applying a declaration upstream
    // never applies.
    if lower != f64::NEG_INFINITY {
      result.push(MediaQueryRule::Pair(MediaRulePair::new(
        format!("min-{dim}"),
        MediaRuleValue::Length(Length::new(lower, state.unit.clone())),
      )));
    }

    if upper != f64::INFINITY {
      result.push(MediaQueryRule::Pair(MediaRulePair::new(
        format!("max-{dim}"),
        MediaRuleValue::Length(Length::new(upper, state.unit.clone())),
      )));
    }
  }

  if result.is_empty() { rules } else { result }
}

/// Extract an ident value from a token that is guaranteed by the parser to be an Ident.
/// The else branch is defensive and unreachable in normal operation.
fn extract_ident_as_media_type(token: SimpleToken) -> String {
  if let SimpleToken::Ident(value) = token {
    value
  } else {
    // This branch is unreachable: `tokens::ident()` only yields Ident tokens.
    "all".to_string()
  }
}

/// Extract an ident value from a token that is guaranteed to be an Ident (word rule context).
fn extract_ident_as_word_rule(token: SimpleToken) -> String {
  if let SimpleToken::Ident(value) = token {
    value
  } else {
    // This branch is unreachable: `tokens::ident()` only yields Ident tokens.
    "color".to_string()
  }
}

/// Convert a Dimension token to a MediaRuleValue::Length.
fn dimension_to_media_rule_value(token: SimpleToken) -> MediaRuleValue {
  if let SimpleToken::Dimension { value, unit } = token {
    MediaRuleValue::Length(Length::new(value, unit))
  } else {
    // This branch is unreachable: `tokens::dimension()` only yields Dimension tokens.
    MediaRuleValue::Number(0.0)
  }
}

/// Convert an Ident token to a MediaRuleValue::String.
fn ident_to_media_rule_value(token: SimpleToken) -> MediaRuleValue {
  if let SimpleToken::Ident(value) = token {
    MediaRuleValue::String(value)
  } else {
    // This branch is unreachable: `tokens::ident()` only yields Ident tokens.
    MediaRuleValue::String(String::new())
  }
}

/// Convert a Number token to a MediaRuleValue::Number.
fn number_to_media_rule_value(token: SimpleToken) -> MediaRuleValue {
  if let SimpleToken::Number(value) = token {
    MediaRuleValue::Number(value)
  } else {
    // This branch is unreachable: `tokens::number()` only yields Number tokens.
    MediaRuleValue::Number(0.0)
  }
}

/// Which end of a range a bound is, and so which way its epsilon moves.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum BoundEnd {
  /// A lower bound: a strict `>` becomes an inclusive `>=` one epsilon up.
  Min,
  /// An upper bound: a strict `<` becomes an inclusive `<=` one epsilon down.
  Max,
}

/// Shift a length bound by one epsilon so a strict inequality can be written as
/// an inclusive one, mirroring the reference implementation's `adjustDimension`.
///
/// Three functions did this before -- one per parser that needed it -- differing
/// only in which way they moved. They are one here for the same reason upstream
/// has one: the epsilon convention is a single rule, and a second copy of it is
/// a second place for it to drift.
///
/// A value that is not a `Length` carries no number to shift and is left alone.
/// None of the inequality parsers can produce one, so this is a defensive arm
/// rather than a case; leaving it alone is the answer all three gave.
fn adjust_dimension(value: &mut MediaRuleValue, end: BoundEnd, epsilon: f64) {
  if let MediaRuleValue::Length(length) = value {
    match end {
      BoundEnd::Min => length.value += epsilon,
      BoundEnd::Max => length.value -= epsilon,
    }
  }
}

/// Basic media type parser: screen | print | all
fn basic_media_type_parser() -> TokenParser<String> {
  tokens::ident()
    .map(extract_ident_as_media_type, Some("extract_media_type"))
    .where_fn(
      |value| matches!(value.as_str(), "screen" | "print" | "all"),
      Some("valid_media_type"),
    )
}

/// Media keyword parser with optional not/only modifiers
fn media_keyword_parser() -> TokenParser<MediaQueryRule> {
  TokenParser::new(
    |tokens| {
      let mut not_value = false;
      let mut only_value = false; // Default to false instead of None

      // Try to parse optional "not" at the beginning
      if let Ok(Some(SimpleToken::Ident(val))) = tokens.peek()
        && val == "not"
      {
        let _ = tokens.consume_next_token(); // consume "not"
        not_value = true;

        // Consume whitespace after "not"
        skip_whitespace(tokens);
      }

      // Try to parse optional "only" after "not" (or at beginning if no "not")
      if let Ok(Some(SimpleToken::Ident(val))) = tokens.peek()
        && val == "only"
      {
        let _ = tokens.consume_next_token(); // consume "only"
        only_value = true;

        // Consume whitespace after "only"
        skip_whitespace(tokens);
      }

      // `not only screen` is accepted. The keyword keeps both flags: a lone
      // keyword prints its `not` and hides the `only`, but a leading `not`
      // over an `and` list moves to the list, and the `only` then prints.
      // `only not screen` does not parse: `only` must come after `not`. A
      // media type alone in a `(not ...)`, as in `(not only screen)`, does not
      // parse either.

      // Parse the media type (required)
      let media_type = (basic_media_type_parser().run)(tokens)?;

      Ok(MediaQueryRule::MediaKeyword(MediaKeyword::new(
        media_type, not_value, only_value,
      )))
    },
    "media_keyword_parser",
  )
}

/// `parser` with `(` before it and `)` after it, and no whitespace between.
fn in_parens<T: Clone + std::fmt::Debug + 'static>(parser: TokenParser<T>) -> TokenParser<T> {
  parser.surrounded_by(
    TokenParser::<SimpleToken>::token(SimpleToken::LeftParen, Some("OpenParen")),
    Some(TokenParser::<SimpleToken>::token(
      SimpleToken::RightParen,
      Some("CloseParen"),
    )),
  )
}

/// Media word rule parser for (color), (monochrome), etc.
fn media_word_rule_parser() -> TokenParser<MediaQueryRule> {
  in_parens(
    tokens::ident()
      .map(extract_ident_as_word_rule, Some("extract_word_rule"))
      .where_fn(
        |value| {
          matches!(
            value.as_str(),
            "color" | "monochrome" | "grid" | "color-index"
          )
        },
        Some("valid_word_rule"),
      ),
  )
  .map(
    |keyword| MediaQueryRule::WordRule(MediaWordRule::new(keyword)),
    Some("create_word_rule"),
  )
}

fn media_rule_value_parser() -> TokenParser<MediaRuleValue> {
  TokenParser::one_of(vec![
    Calc::parser().map(
      |calc| MediaRuleValue::String(calc.to_string()),
      Some("calc_to_string"),
    ),
    // Dimensions (e.g., 768px)
    tokens::dimension().map(dimension_to_media_rule_value, Some("dimension_to_length")),
    tokens::ident().map(ident_to_media_rule_value, Some("ident_to_string")),
    // Fraction parsing (number / number) like aspect-ratio: 16/9
    TokenParser::new(
      |tokens| {
        // Parse first number
        let first_num = if let Ok(Some(SimpleToken::Number(value))) = tokens.consume_next_token() {
          value
        } else {
          return Err(CssParseError::ParseError {
            message: "Expected first number in fraction".to_string(),
          });
        };

        // Optional whitespace before slash
        skip_whitespace(tokens);

        // Parse slash delimiter
        if let Ok(Some(SimpleToken::Delim(ch))) = tokens.consume_next_token() {
          if ch != '/' {
            return Err(CssParseError::ParseError {
              message: "Expected '/' in fraction".to_string(),
            });
          }
        } else {
          return Err(CssParseError::ParseError {
            message: "Expected '/' delimiter".to_string(),
          });
        }

        // Optional whitespace after slash
        skip_whitespace(tokens);

        // Parse second number
        let second_num = if let Ok(Some(SimpleToken::Number(value))) = tokens.consume_next_token() {
          value
        } else {
          return Err(CssParseError::ParseError {
            message: "Expected second number in fraction".to_string(),
          });
        };

        Ok(MediaRuleValue::Fraction(Fraction {
          numerator: first_num,
          denominator: second_num,
        }))
      },
      "fraction_parser",
    ),
    // Numbers (must be last to avoid consuming numbers that are part of fractions)
    tokens::number().map(number_to_media_rule_value, Some("number_to_value")),
  ])
}

/// Simple pair parser for (key: value) media features
fn simple_pair_parser(value_parser: TokenParser<MediaRuleValue>) -> TokenParser<MediaQueryRule> {
  let value_parser_rc = value_parser.run.clone(); // Clone the Rc<dyn Fn>

  TokenParser::new(
    move |tokens| {
      // Parse opening parenthesis
      if let Ok(Some(SimpleToken::LeftParen)) = tokens.consume_next_token() {
        // Good, we have opening paren
      } else {
        return Err(CssParseError::ParseError {
          message: "Expected opening parenthesis".to_string(),
        });
      }

      // Optional whitespace after opening paren
      skip_whitespace(tokens);

      // Parse key (identifier)
      let key = if let Ok(Some(SimpleToken::Ident(key_name))) = tokens.consume_next_token() {
        key_name
      } else {
        return Err(CssParseError::ParseError {
          message: "Expected media feature name".to_string(),
        });
      };

      // Optional whitespace before colon
      skip_whitespace(tokens);

      // Parse colon
      if let Ok(Some(SimpleToken::Colon)) = tokens.consume_next_token() {
        // Good, we have colon
      } else {
        return Err(CssParseError::ParseError {
          message: "Expected colon after media feature name".to_string(),
        });
      }

      // Optional whitespace after colon
      skip_whitespace(tokens);

      // Parse value using the cloned value parser
      let value = (value_parser_rc)(tokens)?;

      // Optional whitespace before closing paren
      skip_whitespace(tokens);

      // Parse closing parenthesis
      if let Ok(Some(SimpleToken::RightParen)) = tokens.consume_next_token() {
        // Good, we have closing paren
      } else {
        return Err(CssParseError::ParseError {
          message: "Expected closing parenthesis".to_string(),
        });
      }

      Ok(MediaQueryRule::Pair(MediaRulePair::new(key, value)))
    },
    "simple_pair_parser",
  )
}

/// Combined inequality parser - handles both forward and reversed inequalities
fn combined_inequality_parser() -> TokenParser<MediaQueryRule> {
  TokenParser::one_of(vec![
    media_inequality_rule_parser(),          // Forward: (width <= 1250px)
    media_inequality_rule_parser_reversed(), // Reversed: (1250px >= width)
  ])
}

/// Forward inequality parser: (width <= 1250px) or (width < 1250px)
fn media_inequality_rule_parser() -> TokenParser<MediaQueryRule> {
  TokenParser::new(
    |tokens| {
      // Expect opening paren
      let open_token =
        tokens
          .consume_next_token()
          .ok()
          .flatten()
          .ok_or(CssParseError::ParseError {
            message: "Expected opening parenthesis".to_string(),
          })?;
      if !matches!(open_token, SimpleToken::LeftParen) {
        return Err(CssParseError::ParseError {
          message: format!("Expected '(' token, got {:?}", open_token),
        });
      }

      // Skip optional whitespace
      skip_whitespace(tokens);

      // Parse property name (width or height)
      let key_token =
        tokens
          .consume_next_token()
          .ok()
          .flatten()
          .ok_or(CssParseError::ParseError {
            message: "Expected property name".to_string(),
          })?;
      let key = if let SimpleToken::Ident(name) = key_token {
        if name == "width" || name == "height" {
          name
        } else {
          return Err(CssParseError::ParseError {
            message: format!("Expected 'width' or 'height', got '{}'", name),
          });
        }
      } else {
        return Err(CssParseError::ParseError {
          message: format!("Expected identifier, got {:?}", key_token),
        });
      };

      // Skip optional whitespace
      skip_whitespace(tokens);

      // Parse operator (< or >)
      let op_token =
        tokens
          .consume_next_token()
          .ok()
          .flatten()
          .ok_or(CssParseError::ParseError {
            message: "Expected comparison operator".to_string(),
          })?;
      let op = if let SimpleToken::Delim(op_char) = op_token {
        if op_char == '<' || op_char == '>' {
          op_char
        } else {
          return Err(CssParseError::ParseError {
            message: format!("Expected '<' or '>', got '{}'", op_char),
          });
        }
      } else {
        return Err(CssParseError::ParseError {
          message: format!("Expected delimiter, got {:?}", op_token),
        });
      };

      // Skip optional whitespace
      skip_whitespace(tokens);

      // Parse optional equals sign
      let has_equals = if let Ok(Some(SimpleToken::Delim('='))) = tokens.peek() {
        let _ = tokens.consume_next_token();
        true
      } else {
        false
      };

      // Skip optional whitespace
      skip_whitespace(tokens);

      // Parse dimension value
      let dim_token =
        tokens
          .consume_next_token()
          .ok()
          .flatten()
          .ok_or(CssParseError::ParseError {
            message: "Expected dimension value".to_string(),
          })?;
      let mut dimension = if let SimpleToken::Dimension { value, unit } = dim_token {
        Length::new(value, unit)
      } else {
        return Err(CssParseError::ParseError {
          message: format!("Expected dimension, got {:?}", dim_token),
        });
      };

      // Skip optional whitespace
      skip_whitespace(tokens);

      // Expect closing paren
      let close_token =
        tokens
          .consume_next_token()
          .ok()
          .flatten()
          .ok_or(CssParseError::ParseError {
            message: "Expected closing parenthesis".to_string(),
          })?;
      if !matches!(close_token, SimpleToken::RightParen) {
        return Err(CssParseError::ParseError {
          message: format!("Expected ')' token, got {:?}", close_token),
        });
      }

      if !has_equals {
        if op == '>' {
          // (width > 400px) -> min-width: 400.01px
          dimension.value += EPSILON;
        } else {
          // (width < 400px) -> max-width: 399.99px
          dimension.value -= EPSILON;
        }
      }

      // Convert to final key: (width < 1250px) becomes max-width
      let final_key = if op == '>' {
        format!("min-{}", key)
      } else {
        format!("max-{}", key)
      };

      Ok(MediaQueryRule::Pair(MediaRulePair::new(
        final_key,
        MediaRuleValue::Length(dimension),
      )))
    },
    "media_inequality_rule_parser",
  )
}

/// Reversed inequality parser: (1250px >= width) or (1250px > width)
fn media_inequality_rule_parser_reversed() -> TokenParser<MediaQueryRule> {
  TokenParser::new(
    |tokens| {
      // Expect opening paren
      let open_token =
        tokens
          .consume_next_token()
          .ok()
          .flatten()
          .ok_or(CssParseError::ParseError {
            message: "Expected opening parenthesis".to_string(),
          })?;
      if !matches!(open_token, SimpleToken::LeftParen) {
        return Err(CssParseError::ParseError {
          message: format!("Expected '(' token, got {:?}", open_token),
        });
      }

      // Skip optional whitespace
      skip_whitespace(tokens);

      // Parse dimension value first
      let dim_token =
        tokens
          .consume_next_token()
          .ok()
          .flatten()
          .ok_or(CssParseError::ParseError {
            message: "Expected dimension value".to_string(),
          })?;
      let dimension = if let SimpleToken::Dimension { value, unit } = dim_token {
        MediaRuleValue::Length(Length::new(value, unit))
      } else {
        return Err(CssParseError::ParseError {
          message: format!("Expected dimension, got {:?}", dim_token),
        });
      };

      // Skip optional whitespace
      skip_whitespace(tokens);

      // Parse operator (< or >)
      let op_token =
        tokens
          .consume_next_token()
          .ok()
          .flatten()
          .ok_or(CssParseError::ParseError {
            message: "Expected comparison operator".to_string(),
          })?;
      let op = if let SimpleToken::Delim(op_char) = op_token {
        if op_char == '<' || op_char == '>' {
          op_char
        } else {
          return Err(CssParseError::ParseError {
            message: format!("Expected '<' or '>', got '{}'", op_char),
          });
        }
      } else {
        return Err(CssParseError::ParseError {
          message: format!("Expected delimiter, got {:?}", op_token),
        });
      };

      // Skip optional whitespace
      skip_whitespace(tokens);

      // Parse optional equals sign
      let has_equals = if let Ok(Some(SimpleToken::Delim('='))) = tokens.peek() {
        let _ = tokens.consume_next_token();
        true
      } else {
        false
      };

      // Skip optional whitespace
      skip_whitespace(tokens);

      // Parse property name (width or height)
      let key_token =
        tokens
          .consume_next_token()
          .ok()
          .flatten()
          .ok_or(CssParseError::ParseError {
            message: "Expected property name".to_string(),
          })?;
      let key = if let SimpleToken::Ident(name) = key_token {
        if name == "width" || name == "height" {
          name
        } else {
          return Err(CssParseError::ParseError {
            message: format!("Expected 'width' or 'height', got '{}'", name),
          });
        }
      } else {
        return Err(CssParseError::ParseError {
          message: format!("Expected identifier, got {:?}", key_token),
        });
      };

      // Skip optional whitespace
      skip_whitespace(tokens);

      // Expect closing paren
      let close_token =
        tokens
          .consume_next_token()
          .ok()
          .flatten()
          .ok_or(CssParseError::ParseError {
            message: "Expected closing parenthesis".to_string(),
          })?;
      if !matches!(close_token, SimpleToken::RightParen) {
        return Err(CssParseError::ParseError {
          message: format!("Expected ')' token, got {:?}", close_token),
        });
      }

      let mut adjusted_dimension = dimension;
      if !has_equals {
        // `>` in a reversed inequality reads as an upper bound: `100px > width`.
        let end = if op == '>' {
          BoundEnd::Max
        } else {
          BoundEnd::Min
        };
        adjust_dimension(&mut adjusted_dimension, end, EPSILON);
      }

      // Convert to final key: (1250px > width) becomes max-width
      let final_key = if op == '>' {
        format!("max-{}", key)
      } else {
        format!("min-{}", key)
      };

      Ok(MediaQueryRule::Pair(MediaRulePair::new(
        final_key,
        adjusted_dimension,
      )))
    },
    "media_inequality_rule_parser_reversed",
  )
}

/// Double inequality parser: (500px <= width <= 1000px)
fn double_inequality_rule_parser() -> TokenParser<MediaQueryRule> {
  TokenParser::new(
    |tokens| {
      // Expect opening paren
      let open_token =
        tokens
          .consume_next_token()
          .ok()
          .flatten()
          .ok_or(CssParseError::ParseError {
            message: "Expected opening parenthesis".to_string(),
          })?;
      if !matches!(open_token, SimpleToken::LeftParen) {
        return Err(CssParseError::ParseError {
          message: format!("Expected '(' token, got {:?}", open_token),
        });
      }

      // Skip optional whitespace
      skip_whitespace(tokens);

      // Parse lower bound dimension
      let lower_dim_token =
        tokens
          .consume_next_token()
          .ok()
          .flatten()
          .ok_or(CssParseError::ParseError {
            message: "Expected lower bound dimension".to_string(),
          })?;
      let lower_dimension = if let SimpleToken::Dimension { value, unit } = lower_dim_token {
        MediaRuleValue::Length(Length::new(value, unit))
      } else {
        return Err(CssParseError::ParseError {
          message: format!("Expected dimension, got {:?}", lower_dim_token),
        });
      };

      // Skip optional whitespace
      skip_whitespace(tokens);

      // Parse first operator (< or >)
      let op1_token =
        tokens
          .consume_next_token()
          .ok()
          .flatten()
          .ok_or(CssParseError::ParseError {
            message: "Expected first comparison operator".to_string(),
          })?;
      let _op1 = if let SimpleToken::Delim(op_char) = op1_token {
        if op_char == '<' || op_char == '>' {
          op_char
        } else {
          return Err(CssParseError::ParseError {
            message: format!("Expected '<' or '>', got '{}'", op_char),
          });
        }
      } else {
        return Err(CssParseError::ParseError {
          message: format!("Expected delimiter, got {:?}", op1_token),
        });
      };

      // Skip optional whitespace
      skip_whitespace(tokens);

      // Parse optional first equals sign
      let _eq1 = if let Ok(Some(SimpleToken::Delim('='))) = tokens.peek() {
        let _ = tokens.consume_next_token();
        true
      } else {
        false
      };

      // Skip optional whitespace
      skip_whitespace(tokens);

      // Parse property name (width or height)
      let key_token =
        tokens
          .consume_next_token()
          .ok()
          .flatten()
          .ok_or(CssParseError::ParseError {
            message: "Expected property name".to_string(),
          })?;
      let key = if let SimpleToken::Ident(name) = key_token {
        if name == "width" || name == "height" {
          name
        } else {
          return Err(CssParseError::ParseError {
            message: format!("Expected 'width' or 'height', got '{}'", name),
          });
        }
      } else {
        return Err(CssParseError::ParseError {
          message: format!("Expected identifier, got {:?}", key_token),
        });
      };

      // Skip optional whitespace
      skip_whitespace(tokens);

      // Parse second operator (< or >)
      let op2_token =
        tokens
          .consume_next_token()
          .ok()
          .flatten()
          .ok_or(CssParseError::ParseError {
            message: "Expected second comparison operator".to_string(),
          })?;
      let _op2 = if let SimpleToken::Delim(op_char) = op2_token {
        if op_char == '<' || op_char == '>' {
          op_char
        } else {
          return Err(CssParseError::ParseError {
            message: format!("Expected '<' or '>', got '{}'", op_char),
          });
        }
      } else {
        return Err(CssParseError::ParseError {
          message: format!("Expected delimiter, got {:?}", op2_token),
        });
      };

      // Skip optional whitespace
      skip_whitespace(tokens);

      // Parse optional second equals sign
      let _eq2 = if let Ok(Some(SimpleToken::Delim('='))) = tokens.peek() {
        let _ = tokens.consume_next_token();
        true
      } else {
        false
      };

      // Skip optional whitespace
      skip_whitespace(tokens);

      // Parse upper bound dimension
      let upper_dim_token =
        tokens
          .consume_next_token()
          .ok()
          .flatten()
          .ok_or(CssParseError::ParseError {
            message: "Expected upper bound dimension".to_string(),
          })?;
      let upper_dimension = if let SimpleToken::Dimension { value, unit } = upper_dim_token {
        MediaRuleValue::Length(Length::new(value, unit))
      } else {
        return Err(CssParseError::ParseError {
          message: format!("Expected dimension, got {:?}", upper_dim_token),
        });
      };

      // Skip optional whitespace
      skip_whitespace(tokens);

      // Expect closing paren
      let close_token =
        tokens
          .consume_next_token()
          .ok()
          .flatten()
          .ok_or(CssParseError::ParseError {
            message: "Expected closing parenthesis".to_string(),
          })?;
      if !matches!(close_token, SimpleToken::RightParen) {
        return Err(CssParseError::ParseError {
          message: format!("Expected ')' token, got {:?}", close_token),
        });
      }

      // Each bound takes its key from its *own* operator, which is what the
      // reference implementation's `doubleInequalityRuleParser` does:
      //
      //   lowerKey = op  === '>' ? `max-${key}` : `min-${key}`
      //   upperKey = op2 === '>' ? `min-${key}` : `max-${key}`
      //
      // So a mixed-direction range puts both bounds on the *same* side, and the
      // interval merge then collapses them to one. Deriving the pair as a fixed
      // `min`/`max` and only choosing which value went where could not express
      // that, and answered a different question: `(500px > width < 1000px)` is
      // `(max-width: 499.99px)` upstream, and came out here as `not all` -- a
      // satisfiable query turned into a rule that can never match.
      //
      // The epsilon follows the key rather than the operator, again as upstream
      // does: its `adjustDimension` ignores the operator it is passed and
      // shifts down for a `max-` bound and up for a `min-` one.
      let end_for = |op: char| match op {
        '>' => BoundEnd::Max,
        _ => BoundEnd::Min,
      };

      let lower_end = end_for(_op1);
      // The upper operator reads the other way: `width > B` is a lower bound.
      let upper_end = match _op2 {
        '>' => BoundEnd::Min,
        _ => BoundEnd::Max,
      };

      let key_for = |end: BoundEnd| match end {
        BoundEnd::Min => format!("min-{}", key),
        BoundEnd::Max => format!("max-{}", key),
      };

      let mut lower_value = lower_dimension;
      let mut upper_value = upper_dimension;

      if !_eq1 {
        adjust_dimension(&mut lower_value, lower_end, EPSILON);
      }

      if !_eq2 {
        adjust_dimension(&mut upper_value, upper_end, EPSILON);
      }

      Ok(MediaQueryRule::And(MediaAndRules::new(vec![
        MediaQueryRule::Pair(MediaRulePair::new(key_for(lower_end), lower_value)),
        MediaQueryRule::Pair(MediaRulePair::new(key_for(upper_end), upper_value)),
      ])))
    },
    "double_inequality_rule_parser",
  )
}

// ==================== Query structure ====================
//
// The grammar of a media query, from the smallest part to the whole:
//
//   item       = media type | (list) | (not ...) | range | feature
//   list       = item (and item)+ | item (or item)+
//   (not ...)  = ( not <list, or one feature, (not ...), (list) or (type)> )
//   bare not   = not <one (list), (not ...), range or feature>
//   query      = <bare not | list | item> (, <bare not | list | item>)*
//
// A list mixes no joiners, and parentheses hold a list of two or more items.
// A bare `not` stands only at the start of a query.

/// The kind of one item in a media query.
///
/// A list takes items of every kind. An item that stands alone must be of a
/// kind that its place allows, and each place states its own kinds.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum ItemKind {
  /// A media type, as in `screen` or `not only print`.
  MediaType,
  /// A list in parentheses, as in `((color) and (hover: hover))`.
  List,
  /// A `not` in parentheses, as in `(not (color))`.
  Not,
  /// A range with two bounds, as in `(100px <= width <= 200px)`. It is not
  /// the same kind as a range with one bound, because neither `not` takes it
  /// alone.
  DoubleRange,
  /// A range with one bound, as in `(width >= 100px)`.
  Range,
  /// A feature and its value, as in `(min-width: 100px)`.
  Pair,
  /// A feature without a value, as in `(color)`.
  Word,
}

/// One parsed item and its kind.
#[derive(Clone, Debug)]
struct MediaItem {
  rule: MediaQueryRule,
  kind: ItemKind,
}

/// The items that can stand alone after a bare `not`.
const BARE_NOT_OPERAND_KINDS: [ItemKind; 5] = [
  ItemKind::List,
  ItemKind::Not,
  ItemKind::Range,
  ItemKind::Pair,
  ItemKind::Word,
];

/// The items that can stand alone in a `(not ...)`.
const NOT_OPERAND_KINDS: [ItemKind; 4] = [
  ItemKind::List,
  ItemKind::Not,
  ItemKind::Pair,
  ItemKind::Word,
];

/// Give each rule that `parser` reads the kind `kind`.
fn with_kind(parser: TokenParser<MediaQueryRule>, kind: ItemKind) -> TokenParser<MediaItem> {
  parser.map(move |rule| MediaItem { rule, kind }, Some("with_kind"))
}

/// One item of a media query. The parsers are tried in this order, and the
/// first that reads the item gives it.
fn media_item_parser() -> TokenParser<MediaItem> {
  TokenParser::one_of(vec![
    with_kind(media_keyword_parser(), ItemKind::MediaType),
    with_kind(parenthesized_list_parser(), ItemKind::List),
    parenthesized_not_parser(),
    with_kind(double_inequality_rule_parser(), ItemKind::DoubleRange),
    with_kind(combined_inequality_parser(), ItemKind::Range),
    with_kind(
      simple_pair_parser(media_rule_value_parser()),
      ItemKind::Pair,
    ),
    with_kind(media_word_rule_parser(), ItemKind::Word),
  ])
}

/// The word that joins the items of a list.
#[derive(Clone, Copy, PartialEq, Eq)]
enum Joiner {
  And,
  Or,
}

/// What [`parse_media_rules`] read: one item, or a list of two or more.
enum MediaRules {
  Single(MediaItem),
  List(MediaQueryRule),
}

/// Parse one item, and then the items that `and` or `or` joins to it.
///
/// This reads an `and` list and an `or` list in one pass. The first joiner
/// sets the type of the list, and a different joiner ends the list. Thus the
/// list in `(a) and (b) or (c)` stops before `or`, and the caller refuses the
/// rest. Two passes, one for each joiner, would read each nested list twice
/// at each level.
fn parse_media_rules(tokens: &mut TokenList) -> Result<MediaRules, CssParseError> {
  let first = (media_item_parser().run)(tokens)?;

  let Some((joiner, second)) = parse_joined_item(tokens, None) else {
    return Ok(MediaRules::Single(first));
  };

  let mut rules = vec![first.rule, second.rule];
  while let Some((_, item)) = parse_joined_item(tokens, Some(joiner)) {
    rules.push(item.rule);
  }

  Ok(MediaRules::List(match joiner {
    Joiner::And => and_list_rule(rules),
    Joiner::Or => MediaQueryRule::Or(MediaOrRules::new(rules)),
  }))
}

/// Parse a joiner with whitespace on each side, and the item after it.
///
/// When `expected` is given, only that joiner is read. When nothing is read,
/// the token position does not change.
fn parse_joined_item(
  tokens: &mut TokenList,
  expected: Option<Joiner>,
) -> Option<(Joiner, MediaItem)> {
  let checkpoint = tokens.save_position();

  let joined = (|| {
    if !matches!(
      tokens.consume_next_token(),
      Ok(Some(SimpleToken::Whitespace))
    ) {
      return None;
    }

    let joiner = match tokens.consume_next_token() {
      Ok(Some(SimpleToken::Ident(word))) if word == "and" => Joiner::And,
      Ok(Some(SimpleToken::Ident(word))) if word == "or" => Joiner::Or,
      _ => return None,
    };

    if expected.is_some_and(|expected| expected != joiner) {
      return None;
    }

    if !matches!(
      tokens.consume_next_token(),
      Ok(Some(SimpleToken::Whitespace))
    ) {
      return None;
    }

    let item = (media_item_parser().run)(tokens).ok()?;
    Some((joiner, item))
  })();

  if joined.is_none() {
    let _ = tokens.restore_position(checkpoint);
  }

  joined
}

/// The rule for an `and` list of two or more rules.
///
/// A leading `not` negates the whole query, so `not screen and (a)` is
/// `not (screen and (a))`, and not `(not screen) and (a)`.
fn and_list_rule(mut rules: Vec<MediaQueryRule>) -> MediaQueryRule {
  if let Some(MediaQueryRule::MediaKeyword(first)) = rules.first_mut()
    && first.not
  {
    first.not = false;
    return MediaQueryRule::Not(MediaNotRule::new(MediaQueryRule::And(MediaAndRules::new(
      rules,
    ))));
  }
  MediaQueryRule::And(MediaAndRules::new(rules))
}

/// Read an item that stands alone, where `(not ...)` comes before a list.
///
/// The text `(not screen or (color))` has two readings. As a list in
/// parentheses, it is `(not screen) or (color)`. As a `(not ...)`, it is `not
/// (screen or (color))`. An item in a list, and the item after a bare `not`,
/// use the list reading. An item that stands alone uses the `(not ...)`
/// reading.
///
/// The item parser gives the list reading, and this function changes it back.
/// Only an `or` list has two different readings. See [`list_reading`].
fn read_alone(item: MediaItem) -> MediaItem {
  let MediaItem {
    rule: MediaQueryRule::Or(mut or_rules),
    kind: ItemKind::List,
  } = item
  else {
    return item;
  };

  match or_rules.rules.first_mut() {
    Some(MediaQueryRule::MediaKeyword(first)) if first.not => {
      first.not = false;
      MediaItem {
        rule: MediaQueryRule::Not(MediaNotRule::new(MediaQueryRule::Or(or_rules))),
        kind: ItemKind::Not,
      }
    },
    _ => MediaItem {
      rule: MediaQueryRule::Or(or_rules),
      kind: ItemKind::List,
    },
  }
}

/// The error for an item that cannot stand alone in its place.
fn item_cannot_stand_alone() -> CssParseError {
  CssParseError::ParseError {
    message: "This condition cannot stand alone here".to_string(),
  }
}

/// Consume the next token if it is `expected`, or fail with `message`.
fn expect_token(
  tokens: &mut TokenList,
  expected: &SimpleToken,
  message: &str,
) -> Result<(), CssParseError> {
  match tokens.peek() {
    Ok(Some(token)) if token == *expected => {
      let _ = tokens.consume_next_token();
      Ok(())
    },
    _ => Err(CssParseError::ParseError {
      message: message.to_string(),
    }),
  }
}

/// Consume the `not` keyword and the whitespace after it.
fn expect_not_keyword(tokens: &mut TokenList) -> Result<(), CssParseError> {
  expect_token(
    tokens,
    &SimpleToken::Ident("not".to_string()),
    "Expected the `not` keyword",
  )?;
  expect_token(
    tokens,
    &SimpleToken::Whitespace,
    "Expected whitespace after `not`",
  )
}

/// A list of two or more items in parentheses, as in `((a) and (b))`.
///
/// No whitespace can follow `(` or come before `)`. One item alone in
/// parentheses, as in `((color))`, does not parse.
///
/// Text that starts with `(not` is left to [`parenthesized_not_parser`],
/// which gives each reading of it. Thus no text is parsed twice. If this
/// parser read `(not screen and X )` as a list, it would read all of `X`
/// before it found the whitespace before `)`. Then the `not` parser would read
/// `X` again, and nested text would double the work at each level.
fn parenthesized_list_parser() -> TokenParser<MediaQueryRule> {
  TokenParser::new(
    |tokens| {
      expect_token(tokens, &SimpleToken::LeftParen, "Expected `(`")?;

      if matches!(tokens.peek(), Ok(Some(SimpleToken::Ident(word))) if word == "not") {
        return Err(CssParseError::ParseError {
          message: "A `(not` starts a `not` condition, not a list".to_string(),
        });
      }

      // Inside the depth budget, because a list holds lists.
      let rule = tokens.with_depth(|tokens| match parse_media_rules(tokens)? {
        MediaRules::List(rule) => Ok(rule),
        MediaRules::Single(_) => Err(CssParseError::ParseError {
          message: "Parentheses must hold two or more conditions".to_string(),
        }),
      })?;

      expect_token(tokens, &SimpleToken::RightParen, "Expected `)`")?;
      Ok(rule)
    },
    "parenthesized_list_parser",
  )
}

/// A media type in parentheses, as in `(screen)`. This is valid only as the
/// whole operand of a `(not ...)`.
fn parenthesized_media_type_parser() -> TokenParser<MediaQueryRule> {
  in_parens(basic_media_type_parser()).map(
    |key| MediaQueryRule::MediaKeyword(MediaKeyword::new(key, false, false)),
    Some("parenthesized_media_type"),
  )
}

/// A `not` in parentheses, as in `(not (color))` or `(not screen and
/// (color))`.
///
/// The operand is a list, or one item of the kinds in [`NOT_OPERAND_KINDS`],
/// or a media type in parentheses. A media type in parentheses must be the
/// whole operand, so `(not (all) and (color))` does not parse. Whitespace can
/// come before `)`, but not after `(`.
///
/// This parser also gives the list reading of its text, where the text has
/// one. See [`list_reading`].
fn parenthesized_not_parser() -> TokenParser<MediaItem> {
  TokenParser::new(
    |tokens| {
      expect_token(tokens, &SimpleToken::LeftParen, "Expected `(`")?;
      expect_not_keyword(tokens)?;

      // Inside the depth budget, because the operand can hold more
      // parentheses. The flag tells if the operand is a list without
      // parentheses.
      let (operand, operand_is_list) = tokens.with_depth(|tokens| {
        if let Ok(media_type) = (parenthesized_media_type_parser().run)(tokens) {
          return Ok((media_type, false));
        }

        match parse_media_rules(tokens)? {
          MediaRules::List(rule) => Ok((rule, true)),
          MediaRules::Single(item) => {
            let item = read_alone(item);
            if NOT_OPERAND_KINDS.contains(&item.kind) {
              Ok((item.rule, false))
            } else {
              Err(item_cannot_stand_alone())
            }
          },
        }
      })?;

      let space_before_close = matches!(tokens.peek(), Ok(Some(SimpleToken::Whitespace)));
      skip_whitespace(tokens);
      expect_token(tokens, &SimpleToken::RightParen, "Expected `)`")?;

      // A list in parentheses cannot have whitespace before `)`.
      if operand_is_list
        && !space_before_close
        && let Some(rule) = list_reading(&operand)
      {
        return Ok(MediaItem {
          rule,
          kind: ItemKind::List,
        });
      }

      Ok(MediaItem {
        rule: MediaQueryRule::Not(MediaNotRule::new(operand)),
        kind: ItemKind::Not,
      })
    },
    "parenthesized_not_parser",
  )
}

/// The list reading of the text of a `(not ...)`, where the text has one.
///
/// The text `(not screen or (color))` is also a list in parentheses, whose
/// first item is `not screen`. That reading exists when the operand is a list
/// without parentheses and its first item is a media type without `not`.
/// Only an `or` list gives a different rule: for an `and` list, both readings
/// give `not (screen and (color))`. Thus this function changes only an `or`
/// list.
fn list_reading(operand: &MediaQueryRule) -> Option<MediaQueryRule> {
  let MediaQueryRule::Or(or_rules) = operand else {
    return None;
  };

  let mut or_rules = or_rules.clone();
  match or_rules.rules.first_mut() {
    Some(MediaQueryRule::MediaKeyword(first)) if !first.not => {
      first.not = true;
      Some(MediaQueryRule::Or(or_rules))
    },
    _ => None,
  }
}

/// A bare `not` at the start of a query, as in `not (min-width: 100px)`.
///
/// It takes one item of the kinds in [`BARE_NOT_OPERAND_KINDS`]. A list after
/// it must be in parentheses, so `not (a) and (b)` does not parse. A media type
/// after `not`, as in `not screen`, is read by [`media_keyword_parser`].
fn leading_not_parser() -> TokenParser<MediaQueryRule> {
  TokenParser::new(
    |tokens| {
      expect_not_keyword(tokens)?;

      let item = tokens.with_depth(|tokens| (media_item_parser().run)(tokens))?;
      if !BARE_NOT_OPERAND_KINDS.contains(&item.kind) {
        return Err(item_cannot_stand_alone());
      }

      Ok(MediaQueryRule::Not(MediaNotRule::new(item.rule)))
    },
    "leading_not_parser",
  )
}

/// One query of a comma list that does not start with a bare `not`: a list,
/// or one item of any kind.
fn normal_rule_parser() -> TokenParser<MediaQueryRule> {
  TokenParser::new(
    |tokens| {
      Ok(match parse_media_rules(tokens)? {
        MediaRules::List(rule) => rule,
        MediaRules::Single(item) => read_alone(item).rule,
      })
    },
    "normal_rule_parser",
  )
}

/// One query of a comma list. A bare `not` is tried first. When it reads its
/// operand, the query ends there.
fn media_query_segment_parser() -> TokenParser<MediaQueryRule> {
  TokenParser::one_of(vec![leading_not_parser(), normal_rule_parser()])
}

/// Parse a whole media query: a comma-separated list of conditions.
///
/// Comma and `or` both mean disjunction, and both end up in the same `Or` node,
/// but they do not bind equally: `or` groups inside one comma segment, and the
/// segments group above it. Flattening the two into one list would be simpler
/// and is wrong -- `(a) and (b), (c) or (d)` is two segments, not three
/// disjuncts, and the last-media-query-wins transform distributes its negations
/// over whatever the top-level `Or` holds. Getting the nesting wrong there
/// changes the emitted rule text, and with it the class name.
///
/// One comma after the last query is read and dropped, so `(color),` is
/// `(color)`. A comma that is not followed by a query ends the list, and the
/// caller refuses the text after it, as in `(color),,`.
fn media_query_rule_parser() -> TokenParser<MediaQueryRule> {
  TokenParser::new(
    |tokens| {
      let mut segments = vec![(media_query_segment_parser().run)(tokens)?];

      loop {
        let checkpoint = tokens.save_position();
        skip_whitespace(tokens);

        if !matches!(tokens.peek(), Ok(Some(SimpleToken::Comma))) {
          let _ = tokens.restore_position(checkpoint);
          break;
        }

        let _ = tokens.consume_next_token();
        skip_whitespace(tokens);

        let after_comma = tokens.save_position();
        match (media_query_segment_parser().run)(tokens) {
          Ok(segment) => segments.push(segment),
          Err(_) => {
            let _ = tokens.restore_position(after_comma);
            break;
          },
        }
      }

      Ok(collapse_single_rule(segments, |segments| {
        MediaQueryRule::Or(MediaOrRules::new(segments))
      }))
    },
    "media_query_rule_parser",
  )
}

/// Consume any run of whitespace tokens, leaving the next real token in place.
fn skip_whitespace(tokens: &mut TokenList) {
  while let Ok(Some(SimpleToken::Whitespace)) = tokens.peek() {
    let _ = tokens.consume_next_token();
  }
}

/// Returns the sole rule when `rules` holds exactly one, otherwise groups them
/// with `combine`. A lone rule needs no combinator wrapper. The array
/// conversion hands back the original `Vec` on any other length, so both arms
/// are reachable and neither needs fallible indexing.
fn collapse_single_rule(
  rules: Vec<MediaQueryRule>,
  combine: impl FnOnce(Vec<MediaQueryRule>) -> MediaQueryRule,
) -> MediaQueryRule {
  match <[MediaQueryRule; 1]>::try_from(rules) {
    Ok([only]) => only,
    Err(rules) => combine(rules),
  }
}

#[cfg(test)]
#[path = "../tests/at_queries/media_query_tests.rs"]
mod tests;

#[cfg(test)]
#[path = "../tests/at_queries/parse_media_query_test.rs"]
mod parse_media_query_test;

#[cfg(test)]
#[path = "../tests/at_queries/validation_media_query_test.rs"]
mod validation_media_query_test;

#[cfg(test)]
#[path = "../tests/at_queries/media_query_coverage_test.rs"]
mod media_query_coverage_test;
