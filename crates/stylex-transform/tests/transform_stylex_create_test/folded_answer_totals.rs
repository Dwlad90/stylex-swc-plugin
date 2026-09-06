//! What a fold's *answer* may allocate, counted across the whole answer.
//!
//! The two ceilings bounded every value the answer held, one value at a time.
//! The engine aliases and this side copies, so an answer whose every value sits
//! exactly on the line is still a tree of the ceiling raised to the nesting —
//! and no rule fired, because no single value was over. Both counts are now
//! running totals of the whole answer, the way the values crossing the other way
//! already were.
//!
//! Every folding case below is measured output of `@stylexjs/babel-plugin`
//! 0.19.0 under the same options. The refusals were measured against it too:
//! the reference compiler has no such ceiling, so it folds each of them, and
//! then refuses all but one of them where the folded value lands. So the
//! modules mostly agree on being refused and disagree on the sentence, and each
//! case says which.

use crate::utils::{
  prelude::*,
  transform::{assert_folds_with, assert_refuses_under, stringify_js},
};

/// The first line of each refusal, so a case cannot be satisfied by some later,
/// unrelated rule firing.
const TOO_MANY_ELEMENTS: &str = "Array length is too large to evaluate at compile time.";
const TOO_MANY_PROPERTIES: &str = "Object is too large to evaluate at compile time.";
const TOO_MUCH_TEXT: &str = "Folded string is too large to evaluate at compile time.";

/// Compile with the two allocation ceilings set to `characters` and `entries`.
///
/// The subject here is a total rather than a value, and a total is only visible
/// where several values are counted against it — so every case moves a ceiling
/// down to where a handful of values reach it, the way an author moves it
/// through the option rather than by asserting a default from the inside.
fn fold_under(input: &str, characters: usize, entries: usize) -> String {
  stringify_js(input, ts_syntax(), move |tr| {
    theme_import_transform_with(tr.comments.clone(), move |builder| {
      builder
        .with_max_folded_characters(characters)
        .with_max_folded_entries(entries)
    })
  })
}

/// The rule `body` is expected to emit under the two ceilings.
#[track_caller]
fn assert_folds_under(decls: &str, body: &str, rule: &str, characters: usize, entries: usize) {
  assert_folds_with(
    decls,
    body,
    rule,
    &format!(
      " under ceilings of {} characters and {} entries",
      characters, entries
    ),
    |module| fold_under(module, characters, entries),
  );
}

/// The same for a refusal.
#[track_caller]
fn assert_refuses_at(decls: &str, body: &str, sentence: &str, characters: usize, entries: usize) {
  assert_refuses_under(decls, body, sentence, move |module| {
    fold_under(module, characters, entries)
  });
}

// ──────────────────────────────────────────────
// Elements, counted across the whole answer
// ──────────────────────────────────────────────

/// The shape the per-value check waved through: one array the engine hands back
/// once per element of itself. Every level is exactly on the line, so nothing a
/// per-value check could ask would refuse it — and the tree is the ceiling
/// squared.
///
/// Upstream folds it, having no such ceiling, and then refuses the module where
/// the array of arrays lands: `A style array value can only contain strings or
/// numbers`. Both compilers refuse; this one refuses before building the tree.
#[test]
fn an_answer_that_repeats_one_array_counts_it_every_time() {
  assert_refuses_at(
    "const x = ['a', 'b', 'c'];",
    "content: x.map(() => x),",
    TOO_MANY_ELEMENTS,
    1_000,
    6,
  );
}

/// The same total spent on values the answer really does hold, so the running
/// count is not refusing an answer that is merely referenced twice. Twelve
/// elements under a ceiling of twelve fold; the same twelve under eleven do not.
///
/// The rule names three values rather than twelve because a fallback list drops
/// the repeats after the fold — which is upstream's answer too, and is the point:
/// all twelve crossed the bridge, and what happened to them afterwards is not
/// this ceiling's business.
#[test]
fn an_answer_at_the_entry_ceiling_still_folds() {
  let concatenated = "fontFamily: x.concat(x).concat(x).concat(x),";

  assert_folds_under(
    "const x = ['a', 'b', 'c'];",
    concatenated,
    ".xvnx2f3{font-family:a;font-family:b;font-family:c}",
    1_000,
    12,
  );

  assert_refuses_at(
    "const x = ['a', 'b', 'c'];",
    concatenated,
    TOO_MANY_ELEMENTS,
    1_000,
    11,
  );
}

/// Properties are the same count as elements and share the total with them, so
/// an answer that is an array of objects is refused by what the two come to
/// together rather than by whichever half is larger. Upstream folds it and then
/// refuses it where it lands, as it does the aliased array above.
#[test]
fn the_properties_of_an_answer_count_against_the_elements_of_it() {
  assert_refuses_at(
    "const o = { a: '1px', b: '2px' };",
    "content: [o, o].map((v) => v),",
    TOO_MANY_PROPERTIES,
    1_000,
    5,
  );
}

// ──────────────────────────────────────────────
// Characters, counted across the whole answer
// ──────────────────────────────────────────────

/// Two strings of six under a ceiling of eleven. Neither is over it; the answer
/// is. This is the one case of the three the reference compiler compiles, and
/// the folding half below is its measured output.
#[test]
fn the_strings_of_an_answer_count_against_one_total() {
  let doubled = "fontFamily: ['abc', 'def'].map((v) => v + v),";

  assert_refuses_at("", doubled, TOO_MUCH_TEXT, 11, 1_000);

  assert_folds_under(
    "",
    doubled,
    ".x1ys8t3w{font-family:abcabc;font-family:defdef}",
    12,
    1_000,
  );
}

/// A key is text too, and an object of few enormous keys is the answer that
/// makes the point: three properties is no count at all, and the keys alone are
/// past the character ceiling. Upstream folds it and then refuses the module
/// where the object lands — `Invalid pseudo or at-rule` — so both compilers
/// refuse and each says why in its own terms.
#[test]
fn the_keys_of_an_answer_count_as_text() {
  assert_refuses_at(
    "",
    "color: ({ aaaa: 1, bbbb: 2, cccc: 3 }).valueOf(),",
    TOO_MUCH_TEXT,
    11,
    1_000,
  );
}
