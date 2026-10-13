//! The width sort key of a rule's at-rule chain, and the order over such keys.
//!
//! Upstream orders the rules of one property by breakpoint, mobile first: the
//! `min-width` rules ascending, then the `max-width` rules descending, and a
//! rule with no single px bound after both. These are the functions that
//! decide it, in upstream's order: `collectWidthBounds`,
//! `mediaQueryWidthSortKey`, `atRulePreludes`, `widthSortKeyForChain`,
//! `declaredPropertyName`, `compareWidthSortKeys`.

use std::cmp::Ordering;

use stylex_css_parser::at_queries::media_query::{MediaQuery, MediaQueryRule, MediaRuleValue};

/// Which side of the width a bound constrains.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WidthBoundKind {
  Min,
  Max,
}

/// A `min-width` or `max-width` bound in px.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct WidthBound {
  pub kind: WidthBoundKind,
  pub value: f64,
}

/// A rule's width bound plus the at-rule context it sits in. Only rules with
/// an identical context are comparable: a breakpoint nested under `@supports`
/// says nothing about one nested under a different condition.
#[derive(Clone, Debug, PartialEq)]
pub struct WidthSortKey {
  pub context: String,
  pub bound: WidthBound,
}

/// The `min-width`/`max-width` px bounds a media query imposes, or `None` if it
/// has none to sort by: `not` and `or` can widen the matched range or split it
/// in two, and rem/em bounds are unreadable at build time. Zero counts as a
/// bound whatever unit it carries, or none at all.
fn collect_width_bounds(rule: &MediaQueryRule) -> Option<Vec<WidthBound>> {
  match rule {
    MediaQueryRule::Pair(pair) => {
      // Media feature names are ASCII case-insensitive.
      let key = pair.key.to_ascii_lowercase();
      let kind = match key.as_str() {
        "min-width" => WidthBoundKind::Min,
        "max-width" => WidthBoundKind::Max,
        // A non-width feature (e.g. `orientation`) constrains no width.
        _ => return Some(Vec::new()),
      };

      match &pair.value {
        // Zero is the one length that needs no unit. Any other unitless number
        // is invalid CSS for a media feature -- the browser drops the whole
        // query -- so leave those unsorted rather than assume px.
        MediaRuleValue::Number(number) if *number == 0.0 => {
          Some(vec![WidthBound { kind, value: 0.0 }])
        },
        // Units are ASCII case-insensitive, and zero is zero in any unit.
        MediaRuleValue::Length(length)
          if length.value == 0.0 || length.unit.eq_ignore_ascii_case("px") =>
        {
          Some(vec![WidthBound {
            kind,
            value: length.value,
          }])
        },
        // A `Fraction` value (e.g. `aspect-ratio`) is not a length; neither is
        // a string, a non-zero unitless number or a length in another unit.
        MediaRuleValue::Number(_)
        | MediaRuleValue::Length(_)
        | MediaRuleValue::String(_)
        | MediaRuleValue::Fraction(_) => None,
      }
    },
    MediaQueryRule::And(and_rules) => {
      let mut bounds = Vec::new();

      for rule in &and_rules.rules {
        bounds.extend(collect_width_bounds(rule)?);
      }

      Some(bounds)
    },
    // A negated media type (e.g. `not screen`) inverts the whole expression.
    MediaQueryRule::MediaKeyword(keyword) => (!keyword.not).then(Vec::new),
    MediaQueryRule::WordRule(_) | MediaQueryRule::Not(_) | MediaQueryRule::Or(_) => None,
  }
}

/// The single width bound a media query sorts by, or `None` if it has none.
///
/// The prelude is read as `MediaQuery.parser.parseToEnd` reads it upstream:
/// parsed, then normalized by the `MediaQuery` constructor, which merges the
/// bounds of one axis (`(min-width: 500px) and (min-width: 900px)` is the
/// 900px bound) and resolves a negated bound (`(min-width: 500px) and (not
/// (max-width: 1000px))` is `(min-width: 1000.01px)`). The reference sorts
/// those merged forms, and so does this.
fn media_query_width_sort_key(prelude: &str) -> Option<WidthBound> {
  let parsed = MediaQuery::parser().parse_to_end(prelude).ok()?;
  let bounds = collect_width_bounds(&parsed.queries)?;

  // Zero bounds means no width condition; more than one (a range, or a
  // redundant pair like `(min-width: 500px) and (min-width: 900px)`) has no
  // single value to sort by.
  match bounds.as_slice() {
    [bound] => Some(*bound),
    _ => None,
  }
}

