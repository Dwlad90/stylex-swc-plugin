use crate::utils::pseudo::{is_pseudo_class, is_pseudo_element, is_pseudo_selector};

#[test]
fn is_pseudo_element_double_colon_prefix_qualifies() {
  assert!(is_pseudo_element("::before"));
  assert!(is_pseudo_element("::after"));
  assert!(is_pseudo_element("::thumb"));
}

#[test]
fn is_pseudo_element_single_colon_prefix_is_a_pseudo_class() {
  assert!(!is_pseudo_element(":hover"));
  assert!(is_pseudo_class(":hover"));
  assert!(!is_pseudo_element(":nth-child(2)"));
  assert!(is_pseudo_class(":nth-child(2)"));
}

#[test]
fn is_pseudo_element_legacy_single_colon_elements_read_as_pseudo_classes() {
  assert!(!is_pseudo_element(":before"));
  assert!(is_pseudo_class(":before"));
  assert!(!is_pseudo_element(":first-letter"));
  assert!(is_pseudo_class(":first-letter"));
}

#[test]
fn is_pseudo_element_rejects_segments_without_a_colon_prefix() {
  assert!(!is_pseudo_element(""));
  assert!(!is_pseudo_element(":"));
  assert!(!is_pseudo_element("[data-active]"));
  assert!(!is_pseudo_element("@media (min-width: 600px)"));
  assert!(!is_pseudo_element("--color"));
  assert!(!is_pseudo_element("color"));
}

#[test]
fn is_pseudo_element_requires_the_prefix_to_lead_the_segment() {
  assert!(!is_pseudo_element(":hover::before"));
  assert!(!is_pseudo_element(" ::before"));
}

#[test]
fn is_pseudo_selector_accepts_both_pseudo_kinds() {
  assert!(is_pseudo_selector(":hover"));
  assert!(is_pseudo_selector("::before"));
}

#[test]
fn is_pseudo_selector_accepts_a_bare_colon() {
  assert!(is_pseudo_selector(":"));
  assert!(is_pseudo_class(":"));
  assert!(!is_pseudo_element(":"));
}

#[test]
fn is_pseudo_selector_rejects_non_pseudo_keys() {
  assert!(!is_pseudo_selector(""));
  assert!(!is_pseudo_selector("color"));
  assert!(!is_pseudo_selector("[data-active]"));
  assert!(!is_pseudo_selector("@media (min-width: 600px)"));
  assert!(!is_pseudo_selector("var(--color)"));
  assert!(!is_pseudo_selector("default"));
}

#[test]
fn is_pseudo_class_accepts_one_colon_and_rejects_two() {
  assert!(is_pseudo_class(":hover"));
  assert!(is_pseudo_class(":nth-child(2)"));
  assert!(!is_pseudo_class("::before"));
  assert!(!is_pseudo_class("::thumb"));
}

#[test]
fn is_pseudo_class_rejects_keys_without_a_colon_prefix() {
  assert!(!is_pseudo_class(""));
  assert!(!is_pseudo_class("color"));
  assert!(!is_pseudo_class("[data-active]"));
  assert!(!is_pseudo_class("@media (min-width: 600px)"));
}
