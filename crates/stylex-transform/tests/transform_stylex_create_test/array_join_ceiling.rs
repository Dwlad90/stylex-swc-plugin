//! The character ceiling, reaching the two joins an array performs -- the one
//! its `ToString` does, and the one `ToNumber` reaches its number through.
//!
//! `maxFoldedCharacters` bounds what a fold builds, what it carries, and -- since
//! the growth of a string became its own reading of the number -- every append
//! `+` and an interpolation make. An array arrived at that buffer as one
//! already-joined string: the coercion rendered every element and joined them,
//! and only the result was measured. So the refusal was correct about the ceiling
//! and about nothing before it -- two hundred elements of nine hundred thousand
//! characters spent 3.9 seconds on the template spelling and 7.5 on the `+` one
//! reaching a diagnostic, where upstream folds the same source in well under a
//! second.
//!
//! **Where the bound went.** Into the join, which is now written into the
//! measured buffer element by element rather than collected and handed over. The
//! other place it could have gone was a length read off the elements before they
//! render, and rendering is where the cost is: each element's `ToString` copies a
//! string the value already holds, so a bound that let the rendering happen would
//! be bounding the cheap half. Written through, the same two cases refuse in 65
//! and 92 milliseconds and no element is copied twice.
//!
//! The separator counts, because it is part of the string being built: two
//! four-character elements join to nine characters and a ceiling of eight refuses
//! them.
//!
//! **The number is the same join, and is bounded differently.** `ToNumber` of an
//! array is `ToNumber` of its join, so `+a` over the same array cost ten seconds
//! and a hundred and eighty megabytes -- and then folded to upstream's own `NaN`,
//! which a ceiling alone would have turned into a divergence. What that bridge
//! keeps is not a string but one `f64`, and its only question is whether the text
//! spells a numeric literal, so it stops reading at the first character no
//! numeric literal holds. A comma is such a character, which settles an array of
//! two or more elements at its first separator. The ceiling is left bounding a
//! text that could still be a number at a million characters. The last section
//! below is that half.
//!
//! Every folding case is measured output of `@stylexjs/babel-plugin` 0.19.0. The
//! refusals are where the two compilers part company: upstream bounds none of
//! this, so each refusal is a divergence a project can configure away by raising
//! the ceiling.

use crate::utils::{
  prelude::*,
  transform::{
    assert_folds, assert_folds_under, assert_refuses, assert_refuses_under,
    base_style_module as module, fold_module_under, stringify_js,
  },
};

/// The first line each refusal opens with. Which of the two fired is half of what
/// a case asserts, so the sentence is read rather than only the panic.
const TEMPLATE_TOO_LARGE: &str =
  "This template literal builds a string too large to evaluate at compile time.";
const CONCATENATION_TOO_LARGE: &str =
  "This concatenation builds a string too large to evaluate at compile time.";

/// An array binding of `count` copies of `name`.
fn array_of(count: usize, name: &str) -> String {
  let mut elements = String::new();

  for index in 0..count {
    if index > 0 {
      elements.push_str(", ");
    }

    elements.push_str(name);
  }

  format!("[{}]", elements)
}

/// The reported input: two hundred elements of nine hundred thousand characters,
/// which join to 180,000,199 -- a hundred and eighty times the default ceiling.
fn reported_array() -> String {
  format!(
    "const x = 'y'.repeat(900000);\nconst a = {};",
    array_of(200, "x")
  )
}

// ──────────────────────────────────────────────
// What the bound must not touch
// ──────────────────────────────────────────────

