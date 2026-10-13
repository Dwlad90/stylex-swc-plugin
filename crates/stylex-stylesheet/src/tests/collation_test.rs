//! The default collation, pinned against what `localeCompare` answers in Node
//! (V8, ICU 78) for the strings the sheet compares. Every expectation here was
//! read off `"a".localeCompare("b")` itself.

use std::cmp::Ordering;

use crate::collation::compare;

#[test]
fn letters_sort_by_base_letter_before_case() {
  assert_eq!(compare("a", "b"), Ordering::Less);
  assert_eq!(compare("a", "B"), Ordering::Less);
  assert_eq!(compare("A", "b"), Ordering::Less);
  assert_eq!(compare("B", "a"), Ordering::Greater);
}

#[test]
fn lowercase_sorts_before_uppercase_only_as_a_tie_break() {
  assert_eq!(compare("a", "A"), Ordering::Less);
  assert_eq!(compare("ab", "aB"), Ordering::Less);
  assert_eq!(compare("aB", "Ab"), Ordering::Less);
  assert_eq!(compare("Ab", "AB"), Ordering::Less);
  // The tie-break reads the first case difference, after the whole string
  // tied at the primary level: `A` before `ab` because `a` is a prefix of
  // `ab` there.
  assert_eq!(compare("A", "ab"), Ordering::Less);
  assert_eq!(compare("ab", "A"), Ordering::Greater);
}

#[test]
fn punctuation_sorts_before_digits_before_letters() {
  assert_eq!(compare("-", "0"), Ordering::Less);
  assert_eq!(compare("0", "a"), Ordering::Less);
  assert_eq!(compare("_", "-"), Ordering::Less);
  assert_eq!(compare(" ", "_"), Ordering::Less);
  assert_eq!(compare(":", "."), Ordering::Less);
  assert_eq!(compare("(", ")"), Ordering::Less);
  assert_eq!(compare("@", "*"), Ordering::Less);
  assert_eq!(compare("$", "0"), Ordering::Less);
}

#[test]
fn a_prefix_sorts_before_the_string_it_opens() {
  assert_eq!(compare("x1", "x10"), Ordering::Less);
  assert_eq!(compare("x10", "x2"), Ordering::Less);
  assert_eq!(compare("", "a"), Ordering::Less);
  assert_eq!(compare("a", ""), Ordering::Greater);
}

#[test]
fn the_sheets_own_texts_order_as_node_orders_them() {
  // Declarations, by property then value.
  assert_eq!(compare("color:blue", "color:red"), Ordering::Less);
  assert_eq!(compare("color:red", "color:Red"), Ordering::Less);
  // Preludes: `1024` before `640` by the digits, as `localeCompare` reads them
  // (no numeric collation).
  assert_eq!(
    compare("@media (min-width: 1024px)", "@media (min-width: 640px)"),
    Ordering::Less
  );
  assert_eq!(compare("@media", "@supports (x)"), Ordering::Less);
  // Class names.
  assert_eq!(compare(".x{}", ".X{}"), Ordering::Less);
  assert_eq!(compare("xa1b2c", "xa1b2d"), Ordering::Less);
  // `-webkit-x` sorts before `_x`? No: `_` comes before `-`.
  assert_eq!(compare("_x", "-webkit-x"), Ordering::Less);
  assert_eq!(compare("width", "Width"), Ordering::Less);
}

#[test]
fn equal_strings_are_equal() {
  assert_eq!(compare("", ""), Ordering::Equal);
  assert_eq!(compare(".a{color:red}", ".a{color:red}"), Ordering::Equal);
}

#[test]
fn a_byte_outside_printable_ascii_sorts_above_every_byte_in_the_table() {
  assert_eq!(compare("a", "é"), Ordering::Less);
  assert_eq!(compare("~", "é"), Ordering::Less);
  assert_eq!(compare("é", "ß"), compare("é", "ß").reverse().reverse());
}
