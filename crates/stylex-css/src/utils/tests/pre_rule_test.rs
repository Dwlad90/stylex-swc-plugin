use crate::utils::pre_rule::{sort_at_rules, sort_pseudos};

// ── sort_pseudos ─────────────────────────────────────────────────────

#[test]
fn sort_pseudos_empty() {
  let result = sort_pseudos(&[]);
  assert!(result.is_empty());
}

#[test]
fn sort_pseudos_single_element() {
  let result = sort_pseudos(&[":hover".into()]);
  assert_eq!(result, vec![":hover"]);
}

#[test]
fn sort_pseudos_single_pseudo_element() {
  let result = sort_pseudos(&["::before".into()]);
  assert_eq!(result, vec!["::before"]);
}

#[test]
fn sort_pseudos_two_pseudo_classes_sorted() {
  let result = sort_pseudos(&[":hover".into(), ":active".into()]);
  // Two pseudo-classes should be grouped and sorted
  assert_eq!(result, vec![":active", ":hover"]);
}

#[test]
fn sort_pseudos_pseudo_element_starts_new_group() {
  let result = sort_pseudos(&["::before".into(), ":hover".into()]);
  // ::before starts a group, :hover starts another (since last group
  // has len==1 and starts with ::)
  assert_eq!(result, vec!["::before", ":hover"]);
}

#[test]
fn sort_pseudos_pseudo_element_followed_by_two_classes() {
  let result = sort_pseudos(&[
    "::after".into(),
    ":hover".into(),
    ":focus".into(),
  ]);
  // ::after → group1, :hover starts group2, :focus joins group2
  // group2 gets sorted
  assert_eq!(result, vec!["::after", ":focus", ":hover"]);
}

#[test]
fn sort_pseudos_multiple_pseudo_elements() {
  let result = sort_pseudos(&[
    "::before".into(),
    "::after".into(),
  ]);
  // Each :: starts its own group (len 1 each)
  assert_eq!(result, vec!["::before", "::after"]);
}

#[test]
fn sort_pseudos_classes_then_element() {
  let result = sort_pseudos(&[
    ":hover".into(),
    ":focus".into(),
    "::before".into(),
  ]);
  // :hover → group1, :focus joins group1, ::before starts group2
  // group1 sorted, group2 is single
  assert_eq!(result, vec![":focus", ":hover", "::before"]);
}

#[test]
fn sort_pseudos_interleaved() {
  let result = sort_pseudos(&[
    ":hover".into(),
    "::before".into(),
    ":focus".into(),
    ":active".into(),
    "::after".into(),
  ]);
  // :hover → group1 (single), ::before → group2 (single),
  // :focus → group3, :active joins group3 → sorted,
  // ::after → group4
  assert_eq!(
    result,
    vec![":hover", "::before", ":active", ":focus", "::after"]
  );
}

// ── sort_at_rules ────────────────────────────────────────────────────

#[test]
fn sort_at_rules_empty() {
  let result = sort_at_rules(&[]);
  assert!(result.is_empty());
}

#[test]
fn sort_at_rules_single() {
  let result = sort_at_rules(&["@media (max-width: 600px)".into()]);
  assert_eq!(result, vec!["@media (max-width: 600px)"]);
}

#[test]
fn sort_at_rules_default_first() {
  let result = sort_at_rules(&[
    "@media (max-width: 600px)".into(),
    "default".into(),
    "@supports (display: grid)".into(),
  ]);
  assert_eq!(result[0], "default");
}

#[test]
fn sort_at_rules_alphabetical_without_default() {
  let result = sort_at_rules(&[
    "@supports (display: grid)".into(),
    "@media (max-width: 600px)".into(),
  ]);
  assert_eq!(result[0], "@media (max-width: 600px)");
  assert_eq!(result[1], "@supports (display: grid)");
}

#[test]
fn sort_at_rules_already_sorted() {
  let result = sort_at_rules(&[
    "default".into(),
    "@media (max-width: 600px)".into(),
    "@supports (display: grid)".into(),
  ]);
  assert_eq!(result[0], "default");
  assert_eq!(result[1], "@media (max-width: 600px)");
  assert_eq!(result[2], "@supports (display: grid)");
}

#[test]
fn sort_at_rules_multiple_media() {
  let result = sort_at_rules(&[
    "@media (min-width: 800px)".into(),
    "@media (max-width: 600px)".into(),
  ]);
  assert_eq!(result[0], "@media (max-width: 600px)");
  assert_eq!(result[1], "@media (min-width: 800px)");
}