/// Every shape an array's `ToString` has an answer for, each folding to
/// upstream's own. Read as the control for the whole file: the bound rewrote the
/// join, so a case that stopped agreeing here would be the rewrite losing a rule
/// rather than the ceiling doing its work.
#[test]
fn every_join_the_coercion_answers_still_folds() {
  let cases: &[(&str, &str, &str)] = &[
    (
      "const a = ['ab', 'cd'];",
      "content: `${a}`,",
      ".x15lqp2i{content:\"ab,cd\"}",
    ),
    // A nested array joins its own elements into the outer join, so the rule has
    // to recurse without a separator appearing between the two levels.
    (
      "const b = ['ab', 'cd']; const a = [b, b];",
      "content: `${a}`,",
      ".x1duxe8t{content:\"ab,cd,ab,cd\"}",
    ),
    // Four levels of nesting around one element, which flattens to that element.
    (
      "const a = [[[['ab']]], 'cd'];",
      "content: `${a}`,",
      ".x15lqp2i{content:\"ab,cd\"}",
    ),
    // An object element takes the `Object.prototype` default.
    (
      "const o = { p: 1 }; const a = ['q', o];",
      "content: `${a}`,",
      ".x5apl4k{content:\"q,[object Object]\"}",
    ),
    // `null` and `undefined` join as nothing rather than as their spelling, so
    // the separators around them are all that is written.
    (
      "const a = ['q', null, undefined, 'r'];",
      "content: `${a}`,",
      ".xshh2ep{content:\"q,,,r\"}",
    ),
    // An empty array joins to nothing at all, and an array of empty arrays to
    // the separators between them.
    (
      "const a = [];",
      "content: `-${a}-`,",
      ".xj1ogob{content:\"--\"}",
    ),
    (
      "const a = [[], []];",
      "content: `-${a}-`,",
      ".x13mk3hp{content:\"-,-\"}",
    ),
    (
      "const a = [[[]]];",
      "content: `-${a}-`,",
      ".xj1ogob{content:\"--\"}",
    ),
    // One element writes no separator.
    (
      "const a = ['q'];",
      "content: `${a}`,",
      ".xdgu3iy{content:\"q\"}",
    ),
    // Every primitive an element can be, including the falsy list.
    (
      "const a = [1, true, false, null, 'x', 2.5];",
      "content: `${a}`,",
      ".x1ymjb0p{content:\"1,true,false,,x,2.5\"}",
    ),
    // Nesting and the empty-joining values together.
    (
      "const a = [[null, undefined], 'q'];",
      "content: `${a}`,",
      ".x1jwo435{content:\",,q\"}",
    ),
    // Astral characters survive a join that writes `str` slices rather than
    // whole strings.
    (
      "const a = ['\u{1F600}', '\u{1F600}'];",
      "content: `${a}`,",
      ".x18sljue{content:\"\u{1F600},\u{1F600}\"}",
    ),
    // Quasis on both sides of the interpolation, so the array's pieces land
    // between the template's own.
    (
      "const a = ['ab', 'cd'];",
      "content: `<${a}>`,",
      ".x1mns6pd{content:\"<ab,cd>\"}",
    ),
  ];

  for (decls, body, rule) in cases {
    assert_folds(decls, body, rule);
  }
}

/// `+` reaches the same join, on either side of the operator, and folds to the
/// same answers.
#[test]
fn a_concatenation_reaches_the_same_join() {
  let cases: &[(&str, &str, &str)] = &[
    (
      "const a = ['ab', 'cd'];",
      "content: '' + a,",
      ".x15lqp2i{content:\"ab,cd\"}",
    ),
    (
      "const a = ['ab', 'cd'];",
      "content: a + '!',",
      ".xrs0elg{content:\"ab,cd!\"}",
    ),
    // Both operands are arrays, so one buffer carries two joins -- and neither
    // join writes a separator, since each holds one element.
    (
      "const a = ['ab']; const b = ['cd'];",
      "content: a + b,",
      ".x1ncg6ds{content:\"abcd\"}",
    ),
  ];

  for (decls, body, rule) in cases {
    assert_folds(decls, body, rule);
  }
}

