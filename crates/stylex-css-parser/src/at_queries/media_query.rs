/*!
Media query parsing and representation.

Core functionality for parsing and representing CSS media queries.
*/

use stylex_utils::number::{to_js_string, write_js_number};

use crate::{
  CssParseError,
  css_types::{Length, calc::Calc},
  token_parser::{TokenParser, tokens},
  token_types::SimpleToken,
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
      queries: Self::normalize(queries),
    }
  }

  pub fn new_from_rule(rule: MediaQueryRule) -> Self {
    Self::new(rule)
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

        let merged = merge_intervals_for_and(flattened);
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
        let should_parenthesize = !is_top_level && !keyword.only && !keyword.not;
        if should_parenthesize {
          format!("{}({})", prefix, keyword.key)
        } else {
          format!("{}{}", prefix, keyword.key)
        }
      },
      MediaQueryRule::WordRule(word_rule) => {
        format!("({})", word_rule.key_value)
      },
      MediaQueryRule::Pair(pair) => match &pair.value {
        MediaRuleValue::Fraction(frac) => {
          format!("({}: {} / {})", pair.key, frac.numerator, frac.denominator)
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
        let rule_strings: Vec<String> = and_rules
          .rules
          .iter()
          .map(|rule| MediaQuery::format_queries(rule, false))
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
            _ => MediaQuery::format_queries(rule, false),
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
        Ok(MediaQuery::new_from_rule(rule))
      },
      "media_query_parser",
    )
  }

  /// Check if parentheses are balanced in a media query string
  pub fn has_balanced_parens(input: &str) -> bool {
    has_balanced_parens(input)
  }
}

/// Validate media query string
pub fn validate_media_query(input: &str) -> Result<MediaQuery, String> {
  if !has_balanced_parens(input) {
    return Err(crate::at_queries::messages::MediaQueryErrors::UNBALANCED_PARENS.to_string());
  }

  match MediaQuery::parser().parse_to_end(input) {
    Ok(media_query) => Ok(media_query),
    Err(_) => Err(crate::at_queries::messages::MediaQueryErrors::SYNTAX_ERROR.to_string()),
  }
}

