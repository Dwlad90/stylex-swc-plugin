//! The width sort key and its order, over the shapes upstream's
//! `transform-process-test.js` sorts: every case there that turns on a
//! breakpoint is pinned here at the key level, so the sheet cases that follow
//! inherit an order already held.

use std::cmp::Ordering;

use crate::width_sort_key::{
  WidthBound, WidthBoundKind, WidthSortKey, at_rule_preludes, compare_width_sort_keys,
  declared_property_name, width_sort_key_for_chain,
};

fn key_of(rule: &str) -> Option<WidthSortKey> {
  width_sort_key_for_chain(&at_rule_preludes(rule))
}

fn bound_of(prelude: &str) -> Option<WidthBound> {
  key_of(&format!("{prelude}{{.x{{color:red}}}}")).map(|key| key.bound)
}

fn min(value: f64) -> Option<WidthBound> {
  Some(WidthBound {
    kind: WidthBoundKind::Min,
    value,
  })
}

fn max(value: f64) -> Option<WidthBound> {
  Some(WidthBound {
    kind: WidthBoundKind::Max,
    value,
  })
}

/// The order the comparator gives two rules, by their chains.
fn order(a: &str, b: &str) -> Ordering {
  compare_width_sort_keys(key_of(a).as_ref(), key_of(b).as_ref())
}

// ==================== at_rule_preludes ====================

#[test]
fn preludes_are_read_outermost_first_and_trimmed() {
  assert_eq!(
    at_rule_preludes("@supports (x){@media (y){.a{color:red}}}"),
    vec!["@supports (x)", "@media (y)"]
  );
  assert_eq!(
    at_rule_preludes("@media (min-width: 500px) {.a{color:red}}"),
    vec!["@media (min-width: 500px)"]
  );
}

#[test]
fn a_rule_with_no_at_rule_has_no_preludes() {
  assert!(at_rule_preludes(".a{color:red}").is_empty());
  assert!(at_rule_preludes("").is_empty());
}

#[test]
fn a_prelude_with_no_block_ends_the_walk() {
  assert_eq!(
    at_rule_preludes("@media (min-width: 500px)"),
    Vec::<String>::new()
  );
  assert_eq!(
    at_rule_preludes("@supports (x){@media (y)"),
    vec!["@supports (x)"]
  );
}

// ==================== declared_property_name ====================

#[test]
fn the_declared_property_is_read_off_the_innermost_block() {
  assert_eq!(declared_property_name(".a{width:500px}"), "width");
  assert_eq!(
    declared_property_name("@media (min-width: 500px){.a{color:red}}"),
    "color"
  );
  assert_eq!(declared_property_name(":root, .x{--a:1px;--b:2px;}"), "--a");
}

#[test]
fn a_declaration_with_no_colon_is_its_own_name() {
  assert_eq!(declared_property_name(".a{}"), "}");
  assert_eq!(declared_property_name("no-braces"), "no-braces");
}

// ==================== the bound a prelude earns ====================

#[test]
fn a_px_min_or_max_width_is_a_bound() {
  assert_eq!(bound_of("@media (min-width: 500px)"), min(500.0));
  assert_eq!(bound_of("@media (max-width: 1000px)"), max(1000.0));
  assert_eq!(bound_of("@media (min-width: 640.5px)"), min(640.5));
}

/// `sorts min-width with screen and media type`
#[test]
fn a_media_type_beside_the_bound_constrains_no_width() {
  assert_eq!(
    bound_of("@media screen and (min-width: 1280px)"),
    min(1280.0)
  );
  assert_eq!(
    bound_of("@media only screen and (min-width: 768px)"),
    min(768.0)
  );
  assert_eq!(
    bound_of("@media (hover: hover) and (max-width: 500px)"),
    max(500.0)
  );
}

/// `does not misorder negated min-width media queries`
#[test]
fn a_negated_query_has_no_bound() {
  assert_eq!(bound_of("@media (not (min-width: 1000px))"), None);
  assert_eq!(bound_of("@media not screen and (min-width: 1000px)"), None);
}

/// `does not sort a min-width paired with a negated max-width`: the
/// reference's own test, whose two rules happen to land in the same order
/// either way. Measured on the plugin with a third rule between them: the
/// normalized form `(min-width: 1000.01px)` is what sorts, between 700px and
/// 2000px, so that is the bound here too.
#[test]
fn a_bound_paired_with_a_negated_bound_sorts_by_the_normalized_form() {
  assert_eq!(
    bound_of("@media (min-width: 500px) and (not (max-width: 1000px))"),
    min(1000.01)
  );
}

