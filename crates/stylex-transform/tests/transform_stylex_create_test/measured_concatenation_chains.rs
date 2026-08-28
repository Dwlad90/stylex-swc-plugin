//! A chain of `+` measured once per operand rather than once per link.
//!
//! A concatenation is grown through a buffer that counts every append against
//! the character ceiling. What the buffer could not know, before, is that its
//! left operand had already been counted: a folded `+` arrives as a plain string
//! literal, which has nowhere to carry a length, so each link re-read everything
//! the links below it had joined. That is the length of the text spent once per
//! remaining link -- the square of a chain rather than its length -- and it is
//! paid by every `+` in every file, whether or not anything else folds.
//!
//! The count now travels with the text, and the level above adopts the buffer
//! instead of copying and re-reading it. Nothing an author can observe may move
//! because of that, which is what this file is: the same chains, the same
//! answers, and the same refusals at the same lengths.
//!
//! Every folded value here is measured output of `@stylexjs/babel-plugin`
//! 0.19.0 for the same input. The refusals are this compiler's own ceiling,
//! which upstream does not have.

use crate::utils::{
  prelude::*,
  transform::{
    assert_folds, assert_folds_under, assert_refuses, assert_refuses_under, fold_module_under,
  },
};

/// The sentence a concatenation past the ceiling opens with, so a case cannot be
/// satisfied by some other rule firing.
const CONCATENATION_TOO_LARGE: &str =
  "This concatenation builds a string too large to evaluate at compile time.";

/// A left-associated chain of `n` single-character string literals, which is the
/// shape the cost was paid on: every link's left operand is the whole of the
/// text joined so far.
fn letter_chain(links: usize) -> String {
  (0..links)
    .map(|index| format!("'{}'", (b'a' + (index % 26) as u8) as char))
    .collect::<Vec<_>>()
    .join(" + ")
}

/// The text `letter_chain` spells, read off independently of the chain so the
/// expectation is not the same construction as the input.
fn letter_text(links: usize) -> String {
  (0..links)
    .map(|index| (b'a' + (index % 26) as u8) as char)
    .collect()
}

// ──────────────────────────────────────────────
// The chain still answers what it answered
// ──────────────────────────────────────────────

/// The shapes that decide which branch of the descent runs: a chain of strings,
/// a chain whose left side is arithmetic, a chain a string starts, and the
/// parenthesised spellings of the same trees.
#[test]
fn a_chain_folds_to_what_it_spells() {
  let cases: &[(&str, &str, &str)] = &[
    (
      "",
      "content: 'a' + 'b' + 'c' + 'd' + 'e',",
      ".xxb6vyk{content:\"abcde\"}",
    ),
    // Parentheses are unwrapped before the fold is asked, so the same tree
    // reaches the same answer written either way.
    (
      "",
      "content: ('a' + 'b') + 'c',",
      ".x6ojgef{content:\"abc\"}",
    ),
    (
      "",
      "content: ((('a' + 'b') + 'c') + 'd'),",
      ".x1ncg6ds{content:\"abcd\"}",
    ),
    // A left side that is arithmetic hands back a number, not a measured
    // string, and the concatenation starts at the first string.
    ("", "content: 1 + 2 + 'a',", ".x1jlgiil{content:\"3a\"}"),
    // The mirror: the string comes first and every operand after it is appended.
    ("", "content: 'a' + 1 + 2,", ".xq4ruai{content:\"a12\"}"),
    // Arithmetic on the right is a subtree the descent never enters, since only
    // a left operand is folded here.
    ("", "content: 'x' + (1 + 2),", ".xmmgmrj{content:\"x3\"}"),
    // No string anywhere: the chain stays arithmetic the whole way up.
    ("", "width: 1 + 2 + 3 + 4,", ".x1fsd2vl{width:10px}"),
    // Operands with no spelling of their own still have a `ToString`.
    (
      "",
      "content: 'a' + true + null,",
      ".xcxfhgo{content:\"atruenull\"}",
    ),
    (
      "",
      "content: 'a' + undefined + 'b',",
      ".x12v6j65{content:\"aundefinedb\"}",
    ),
    (
      "const o = {};",
      "content: 'a' + o + 'b',",
      ".x3a98e0{content:\"a[object Object]b\"}",
    ),
    // An array operand is written through the coercion element by element, so
    // it cannot arrive as an already-measured buffer -- both sides of the chain
    // reach one.
    (
      "const a = [1,2];",
      "content: a + '-' + a,",
      ".x16lgk3v{content:\"1,2-1,2\"}",
    ),
    // Empty operands grow nothing, and an adopted empty buffer is still a
    // buffer with a count.
    ("", "content: '' + '' + 'z',", ".x1609fvb{content:\"z\"}"),
    // A template beside a chain: the same buffer type grows both.
    (
      "const p = 'xy';",
      "content: `${p}${p}` + p,",
      ".x1kgjwt7{content:\"xyxyxy\"}",
    ),
  ];

  for (decls, body, rule) in cases {
    assert_folds(decls, body, rule);
  }
}

/// A template grows the same buffer as a `+`, so the two have to meet without
/// either losing what it measured: a chain inside an interpolation, a template
/// as an operand of a chain, and a length read off a template that holds one.
#[test]
fn a_template_and_a_chain_grow_the_same_buffer() {
  let cases: &[(&str, &str, &str)] = &[
    (
      "",
      "content: `${'a' + 'b' + 'c'}-`,",
      ".xvdxi96{content:\"abc-\"}",
    ),
    (
      "const p = 'x';",
      "content: 'a' + `${p}${p}` + 'b',",
      ".x1cg30om{content:\"axxb\"}",
    ),
    ("", "width: `${'a' + 'b'}cd`.length,", ".x51ohtg{width:4px}"),
  ];

  for (decls, body, rule) in cases {
    assert_folds(decls, body, rule);
  }
}

