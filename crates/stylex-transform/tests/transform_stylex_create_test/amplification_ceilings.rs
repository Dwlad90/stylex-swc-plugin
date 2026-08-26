//! What one fold may allocate, and who gets to say so.
//!
//! Two ceilings bound a fold's allocation, because the engine it runs on does
//! not: growth inside a native builtin is not a counted loop, so a mistyped
//! `'x'.repeat(200000000)` agrees with the language and reaches gigabytes of
//! resident memory. One bounds the string a fold builds or carries, the other
//! the elements and properties, because a bounded string can still become one
//! element per code unit and cost far more as a tree than it did as text.
//!
//! Two things changed here. The bound is now **arithmetic on values** rather
//! than a shape: a count is bounded by reading it, so `'x'.repeat(n)` and
//! `'x'.repeat(2 * 2)` are bounded exactly as `'x'.repeat(4)` is, and `repeat`
//! multiplies its receiver's own length into the product. And both ceilings are
//! **project options** with an environment override, on the precedence
//! `maxEvaluationDepth` already set.
//!
//! Every folding case below is measured output of `@stylexjs/babel-plugin`
//! 0.19.0 under the same options, so each asserts agreement with the reference
//! compiler rather than with this compiler's own previous answer. The refusals
//! are where the two part company, and each says which way.

use crate::utils::{
  prelude::*,
  transform::{
    assert_folds, assert_refuses, assert_refuses_under, base_style_module as module,
    fold_module as fold, stringify_js,
  },
};

/// The first line both refusals of an amplifying call open with, so a case
/// cannot be satisfied by some later, unrelated rule firing.
const CANNOT_BOUND: &str = "Cannot bound the string 'repeat' would build.";

/// Compile with the two allocation ceilings set to `characters` and `entries`.
///
/// The whole point of this ticket is that a project can say what its own folds
/// cost, so the cases that assert an author *can* move a ceiling have to move it
/// the way an author does — through the option — rather than by asserting the
/// default from the inside.
fn fold_under(input: &str, characters: usize, entries: usize) -> String {
  stringify_js(input, ts_syntax(), move |tr| {
    theme_import_transform_with(tr.comments.clone(), move |builder| {
      builder
        .with_max_folded_characters(characters)
        .with_max_folded_entries(entries)
    })
  })
}

// ──────────────────────────────────────────────
// A count is bounded by reading it
// ──────────────────────────────────────────────

/// A count no longer has to be written out as a number. Each of these refused
/// before, and upstream folds every one — so each row closes a divergence
/// rather than opening one.
#[test]
fn a_count_folds_however_it_is_spelled() {
  let cases: &[(&str, &str, &str)] = &[
    // An expression, which is the case the parity corpus pinned as divergent.
    (
      "",
      "content: 'x'.repeat(2 * 2),",
      ".xwjk6qn{content:\"xxxx\"}",
    ),
    (
      "",
      "gridArea: '7'.padStart([1, 2, 3].length, '0'),",
      ".x1ffk7c0{grid-area:007}",
    ),
    // A name, which is the shape a real project writes: a spacing scale or a
    // column count lives in a constant, not in the declaration.
    (
      "const n = 5;",
      "content: 'x'.repeat(n),",
      ".x1yd5vb{content:\"xxxxx\"}",
    ),
    (
      "const n = 2;",
      "content: 'ab'.repeat(n + 1),",
      ".x5ryvnc{content:\"ababab\"}",
    ),
    (
      "const cfg = { pad: 3 };",
      "gridArea: '7'.padStart(cfg.pad, '0'),",
      ".x1ffk7c0{grid-area:007}",
    ),
    (
      "const n = 5;",
      "content: 'ab'.padEnd(n, '-'),",
      ".x1s5xiu0{content:\"ab---\"}",
    ),
  ];

  for (decls, body, rule) in cases {
    assert_folds(decls, body, rule);
  }
}