/// Check if parentheses are balanced
fn has_balanced_parens(input: &str) -> bool {
  let mut count = 0;
  for ch in input.chars() {
    match ch {
      '(' => count += 1,
      ')' => {
        count -= 1;
        if count < 0 {
          return false;
        }
      },
      _ => {},
    }
  }
  count == 0
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
/// A negated bound is the first: `not (min-width: 600px)` is
/// `max-width: 599.99px`. A strict inequality is the second: a `width` greater
/// than `400px` is `min-width: 400.01px`, and `(400px < width <= 700px)` nudges
/// its lower bound the same way.
///
/// Both ask how to spell "not this value" in a syntax that has only inclusive
/// bounds, so they read one constant rather than three copies of it.
const EPSILON: f64 = 0.01;

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
fn constraint_interval(bound: Bound, length: &Length, negated: bool) -> (f64, f64) {
  let value = length.value;

  // A negated `min-` bound is a `max-` bound just below it, and vice versa.
  match (bound, negated) {
    (Bound::Min, false) => (value, f64::INFINITY),
    (Bound::Max, false) => (f64::NEG_INFINITY, value),
    (Bound::Min, true) => (f64::NEG_INFINITY, value - EPSILON),
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

/// Whether distributing `rules` by DeMorgan can only ever yield contradictions,
/// so the whole subtree can be dropped without expanding it.
///
/// Distribution never removes a constraint, it only adds one negated operand
/// per `not (A and B)` clause. So if the numeric constraints already present
/// contradict each other, every leaf below this point contradicts too — but
/// only when every leaf is decided by that numeric merge. A rule the merge
/// cannot read hands the list back unchanged instead of empty, which is a
/// surviving branch, so this demands that each rule is either a numeric
/// constraint itself or a two-operand `not (A and B)` whose operands both
/// become one when negated.
///
/// Without this the ladder of disjoint breakpoints that authors actually write
/// costs 2^n branch expansions: each negated neighbour splits the list in two
/// and the dead half is only recognized after it has been expanded in full.
fn distribution_is_hopeless(rules: &[MediaQueryRule]) -> bool {
  /// Whether `not operand` is one of the numeric constraints the merge reads.
  /// Only a bare pair qualifies: negating an operand that is itself a `not`
  /// leaves a double negation the merge declines to read.
  fn negation_is_numeric(operand: &MediaQueryRule) -> bool {
    matches!(operand, MediaQueryRule::Pair(_))
      && DIMENSIONS
        .iter()
        .any(|dim| dimension_constraint(operand, dim).is_some())
  }

  let mut dimensions = new_dimension_intervals();

  for rule in rules {
    let numeric = dimensions.iter_mut().find_map(|(dim, state)| {
      dimension_constraint(rule, dim).map(|(bound, length, negated)| {
        state.push(constraint_interval(bound, length, negated), &length.unit);
      })
    });

    if numeric.is_some() {
      continue;
    }

    // The only other rule that may appear is a clause distribution will turn
    // into numeric constraints on both branches.
    let distributable = matches!(
      rule,
      MediaQueryRule::Not(not_rule)
        if matches!(
          not_rule.rule.as_ref(),
          MediaQueryRule::And(and_rules)
            if and_rules.rules.len() == 2
              && and_rules.rules.iter().all(negation_is_numeric)
        )
    );

    if !distributable {
      return false;
    }
  }

  // Mixed units are handed back unchanged rather than merged, so they are not
  // a contradiction this may act on.
  !dimensions.iter().any(|(_, state)| state.unit_conflict)
    && dimensions
      .iter()
      .any(|(_, state)| !state.intervals.is_empty() && state.intersect().is_none())
}

/// Merge the numeric width/height constraints of an `and` list into a single
/// interval per dimension.
///
/// The returned `Vec` carries three outcomes, and callers must read all three:
/// empty means the constraints contradict each other, which the caller turns
/// into `not all`; `rules` handed back unchanged means the list was not
/// interval-mergeable, whether from a non-numeric rule or from units that
/// disagree; anything else is the merged interval pairs. Collapsing the three
/// into one `Vec` is the shape the canonicalization pipeline is specified
/// against, so it stays.
fn merge_intervals_for_and(rules: Vec<MediaQueryRule>) -> Vec<MediaQueryRule> {
  // Every branch below this one contradicts, so none of them need building.
  if distribution_is_hopeless(&rules) {
    return Vec::new();
  }

  let mut dimensions = new_dimension_intervals();

  // Handle DeMorgan's law: not (A and B) = (not A) or (not B)
  for rule in &rules {
    if let MediaQueryRule::Not(not_rule) = rule
      && let MediaQueryRule::And(and_rules) = not_rule.rule.as_ref()
      && and_rules.rules.len() == 2
    {
      let left = &and_rules.rules[0];
      let right = &and_rules.rules[1];

      // Create left branch: all rules except current, plus (not left)
      let mut left_branch_rules: Vec<MediaQueryRule> = rules
        .iter()
        .filter(|r| !std::ptr::eq(*r, rule))
        .cloned()
        .collect();
      left_branch_rules.push(MediaQueryRule::Not(MediaNotRule::new(left.clone())));

      // Create right branch: all rules except current, plus (not right)
      let mut right_branch_rules: Vec<MediaQueryRule> = rules
        .iter()
        .filter(|r| !std::ptr::eq(*r, rule))
        .cloned()
        .collect();
      right_branch_rules.push(MediaQueryRule::Not(MediaNotRule::new(right.clone())));

      // Recursively process each branch
      let left_branch = merge_intervals_for_and(left_branch_rules);
      let right_branch = merge_intervals_for_and(right_branch_rules);

      // Contradictory branches are dropped; a branch of several rules is
      // re-wrapped in `and`. An `or` left empty by this is kept as-is and
      // collapsed to `not all` by serialization.
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

      state.push(constraint_interval(bound, length, negated), &length.unit);

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

    if lower.is_finite() {
      result.push(MediaQueryRule::Pair(MediaRulePair::new(
        format!("min-{dim}"),
        MediaRuleValue::Length(Length::new(lower, state.unit.clone())),
      )));
    }

    if upper.is_finite() {
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

/// Adjust a `MediaRuleValue::Length` in a reversed inequality by epsilon.
/// The else branch (non-Length) is a defensive arm unreachable via the reversed
/// inequality parser, which always produces `MediaRuleValue::Length`.
fn adjust_reversed_inequality_dimension(value: &mut MediaRuleValue, op: char, epsilon: f64) {
  if let MediaRuleValue::Length(length) = value {
    if op == '>' {
      length.value -= epsilon;
    } else {
      length.value += epsilon;
    }
  }
  // else: non-Length defensive arm — unreachable via the reversed inequality parser
}

/// Determine the (min_value, max_value) pair for a double-inequality expression
/// `A op1 width op2 B`, based on operator types and strictness.
///
/// The final `else` branch (when both ops are inclusive but op1 is neither `<` nor `>`)
/// is a defensive fallback; it is unreachable through the public parser because the
/// double-inequality parser validates that op1 must be `<` or `>`.
fn select_double_inequality_values(
  op1: char,
  eq1: bool,
  op2: char,
  eq2: bool,
  lower: MediaRuleValue,
  upper: MediaRuleValue,
) -> (MediaRuleValue, MediaRuleValue) {
  if !eq1 {
    if op1 == '>' {
      (upper, lower)
    } else {
      (lower, upper)
    }
  } else if !eq2 {
    if op2 == '>' {
      (upper, lower)
    } else {
      (lower, upper)
    }
  } else {
    // Both operators are inclusive — determine by operator type
    if op1 == '>' && eq1 {
      (upper, lower)
    } else if op1 == '<' && eq1 {
      (lower, upper)
    } else {
      // Fallback: op1 is neither '<' nor '>' — unreachable via normal parsing
      (lower, upper)
    }
  }
}

/// Apply an additive epsilon to a `MediaRuleValue::Length`'s value.
/// The else branch (non-Length) is a defensive arm unreachable via the
/// double-inequality parser, which always produces `MediaRuleValue::Length`.
fn apply_epsilon_to_min_value(value: &mut MediaRuleValue, epsilon: f64) {
  if let MediaRuleValue::Length(length) = value {
    length.value += epsilon;
  }
  // else: non-Length defensive arm — unreachable via the double inequality parser
}

/// Apply a subtractive epsilon to a `MediaRuleValue::Length`'s value.
/// The else branch (non-Length) is a defensive arm unreachable via the
/// double-inequality parser, which always produces `MediaRuleValue::Length`.
fn apply_epsilon_to_max_value(value: &mut MediaRuleValue, epsilon: f64) {
  if let MediaRuleValue::Length(length) = value {
    length.value -= epsilon;
  }
  // else: non-Length defensive arm — unreachable via the double inequality parser
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
        while let Ok(Some(SimpleToken::Whitespace)) = tokens.peek() {
          let _ = tokens.consume_next_token();
        }
      }

      // Try to parse optional "only" after "not" (or at beginning if no "not")
      if let Ok(Some(SimpleToken::Ident(val))) = tokens.peek()
        && val == "only"
      {
        let _ = tokens.consume_next_token(); // consume "only"
        only_value = true;

        // Consume whitespace after "only"
        while let Ok(Some(SimpleToken::Whitespace)) = tokens.peek() {
          let _ = tokens.consume_next_token();
        }
      }

      if not_value && only_value {
        return Err(CssParseError::ParseError {
          message: "Media query modifiers 'not' and 'only' cannot be combined".to_string(),
        });
      }

      // Parse the media type (required)
      let media_type = (basic_media_type_parser().run)(tokens)?;

      Ok(MediaQueryRule::MediaKeyword(MediaKeyword::new(
        media_type, not_value, only_value,
      )))
    },
    "media_keyword_parser",
  )
}

/// Media word rule parser for (color), (monochrome), etc.
fn media_word_rule_parser() -> TokenParser<MediaQueryRule> {
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
    )
    .surrounded_by(
      TokenParser::<SimpleToken>::token(SimpleToken::LeftParen, Some("OpenParen")),
      Some(TokenParser::<SimpleToken>::token(
        SimpleToken::RightParen,
        Some("CloseParen"),
      )),
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
        while let Ok(Some(SimpleToken::Whitespace)) = tokens.peek() {
          let _ = tokens.consume_next_token();
        }

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
        while let Ok(Some(SimpleToken::Whitespace)) = tokens.peek() {
          let _ = tokens.consume_next_token();
        }

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
      while let Ok(Some(SimpleToken::Whitespace)) = tokens.peek() {
        let _ = tokens.consume_next_token();
      }

      // Parse key (identifier)
      let key = if let Ok(Some(SimpleToken::Ident(key_name))) = tokens.consume_next_token() {
        key_name
      } else {
        return Err(CssParseError::ParseError {
          message: "Expected media feature name".to_string(),
        });
      };

      // Optional whitespace before colon
      while let Ok(Some(SimpleToken::Whitespace)) = tokens.peek() {
        let _ = tokens.consume_next_token();
      }

      // Parse colon
      if let Ok(Some(SimpleToken::Colon)) = tokens.consume_next_token() {
        // Good, we have colon
      } else {
        return Err(CssParseError::ParseError {
          message: "Expected colon after media feature name".to_string(),
        });
      }

      // Optional whitespace after colon
      while let Ok(Some(SimpleToken::Whitespace)) = tokens.peek() {
        let _ = tokens.consume_next_token();
      }

      // Parse value using the cloned value parser
      let value = (value_parser_rc)(tokens)?;

      // Optional whitespace before closing paren
      while let Ok(Some(SimpleToken::Whitespace)) = tokens.peek() {
        let _ = tokens.consume_next_token();
      }

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
      while let Ok(Some(SimpleToken::Whitespace)) = tokens.peek() {
        let _ = tokens.consume_next_token();
      }

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
      while let Ok(Some(SimpleToken::Whitespace)) = tokens.peek() {
        let _ = tokens.consume_next_token();
      }

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
      while let Ok(Some(SimpleToken::Whitespace)) = tokens.peek() {
        let _ = tokens.consume_next_token();
      }

      // Parse optional equals sign
      let has_equals = if let Ok(Some(SimpleToken::Delim('='))) = tokens.peek() {
        let _ = tokens.consume_next_token();
        true
      } else {
        false
      };

      // Skip optional whitespace
      while let Ok(Some(SimpleToken::Whitespace)) = tokens.peek() {
        let _ = tokens.consume_next_token();
      }

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
      while let Ok(Some(SimpleToken::Whitespace)) = tokens.peek() {
        let _ = tokens.consume_next_token();
      }

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
      while let Ok(Some(SimpleToken::Whitespace)) = tokens.peek() {
        let _ = tokens.consume_next_token();
      }

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
      while let Ok(Some(SimpleToken::Whitespace)) = tokens.peek() {
        let _ = tokens.consume_next_token();
      }

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
      while let Ok(Some(SimpleToken::Whitespace)) = tokens.peek() {
        let _ = tokens.consume_next_token();
      }

      // Parse optional equals sign
      let has_equals = if let Ok(Some(SimpleToken::Delim('='))) = tokens.peek() {
        let _ = tokens.consume_next_token();
        true
      } else {
        false
      };

      // Skip optional whitespace
      while let Ok(Some(SimpleToken::Whitespace)) = tokens.peek() {
        let _ = tokens.consume_next_token();
      }

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
      while let Ok(Some(SimpleToken::Whitespace)) = tokens.peek() {
        let _ = tokens.consume_next_token();
      }

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
        adjust_reversed_inequality_dimension(&mut adjusted_dimension, op, EPSILON);
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
      while let Ok(Some(SimpleToken::Whitespace)) = tokens.peek() {
        let _ = tokens.consume_next_token();
      }

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
      while let Ok(Some(SimpleToken::Whitespace)) = tokens.peek() {
        let _ = tokens.consume_next_token();
      }

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
      while let Ok(Some(SimpleToken::Whitespace)) = tokens.peek() {
        let _ = tokens.consume_next_token();
      }

      // Parse optional first equals sign
      let _eq1 = if let Ok(Some(SimpleToken::Delim('='))) = tokens.peek() {
        let _ = tokens.consume_next_token();
        true
      } else {
        false
      };

      // Skip optional whitespace
      while let Ok(Some(SimpleToken::Whitespace)) = tokens.peek() {
        let _ = tokens.consume_next_token();
      }

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
      while let Ok(Some(SimpleToken::Whitespace)) = tokens.peek() {
        let _ = tokens.consume_next_token();
      }

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
      while let Ok(Some(SimpleToken::Whitespace)) = tokens.peek() {
        let _ = tokens.consume_next_token();
      }

      // Parse optional second equals sign
      let _eq2 = if let Ok(Some(SimpleToken::Delim('='))) = tokens.peek() {
        let _ = tokens.consume_next_token();
        true
      } else {
        false
      };

      // Skip optional whitespace
      while let Ok(Some(SimpleToken::Whitespace)) = tokens.peek() {
        let _ = tokens.consume_next_token();
      }

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
      while let Ok(Some(SimpleToken::Whitespace)) = tokens.peek() {
        let _ = tokens.consume_next_token();
      }

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

      // Return an AND rule with min and max constraints
      // For (A op1 width op2 B), we need to determine min and max constraints
      let min_key = format!("min-{}", key);
      let max_key = format!("max-{}", key);

      // Adjust values with epsilon only for strict inequalities

      // Determine which dimension is min vs max based on the operators
      // For (A op1 width op2 B), we need to map to min/max constraints

      let (mut min_value, mut max_value) =
        select_double_inequality_values(_op1, _eq1, _op2, _eq2, lower_dimension, upper_dimension);

      // Apply epsilon for strict (non-inclusive) operators
      if (_op1 == '<' && !_eq1) || (_op2 == '>' && !_eq2) {
        apply_epsilon_to_min_value(&mut min_value, EPSILON);
      }
      if (_op1 == '>' && !_eq1) || (_op2 == '<' && !_eq2) {
        apply_epsilon_to_max_value(&mut max_value, EPSILON);
      }

      Ok(MediaQueryRule::And(MediaAndRules::new(vec![
        MediaQueryRule::Pair(MediaRulePair::new(min_key, min_value)),
        MediaQueryRule::Pair(MediaRulePair::new(max_key, max_value)),
      ])))
    },
    "double_inequality_rule_parser",
  )
}

/// Enhanced NOT parser that handles complex nested expressions
fn leading_not_parser() -> TokenParser<MediaQueryRule> {
  TokenParser::new(
    |tokens| {
      // Expect "not" keyword
      let not_token =
        tokens
          .consume_next_token()
          .ok()
          .flatten()
          .ok_or(CssParseError::ParseError {
            message: "Expected 'not' keyword".to_string(),
          })?;
      if let SimpleToken::Ident(keyword) = not_token {
        if keyword != "not" {
          return Err(CssParseError::ParseError {
            message: format!("Expected 'not', got '{}'", keyword),
          });
        }
      } else {
        return Err(CssParseError::ParseError {
          message: format!("Expected identifier, got {:?}", not_token),
        });
      }

      // Skip whitespace after "not"
      let whitespace_token =
        tokens
          .consume_next_token()
          .ok()
          .flatten()
          .ok_or(CssParseError::ParseError {
            message: "Expected whitespace after 'not'".to_string(),
          })?;
      if !matches!(whitespace_token, SimpleToken::Whitespace) {
        return Err(CssParseError::ParseError {
          message: format!("Expected whitespace, got {:?}", whitespace_token),
        });
      }

      if let Ok(Some(SimpleToken::Ident(keyword))) = tokens.peek()
        && keyword == "only"
      {
        return Err(CssParseError::ParseError {
          message: "Media query modifiers 'not' and 'only' cannot be combined".to_string(),
        });
      }

      // Parse the rule that follows "not" using normal rule parser
      let inner_rule = (normal_rule_parser().run)(tokens)?;
      Ok(MediaQueryRule::Not(MediaNotRule::new(inner_rule)))
    },
    "leading_not_parser",
  )
}

/// This parser specifically handles "(not ...)" patterns
fn parenthesized_not_parser() -> TokenParser<MediaQueryRule> {
  TokenParser::new(
    |tokens| {
      // Expect opening parenthesis
      if let Ok(Some(SimpleToken::LeftParen)) = tokens.peek() {
        let _ = tokens.consume_next_token(); // consume '('

        // Skip optional whitespace
        while let Ok(Some(SimpleToken::Whitespace)) = tokens.peek() {
          let _ = tokens.consume_next_token();
        }

        // Expect "not" keyword
        if let Ok(Some(SimpleToken::Ident(keyword))) = tokens.peek() {
          if keyword == "not" {
            let _ = tokens.consume_next_token(); // consume "not"

            // Skip mandatory whitespace after "not"
            if let Ok(Some(SimpleToken::Whitespace)) = tokens.peek() {
              let _ = tokens.consume_next_token();
            } else {
              return Err(CssParseError::ParseError {
                message: "Expected whitespace after 'not' in parenthesized expression".to_string(),
              });
            }

            if let Ok(Some(SimpleToken::Ident(keyword))) = tokens.peek()
              && keyword == "only"
            {
              return Err(CssParseError::ParseError {
                message: "Media query modifiers 'not' and 'only' cannot be combined".to_string(),
              });
            }

            // Parse the rule after "not" using the normal rule parser
            let inner_rule = (normal_rule_parser().run)(tokens)?;

            // Skip optional whitespace before closing
            while let Ok(Some(SimpleToken::Whitespace)) = tokens.peek() {
              let _ = tokens.consume_next_token();
            }

            // Expect closing parenthesis
            if let Ok(Some(SimpleToken::RightParen)) = tokens.peek() {
              let _ = tokens.consume_next_token(); // consume ')'
              Ok(MediaQueryRule::Not(MediaNotRule::new(inner_rule)))
            } else {
              Err(CssParseError::ParseError {
                message: "Expected closing parenthesis after parenthesized NOT expression"
                  .to_string(),
              })
            }
          } else {
            Err(CssParseError::ParseError {
              message: "Expected 'not' keyword in parenthesized NOT expression".to_string(),
            })
          }
        } else {
          Err(CssParseError::ParseError {
            message: "Expected 'not' keyword in parenthesized NOT expression".to_string(),
          })
        }
      } else {
        Err(CssParseError::ParseError {
          message: "Expected opening parenthesis for parenthesized NOT expression".to_string(),
        })
      }
    },
    "parenthesized_not_parser",
  )
}

fn media_query_rule_parser() -> TokenParser<MediaQueryRule> {
  // Parse OR-separated rules (comma-separated)
  or_combinator_parser()
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

/// Parse OR-separated media query rules (comma-separated OR "or" keyword)
fn or_combinator_parser() -> TokenParser<MediaQueryRule> {
  TokenParser::new(
    |tokens| {
      let mut rules = Vec::new();

      // Parse the first rule
      let first_rule = (and_combinator_parser().run)(tokens)?;
      rules.push(first_rule);

      // Parse additional OR rules (comma-separated OR "or" keyword)
      loop {
        let checkpoint = tokens.save_position();

        // Skip optional whitespace
        while let Ok(Some(SimpleToken::Whitespace)) = tokens.peek() {
          let _ = tokens.consume_next_token();
        }

        // Check for comma-separated OR
        if let Ok(Some(SimpleToken::Comma)) = tokens.peek() {
          let _ = tokens.consume_next_token(); // consume comma

          // Skip optional whitespace after comma
          while let Ok(Some(SimpleToken::Whitespace)) = tokens.peek() {
            let _ = tokens.consume_next_token();
          }

          let rule = (and_combinator_parser().run)(tokens)?;
          rules.push(rule);
          continue;
        }

        // Check for "or" keyword
        if let Ok(Some(SimpleToken::Ident(keyword))) = tokens.peek()
          && keyword == "or"
        {
          let _ = tokens.consume_next_token(); // consume "or"

          // Skip whitespace after "or"
          while let Ok(Some(SimpleToken::Whitespace)) = tokens.peek() {
            let _ = tokens.consume_next_token();
          }

          let rule = (and_combinator_parser().run)(tokens)?;
          rules.push(rule);
          continue;
        }

        // No more OR patterns found, restore position and break
        let _ = tokens.restore_position(checkpoint);
        break;
      }

      // If we only have one rule, return it directly
      Ok(collapse_single_rule(rules, |rules| {
        MediaQueryRule::Or(MediaOrRules::new(rules))
      }))
    },
    "or_combinator_parser",
  )
}

/// Parse AND-separated media query rules
fn and_combinator_parser() -> TokenParser<MediaQueryRule> {
  TokenParser::new(
    |tokens| {
      let mut rules = Vec::new();

      // Parse the first rule
      let first_rule = (normal_rule_parser().run)(tokens)?;
      rules.push(first_rule);

      // Parse additional AND rules
      while let Ok(Some(SimpleToken::Whitespace)) = tokens.peek() {
        // Check if next non-whitespace token is "and"
        let checkpoint = tokens.save_position();

        // Skip whitespace
        while let Ok(Some(SimpleToken::Whitespace)) = tokens.peek() {
          let _ = tokens.consume_next_token();
        }

        // Check for "and" keyword
        if let Ok(Some(SimpleToken::Ident(keyword))) = tokens.peek() {
          if keyword == "and" {
            let _ = tokens.consume_next_token(); // consume "and"

            // Skip whitespace after "and"
            while let Ok(Some(SimpleToken::Whitespace)) = tokens.peek() {
              let _ = tokens.consume_next_token();
            }

            let rule = (normal_rule_parser().run)(tokens)?;
            rules.push(rule);
          } else {
            // Not an "and", restore position and break
            let _ = tokens.restore_position(checkpoint);
            break;
          }
        } else {
          // No identifier after whitespace, restore position and break
          let _ = tokens.restore_position(checkpoint);
          break;
        }
      }

      // If we only have one rule, return it directly
      Ok(collapse_single_rule(rules, |rules| {
        MediaQueryRule::And(MediaAndRules::new(rules))
      }))
    },
    "and_combinator_parser",
  )
}

/// Normal rule parser that combines all rule types
fn normal_rule_parser() -> TokenParser<MediaQueryRule> {
  TokenParser::one_of(vec![
    // Media keyword parser must come first to handle "not screen", "only print" etc.
    // as MediaKeyword rules, not as separate NOT rules
    media_keyword_parser(),
    // Parenthesized NOT parser for "(not ...)" patterns
    parenthesized_not_parser(),
    // Leading not parser for cases where NOT is not part of media keywords
    leading_not_parser(),
    // Parenthesized expressions parser for complex nested cases
    parenthesized_expression_parser(),
    // Double inequality parser: (500px <= width <= 1000px)
    double_inequality_rule_parser(),
    // Combined inequality parser: (width <= 1250px) and (1250px >= width)
    combined_inequality_parser(),
    // Word rule parser for (color), (monochrome), (grid), (color-index)
    media_word_rule_parser(),
    // Pair parser for (key: value) patterns like (min-width: 768px)
    simple_pair_parser(media_rule_value_parser()),
  ])
}

/// Parse parenthesized expressions, including complex NOT expressions
/// Handles: (not (max-width: 1024px)), ((min-width: 500px) and (max-width:
/// 600px))
fn parenthesized_expression_parser() -> TokenParser<MediaQueryRule> {
  TokenParser::new(
    |tokens| {
      // Expect opening parenthesis
      if let Ok(Some(SimpleToken::LeftParen)) = tokens.peek() {
        let _ = tokens.consume_next_token(); // consume '('

        // Skip optional whitespace
        while let Ok(Some(SimpleToken::Whitespace)) = tokens.peek() {
          let _ = tokens.consume_next_token();
        }

        // Try to parse a NOT expression first
        if let Ok(Some(SimpleToken::Ident(keyword))) = tokens.peek()
          && keyword == "not"
        {
          // Parse NOT expression within parentheses
          let not_rule = (leading_not_parser().run)(tokens)?;

          // Skip optional whitespace before closing
          while let Ok(Some(SimpleToken::Whitespace)) = tokens.peek() {
            let _ = tokens.consume_next_token();
          }

          // Expect closing parenthesis
          if let Ok(Some(SimpleToken::RightParen)) = tokens.peek() {
            let _ = tokens.consume_next_token(); // consume ')'
            return Ok(not_rule);
          } else {
            return Err(CssParseError::ParseError {
              message: "Expected closing parenthesis after parenthesized NOT expression"
                .to_string(),
            });
          }
        }

        // Parse complex expression using full combinator parser
        let inner_expression = (and_combinator_parser().run)(tokens)?;

        // Skip optional whitespace before closing
        while let Ok(Some(SimpleToken::Whitespace)) = tokens.peek() {
          let _ = tokens.consume_next_token();
        }

        // Expect closing parenthesis
        if let Ok(Some(SimpleToken::RightParen)) = tokens.peek() {
          let _ = tokens.consume_next_token(); // consume ')'
          Ok(inner_expression)
        } else {
          Err(CssParseError::ParseError {
            message: "Expected closing parenthesis after parenthesized expression".to_string(),
          })
        }
      } else {
        Err(CssParseError::ParseError {
          message: "Expected opening parenthesis for parenthesized expression".to_string(),
        })
      }
    },
    "parenthesized_expression_parser",
  )
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
