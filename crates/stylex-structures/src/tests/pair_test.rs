//! Tests for the lightweight key/value `Pair` helper.

use crate::pair::Pair;

/// `Pair::new` should accept both borrowed and owned string inputs.
#[test]
fn pair_new_converts_into_owned_strings() {
  let pair = Pair::new("left", String::from("right"));
  assert_eq!(pair.key, "left");
  assert_eq!(pair.value, "right");
}
