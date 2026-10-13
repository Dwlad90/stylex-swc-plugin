//! `String.prototype.localeCompare` as the reference implementation calls it.
//!
//! `processStylexRules` sorts with `localeCompare` and no locale argument, so
//! the sheet's order is the host's default collation: in Node that is ICU's
//! root collation, which is not byte order. Over printable ASCII it reads two
//! levels. The primary level weighs each character by its base letter, case
//! folded, with the punctuation before the digits and the digits before the
//! letters, and a shorter string before a longer one it is a prefix of. Where
//! two strings tie there, the tertiary level breaks the tie at the first
//! position whose characters differ in case only, lowercase first (`a` before
//! `A`, so `ab` before `aB` before `Ab`).
//!
//! That is the whole of what the sheet asks: class names, at-rule preludes,
//! declarations and rule texts, all printable ASCII. The table below is V8's
//! answer (ICU 78) for every printable ASCII pair, single characters and
//! two-character strings, read off `localeCompare` itself, and the parity
//! harness holds the resulting sheet against the plugin's. A character outside
//! printable ASCII weighs by its code point above every character in the
//! table, which keeps the order total; no such character reaches a sort key.

use std::cmp::Ordering;

/// Printable ASCII in primary weight order, as V8 (ICU 78) collates it with
/// the default locale, letters case folded: the space, then the punctuation
/// and symbols in the collation's order, then the digits, then the letters.
const PRIMARY_ORDER: &[u8] =
  b" _-,;:!?.'\"()[]{}@*/\\&#%`^+<=>|~$0123456789abcdefghijklmnopqrstuvwxyz";

/// The primary weight of one byte: its position in `PRIMARY_ORDER`, case
/// folded; a byte outside the table sorts above every byte in it.
fn primary_weight(byte: u8) -> u32 {
  let folded = byte.to_ascii_lowercase();

  match PRIMARY_ORDER
    .iter()
    .position(|&candidate| candidate == folded)
  {
    Some(position) => position as u32,
    None => PRIMARY_ORDER.len() as u32 + u32::from(byte),
  }
}

/// The order `a.localeCompare(b)` answers in Node for two printable ASCII
/// strings: the primary level over the whole string, then the case tie-break.
pub(crate) fn compare(a: &str, b: &str) -> Ordering {
  let a_bytes = a.as_bytes();
  let b_bytes = b.as_bytes();

  for (left, right) in a_bytes.iter().zip(b_bytes) {
    let primary = primary_weight(*left).cmp(&primary_weight(*right));

    if primary != Ordering::Equal {
      return primary;
    }
  }

  let by_length = a_bytes.len().cmp(&b_bytes.len());

  if by_length != Ordering::Equal {
    return by_length;
  }

  for (left, right) in a_bytes.iter().zip(b_bytes) {
    if left != right {
      // The two differ in case alone here (their primaries tied): lowercase
      // sorts first.
      return if left.is_ascii_lowercase() {
        Ordering::Less
      } else {
        Ordering::Greater
      };
    }
  }

  Ordering::Equal
}
