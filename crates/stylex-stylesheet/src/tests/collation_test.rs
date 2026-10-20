//! The default collation, pinned against what `localeCompare` answers in Node
//! (V8, ICU 78) for the strings the sheet compares. Every expectation here was
//! read off `"a".localeCompare("b")` itself.

use std::cmp::Ordering;

use crate::collation::{
  CLDR_DATA_VERSION, ICU_DATA_VERSION, PRIMARY_ORDER, PRIMARY_RANK, UNICODE_DATA_VERSION,
  build_primary_rank, compare, compare_by_collator,
};

/// The data version the addon compares with Node's follows the data crate.
/// When this fails after an update of `icu_collator_data`, read the ICU and
/// Unicode versions in that crate's README, and set both constants.
#[test]
fn the_data_version_follows_the_data_crate() {
  let lock_path = concat!(env!("CARGO_MANIFEST_DIR"), "/../../Cargo.lock");
  let lock = match std::fs::read_to_string(lock_path) {
    Ok(lock) => lock,
    Err(error) => panic!("cannot read {lock_path}: {error}"),
  };
  let data_version = lock
    .split("[[package]]")
    .find(|package| package.contains("name = \"icu_collator_data\""))
    .and_then(|package| {
      package
        .lines()
        .find_map(|line| line.strip_prefix("version = "))
    });

  assert_eq!(
    data_version,
    Some("\"2.3.0\""),
    "ICU {ICU_DATA_VERSION}, Unicode {UNICODE_DATA_VERSION} and CLDR {CLDR_DATA_VERSION} are the \
     data of version 2.3.0"
  );
}

/// The rank table names every printable byte once, one-based, a letter's two
/// cases sharing a rank, and nothing else.
#[test]
fn the_rank_table_is_the_order_inverted() {
  let built = build_primary_rank();
  assert_eq!(built, PRIMARY_RANK);

  for (index, &byte) in PRIMARY_ORDER.iter().enumerate() {
    let Ok(rank) = u8::try_from(index + 1) else {
      panic!("the order is shorter than 256 bytes");
    };
    assert_eq!(built[usize::from(byte)], rank, "{:?}", byte as char);

    if byte.is_ascii_lowercase() {
      assert_eq!(built[usize::from(byte.to_ascii_uppercase())], rank);
    }
  }

  let ranked = built.iter().filter(|&&rank| rank != 0).count();
  assert_eq!(ranked, 0x7f - 0x20, "every printable byte and nothing else");
  assert_eq!(built[0], 0);
  assert_eq!(built[0x7f], 0);
}

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
  // `_` sorts before `-`, so `_x` sorts before `-webkit-x`.
  assert_eq!(compare("_x", "-webkit-x"), Ordering::Less);
  assert_eq!(compare("width", "Width"), Ordering::Less);
}

#[test]
fn equal_strings_are_equal() {
  assert_eq!(compare("", ""), Ordering::Equal);
  assert_eq!(compare(".a{color:red}", ".a{color:red}"), Ordering::Equal);
}

/// Every expectation here was read off `localeCompare` in Node (V8, ICU 78).
#[test]
fn a_character_outside_printable_ascii_weighs_where_root_collation_weighs_it() {
  // An accented letter sits beside its base letter, after the digits.
  assert_eq!(compare("e", "é"), Ordering::Less);
  assert_eq!(compare("é", "f"), Ordering::Less);
  assert_eq!(compare("E", "é"), Ordering::Less);
  assert_eq!(compare("9", "é"), Ordering::Less);
  // The middle dot is punctuation: after the full stop, before the apostrophe,
  // which is what orders `content:'·'` before `content:''`.
  assert_eq!(compare(".", "\u{b7}"), Ordering::Less);
  assert_eq!(compare("\u{b7}", "'"), Ordering::Less);
  assert_eq!(
    compare("{content:'\u{b7}'}", "{content:''}"),
    Ordering::Less
  );
  // A dash is a dash, an ellipsis sits with the full stop, a euro sign with
  // the currency symbols, a CJK ideograph after every letter.
  assert_eq!(compare("-", "\u{2013}"), Ordering::Less);
  assert_eq!(compare("\u{2013}", ","), Ordering::Less);
  assert_eq!(compare(".", "\u{2026}"), Ordering::Less);
  assert_eq!(compare("$", "\u{20ac}"), Ordering::Less);
  assert_eq!(compare("\u{20ac}", "0"), Ordering::Less);
  assert_eq!(compare("Z", "\u{4e2d}"), Ordering::Less);
  // A control character has no weight of its own: it goes to the collator
  // with everything outside the printable range.
  assert_eq!(compare("a\u{2}", "a"), Ordering::Equal);
}

/// The table and the collator are one order over every printable pair, which
/// is what keeps the comparator total across the boundary between them.
#[test]
fn the_table_and_root_collation_agree_on_every_printable_pair() {
  let printable: Vec<String> = (0x20u8..=0x7e)
    .map(|byte| (byte as char).to_string())
    .collect();

  for a in &printable {
    for b in &printable {
      assert_eq!(
        compare(a, b),
        compare_by_collator(a, b),
        "{a:?} against {b:?}"
      );

      for c in ["a", "Z", "0", "-"] {
        let x = format!("{a}{c}");
        let y = format!("{c}{b}");

        assert_eq!(
          compare(&x, &y),
          compare_by_collator(&x, &y),
          "{x:?} against {y:?}"
        );
      }
    }
  }
}
