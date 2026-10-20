//! The string operations, against what Node answers for the same call.

use super::{js_replace_all, js_replace_first, js_substitution, split_at_utf16_lengths};

#[test]
fn an_ascii_text_splits_at_its_byte_lengths() {
  assert_eq!(
    split_at_utf16_lengths("x1.x1{color:red}", &[2, 14]),
    Some(vec!["x1", ".x1{color:red}"])
  );
}

#[test]
fn an_empty_piece_is_kept_where_it_falls() {
  assert_eq!(
    split_at_utf16_lengths("ab", &[0, 2, 0]),
    Some(vec!["", "ab", ""])
  );
  assert_eq!(
    split_at_utf16_lengths("é", &[0, 1, 0]),
    Some(vec!["", "é", ""])
  );
  assert_eq!(split_at_utf16_lengths("", &[]), Some(vec![]));
  assert_eq!(split_at_utf16_lengths("", &[0, 0]), Some(vec!["", ""]));
}

/// `'é'.length` is 1 and `'😀'.length` is 2, where the bytes are 2 and 4.
#[test]
fn a_text_beyond_ascii_splits_at_its_utf16_lengths() {
  assert_eq!(
    split_at_utf16_lengths("é😀a😀", &[1, 2, 3]),
    Some(vec!["é", "😀", "a😀"])
  );
}

#[test]
fn lengths_that_do_not_cover_the_text_are_refused() {
  // Short of the end, in both paths.
  assert_eq!(split_at_utf16_lengths("abc", &[1, 1]), None);
  assert_eq!(split_at_utf16_lengths("éée", &[1, 1]), None);
  // Past the end, in both paths.
  assert_eq!(split_at_utf16_lengths("abc", &[2, 2]), None);
  assert_eq!(split_at_utf16_lengths("éé", &[1, 2]), None);
}

/// A cut inside a surrogate pair names half a character, which a `str`
/// cannot hold.
#[test]
fn a_cut_inside_a_surrogate_pair_is_refused() {
  assert_eq!(split_at_utf16_lengths("😀", &[1, 1]), None);
  assert_eq!(split_at_utf16_lengths("a😀", &[2, 1]), None);
}

/// Many pieces, each a few characters long, as a large build packs them.
#[test]
fn lengths_past_the_end_of_the_text_are_refused_without_overflow() {
  assert_eq!(split_at_utf16_lengths("ab", &[1, u32::MAX]), None);
  assert_eq!(split_at_utf16_lengths("é", &[u32::MAX, u32::MAX]), None);
}

#[test]
fn a_large_text_splits_into_every_piece() {
  let pieces: Vec<String> = (0..50_000).map(|index| format!("x{index}é")).collect();
  let text = pieces.concat();
  let lengths: Vec<u32> = pieces
    .iter()
    .map(|piece| piece.encode_utf16().count() as u32)
    .collect();

  let Some(split) = split_at_utf16_lengths(&text, &lengths) else {
    panic!("the lengths cover the text");
  };
  assert_eq!(split, pieces);
}

// ==================== replace and replaceAll ====================

#[test]
fn a_string_replacement_reads_the_dollar_patterns_of_get_substitution() {
  assert_eq!(js_substitution("[$&]", "m", "b", "a"), "[m]");
  assert_eq!(
    js_substitution("$`|$'", "m", "before", "after"),
    "before|after"
  );
  assert_eq!(js_substitution("$$", "m", "", ""), "$");
  assert_eq!(js_substitution("$1 $<n> $x $", "m", "", ""), "$1 $<n> $x $");
  assert_eq!(js_substitution("plain", "m", "", ""), "plain");
}

#[test]
fn replace_first_replaces_one_occurrence_or_none() {
  assert_eq!(js_replace_first("a-b-a", "a", "x"), "x-b-a");
  assert_eq!(js_replace_first("a-b-a", "z", "x"), "a-b-a");
  // The reference's `result.replace(match[0], replacement.toString())` passes a
  // string, so a `$&` in a constant's value doubles the match (0.19.1).
  assert_eq!(
    js_replace_first("var(--a)", "var(--a)", "$&$&"),
    "var(--a)var(--a)"
  );
}

#[test]
fn replace_all_replaces_every_occurrence_each_with_its_own_surroundings() {
  assert_eq!(js_replace_all("a-b-a", "a", "x"), "x-b-x");
  assert_eq!(js_replace_all("aaa", "a", "$`"), "aaa");
  assert_eq!(js_replace_all("aaa", "a", "$'"), "aaa");
  assert_eq!(
    js_replace_all("1 var(--a) 2 var(--a)", "var(--a)", "[$&]"),
    "1 [var(--a)] 2 [var(--a)]"
  );
  assert!(matches!(
    js_replace_all("a-b", "z", "x"),
    std::borrow::Cow::Borrowed("a-b")
  ));
}

/// Answers measured in Node 24. JavaScript also cuts a character outside the
/// Basic Multilingual Plane in two (`"😀"` gives `"-\uD83D-\uDE00-"`), which a
/// `str` cannot hold, so such a character is passed over whole.
#[test]
fn replace_all_with_an_empty_pattern_writes_the_replacement_between_characters() {
  assert_eq!(js_replace_all("ab", "", "-"), "-a-b-");
  assert_eq!(js_replace_all("", "", "-"), "-");
  assert_eq!(
    js_replace_all("ab", "", "[$`|$&|$']"),
    "[||ab]a[a||b]b[ab||]"
  );
  assert_eq!(js_replace_all("é😀", "", "-"), "-é-😀-");
  assert_eq!(js_replace_first("ab", "", "-"), "-ab");
}

#[test]
fn replace_all_handles_a_large_text_with_many_occurrences() {
  let text = "var(--a) ".repeat(100_000);
  let replaced = js_replace_all(&text, "var(--a)", "1px");
  assert_eq!(replaced, "1px ".repeat(100_000));

  let letters = "x".repeat(10_000);
  assert_eq!(js_replace_all(&letters, "", "-").len(), 20_001);
}
