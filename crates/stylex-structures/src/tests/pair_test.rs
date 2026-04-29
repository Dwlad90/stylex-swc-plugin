//! Tests for the lightweight key/value `Pair` helper.

use crate::pair::Pair;

/// `Pair::new` should accept both borrowed and owned string inputs.
#[test]
fn pair_new_converts_into_owned_strings() {
  let pair = Pair::new("left", String::from("right"));
  assert_eq!(pair.key, "left");
  assert_eq!(pair.value, "right");
}

#[test]
fn pair_cow_borrows_and_converts_back_to_owned_pair() {
  let pair = Pair::new("display", "block");
  let borrowed = crate::pair::PairCow::borrowed(&pair);

  assert_eq!(borrowed, pair);
  assert!(matches!(
    borrowed.key,
    std::borrow::Cow::Borrowed("display")
  ));
  assert!(matches!(
    borrowed.value,
    std::borrow::Cow::Borrowed("block")
  ));

  let owned = borrowed.into_owned();
  assert_eq!(owned, pair);
}
