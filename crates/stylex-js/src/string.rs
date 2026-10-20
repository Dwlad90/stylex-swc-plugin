//! JavaScript string operations over Rust strings.
//!
//! JavaScript counts a string in UTF-16 code units, and Rust in UTF-8 bytes.
//! These functions take the counts and positions that JavaScript gives and
//! answer what JavaScript answers.

/// `text` cut into consecutive pieces of the given lengths, each counted in
/// UTF-16 code units as JavaScript's `length` counts it.
///
/// `None` when the lengths do not add up to the whole text, or when a cut
/// falls between the two halves of a character outside the Basic
/// Multilingual Plane, which a `str` cannot hold apart.
pub fn split_at_utf16_lengths<'t>(text: &'t str, lengths: &[u32]) -> Option<Vec<&'t str>> {
  let mut pieces = Vec::with_capacity(lengths.len());

  if text.is_ascii() {
    // One byte is one unit.
    let mut start = 0;
    for &length in lengths {
      let end = start + length as usize;
      pieces.push(text.get(start..end)?);
      start = end;
    }
    return (start == text.len()).then_some(pieces);
  }

  let mut characters = text.char_indices();
  let mut start = 0;
  for &length in lengths {
    let mut units = 0;
    let mut end = start;
    while units < length as usize {
      let (index, character) = characters.next()?;
      units += character.len_utf16();
      end = index + character.len_utf8();
    }
    if units != length as usize {
      return None;
    }
    pieces.push(&text[start..end]);
    start = end;
  }

  characters.next().is_none().then_some(pieces)
}

#[cfg(test)]
#[path = "tests/string_tests.rs"]
mod tests;
