//! `String.prototype.localeCompare` as the reference implementation calls it.
//!
//! `processStylexRules` sorts with `localeCompare` and no locale argument, so
//! the sheet's order is the host's default collation: in Node that is ICU's
//! root collation, which is not byte order. This module answers it the way
//! `stylex-css` answers the same question for pseudo-class order
//! (`utils/pre_rule.rs`, `pseudo_comparator`): a table for printable ASCII,
//! the path almost every comparison takes, and `icu_collator` at the root
//! locale for anything else, so a `content: '·'` or an `é` in a declaration
//! weighs where ICU weighs it.
//!
//! Over printable ASCII root collation reads two levels. The primary level
//! weighs each character by its base letter, case folded, with the
//! punctuation before the digits and the digits before the letters, and a
//! shorter string before a longer one it is a prefix of. Where two strings tie
//! there, the tertiary level breaks the tie at the first position whose
//! characters differ in case only, lowercase first (`a` before `A`, so `ab`
//! before `aB` before `Ab`). The table below is V8's answer (ICU 78) for every
//! printable ASCII pair, single characters and two-character strings, read
//! off `localeCompare` itself; `collation_test.rs` asserts the table and the
//! collator agree on every printable pair, which is what keeps the comparator
//! one total order across the boundary.
//!
//! The boundary is printable ASCII, not `is_ascii()`: root collation gives a
//! control character no weight, and a table that ranked one would answer the
//! other way, so a control character goes to the collator with everything
//! else.
//!
//! The locale is root, deliberately, not the host's. Upstream's bare
//! `localeCompare` follows the build machine's default locale, and a Swedish
//! or Danish machine sorts `ö` after `z` where every other locale sorts it
//! beside `o`; a sheet whose order depended on the environment would be worse
//! than one that diverges from a Swedish machine.

use std::cmp::Ordering;
use std::sync::LazyLock;

use icu_collator::{CollatorBorrowed, options::CollatorOptions};

/// The ICU major version whose collation data the collator was built with:
/// `icu_collator_data` 2.3 holds the data of ICU 78 (CLDR 48). Beyond
/// printable ASCII, the order is Node's order only on a Node that ships the
/// same data, so the addon compares this with `process.versions.icu`.
pub const ICU_DATA_VERSION: &str = "78";

/// The Unicode version of the same data, compared with
/// `process.versions.unicode`.
pub const UNICODE_DATA_VERSION: &str = "17.0";

/// The CLDR major version of the same data, compared with
/// `process.versions.cldr`.
pub const CLDR_DATA_VERSION: &str = "48";

/// Printable ASCII in primary weight order, as V8 (ICU 78) collates it with
/// the default locale, letters case folded: the space, then the punctuation
/// and symbols in the collation's order, then the digits, then the letters.
pub(crate) const PRIMARY_ORDER: &[u8] =
  b" _-,;:!?.'\"()[]{}@*/\\&#%`^+<=>|~$0123456789abcdefghijklmnopqrstuvwxyz";

/// `PRIMARY_ORDER` inverted: a byte to its one-based position in it, case
/// folded, and 0 for a byte the order does not name (none of which reaches the
/// fast path). Built once at compile time: the comparator runs once per byte
/// of every comparison of the sort.
pub(crate) const PRIMARY_RANK: [u8; 128] = build_primary_rank();

/// Callable at runtime as well as in a `const`, so the test can assert the
/// table's invariants rather than argue them.
pub(crate) const fn build_primary_rank() -> [u8; 128] {
  let mut table = [0u8; 128];
  let mut index = 0;

  while index < PRIMARY_ORDER.len() {
    let byte = PRIMARY_ORDER[index];
    let rank = (index + 1) as u8;
    table[byte as usize] = rank;

    if byte.is_ascii_lowercase() {
      table[byte.to_ascii_uppercase() as usize] = rank;
    }

    index += 1;
  }

  table
}

/// The primary weight of one printable ASCII byte.
fn primary_weight(byte: u8) -> u8 {
  PRIMARY_RANK[usize::from(byte & 0x7f)]
}