/// The operands a chain resolves rather than reads: a name, and a call the
/// engine answers. Neither arrives as a measured buffer, so both are written
/// through the coercion and measured where they land.
#[test]
fn a_chain_measures_operands_it_had_to_resolve() {
  assert_folds(
    "const a = 'a'; const b = 'b';",
    "content: a + b + a + b,",
    ".xvxxpsj{content:\"abab\"}",
  );
  assert_folds(
    "",
    "content: 'x'.repeat(2) + 'y' + 'z',",
    ".x1vmatha{content:\"xxyz\"}",
  );
}

/// The count the chain carries is the length the chain has, read back through
/// `.length` -- the one place an author can see the number the buffer kept.
#[test]
fn a_chain_reports_the_length_it_grew() {
  assert_folds(
    "",
    "width: ('a' + 'b' + 'c').length,",
    ".x1g8rjiy{width:3px}",
  );
}

/// Twenty-six links, which is a chain long enough that the difference between
/// measuring each operand and measuring every prefix is most of the work.
#[test]
fn a_long_chain_folds_whole() {
  assert_folds(
    "",
    &format!("content: {},", letter_chain(26)),
    ".x116rze4{content:\"abcdefghijklmnopqrstuvwxyz\"}",
  );

  // Twenty links of one repeated letter, which is the same chain written so
  // that the answer says nothing about the order the links ran in.
  assert_folds(
    "",
    &format!("content: {},", vec!["'a'"; 20].join(" + ")),
    ".xv5wlv9{content:\"aaaaaaaaaaaaaaaaaaaa\"}",
  );
}

// ──────────────────────────────────────────────
// The count carried is the count that was checked
// ──────────────────────────────────────────────

/// A ceiling refuses at the append that passes it, and the buffer's own count
/// is what that append is measured against. Adopted rather than re-read, that
/// count has to be the same number: one that drifted low would fold a chain
/// past the ceiling, and one that drifted high would refuse a chain inside it.
///
/// Walked link by link rather than asserted at one length, because a count
/// wrong by a single operand is exactly what a chain hides -- it comes right
/// again at the next link. The rule is matched without its class name here,
/// since what is under test is the length the chain reached rather than the
/// hash of it.
#[test]
fn a_chain_refuses_at_the_link_that_passes_the_ceiling() {
  const CEILING: usize = 6;

  for links in 1..=CEILING {
    assert_folds_under(
      "",
      &format!("content: {},", letter_chain(links)),
      &format!("{{content:\"{}\"}}", letter_text(links)),
      CEILING,
    );
  }

  for links in CEILING + 1..=CEILING + 3 {
    assert_refuses_under(
      "",
      &format!("content: {},", letter_chain(links)),
      CONCATENATION_TOO_LARGE,
      |input| fold_module_under(input, CEILING),
    );
  }
}

/// The same boundary stated as the two lengths either side of it, with the
/// class names measured rather than built: four letters is the last chain that
/// folds under a ceiling of four, and five is the first that does not.
#[test]
fn the_bound_admits_exactly_the_ceiling_along_a_chain() {
  assert_folds_under(
    "",
    "content: 'a' + 'b' + 'c' + 'd',",
    ".x1ncg6ds{content:\"abcd\"}",
    4,
  );

  assert_refuses_under(
    "",
    "content: 'a' + 'b' + 'c' + 'd' + 'e',",
    CONCATENATION_TOO_LARGE,
    |input| fold_module_under(input, 4),
  );
}

/// A count in code units, carried across a link. Two astral characters are four
/// code units and eight bytes, so a chain that adopted a byte count would fold
/// where this refuses, and one that adopted a scalar count would fold twice as
/// far.
#[test]
fn an_adopted_count_is_in_code_units() {
  assert_folds_under(
    "",
    "content: '\u{1F600}' + '\u{1F600}',",
    ".x119eoyx{content:\"\u{1F600}\u{1F600}\"}",
    4,
  );

  assert_refuses_under(
    "",
    "content: '\u{1F600}' + '\u{1F600}' + 'b',",
    CONCATENATION_TOO_LARGE,
    |input| fold_module_under(input, 4),
  );

  assert_folds(
    "",
    "content: '\u{1F600}' + '\u{1F600}' + 'b',",
    ".xp28tcv{content:\"\u{1F600}\u{1F600}b\"}",
  );
}

/// An unpaired surrogate reads exactly as it read before the count started
/// travelling: the operand has no compile-time string at all, so the `+` is
/// left to the runtime whole rather than folded to a replacement character.
///
/// Pinned here because the counting is what changed and this is the one text a
/// count cannot be taken of -- there is no `&str` holding a lone surrogate, so
/// it never reaches the buffer to be measured. It is refused a step earlier,
/// and by the reading of the operand rather than by the ceiling, which is what
/// the second half of the case says. Upstream folds it to a replacement
/// character; that divergence is older than this file.
#[test]
fn a_lone_surrogate_operand_is_refused_rather_than_measured() {
  assert_refuses(
    "",
    "content: '\\uD800' + 'ab',",
    "Unsupported expression: BinaryExpression",
  );

  // A raised ceiling changes nothing, so the refusal is not the ceiling's.
  assert_refuses_under(
    "",
    "content: '\\uD800' + 'ab',",
    "Unsupported expression: BinaryExpression",
    |input| fold_module_under(input, 4_000_000),
  );
}