/// `sorts CSS Level 4 range syntax (width >= Xpx) as min-width`
#[test]
fn range_syntax_reads_as_a_bound() {
  assert_eq!(bound_of("@media (width >= 500px)"), min(500.0));
  assert_eq!(bound_of("@media (width <= 1000px)"), max(1000.0));
  assert_eq!(bound_of("@media (500px <= width)"), min(500.0));
}

/// `range queries (both min and max-width) fall through to existing sort`
#[test]
fn a_query_with_both_bounds_has_no_single_bound() {
  assert_eq!(
    bound_of("@media (min-width: 500px) and (max-width: 1000px)"),
    None
  );
  assert_eq!(bound_of("@media (500px <= width <= 1000px)"), None);
}

/// `does not sort a query with two bounds on the same side`: the
/// reference's test reads the two-rule output as unsorted, but its parser
/// normalizes the query to the 900px bound before `collectWidthBounds` sees
/// it, and with a 700px and a 2000px rule beside it the plugin places it
/// between them. The same bound here.
#[test]
fn two_bounds_on_one_side_sort_by_their_merged_bound() {
  assert_eq!(
    bound_of("@media (min-width: 500px) and (min-width: 900px)"),
    min(900.0)
  );
  // Two bounds on different sides stay two, and sort nothing.
  assert_eq!(
    bound_of("@media (min-width: 500px) and (max-width: 900px)"),
    None
  );
}

/// `sorts unitless zero, any-unit zero, and mixed-case px`
#[test]
fn zero_is_a_bound_in_any_unit_or_none_and_px_is_case_insensitive() {
  assert_eq!(bound_of("@media (min-width: 0)"), min(0.0));
  assert_eq!(bound_of("@media (min-width: 0rem)"), min(0.0));
  assert_eq!(bound_of("@media (min-width: 0em)"), min(0.0));
  assert_eq!(bound_of("@media (min-width: 500PX)"), min(500.0));
  assert_eq!(bound_of("@media (MIN-WIDTH: 500px)"), min(500.0));
}

/// `does not sort a width whose value is a ratio`
#[test]
fn a_ratio_is_not_a_width() {
  assert_eq!(bound_of("@media (min-width: 16/9)"), None);
  // A non-width feature carrying a ratio is no bound either, and no refusal.
  assert_eq!(
    bound_of("@media (min-aspect-ratio: 16/9) and (min-width: 500px)"),
    min(500.0)
  );
}

/// `does not sort a unitless non-zero width`
#[test]
fn a_unitless_non_zero_width_is_not_a_bound() {
  assert_eq!(bound_of("@media (min-width: 500)"), None);
}

#[test]
fn a_width_that_is_a_word_or_a_calc_is_not_a_bound() {
  assert_eq!(bound_of("@media (min-width: auto)"), None);
  assert_eq!(bound_of("@media (min-width: calc(600px + 40px))"), None);
}

/// `does not sort rem breakpoints`
#[test]
fn a_width_in_another_unit_is_not_a_bound() {
  assert_eq!(bound_of("@media (min-width: 40rem)"), None);
  assert_eq!(bound_of("@media (max-width: 30em)"), None);
  assert_eq!(bound_of("@media (min-width: 50vw)"), None);
}

#[test]
fn a_query_with_no_width_feature_has_no_bound() {
  assert_eq!(bound_of("@media (hover: hover)"), None);
  assert_eq!(bound_of("@media screen"), None);
  assert_eq!(bound_of("@media (prefers-reduced-motion: reduce)"), None);
  assert_eq!(bound_of("@media (color)"), None);
}

#[test]
fn an_or_query_has_no_bound() {
  assert_eq!(
    bound_of("@media (min-width: 500px), (max-width: 100px)"),
    None
  );
  assert_eq!(
    bound_of("@media (min-width: 500px) or (hover: hover)"),
    None
  );
}

#[test]
fn a_prelude_the_parser_refuses_has_no_bound() {
  // An unbalanced parenthesis is refused before the parse, as upstream's
  // `validateMediaQuery` refuses it; the parser alone would read the bound.
  assert_eq!(bound_of("@media (min-width: 500px"), None);
  assert_eq!(bound_of("@media (min-width: 500px))"), None);
  assert_eq!(bound_of("@media )(min-width: 500px"), None);
  assert_eq!(bound_of("@media garbage((("), None);
  assert_eq!(bound_of("@media"), None);
}

#[test]
fn a_non_media_at_rule_has_no_bound() {
  assert_eq!(bound_of("@supports (display: grid)"), None);
  assert_eq!(bound_of("@container (min-width: 500px)"), None);
  assert_eq!(bound_of("@keyframes x"), None);
}

// ==================== the key and its context ====================

