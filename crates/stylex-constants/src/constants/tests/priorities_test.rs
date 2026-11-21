//! Tests for the priority helpers that the compile step and the stylesheet
//! step share.

use crate::constants::priorities::{at_rule_priority, compound_pseudo_priority};

/// A key that starts with an at-rule of the table gets its priority, by
/// prefix. Any other key gets none.
#[test]
fn at_rule_priority_reads_the_table_by_prefix() {
  assert_eq!(at_rule_priority("@supports (display: grid)"), Some(30.0));
  assert_eq!(at_rule_priority("@media (min-width: 10rem)"), Some(200.0));
  assert_eq!(at_rule_priority("@container (width > 10rem)"), Some(300.0));
  assert_eq!(at_rule_priority("@starting-style"), None);
  assert_eq!(at_rule_priority("color"), None);
}

/// A chain is the sum of its parts: a pseudo-element is 5000, a known
/// pseudo-class has its own priority and an unknown one is 40.
#[test]
fn compound_pseudo_priority_sums_the_parts() {
  assert_eq!(
    compound_pseudo_priority(&[":hover", ":active"]),
    Some(300.0)
  );
  assert_eq!(
    compound_pseudo_priority(&[":hover", "::after"]),
    Some(5130.0)
  );
  assert_eq!(compound_pseudo_priority(&[":x", ":hover"]), Some(170.0));
}

/// One part or none is not a chain, and a chain with a functional
/// pseudo-class is not priced.
#[test]
fn compound_pseudo_priority_refuses_a_part_alone_and_a_functional_part() {
  assert_eq!(compound_pseudo_priority(&[]), None);
  assert_eq!(compound_pseudo_priority(&[":hover"]), None);
  assert_eq!(compound_pseudo_priority(&[":hover", ":not(.x)"]), None);
}