/// The left operand is measured as well as the right, which is the one thing
/// `+` changed beyond the join: the left used to arrive as a finished string and
/// be adopted as the buffer's own allocation, and an array on that side had
/// therefore joined in full before anything looked at it.
///
/// Both cases agreed with what this compiler did before -- measured, since the
/// adoption skipped the left's own measurement and only the *sum* was ever
/// refused, which is the same answer read a different way. A long operand at
/// exactly the ceiling still folds, and an array past it refuses on the left as
/// it does on the right.
#[test]
fn a_left_operand_is_measured_as_the_right_one_is() {
  assert_folds(
    "const a0 = 'x'.repeat(1000000);",
    "width: (a0 + '').length,",
    ".x5kx0sl{width:1000000px}",
  );

  assert_refuses(
    "const x = 'y'.repeat(600000);\nconst a = [x, x];",
    "width: (a + '').length,",
    CONCATENATION_TOO_LARGE,
  );
}

/// A long array well inside the ceiling is not a refusal, however many elements
/// it has -- which is the case a ceiling read on the element count rather than on
/// the length would have refused.
#[test]
fn an_array_inside_the_ceiling_folds_however_many_elements_it_has() {
  assert_folds(
    &format!("const a = {};", array_of(50, "'z'")),
    "width: `${a}`.length,",
    ".xgf8fm{width:99px}",
  );

  // A thousand elements of nine hundred and ninety-nine characters is 999,000
  // written characters and 999 separators: 999,999, one inside the default.
  assert_folds(
    &format!(
      "const z = 'y'.repeat(999);\nconst a = {};",
      array_of(1000, "z")
    ),
    "width: `${a}`.length,",
    ".xc346c6{width:999999px}",
  );
}

// ──────────────────────────────────────────────
// The join the ticket is about
// ──────────────────────────────────────────────

/// The reported shape, in both spellings. Both refused before this change too --
/// the ceiling was right about the number -- so what each case asserts is the
/// sentence, and the timing case below asserts the rest.
#[test]
fn a_join_past_the_ceiling_refuses_under_the_ceiling_s_own_name() {
  let decls = reported_array();

  assert_refuses(&decls, "width: `${a}`.length,", TEMPLATE_TOO_LARGE);
  assert_refuses(&decls, "width: ('' + a).length,", CONCATENATION_TOO_LARGE);

  // Two elements are enough once they pass the ceiling between them, so the
  // refusal does not depend on an array being long.
  assert_refuses(
    "const x = 'y'.repeat(600000);\nconst a = [x, x];",
    "width: `${a}`.length,",
    TEMPLATE_TOO_LARGE,
  );

  // Nesting reaches the ceiling by the same route, since an inner array's
  // elements are written into the same buffer.
  assert_refuses(
    "const x = 'y'.repeat(600000);\nconst b = [x, x];\nconst a = [b, b];",
    "width: `${a}`.length,",
    TEMPLATE_TOO_LARGE,
  );

  // An array written where it is used rather than bound to a name reaches the
  // same join, which is why the bound is on the growth rather than on what a
  // binding may hold.
  assert_refuses(
    "const x = 'y'.repeat(600000);",
    "width: `${[x, x]}`.length,",
    TEMPLATE_TOO_LARGE,
  );
}

/// An element with no string form at all is the other way a join ends, and it
/// keeps its own sentence -- a function's `ToString` is its source text, which
/// this compiler does not retain. Asserted here because the ceiling rewrote this
/// path as well, and a ceiling sentence appearing for it would mean the two
/// refusals had been folded into one.
#[test]
fn an_element_with_no_string_form_keeps_its_own_refusal() {
  let sentence = "Expected a string value but received a non-string expression.";

  assert_refuses(
    "const f = () => 1;\nconst a = ['q', f];",
    "content: `${a}`,",
    sentence,
  );

  // Nested one level down, so the refusal travels back out of the inner join.
  assert_refuses(
    "const f = () => 1;\nconst a = [['q', f]];",
    "content: `${a}`,",
    sentence,
  );
}

// ──────────────────────────────────────────────
// Where exactly the line falls
// ──────────────────────────────────────────────

