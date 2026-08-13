use crate::utils::pseudo::is_pseudo_element;

#[test]
fn double_colon_prefix_is_a_pseudo_element() {
  assert!(is_pseudo_element("::before"));
  assert!(is_pseudo_element("::after"));
  assert!(is_pseudo_element("::thumb"));
}

#[test]
fn single_colon_prefix_is_a_pseudo_class() {
  assert!(!is_pseudo_element(":hover"));
  assert!(!is_pseudo_element(":nth-child(2)"));
}

#[test]
fn legacy_single_colon_elements_read_as_pseudo_classes() {
  assert!(!is_pseudo_element(":before"));
  assert!(!is_pseudo_element(":first-letter"));
}

#[test]
fn segments_without_a_colon_prefix_are_not_pseudo_elements() {
  assert!(!is_pseudo_element(""));
  assert!(!is_pseudo_element(":"));
  assert!(!is_pseudo_element("[data-active]"));
  assert!(!is_pseudo_element("@media (min-width: 600px)"));
  assert!(!is_pseudo_element("--color"));
  assert!(!is_pseudo_element("color"));
}

#[test]
fn the_prefix_must_lead_the_segment() {
  assert!(!is_pseudo_element(":hover::before"));
  assert!(!is_pseudo_element(" ::before"));
}
