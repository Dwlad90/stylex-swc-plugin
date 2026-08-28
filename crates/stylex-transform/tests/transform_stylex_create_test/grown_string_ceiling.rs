//! The character ceiling, reaching a string the **evaluator** grows.
//!
//! `maxFoldedCharacters` bounds what a fold builds and what it carries, and both
//! readings sit where a value crosses into or out of the engine. Nothing crosses
//! for `a + a`: the evaluator answers a binary expression itself, so a chain that
//! doubles its own result was bounded by no number at all. What stopped it was
//! the *depth* budget, a limit on how far a walk descends rather than a claim
//! about how large a value gets -- so the ten-doubling chain below folded to a
//! hundred and two million characters in three seconds, past a ceiling a hundred
//! times smaller that never looked.
//!
//! **Where the bound went, and why there.** On the growth, not on what a binding
//! ends up holding. Three measurements decide it: an inline `(a + a).length`
//! allocates exactly as much while no binding holds the result, so a bound on
//! bindings would miss the same string written one way; the growth is where the
//! memory is spent, so refusing there refuses before the *next* doubling
//! allocates rather than after; and a long string a binding merely holds is one
//! allocation the author asked for, where what turns a typo into gigabytes is
//! compounding, which only the growth site sees.
//!
//! Measured against the same question, `concat` and `repeat` need nothing here --
//! both are calls, and each already carries a bound of its own, in and out of the
//! fold. The two expressions the evaluator grows a string with itself are `+` and
//! an interpolation, and those are the two this file covers.
//!
//! Every folding case is measured output of `@stylexjs/babel-plugin` 0.19.0. The
//! refusals are where the two compilers part company: upstream bounds none of
//! this and folds every chain below, so each refusal is a divergence a project
//! can configure away by raising the ceiling.

use crate::utils::{
  prelude::*,
  transform::{
    assert_folds, assert_folds_under, assert_refuses, assert_refuses_under,
    base_style_module as module, fold_module_under, stringify_js,
  },
};

/// The first line both refusals open with, so a case cannot be satisfied by some
/// later, unrelated rule firing. Which of the two fired is half of what each case
/// asserts, so they are two constants rather than one shared prefix.
const CONCATENATION_TOO_LARGE: &str =
  "This concatenation builds a string too large to evaluate at compile time.";
const TEMPLATE_TOO_LARGE: &str =
  "This template literal builds a string too large to evaluate at compile time.";

/// `n` bindings, each the concatenation of the one before it with itself, over a
/// base of `base` characters. So `a{n}` is `base * 2^n` characters long and every
/// single line of it is innocent, which is the whole shape of the bug.
fn doubling_chain(base: usize, doublings: usize) -> String {
  let mut source = format!("const a0 = 'x'.repeat({});\n", base);

  for step in 1..=doublings {
    source.push_str(&format!(
      "const a{} = a{} + a{};\n",
      step,
      step - 1,
      step - 1
    ));
  }

  source
}

// ──────────────────────────────────────────────
// What the bound must not touch
// ──────────────────────────────────────────────

/// The declarations a project actually writes are nowhere near the ceiling, and
/// every one of these folds to upstream's answer. Read as the control for the
/// whole file: a bound that refused any of them would be a regression rather
/// than a fix.
#[test]
fn an_ordinary_concatenation_is_untouched() {
  let cases: &[(&str, &str, &str)] = &[
    (
      "",
      "content: 'abcd' + 'efgh',",
      ".x18j2i3r{content:\"abcdefgh\"}",
    ),
    // Numbers on both sides never reach the string path at all.
    ("", "width: 2 + 3,", ".x1ftt334{width:5px}"),
    // One string anywhere in a chain carries it, so the bound has to read a
    // coerced operand rather than only a written one.
    ("", "content: 1 + '-' + 2,", ".xy02iow{content:\"1-2\"}"),
    // An empty operand grows nothing, so nothing can be refused for it.
    ("", "content: 'a' + '',", ".x16319ns{content:\"a\"}"),
    (
      "",
      "content: `abcd${'efgh'}`,",
      ".x18j2i3r{content:\"abcdefgh\"}",
    ),
    (
      "const p = 'xy';",
      "content: `${p}${p}${p}${p}`,",
      ".x1n0pqmm{content:\"xyxyxyxy\"}",
    ),
    // An interpolated array reaches its text through `ToString`, which is a
    // second way a template grows past what its quasis spell.
    (
      "const a = [1, 2, 3];",
      "content: `${a}`,",
      ".x8wkgwi{content:\"1,2,3\"}",
    ),
  ];

  for (decls, body, rule) in cases {
    assert_folds(decls, body, rule);
  }
}