/// The count goes through the language's own `ToNumber`, because that is what
/// the engine is about to do to it. Bounding it any other way would refuse an
/// input the reference compiler folds, and bound the call by a number nothing
/// uses.
///
/// Every row is upstream's measured answer, including the two that look like
/// mistakes: a count that is not a number at all is zero, so the call folds to
/// the empty string rather than throwing.
#[test]
fn a_count_is_coerced_the_way_the_language_coerces_it() {
  let cases: &[(&str, &str, &str)] = &[
    (
      "",
      "content: 'x'.repeat('3'),",
      ".x1r4hedj{content:\"xxx\"}",
    ),
    (
      "const n = '3';",
      "content: 'x'.repeat(n),",
      ".x1r4hedj{content:\"xxx\"}",
    ),
    ("", "content: 'ab'.repeat(true),", ".xarbti{content:\"ab\"}"),
    (
      "",
      "content: 'ab'.repeat([2]),",
      ".xvxxpsj{content:\"abab\"}",
    ),
    // `ToIntegerOrInfinity` truncates toward zero, so three and a half repeats
    // are three.
    (
      "",
      "content: 'ab'.repeat(3.5),",
      ".x5ryvnc{content:\"ababab\"}",
    ),
    // And reads everything it cannot make a number of as zero.
    ("", "content: 'ab'.repeat(null),", ".x14axycx{content:\"\"}"),
    (
      "",
      "content: 'x'.repeat('lots'),",
      ".x14axycx{content:\"\"}",
    ),
    ("", "content: 'x'.repeat(0 / 0),", ".x14axycx{content:\"\"}"),
  ];

  for (decls, body, rule) in cases {
    assert_folds(decls, body, rule);
  }
}

/// The receiver is read the same way, so a name holding a string is a receiver
/// like the literal it was given the name of.
#[test]
fn a_receiver_length_is_read_wherever_it_is_written() {
  let cases: &[(&str, &str, &str)] = &[
    (
      "const base = 'ab';",
      "content: base.repeat(3),",
      ".x5ryvnc{content:\"ababab\"}",
    ),
    (
      "",
      "content: `ab`.repeat(3),",
      ".x5ryvnc{content:\"ababab\"}",
    ),
    (
      "",
      "content: ('ab').repeat(3),",
      ".x5ryvnc{content:\"ababab\"}",
    ),
  ];

  for (decls, body, rule) in cases {
    assert_folds(decls, body, rule);
  }
}

/// A count that cannot be *read* is still a count that cannot be bounded, and
/// reading one is not a way around the ceiling.
///
/// The first two are rejected upstream as well, in its own words — `Invalid
/// count value` — so only the sentences differ. The rest are this compiler's
/// deliberate divergences, and each is a length upstream really does build.
#[test]
fn a_length_that_cannot_be_bounded_still_refuses() {
  let refusals = [
    // The language throws on both of these too, so nothing is lost by refusing
    // before the engine gets there.
    ("", "content: 'x'.repeat(1 / 0),", CANNOT_BOUND),
    ("", "content: 'x'.repeat(...[2]),", CANNOT_BOUND),
    // A named count past the ceiling refuses exactly as a written one does:
    // the bound is read from the value, so a name is not a way round it.
    (
      "const n = 200000000;",
      "content: 'x'.repeat(n),",
      CANNOT_BOUND,
    ),
    // The product is what is bounded, so a two-character receiver reaches the
    // ceiling at half the count a one-character receiver does.
    ("", "content: 'xx'.repeat(600000).length,", CANNOT_BOUND),
    // And a receiver that is itself a call has no readable length, which is the
    // rule that keeps two allowed lengths from multiplying into one that is
    // not. Upstream folds this one to a million characters.
    (
      "",
      "content: 'x'.repeat(1000).repeat(1000).length,",
      CANNOT_BOUND,
    ),
  ];

  for (decls, body, sentence) in refusals {
    assert_refuses(decls, body, sentence);
  }
}

/// A negative count is the one case left to the language rather than answered
/// here. The bound reads it as `ToIntegerOrInfinity` does, which puts it under
/// the ceiling, and the engine then raises the `RangeError` the language really
/// raises for it. Upstream rejects the same input in its own engine's words —
/// `Invalid count value: -1` — so the two agree on the answer and differ only in
/// the sentence, which is not a parity obligation.
#[test]
#[should_panic(expected = "RangeError: repeat count must be a positive finite number")]
fn a_negative_count_carries_the_language_s_own_sentence() {
  fold(&module("", "content: 'x'.repeat(-1),"));
}

