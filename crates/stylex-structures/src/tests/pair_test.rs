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

/// A pair only becomes a declaration when both halves spell something. `top:`
/// is not valid CSS, so a blank value yields no declaration at all rather than a
/// declaration with nothing in it.
#[test]
fn pair_declares_only_when_both_halves_spell_something() {
  assert_eq!(
    Pair::new("top", "10px").as_declaration(),
    Some("top:10px;".to_string())
  );
  // `0` spells a value even though JS calls it falsy.
  assert_eq!(
    Pair::new("z-index", "0").as_declaration(),
    Some("z-index:0;".to_string())
  );
  // Empty quotes are CSS text, which is what a blank `content` becomes.
  assert_eq!(
    Pair::new("content", "\"\"").as_declaration(),
    Some("content:\"\";".to_string())
  );

  assert_eq!(Pair::new("top", "").as_declaration(), None);
  assert_eq!(Pair::new("top", " ").as_declaration(), None);
  assert_eq!(Pair::new("top", "  \t ").as_declaration(), None);
  assert_eq!(Pair::new("", "10px").as_declaration(), None);
}