/// A long value is not a refusal on its own. Four hundred thousand characters is
/// well inside the default ceiling, so a binding may hold it, a template may
/// interpolate it and a `+` may put an empty string beside it -- which is the
/// measurement behind putting the bound on the growth rather than on what a
/// binding holds.
#[test]
fn a_long_value_a_binding_merely_holds_still_folds() {
  let cases: &[(&str, &str, &str)] = &[
    (
      "const a0 = 'x'.repeat(400000);",
      "width: `${a0}`.length,",
      ".xpn81sq{width:400000px}",
    ),
    (
      "const a0 = 'x'.repeat(600000);",
      "width: ('' + a0).length,",
      ".xs4l4sx{width:600000px}",
    ),
  ];

  for (decls, body, rule) in cases {
    assert_folds(decls, body, rule);
  }
}

// ──────────────────────────────────────────────
// The chain the ticket is about
// ──────────────────────────────────────────────

/// The reported shape and the two spellings of it that hold nothing. All three
/// folded before, each to a string past the ceiling, and each refuses now under
/// the ceiling's own name.
///
/// The inline case is the one that decides where the bound goes: no binding holds
/// `a0 + a0` there, so a bound on what a binding may hold would let it through.
#[test]
fn a_doubling_chain_refuses_where_it_passes_the_ceiling() {
  // Ten doublings of a hundred thousand characters — the reported input, which
  // folded to 102,400,000 characters.
  assert_refuses(
    &doubling_chain(100_000, 10),
    "width: a10.length,",
    CONCATENATION_TOO_LARGE,
  );

  // One doubling is enough once the base is over half the ceiling, so the
  // refusal does not depend on a chain being long.
  assert_refuses(
    &doubling_chain(600_000, 1),
    "width: a1.length,",
    CONCATENATION_TOO_LARGE,
  );

  assert_refuses(
    "const a0 = 'x'.repeat(600000);",
    "width: (a0 + a0).length,",
    CONCATENATION_TOO_LARGE,
  );
}

/// A template literal grows a string the same way and is bounded by the same
/// number, whether the length comes from an interpolation or from the quasis
/// around it.
#[test]
fn a_template_literal_is_bounded_by_the_same_number() {
  assert_refuses(
    "const a0 = 'x'.repeat(600000);",
    "width: `${a0}${a0}`.length,",
    TEMPLATE_TOO_LARGE,
  );

  // Interpolating a long value once is the shorter shape of the same thing: the
  // bound is on what the template builds, not on how many holes it has.
  assert_refuses(
    "const a0 = 'x'.repeat(999999);",
    "width: `-${a0}-`.length,",
    TEMPLATE_TOO_LARGE,
  );
}

/// The two spellings that already had a bound keep their own rule, which is why
/// neither needed anything here. Each names what an author has to look at, and
/// neither is the sentence the evaluator's own growth raises.
#[test]
fn a_call_that_grows_a_string_keeps_its_own_rule() {
  // Bounded on the way back out of the fold.
  assert_refuses(
    "const a0 = 'x'.repeat(600000);",
    "width: a0.concat(a0).length,",
    "Folded string is too large to evaluate at compile time.",
  );

  // Bounded before the fold runs, by the arithmetic the count states.
  assert_refuses(
    "const a0 = 'x'.repeat(600000);",
    "width: a0.repeat(2).length,",
    "Cannot bound the string 'repeat' would build.",
  );
}

/// A chain long enough to double past the ceiling from a single character is the
/// *depth* budget's refusal, not this one. The two ceilings bound different
/// things and the deeper chain reaches the nesting limit first, so a case that
/// read the wrong sentence here would be reading the wrong ceiling.
#[test]
fn a_deeper_chain_is_the_nesting_budget_s_refusal() {
  assert_refuses(
    &doubling_chain(1, 20),
    "width: a20.length,",
    "Expression is too deeply nested to evaluate at compile time.",
  );
}

// ──────────────────────────────────────────────
// The bound is a project's own number
// ──────────────────────────────────────────────

/// Raised, the same chain folds -- to upstream's own answer, so this is the
/// divergence closing rather than a behaviour of this compiler's own.
#[test]
fn a_raised_ceiling_folds_the_chain_the_default_refuses() {
  assert_folds_under(
    &doubling_chain(600_000, 1),
    "width: a1.length,",
    ".x1sdnrpv{width:1200000px}",
    4_000_000,
  );
}