/// The separator is part of the string being built, so it is part of what the
/// ceiling bounds. Two four-character elements are nine characters together: a
/// ceiling of nine folds them and a ceiling of eight refuses.
#[test]
fn the_separator_counts_toward_the_ceiling() {
  assert_folds_under(
    "const a = ['abcd', 'efgh'];",
    "width: `${a}`.length,",
    ".x1wc42o8{width:9px}",
    9,
  );

  assert_refuses_under(
    "const a = ['abcd', 'efgh'];",
    "width: `${a}`.length,",
    TEMPLATE_TOO_LARGE,
    |input| fold_module_under(input, 8),
  );
}

/// The length is counted in UTF-16 code units, the unit every other reading of
/// this number spends. Two astral characters either side of a separator is five.
#[test]
fn a_length_is_counted_in_code_units() {
  assert_folds_under(
    "const a = ['\u{1F600}', '\u{1F600}'];",
    "content: `${a}`,",
    ".x18sljue{content:\"\u{1F600},\u{1F600}\"}",
    5,
  );

  assert_refuses_under(
    "const a = ['\u{1F600}', '\u{1F600}'];",
    "content: `${a}`,",
    TEMPLATE_TOO_LARGE,
    |input| fold_module_under(input, 4),
  );
}

/// Raised, the array the default refuses folds -- to upstream's own answer, so
/// this is the divergence closing rather than a behaviour of this compiler's own.
#[test]
fn a_raised_ceiling_folds_the_array_the_default_refuses() {
  assert_folds_under(
    "const x = 'y'.repeat(600000);\nconst a = [x, x];",
    "width: `${a}`.length,",
    ".xmzhmha{width:1200001px}",
    4_000_000,
  );
}

/// A ceiling of zero is read as unset and the default answers, the reading every
/// other ceiling in the compiler gives it.
#[test]
fn a_configured_zero_leaves_the_default_in_place() {
  assert_refuses_under(
    "const x = 'y'.repeat(600000);\nconst a = [x, x];",
    "width: `${a}`.length,",
    "At most 1000000 characters are supported.",
    |input| fold_module_under(input, 0),
  );
}

// ──────────────────────────────────────────────
// The refusal arrives before the allocation
// ──────────────────────────────────────────────

/// A ceiling that reports after the fact is not a ceiling. The reported array
/// spent 3.9 seconds on the template spelling and 7.5 on the `+` one before
/// reaching a diagnostic; measured element by element it refuses in 65 and 92
/// milliseconds, having written the two elements that crossed the ceiling and
/// nothing more.
///
/// The threshold is an order of magnitude above the measurement rather than close
/// to it, so the case tells a refusal that arrives on time from one that arrives
/// after the allocation without turning into a benchmark of the machine it runs
/// on.
#[test]
fn the_refusal_arrives_before_the_join_allocates() {
  let decls = reported_array();

  for body in ["width: `${a}`.length,", "width: ('' + a).length,"] {
    let source = module(&decls, body);
    let started = std::time::Instant::now();

    let refusal = std::panic::catch_unwind(|| {
      stringify_js(&source, ts_syntax(), |tr| {
        theme_import_transform(tr.comments.clone())
      })
    });

    let elapsed = started.elapsed();

    assert!(refusal.is_err(), "expected `{}` to refuse", body);
    assert!(
      elapsed < std::time::Duration::from_secs(1),
      "expected `{}` to refuse before the join allocated, took {:?}",
      body,
      elapsed
    );
  }
}

// ──────────────────────────────────────────────
// The join a number reaches through
// ──────────────────────────────────────────────
//
// `ToNumber` of an array is `ToNumber` of the same join, and the number bridge
// collected it: `+a` over the reported array folded to upstream's own `NaN`
// after ten seconds and a hundred and eighty megabytes of string. The right
// answer, bought in full.
//
// A ceiling alone would have made it a refusal where upstream folds, so the
// bridge reads what the ceiling bounds instead. `ToNumber` never keeps the text
// it reads -- its answer is one `f64` however wide the string was -- and its
// only question is whether the text spells a numeric literal. So the sink drops
// the text at the first character no numeric literal holds, and a comma is such
// a character: an array of two or more elements is settled at the separator
// after its first, before the second renders. What the ceiling is left bounding
// is a text that really could still be a number at a million characters, which
// is a refusal a project can configure away.