/// `sorts min-width breakpoints nested inside another at-rule`
#[test]
fn a_breakpoint_nested_in_another_at_rule_keeps_its_context() {
  let key = key_of("@supports (display: grid){@media (min-width: 500px){.a{color:red}}}")
    .expect("a nested breakpoint sorts");

  assert_eq!(key.context, "@supports (display: grid){@media");
  assert_eq!(key.bound, min(500.0).expect("a bound"));

  let inner = key_of("@media (min-width: 500px){@supports (display: grid){.a{color:red}}}")
    .expect("a breakpoint around another at-rule sorts");

  assert_eq!(inner.context, "@media{@supports (display: grid)");
}

#[test]
fn two_nested_media_queries_have_no_key() {
  assert_eq!(
    key_of("@media (min-width: 500px){@media (hover: hover){.a{color:red}}}"),
    None
  );
}

#[test]
fn a_rule_with_no_media_prelude_has_no_key() {
  assert_eq!(key_of(".a{color:red}"), None);
  assert_eq!(key_of("@supports (display: grid){.a{color:red}}"), None);
}

// ==================== the order ====================

/// `sorts min-width defineConsts breakpoints in ascending px order`
#[test]
fn min_width_sorts_ascending() {
  assert_eq!(
    order(
      "@media (min-width: 500px){.a{color:red}}",
      "@media (min-width: 900px){.b{color:red}}"
    ),
    Ordering::Less
  );
  assert_eq!(
    order(
      "@media (min-width: 1280px){.a{color:red}}",
      "@media (min-width: 768px){.b{color:red}}"
    ),
    Ordering::Greater
  );
}

/// `sorts max-width defineConsts breakpoints in descending px order`
#[test]
fn max_width_sorts_descending() {
  assert_eq!(
    order(
      "@media (max-width: 1000px){.a{color:red}}",
      "@media (max-width: 500px){.b{color:red}}"
    ),
    Ordering::Less
  );
  assert_eq!(
    order(
      "@media (max-width: 300px){.a{color:red}}",
      "@media (max-width: 900px){.b{color:red}}"
    ),
    Ordering::Greater
  );
}

/// `orders the min-width group before the max-width group`
#[test]
fn every_min_width_sorts_before_every_max_width() {
  assert_eq!(
    order(
      "@media (min-width: 9000px){.a{color:red}}",
      "@media (max-width: 1px){.b{color:red}}"
    ),
    Ordering::Less
  );
}

/// `sort is a total order across px min- and max-width rules`
#[test]
fn a_rule_with_no_key_sorts_after_every_rule_with_one_and_equal_to_its_kind() {
  assert_eq!(
    order(
      "@media (min-width: 500px){.a{color:red}}",
      "@media (min-width: 40rem){.b{color:red}}"
    ),
    Ordering::Less
  );
  assert_eq!(
    order("@media (max-width: 500px){.a{color:red}}", ".b{color:red}"),
    Ordering::Less
  );
  assert_eq!(
    order("@media (hover: hover){.a{color:red}}", ".b{color:red}"),
    Ordering::Equal
  );
  assert_eq!(
    order(
      "@media (min-width: 500px){.a{color:red}}",
      "@media (min-width: 500px){.b{color:blue}}"
    ),
    Ordering::Equal
  );
}

#[test]
fn keys_in_different_contexts_sort_by_context_before_bound() {
  // `@media{...}` against `@supports (x){@media ...}`: the contexts differ, and
  // `@media` sorts before `@supports` whatever the bounds say.
  assert_eq!(
    order(
      "@media (min-width: 9000px){.a{color:red}}",
      "@supports (display: grid){@media (min-width: 1px){.b{color:red}}}"
    ),
    Ordering::Less
  );
}

#[test]
fn the_order_is_antisymmetric_and_transitive_over_the_shapes_above() {
  let rules = [
    "@media (min-width: 500px){.a{color:red}}",
    "@media (min-width: 900px){.b{color:red}}",
    "@media (max-width: 1000px){.c{color:red}}",
    "@media (max-width: 300px){.d{color:red}}",
    "@media (min-width: 40rem){.e{color:red}}",
    "@supports (display: grid){@media (min-width: 1px){.f{color:red}}}",
    ".g{color:red}",
  ];

  for a in rules {
    assert_eq!(order(a, a), Ordering::Equal, "{a} against itself");

    for b in rules {
      assert_eq!(order(a, b), order(b, a).reverse(), "{a} against {b}");

      for c in rules {
        if order(a, b) != Ordering::Greater && order(b, c) != Ordering::Greater {
          assert_ne!(order(a, c), Ordering::Greater, "{a} <= {b} <= {c}");
        }
      }
    }
  }
}