/// Lowered, it refuses a concatenation the default folds -- and a template
/// literal beside it, since one number bounds both.
#[test]
fn a_lowered_ceiling_refuses_what_the_default_folds() {
  assert_refuses_under(
    "",
    "content: 'abcd' + 'efghi',",
    CONCATENATION_TOO_LARGE,
    |input| fold_module_under(input, 8),
  );

  assert_refuses_under(
    "",
    "content: `abcd${'efghi'}`,",
    TEMPLATE_TOO_LARGE,
    |input| fold_module_under(input, 8),
  );
}

/// A ceiling of zero is read as unset and the default answers, the reading every
/// other ceiling in the compiler gives it -- otherwise a project could switch off
/// the folds the compiler runs to do its own work.
#[test]
fn a_configured_zero_leaves_the_default_in_place() {
  assert_refuses_under(
    &doubling_chain(600_000, 1),
    "width: a1.length,",
    "At most 1000000 characters are supported.",
    |input| fold_module_under(input, 0),
  );
}

// ──────────────────────────────────────────────
// Where exactly the line falls
// ──────────────────────────────────────────────

/// The ceiling is a length that folds, not one that refuses. Asserted at the
/// boundary in both directions and in both expressions, because an off-by-one
/// here is a build that fails on a value a project configured for.
#[test]
fn the_bound_admits_exactly_the_ceiling() {
  assert_folds_under(
    "",
    "content: 'abcd' + 'efgh',",
    ".x18j2i3r{content:\"abcdefgh\"}",
    8,
  );
  assert_folds_under(
    "",
    "content: `abcd${'efgh'}`,",
    ".x18j2i3r{content:\"abcdefgh\"}",
    8,
  );

  assert_refuses_under(
    "",
    "content: 'abcd' + 'efgh' + 'i',",
    CONCATENATION_TOO_LARGE,
    |input| fold_module_under(input, 8),
  );

  // And at the default's own scale, where the two operands are built rather
  // than written: a million characters exactly is the fold, a million and one
  // the refusal.
  assert_folds(
    "",
    "width: ('x'.repeat(1000000) + '').length,",
    ".x5kx0sl{width:1000000px}",
  );
  assert_refuses(
    "",
    "width: ('x'.repeat(999999) + 'xx').length,",
    CONCATENATION_TOO_LARGE,
  );
}

/// The length is counted in UTF-16 code units, which is the length JavaScript
/// reports and the unit every other reading of this ceiling spends. An astral
/// character occupies two of them, so a pair of them is four -- not two scalars,
/// and not the eight bytes they spell as.
#[test]
fn a_length_is_counted_in_code_units() {
  assert_folds_under(
    "",
    "content: '\u{1F600}' + '\u{1F600}',",
    ".x119eoyx{content:\"\u{1F600}\u{1F600}\"}",
    4,
  );

  assert_refuses_under(
    "",
    "content: '\u{1F600}' + '\u{1F600}',",
    CONCATENATION_TOO_LARGE,
    |input| fold_module_under(input, 3),
  );

  // Three of them is six code units, so a ceiling of four refuses the third
  // append rather than the second.
  assert_refuses_under(
    "",
    "content: '\u{1F600}' + '\u{1F600}' + '\u{1F600}',",
    CONCATENATION_TOO_LARGE,
    |input| fold_module_under(input, 4),
  );

  assert_folds(
    "",
    "content: '\u{1F600}' + '\u{1F600}' + '\u{1F600}',",
    ".x58l0r3{content:\"\u{1F600}\u{1F600}\u{1F600}\"}",
  );
}

// ──────────────────────────────────────────────
// The refusal arrives before the allocation
// ──────────────────────────────────────────────

/// A ceiling that reports after the fact is not a ceiling. The reported chain
/// spent three seconds and 102 MB reaching its answer and then had it accepted;
/// bounded at the growth it refuses at the fourth doubling, having allocated
/// eight hundred thousand characters and nothing more -- measured at 0.11s
/// against the 3.05s before.
///
/// The threshold is an order of magnitude above the measurement rather than close
/// to it, so the case tells a refusal that arrives on time from one that arrives
/// after the allocation without turning into a benchmark of the machine it runs
/// on.
#[test]
fn the_refusal_arrives_before_the_allocation() {
  let source = module(&doubling_chain(100_000, 10), "width: a10.length,");
  let started = std::time::Instant::now();

  let refusal = std::panic::catch_unwind(|| {
    stringify_js(&source, ts_syntax(), |tr| {
      theme_import_transform(tr.comments.clone())
    })
  });

  let elapsed = started.elapsed();

  assert!(refusal.is_err(), "expected the chain to refuse");
  assert!(
    elapsed < std::time::Duration::from_secs(2),
    "expected the refusal before the allocation, took {:?}",
    elapsed
  );
}
