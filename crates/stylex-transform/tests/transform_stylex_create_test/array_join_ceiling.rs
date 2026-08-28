//! The character ceiling, reaching the join an array's `ToString` performs.
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
