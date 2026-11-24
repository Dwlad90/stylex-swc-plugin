//! Tests for the lightweight key/value `Pair` helper.

use std::borrow::Cow;

use crate::pair::{Pair, PairCow};

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
  let borrowed = PairCow::borrowed(&pair);

  assert_eq!(borrowed, pair);
  assert!(matches!(borrowed.key, Cow::Borrowed("display")));
  assert!(matches!(borrowed.value, Cow::Borrowed("block")));

  let owned = borrowed.into_owned();
  assert_eq!(owned, pair);
}

/// The CSS text a pair writes into a buffer.
fn css_text(pair: &Pair) -> String {
  let mut out = String::new();
  pair.push_css_text(&mut out);
  out
}

/// A pair writes `key:value;` and keeps each half as it is. A blank key stays,
/// as the reference keeps it.
#[test]
fn pair_writes_each_half_as_it_is() {
  assert_eq!(css_text(&Pair::new("top", "10px")), "top:10px;");
  // `0` spells a value even though JS calls it falsy.
  assert_eq!(css_text(&Pair::new("z-index", "0")), "z-index:0;");
  // Empty quotes are CSS text, which is what a blank `content` becomes.
  assert_eq!(css_text(&Pair::new("content", "\"\"")), "content:\"\";");
  assert_eq!(css_text(&Pair::new("", "red")), ":red;");
  assert_eq!(css_text(&Pair::new(" ", "red")), " :red;");
}

/// Each pair is appended after the text the buffer already holds.
#[test]
fn pairs_append_to_the_buffer() {
  let mut out = String::from("from{");
  Pair::new("color", "red").push_css_text(&mut out);
  Pair::new("opacity", "0").push_css_text(&mut out);

  assert_eq!(out, "from{color:red;opacity:0;");
}

/// A borrowed pair writes the text an owned pair writes, and it does so without
/// owning either half first, which is the only reason this kind exists. The
/// text and the borrow are asserted together, because a pair that copied its
/// halves would still give the right text.
#[test]
fn pair_cow_writes_without_owning_its_halves() {
  let pair = Pair::new("top", "10px");
  let borrowed = PairCow::borrowed(&pair);

  assert_eq!(borrowed.key.as_ptr(), pair.key.as_ptr());
  assert_eq!(borrowed.value.as_ptr(), pair.value.as_ptr());

  let mut out = String::new();
  borrowed.push_css_text(&mut out);
  assert_eq!(out, css_text(&pair));
}

/// A pair that owns its halves writes the same text. Nothing builds one today,
/// but the kind admits it, and the text must not depend on where the halves are
/// kept.
#[test]
fn an_owning_pair_cow_writes_what_a_borrowing_one_writes() {
  let owning = PairCow {
    key: Cow::Owned("top".to_string()),
    value: Cow::Owned("10px".to_string()),
  };

  let mut out = String::new();
  owning.push_css_text(&mut out);
  assert_eq!(out, "top:10px;");
}