// ──────────────────────────────────────────────
// The ceilings are the project's
// ──────────────────────────────────────────────

/// Lowering the string ceiling refuses a call the default folds, which is the
/// observable half of the option existing at all.
///
/// Asserted by moving the ceiling rather than by asserting the default's own
/// number, so the case says what an author can do rather than what this
/// compiler happens to ship.
#[test]
#[should_panic(
  expected = "It asks for 10 copies of the value it is called on, which is 40 characters, and at most 8 are supported."
)]
fn a_lowered_character_ceiling_refuses_what_the_default_folds() {
  fold_under(&module("", "content: 'xxxx'.repeat(10),"), 8, 10_000);
}

/// And raising it folds a call the default refuses, which is the half a project
/// that really generates large values needs.
#[test]
fn a_raised_character_ceiling_folds_what_the_default_refuses() {
  let output = fold_under(
    &module("", "content: 'x'.repeat(2000000).length,"),
    4_000_000,
    10_000,
  );

  assert!(
    output.contains(".xjzom13{content:\"2000000px\"}"),
    "expected a raised ceiling to fold two million characters, got:\n{}",
    output
  );
}

/// The entry ceiling moves the same way, on the same option shape — and it is
/// the one that catches a *bounded* string turned into one element per code
/// unit.
#[test]
#[should_panic(expected = "Array length is too large to evaluate at compile time.")]
fn a_lowered_entry_ceiling_refuses_a_shorter_array() {
  fold_under(&module("", "fontFamily: 'xxxx'.split(''),"), 1_000_000, 3);
}

/// Raised, the same array folds. Upstream folds it at every ceiling, so this is
/// the divergence closing rather than a behaviour of its own.
#[test]
fn a_raised_entry_ceiling_folds_a_longer_array() {
  let output = fold_under(
    &module("", "content: 'x'.repeat(20000).split('').length,"),
    1_000_000,
    50_000,
  );

  assert!(
    output.contains(".x7ycng{content:\"20000px\"}"),
    "expected a raised ceiling to fold twenty thousand elements, got:\n{}",
    output
  );
}

/// A ceiling of zero is not a ceiling — it would refuse the folds the compiler
/// runs to do its own work — so it is read as unset and the default answers.
/// The same reading `maxEvaluationDepth` gives one.
#[test]
fn a_configured_zero_leaves_the_default_in_place() {
  let output = fold_under(&module("", "content: 'ab'.repeat(3),"), 0, 0);

  assert!(
    output.contains(".x5ryvnc{content:\"ababab\"}"),
    "expected a zero ceiling to fall back to the default, got:\n{}",
    output
  );
}

/// Both ceilings bound the way *in* as well as the way out, because a resolved
/// name is copied into the engine element by element. So a name holding more
/// than the fold may carry is refused before anything is printed, and the
/// refusal names the binding rather than the method: the size belongs to what
/// the name holds, and the same call on a smaller value folds.
#[test]
fn the_ceilings_bound_a_resolved_value_on_the_way_in() {
  let long = format!("const text = '{}';", "a".repeat(64));

  let refusals = [
    (
      long.as_str(),
      "content: text.toUpperCase(),",
      "Cannot carry the value of 'text' into a fold.",
    ),
    (
      "const parts = ['a', 'b', 'c', 'd'];",
      "content: parts.join('-'),",
      "Cannot carry the value of 'parts' into a fold.",
    ),
  ];

  for (decls, body, sentence) in refusals {
    assert_refuses_under(decls, body, sentence, |module| fold_under(module, 8, 3));
  }

  // And both fold under the shipped default, so the cases above are the ceiling
  // answering rather than anything else about the input.
  assert_folds(
    "const parts = ['a', 'b', 'c', 'd'];",
    "content: parts.join('-'),",
    ".xd86k44{content:\"a-b-c-d\"}",
  );
  assert_folds(
    &long,
    "content: text.toUpperCase(),",
    ".xckdrkj{content:\"AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA\"}",
  );
}