/// The leading at-rule preludes of a rule, outermost first, e.g.
/// `@supports (x){@media (y){.a{…}}}` -> `["@supports (x)", "@media (y)"]`.
pub fn at_rule_preludes(rule: &str) -> Vec<String> {
  let mut preludes = Vec::new();
  let mut index = 0;

  while rule[index..].starts_with('@') {
    let Some(brace) = rule[index..].find('{') else {
      break;
    };

    preludes.push(rule[index..index + brace].trim_end().to_string());
    index += brace + 1;
  }

  preludes
}

/// The sort key for a rule's at-rule chain, or `None` if it has no single
/// `@media` to sort by. Media queries nested in other at-rules still sort, but
/// only against rules sharing the same surrounding conditions.
pub fn width_sort_key_for_chain(preludes: &[String]) -> Option<WidthSortKey> {
  let media_indexes: Vec<usize> = preludes
    .iter()
    .enumerate()
    .filter(|(_, prelude)| prelude.starts_with("@media "))
    .map(|(index, _)| index)
    .collect();

  // Zero means nothing to sort by; more than one means nested media queries
  // whose combined bound isn't a single value.
  let [media_index] = media_indexes.as_slice() else {
    return None;
  };

  let bound = media_query_width_sort_key(&preludes[*media_index])?;

  // Blank out the media query itself so the context captures the surrounding
  // at-rules and the query's depth among them, but not its breakpoint.
  let context = preludes
    .iter()
    .enumerate()
    .map(|(index, prelude)| {
      if index == *media_index {
        "@media"
      } else {
        prelude.as_str()
      }
    })
    .collect::<Vec<&str>>()
    .join("{");

  Some(WidthSortKey { context, bound })
}

/// The property name a rule declares, e.g. `.a{width:500px}` -> `width`.
/// Breakpoints are ordered within a property, so the property has to be
/// compared without its value -- two breakpoints for the same property differ
/// in value by definition, and comparing that first would preempt the order.
pub fn declared_property_name(rule: &str) -> &str {
  let declaration = match rule.rfind('{') {
    Some(brace) => &rule[brace + 1..],
    None => rule,
  };

  match declaration.find(':') {
    Some(colon) => &declaration[..colon],
    None => declaration,
  }
}

/// A total ordering over width sort keys, so the sort stays consistent no
/// matter which pairs get compared: rules with a bound first, then grouped by
/// at-rule context, then `min-width` ascending ahead of `max-width` descending
/// -- the conventional mobile-first order. The context compares as
/// `localeCompare` compares it (`collation`).
pub fn compare_width_sort_keys(a: Option<&WidthSortKey>, b: Option<&WidthSortKey>) -> Ordering {
  let (a, b) = match (a, b) {
    (None, None) => return Ordering::Equal,
    (None, Some(_)) => return Ordering::Greater,
    (Some(_), None) => return Ordering::Less,
    (Some(a), Some(b)) => (a, b),
  };

  if a.context != b.context {
    return crate::collation::compare(&a.context, &b.context);
  }

  if a.bound.kind != b.bound.kind {
    return match a.bound.kind {
      WidthBoundKind::Min => Ordering::Less,
      WidthBoundKind::Max => Ordering::Greater,
    };
  }

  // `a.bound.value - b.bound.value`: a subtraction, which reads `-0` and `0`
  // as a tie (`total_cmp` would not), and the tie then goes to the rule text.
  let ordering = a
    .bound
    .value
    .partial_cmp(&b.bound.value)
    .unwrap_or(Ordering::Equal);

  match a.bound.kind {
    WidthBoundKind::Min => ordering,
    WidthBoundKind::Max => ordering.reverse(),
  }
}

#[cfg(test)]
#[path = "tests/width_sort_key_test.rs"]
mod width_sort_key_test;
