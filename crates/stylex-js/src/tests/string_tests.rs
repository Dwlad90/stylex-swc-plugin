//! `split_at_utf16_lengths`, with the lengths JavaScript's `length` gives.

use super::split_at_utf16_lengths;

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