/// Whether every byte of `text` is printable ASCII (`0x20..=0x7e`), the fast
/// path's precondition.
fn is_printable_ascii(text: &str) -> bool {
  text
    .bytes()
    .all(|byte| byte.is_ascii_graphic() || byte == b' ')
}

/// Root collation, built once. `CollatorBorrowed::new_root` has no failure to
/// handle: the compiled CLDR data is a build-time dependency of the crate.
static ROOT_COLLATOR: LazyLock<CollatorBorrowed<'static>> =
  LazyLock::new(|| CollatorBorrowed::new_root(CollatorOptions::default()));

/// The two printable ASCII strings in root collation order: the primary level
/// over the whole string, then the case tie-break.
fn compare_printable_ascii(a: &[u8], b: &[u8]) -> Ordering {
  // Equal bytes weigh the same at both levels, so the shared prefix decides
  // nothing: start after it. Texts of one property share long prefixes.
  let shared = a
    .iter()
    .zip(b)
    .take_while(|(left, right)| left == right)
    .count();
  let (a, b) = (&a[shared..], &b[shared..]);

  for (left, right) in a.iter().zip(b) {
    let primary = primary_weight(*left).cmp(&primary_weight(*right));

    if primary != Ordering::Equal {
      return primary;
    }
  }

  let by_length = a.len().cmp(&b.len());

  if by_length != Ordering::Equal {
    return by_length;
  }

  // Past the shared prefix, the first bytes differ, in case alone (their
  // primaries tied): lowercase sorts first. No byte left is two equal texts.
  match a.first() {
    Some(left) if left.is_ascii_lowercase() => Ordering::Less,
    Some(_) => Ordering::Greater,
    None => Ordering::Equal,
  }
}

/// Root collation alone, for the tests that hold the two paths to one answer.
pub(crate) fn compare_by_collator(a: &str, b: &str) -> Ordering {
  ROOT_COLLATOR.compare(a, b)
}

/// A text as `localeCompare` reads it, with the test for the printable ASCII
/// path made once: a sort compares each text many times.
#[derive(Clone, Copy, Debug)]
pub(crate) struct Collated<'t> {
  text: &'t str,
  printable_ascii: bool,
}

impl<'t> Collated<'t> {
  pub(crate) fn new(text: &'t str) -> Self {
    Collated {
      text,
      printable_ascii: is_printable_ascii(text),
    }
  }

  /// A slice of this text, read without a second scan when this text is
  /// printable ASCII: every slice of it is too. The part must be a slice of
  /// this text.
  pub(crate) fn part(&self, part: &'t str) -> Self {
    if self.printable_ascii {
      Collated {
        text: part,
        printable_ascii: true,
      }
    } else {
      Collated::new(part)
    }
  }
}

/// The order `a.localeCompare(b)` answers in Node.
#[cfg(test)]
pub(crate) fn compare(a: &str, b: &str) -> Ordering {
  compare_collated(Collated::new(a), Collated::new(b))
}

fn compare_collated(a: Collated<'_>, b: Collated<'_>) -> Ordering {
  if a.printable_ascii && b.printable_ascii {
    compare_printable_ascii(a.text.as_bytes(), b.text.as_bytes())
  } else {
    compare_by_collator(a.text, b.text)
  }
}

/// `a.localeCompare(b)` as the number it answers in Node: -1, 0 or 1.
pub(crate) fn locale_compare(a: &str, b: &str) -> f64 {
  locale_compare_collated(Collated::new(a), Collated::new(b))
}

/// [`locale_compare`] of two texts read once.
pub(crate) fn locale_compare_collated(a: Collated<'_>, b: Collated<'_>) -> f64 {
  match compare_collated(a, b) {
    Ordering::Less => -1.0,
    Ordering::Equal => 0.0,
    Ordering::Greater => 1.0,
  }
}

#[cfg(test)]
#[path = "tests/collation_test.rs"]
mod collation_test;