/// The number ceiling's own refusal, which names the conversion an author wrote
/// rather than the join inside it.
const CONVERSION_TOO_LARGE: &str =
  "This numeric conversion builds a string too large to evaluate at compile time.";

/// Every number the coercion answers, each folding to upstream's own. The
/// control for this section, in the way the join's own control is: the bridge
/// was rewritten, so a case that stopped agreeing here would be the rewrite
/// losing a rule rather than the bound doing its work.
#[test]
fn every_number_the_coercion_answers_still_folds() {
  for (decls, body, rule) in [
    // An empty array joins to nothing, which is zero rather than `NaN`.
    ("", "width: +[],", ".xnalus7{width:0}"),
    ("", "width: +[5],", ".x1ftt334{width:5px}"),
    // A nested array reaches its number through its own join, at any depth.
    ("", "width: +[[7]],", ".xci0xqf{width:7px}"),
    ("", "width: +[[[9]]],", ".x1wc42o8{width:9px}"),
    // The radix prefixes, which `f64::from_str` rejects and the language does
    // not.
    ("", "width: +['0x10'],", ".x1kky2od{width:16px}"),
    ("", "width: +['0o17'],", ".x1a00udw{width:15px}"),
    ("", "width: +['0b101'],", ".x1ftt334{width:5px}"),
    // Surrounding whitespace is trimmed, and an exponent and a sign are read.
    ("", "width: +[' 5 '],", ".x1ftt334{width:5px}"),
    ("", "width: +['1e2'],", ".x1exxlbk{width:100px}"),
    ("", "width: +['+5'],", ".x1ftt334{width:5px}"),
    ("", "width: +['Infinity'],", ".x1fssspx{width:Infinitypx}"),
    // The values that join as nothing, so a one-element array of one is zero.
    ("", "width: +[''],", ".xnalus7{width:0}"),
    ("", "width: +[null],", ".xnalus7{width:0}"),
    ("", "width: +[undefined],", ".xnalus7{width:0}"),
    // `NaN` is a value rather than a refusal, and each of these spells one.
    ("", "width: +[1, 2],", ".x1c9rq88{width:NaNpx}"),
    ("", "width: +['a'],", ".x1c9rq88{width:NaNpx}"),
    ("", "width: +[true],", ".x1c9rq88{width:NaNpx}"),
    // A numeric separator is a numeric *literal*'s and not a string's.
    ("", "width: +['1_0'],", ".x1c9rq88{width:NaNpx}"),
    // A function has a number even though it has no string, so one inside an
    // array does not make the array's number unknowable.
    ("", "width: +[() => 1],", ".x1c9rq88{width:NaNpx}"),
    // An object takes the prototype default unless it owns a `valueOf`.
    ("", "width: +({}),", ".x1c9rq88{width:NaNpx}"),
    ("", "width: +({ valueOf: () => 2 }),", ".xfo62xy{width:2px}"),
    // The other two operators that read the same bridge.
    ("", "width: -[3],", ".x1k09fpp{width:-3px}"),
    ("", "width: ~[1],", ".xxq3dvr{width:-2px}"),
    // Through a name, which is the spelling that reaches the evaluator's own
    // list rather than the literal.
    ("const a = [8];", "width: +a,", ".x1xc55vz{width:8px}"),
  ] {
    assert_folds(decls, body, rule);
  }
}

/// The reported shape: two hundred elements of nine hundred thousand characters
/// under `+`, which upstream folds to `NaN` and so does this -- now without
/// materialising a hundred and eighty megabytes to get there.
#[test]
fn the_reported_array_reaches_upstream_s_answer_without_the_join() {
  assert_folds(&reported_array(), "width: +a,", ".x1c9rq88{width:NaNpx}");
}

