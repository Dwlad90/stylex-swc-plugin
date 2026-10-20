//! JavaScript string operations over Rust strings.
//!
//! JavaScript counts a string in UTF-16 code units, and Rust in UTF-8 bytes.
//! These functions take the counts and positions that JavaScript gives and
//! answer what JavaScript answers.

use std::borrow::Cow;

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
    let mut start: usize = 0;
    for &length in lengths {
      // A sum past `usize` is past the end of the text too.
      let end = start.checked_add(length as usize)?;
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

/// `String.prototype.replace` and `replaceAll` with a string pattern and a
/// string replacement, as the reference calls them: the replacement is a
/// template in which `$$` is a dollar, `$&` the matched text, `` $` `` the text
/// before the match and `$'` the text after it (ECMA-262 GetSubstitution; a
/// string pattern has no capture groups, so `$1` and `$<name>` stay literal).
pub fn js_substitution(template: &str, matched: &str, before: &str, after: &str) -> String {
  let mut out = String::with_capacity(template.len());
  push_substitution(&mut out, template, matched, before, after);
  out
}

/// Writes [`js_substitution`] into `out`. A template with no `$` is the
/// text itself, which is nearly every replacement a sheet makes.
fn push_substitution(out: &mut String, template: &str, matched: &str, before: &str, after: &str) {
  if !template.contains('$') {
    out.push_str(template);
    return;
  }

  let mut characters = template.chars().peekable();

  while let Some(character) = characters.next() {
    if character != '$' {
      out.push(character);
      continue;
    }

    match characters.peek() {
      Some('$') => {
        characters.next();
        out.push('$');
      },
      Some('&') => {
        characters.next();
        out.push_str(matched);
      },
      Some('`') => {
        characters.next();
        out.push_str(before);
      },
      Some('\'') => {
        characters.next();
        out.push_str(after);
      },
      _ => out.push('$'),
    }
  }
}

/// Writes the substitution for the occurrence of `pattern` at `start` of
/// `text`: the template read against that occurrence's surroundings, the text
/// before it and the text after it.
fn push_substitution_at(out: &mut String, text: &str, start: usize, pattern: &str, template: &str) {
  push_substitution(
    out,
    template,
    pattern,
    &text[..start],
    &text[start + pattern.len()..],
  );
}

/// `text.replace(pattern, replacement)`: the first occurrence.
pub fn js_replace_first(text: &str, pattern: &str, replacement: &str) -> String {
  match text.find(pattern) {
    Some(start) => {
      let mut out = String::with_capacity(text.len() + replacement.len());
      out.push_str(&text[..start]);
      push_substitution_at(&mut out, text, start, pattern, replacement);
      out.push_str(&text[start + pattern.len()..]);
      out
    },
    None => text.to_string(),
  }
}

/// `text.replaceAll(pattern, replacement)`: every occurrence, left to right,
/// each substitution reading its own surroundings in the original text. A text
/// with no occurrence is handed back as it is: the loop below runs once per
/// rule and constant, and nearly every such pair has nothing to replace.
///
/// An empty pattern occurs before each character and at the end. JavaScript
/// also finds it between the two halves of a character outside the Basic
/// Multilingual Plane; a `str` cannot cut such a character, so there is one
/// replacement less for each.
pub fn js_replace_all<'t>(text: &'t str, pattern: &str, replacement: &str) -> Cow<'t, str> {
  if pattern.is_empty() {
    return Cow::Owned(replace_empty_pattern(text, replacement));
  }

  let Some(first) = text.find(pattern) else {
    return Cow::Borrowed(text);
  };

  let mut out = String::with_capacity(text.len());
  let mut rest = &text[first..];
  let mut consumed = first;
  out.push_str(&text[..first]);

  while let Some(offset) = rest.find(pattern) {
    let start = consumed + offset;
    out.push_str(&rest[..offset]);
    push_substitution_at(&mut out, text, start, pattern, replacement);
    rest = &rest[offset + pattern.len()..];
    consumed = start + pattern.len();
  }

  out.push_str(rest);
  Cow::Owned(out)
}

/// [`js_replace_all`] of the empty pattern: the replacement at each character
/// boundary of `text`, and at its end.
fn replace_empty_pattern(text: &str, replacement: &str) -> String {
  let boundaries = text
    .char_indices()
    .map(|(index, _)| index)
    .chain([text.len()]);
  // One replacement for each boundary: the exact length when the replacement
  // holds no `$` pattern, so the text is written without a reallocation. A
  // length past `usize` starts from the text alone and grows as it must.
  let boundary_count = text.chars().count() + 1;
  let capacity = boundary_count
    .checked_mul(replacement.len())
    .and_then(|replacements| replacements.checked_add(text.len()))
    .unwrap_or(text.len());
  let mut out = String::with_capacity(capacity);
  let mut previous = 0;

  for boundary in boundaries {
    out.push_str(&text[previous..boundary]);
    push_substitution(
      &mut out,
      replacement,
      "",
      &text[..boundary],
      &text[boundary..],
    );
    previous = boundary;
  }

  out
}

#[cfg(test)]
#[path = "tests/string_tests.rs"]
mod tests;
