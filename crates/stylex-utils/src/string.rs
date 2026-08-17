use std::borrow::Cow;

use stylex_regex::regex::DASHIFY_REGEX;

/// Converts a camelCase or PascalCase string to its hyphenated (kebab-case)
/// equivalent by inserting hyphens before uppercase letters and lowercasing
/// the result.
///
/// This is used to convert JavaScript-style CSS property names (e.g.
/// `marginTop`, `WebkitTransform`) to their CSS equivalents (`margin-top`,
/// `-webkit-transform`).
/// `dashify` lowercases unconditionally, so the borrowed fast path is only
/// sound for input the lowercasing cannot change. ASCII without an uppercase
/// letter qualifies; anything non-ASCII does not, because a scalar can lowercase
/// to something other than itself while satisfying neither `is_uppercase` nor
/// `is_lowercase` — a titlecase scalar such as `ǅ` lowercases to `ǆ`.
pub fn dashify(s: &str) -> Cow<'_, str> {
  if s.is_ascii() && !s.bytes().any(|byte| byte.is_ascii_uppercase()) {
    return Cow::Borrowed(s);
  }

  Cow::Owned(DASHIFY_REGEX.replace_all(s, "-$1").to_lowercase())
}

/// Whether a value spells no CSS text at all — empty, or nothing but
/// characters the value scanner reads as whitespace.
///
/// A declaration built from one of these is `color:`, which no browser accepts,
/// so the property is left undeclared instead. Asked of both an authored value
/// and the value it transforms to, which is why it lives here rather than on
/// either type.
///
/// Deliberately narrower than `str::trim`. The scanner calls a character
/// whitespace when its code is at most 32 and nothing else, so U+00A0 or
/// U+3000 is a *word token* to it, not a gap — and the reference compiler
/// emits such a value verbatim rather than dropping the property. Testing
/// bytes is exact here: every byte of a multi-byte character is at least 0x80.
///
/// Wider than `str::trim` in the other direction, which is the half that
/// changes an answer rather than fixing one. A C0 control is not Unicode
/// whitespace, so `trim` left `"\u{1}"` in place and the value reached
/// `normalize_whitespace`, whose `trim_edges` emptied the token list and
/// rejected with `LINT_VALUE_HAS_NO_TOKENS` — which happens to match the
/// `TypeError` the reference throws at the same point. Reading it as blank
/// drops the property silently instead.
///
/// That is the answer this seam already gave for `""` and `" "`, which the
/// reference also throws on and which the parity corpus already carries as
/// acceptance divergences. Chosen for that consistency: the alternative is a
/// rule under which `"\t"` is dropped and `"\u{1}"` is a hard error, which no
/// author could predict from the value they wrote. Pinned by
/// `c0_controls_that_are_not_unicode_whitespace_are_blank`.
pub fn is_blank_css_text(s: &str) -> bool {
  s.bytes().all(|byte| byte <= 32)
}

/// Strips surrounding double-quote characters from a string.
pub fn remove_quotes(s: &str) -> Cow<'_, str> {
  let trimmed = s.trim_matches('"');

  if trimmed.len() == s.len() {
    Cow::Borrowed(s)
  } else {
    Cow::Borrowed(trimmed)
  }
}

/// Wraps a key in double quotes when `should_wrap_in_quotes` is true,
/// otherwise returns the key unchanged.
pub fn wrap_key_in_quotes(key: &str, should_wrap_in_quotes: bool) -> Cow<'_, str> {
  if should_wrap_in_quotes {
    Cow::Owned(format!("\"{}\"", key))
  } else {
    Cow::Borrowed(key)
  }
}

/// Returns the UTF-16 code unit at the given index, or `None` if the index is
/// out of bounds.
///
/// `charCodeAt` indexes by UTF-16 code unit and returns a single code unit, so
/// an astral scalar occupies two indices and reads back as its surrogate halves
/// — `"🎉"` is `0xD83C` at 0 and `0xDF89` at 1, never the `0x1F389` scalar.
/// Indexing by `char` instead would return the whole scalar and shift every
/// index that follows one.
pub fn char_code_at(s: &str, index: usize) -> Option<u32> {
  s.encode_utf16().nth(index).map(u32::from)
}

/// `char_code_at` for an index that arrived as a JS number, applying
/// `charCodeAt`'s own argument coercion.
///
/// `charCodeAt` runs `ToIntegerOrInfinity` on its argument: `NaN` becomes `0`,
/// fractional values truncate toward zero, and any negative or infinite index is
/// out of range. `index as usize` saturates rather than wrapping, so a bare cast
/// would silently turn `charCodeAt(-1)` — which JS answers with `NaN` — into the
/// code unit at index 0.
pub fn char_code_at_f64(s: &str, index: f64) -> Option<u32> {
  if index.is_nan() {
    return char_code_at(s, 0);
  }

  if index < 0.0 || index.is_infinite() {
    return None;
  }

  char_code_at(s, index as usize)
}

#[cfg(test)]
#[path = "tests/string_test.rs"]
mod tests;