/// The comma settles the answer, which is what keeps the two compilers agreeing
/// where a bound alone would have parted them: under a ceiling that fits the
/// first element and not the join, the array still folds to `NaN`.
#[test]
fn a_separator_settles_the_answer_before_the_next_element_renders() {
  assert_folds_under(
    "const a = ['1234', '5678'];",
    "width: +a,",
    ".x1c9rq88{width:NaNpx}",
    4,
  );

  // One character short of the first element, so the ceiling is reached before
  // the separator can settle anything.
  assert_refuses_under(
    "const a = ['1234', '5678'];",
    "width: +a,",
    CONVERSION_TOO_LARGE,
    |input| fold_module_under(input, 3),
  );
}

/// A text that really could still be a number is what the ceiling is left
/// bounding, and the refusal names `maxFoldedCharacters` as every other reading
/// of it does.
#[test]
fn a_numeric_text_past_the_ceiling_refuses_under_the_ceiling_s_own_name() {
  assert_refuses_under(
    "const a = ['123456789'];",
    "width: +a,",
    CONVERSION_TOO_LARGE,
    |input| fold_module_under(input, 8),
  );

  assert_refuses_under(
    "const a = ['123456789'];",
    "width: +a,",
    "At most 8 characters are supported.",
    |input| fold_module_under(input, 8),
  );
}

/// Raised past the text, the same source folds to upstream's own answer -- so
/// the refusal above is the configurable kind rather than a gap.
#[test]
fn a_raised_ceiling_folds_the_number_the_default_refuses() {
  assert_folds_under(
    "const a = ['123456789'];",
    "width: +a,",
    ".x1gm1p2f{width:123456789px}",
    9,
  );
}

/// A ceiling of zero is read as unset here too, and the default answers.
#[test]
fn a_configured_zero_leaves_the_number_ceiling_at_its_default() {
  // Written out rather than built by a call, so what the ceiling reads is the
  // text itself and not a length an amplifying call declares.
  let decls = format!("const a = ['{}'];", "1".repeat(1_000_001));

  assert_refuses_under(
    &decls,
    "width: +a,",
    "At most 1000000 characters are supported.",
    |input| fold_module_under(input, 0),
  );
}

/// An element with no string form keeps its own refusal on this path as well,
/// so the number bridge did not fold two refusals into one. A function is not
/// that case -- it has a number even though it has no string -- so this is the
/// lone surrogate, which Rust has no `str` for.
#[test]
fn a_number_through_an_unreadable_element_keeps_its_own_refusal() {
  assert_refuses(
    "const a = ['\\uD800'];",
    "width: +a,",
    "A style value can only contain an array, string or number.",
  );

  // And an element that never evaluated keeps the resolution's own sentence,
  // which arrives before the coercion is asked anything.
  assert_refuses(
    "const a = [q];",
    "width: +a,",
    "Referenced constant is not defined.",
  );
}

/// The answer arrives before the join allocates. Measured, the reported array
/// took 10.2 seconds through the number bridge before this change and 104
/// milliseconds after it; the threshold sits between the two by an order of
/// magnitude either way, so a loaded machine does not fail the case and a
/// materialised join cannot pass it.
#[test]
fn the_number_arrives_before_the_join_allocates() {
  let source = module(&reported_array(), "width: +a,");
  let started = std::time::Instant::now();

  let output = stringify_js(&source, ts_syntax(), |tr| {
    theme_import_transform(tr.comments.clone())
  });

  let elapsed = started.elapsed();

  assert!(
    output.contains(".x1c9rq88{width:NaNpx}"),
    "expected the reported array to fold to `NaN`, got:\n{}",
    output
  );
  assert!(
    elapsed < std::time::Duration::from_secs(3),
    "expected the number to be reached before the join allocated, took {:?}",
    elapsed
  );
}
