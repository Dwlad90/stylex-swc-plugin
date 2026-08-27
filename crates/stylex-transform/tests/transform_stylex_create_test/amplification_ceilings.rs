//! What one fold may allocate, and who gets to say so.
//!
//! Two ceilings bound a fold's allocation, because the engine it runs on does
//! not: growth inside a native builtin is not a counted loop, so a mistyped
//! `'x'.repeat(200000000)` agrees with the language and reaches gigabytes of
//! resident memory. One bounds the string a fold builds or carries, the other
//! the elements and properties, because a bounded string can still become one
//! element per code unit and cost far more as a tree than it did as text.
//!
//! Three things changed here. The bound is now **arithmetic on values** rather
//! than a shape: a count is bounded by reading it, so `'x'.repeat(n)` and
//! `'x'.repeat(2 * 2)` are bounded exactly as `'x'.repeat(4)` is, and `repeat`
//! multiplies its receiver's own length into the product. Both ceilings are
//! **project options** with an environment override, on the precedence
//! `maxEvaluationDepth` already set. And the entry ceiling is now read where an
//! array's length is **declared** rather than only where the array crosses back,
//! which is what turns `Array(100000000).fill(0)` from half a minute of work
//! followed by a refusal into a refusal.
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

/// The same for the two spellings that declare an array length. Separate
/// constants rather than one, because which of the two a case refuses through is
/// half of what it is asserting.
const CANNOT_BOUND_ARRAY: &str = "Cannot bound the array 'Array' would build.";
const CANNOT_BOUND_FROM: &str = "Cannot bound the array 'Array.from' would build.";

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

// ──────────────────────────────────────────────
// A length a call declares
// ──────────────────────────────────────────────

/// `Array(n)` declares how long its array will be and allocates nothing, so the
/// cost lands on whichever call in the chain touches it — and by then the length
/// belongs to the engine rather than to the guard.
///
/// One case per touching call, because the claim is that the bound is on the
/// declaration rather than on any of them: every one of these took between twelve
/// and forty-four seconds before, each ending either in a refusal nobody had
/// waited for or in a folded value nothing had bounded. Upstream folds all but
/// the first, which is the divergence this trades for a build that answers.
#[test]
fn a_declared_length_past_the_ceiling_refuses_whatever_reads_it() {
  let calls = [
    "fill(0).length",
    "join(',').length",
    "map(x => x).length",
    "filter(x => x).length",
    "slice(0).length",
    "concat([1]).length",
    "flat().length",
    "sort().length",
    "reverse().length",
    "copyWithin(0, 1).length",
    "indexOf(1)",
    "lastIndexOf(1)",
    "includes(1)",
  ];

  for call in calls {
    assert_refuses(
      "",
      &format!("content: Array(100000000).{},", call),
      CANNOT_BOUND_ARRAY,
    );
  }

  // And where nothing touches it at all: the array itself, and a conversion that
  // used to fold because only the string it came to crossed back.
  assert_refuses("", "content: Array(100000000).length,", CANNOT_BOUND_ARRAY);
  assert_refuses("", "content: String(Array(10001)),", CANNOT_BOUND_ARRAY);

  // The length is read as a value, so a name and an expression are bounded
  // exactly as a written number is.
  assert_refuses(
    "const n = 100000000;",
    "content: Array(n).fill(0).length,",
    CANNOT_BOUND_ARRAY,
  );
  assert_refuses(
    "",
    "content: Array(10000 * 10000).fill(0).length,",
    CANNOT_BOUND_ARRAY,
  );
}

/// The claim the sentence alone cannot make: the refusal arrives *before* the
/// array exists.
///
/// This input refused before this work too, so a case asserting only the refusal
/// would have passed against the failure it was written for — thirty-four seconds
/// and four hundred megabytes of resident memory, then a diagnostic. The wall
/// clock is what tells a bound in front of the engine from one behind it.
///
/// The ceiling is generous by three orders of magnitude rather than tight, which
/// is what keeps a timing assertion out of the territory `PERFORMANCE.md` keeps
/// for `bench:verdict`. That policy exists because cross-run noise is 16-34% and
/// too coarse to read a 10-20% regression; this is not a comparison and does not
/// need to be read that close. The guard answers in milliseconds, the failure it
/// replaces took thirty-four seconds, and ten is between them by a factor of a
/// thousand on one side and three on the other. A loaded machine moves the
/// measurement by a few milliseconds, not by three seconds.
#[test]
fn a_refusal_arrives_before_the_array_is_built() {
  let at = std::time::Instant::now();

  assert_refuses(
    "",
    "content: Array(100000000).fill(0).length,",
    CANNOT_BOUND_ARRAY,
  );

  let took = at.elapsed();

  assert!(
    took < std::time::Duration::from_secs(10),
    "expected the declaration to be refused before the array was built, which took {:?}",
    took
  );
}

/// `Array.from` declares the same length one property along, off the array-like
/// it is handed.
///
/// The argument is *resolved* rather than read as syntax, which is what puts a
/// name, a spread and a coerced length under the same bound — `{ ...{ length: n
/// } }` is not a way round it, and neither is `'100000000'`, because the language
/// coerces an array-like's length where `Array(n)` does not coerce its argument.
#[test]
fn a_declared_length_on_a_from_argument_refuses() {
  let refusals = [
    ("", "content: Array.from({length: 100000000}).length,"),
    ("", "content: Array.from({'length': 100000000}).length,"),
    ("", "content: Array.from({...{length: 100000000}}).length,"),
    ("", "content: Array.from({length: '100000000'}).length,"),
    (
      "",
      "content: Array.from({length: 100000000}, (_, i) => i).length,",
    ),
    (
      "const spec = {length: 100000000};",
      "content: Array.from(spec).length,",
    ),
    // The last `length` wins, as it does in the language, so a duplicate key
    // past the ceiling is refused where it is the one the object ends up with.
    (
      "",
      "content: Array.from({length: 3, length: 100000000}).length,",
    ),
  ];

  for (decls, body) in refusals {
    assert_refuses(decls, body, CANNOT_BOUND_FROM);
  }

  // A length the language will not accept is its own throw, in either spelling:
  // `ArrayCreate` refuses it before the copy loop, so there is nothing for a
  // ceiling in front of it to save and the accurate sentence is kept. The throw
  // is reported under the method the call names, which is how every engine throw
  // is reported.
  for length in ["Infinity", "4294967296", "1e30"] {
    assert_refuses(
      "",
      &format!("content: Array.from({{length: {}}}).length,", length),
      "invalid array length",
    );
  }
}

/// A call that declares no length is untouched, which is most of what `Array`
/// is written for.
///
/// `Array` is a length only when it is handed exactly one argument that *is* a
/// number: `Array('3')` is one element holding a string, and `Array('a', 'b')` is
/// two elements the source wrote out. So the reading is the language's rather
/// than `ToNumber`'s — the opposite choice from an amplifying count, where
/// `'x'.repeat('3')` really does repeat three times.
///
/// Every row is upstream's measured answer, so each asserts agreement rather than
/// this compiler's own previous behaviour.
#[test]
fn a_call_declaring_no_length_folds() {
  let cases: &[(&str, &str, &str)] = &[
    // Elements written out, which is not a length at all.
    (
      "",
      "content: Array('a', 'b').join('-'),",
      ".x1t42mo{content:\"a-b\"}",
    ),
    (
      "",
      "content: Array('3').join('-'),",
      ".x1ih1qui{content:\"3\"}",
    ),
    ("", "content: String(Array()),", ".x14axycx{content:\"\"}"),
    ("", "content: String(Array(0)),", ".x14axycx{content:\"\"}"),
    // A length under the ceiling, written, named and computed.
    (
      "",
      "content: Array(3).fill('a').join('-'),",
      ".x1l9y9nl{content:\"a-a-a\"}",
    ),
    (
      "const n = 3;",
      "content: Array(n).fill('a').join('-'),",
      ".x1l9y9nl{content:\"a-a-a\"}",
    ),
    (
      "",
      "content: Array(Array(3).length).fill('z').join(''),",
      ".x19r2zee{content:\"zzz\"}",
    ),
    (
      "",
      "content: Array(2).fill('x').concat(Array(3).fill('y')).join(''),",
      ".xhrxpbu{content:\"xxyyy\"}",
    ),
    // `Array.from` over something that holds what it declares, which the
    // ceilings already bounded on the way in.
    (
      "",
      "content: Array.from('abc').join('-'),",
      ".xkf3utw{content:\"a-b-c\"}",
    ),
    (
      "",
      "content: Array.from([1, 2, 3]).join('-'),",
      ".x1j4v6j8{content:\"1-2-3\"}",
    ),
    (
      "",
      "content: Array.from({length: 3}, (_, i) => i).join('-'),",
      ".x1x0lxg0{content:\"0-1-2\"}",
    ),
    (
      "const spec = {length: 3};",
      "content: Array.from(spec).length,",
      ".x5kqsb8{content:\"3px\"}",
    ),
    (
      "",
      "content: Array.from({length: 2 * 2}).length,",
      ".xblpyw3{content:\"4px\"}",
    ),
    (
      "",
      "content: Array.from({'length': 3}).length,",
      ".x5kqsb8{content:\"3px\"}",
    ),
    (
      "",
      "content: Array.from({...{length: 3}}).length,",
      ".x5kqsb8{content:\"3px\"}",
    ),
    // An array-like declaring no length is the empty array, not an unbounded
    // one.
    (
      "",
      "content: String(Array.from({a: 1})),",
      ".x14axycx{content:\"\"}",
    ),
    // `ToLength` floors a fraction and reads a negative as zero, so neither is
    // a length the bound has to refuse.
    (
      "",
      "content: Array.from({length: 1.9}).length,",
      ".x1fy28pd{content:\"1px\"}",
    ),
    (
      "",
      "content: Array.from({length: -5}).length,",
      ".xxsd04i{content:\"0px\"}",
    ),
    // A coerced length under the ceiling folds too, which is the other half of
    // reading it the way the language does.
    (
      "",
      "content: Array.from({length: '3'}).length,",
      ".x5kqsb8{content:\"3px\"}",
    ),
    // The duplicate key the object ends up with is under the ceiling, so the
    // one above it is not the length anything builds.
    (
      "",
      "content: Array.from({length: 100000000, length: 3}).length,",
      ".x5kqsb8{content:\"3px\"}",
    ),
    // Right at the ceiling, on both spellings: the bound is `>` and not `>=`.
    (
      "",
      "content: Array(10000).fill('a').length,",
      ".xu5vawl{content:\"10000px\"}",
    ),
    (
      "",
      "content: Array.from({length: 10000}).length,",
      ".xu5vawl{content:\"10000px\"}",
    ),
  ];

  for (decls, body, rule) in cases {
    assert_folds(decls, body, rule);
  }
}

/// A count the language itself rejects is left to the language, so the ceiling
/// does not take an accurate sentence away and put a misleading one in its place.
///
/// None of these is a valid array length — fractional, negative, `NaN`, infinite,
/// or `2 ** 32` and up — and `Array` answers each with a `RangeError` before it
/// allocates anything. So there is nothing for a bound in front of it to save,
/// and the fold reports the throw. Upstream rejects every one in its own words.
#[test]
fn a_count_the_language_rejects_keeps_the_languages_sentence() {
  for count in ["2.5", "-1", "NaN", "Infinity", "4294967296", "1e30"] {
    assert_refuses(
      "",
      &format!("content: Array({}).length,", count),
      "Cannot fold 'Array' at compile time.",
    );
  }
}

/// A declared length inside a callback is bounded by the product of the length
/// and the receiver's element count, which is the rule an amplifying string count
/// carries in the other unit and for the same reason: a callback body runs once
/// per element, so a length written into the source declares that many arrays.
///
/// The first two fold and agree with upstream. The third is the same declaration
/// over three elements rather than two and is past the entry ceiling, so it is
/// the product being the bound rather than the length alone.
#[test]
fn a_declared_length_inside_a_callback_is_bounded_by_the_product() {
  let folds = [
    (
      "content: ['a','b'].map(x => Array(2).fill(x).join('')).join('-'),",
      ".xlpoh5y{content:\"aa-bb\"}",
    ),
    (
      "content: ['a','b'].map(x => Array.from({length: 3}).length).join('-'),",
      ".xzy23d7{content:\"3-3\"}",
    ),
  ];

  for (body, rule) in folds {
    assert_folds("", body, rule);
  }

  assert_refuses(
    "",
    "content: ['a','b','c'].map(x => Array(9999).fill(x).length).join('-'),",
    CANNOT_BOUND_ARRAY,
  );

  // A length the guard cannot read keeps the blanket refusal, and only inside a
  // callback. This is the shape that made the rule necessary rather than merely
  // consistent: the declaration arrives through a parameter, so nothing in front
  // of the engine can see it, and `[{length: 100000000}].map(x =>
  // Array.from(x).length)` folded in sixty-eight seconds while every readable
  // spelling was already refusing. Refused whatever the element count came to,
  // because it is the length that is unreadable rather than the repeats.
  let unreadable = [
    "content: [{length: 100000000}].map(x => Array.from(x).length).join('-'),",
    "content: [100000000].map(x => Array(x).fill(0).length).join('-'),",
    "content: ['ab','cd'].map(x => Array.from(x).join('')).join('-'),",
  ];

  for body in unreadable {
    assert_refuses("", body, "would build inside a callback");
  }

  assert_folds(
    "",
    "content: ['a','b'].map(x => Array(x, x).join('')).join('-'),",
    ".xlpoh5y{content:\"aa-bb\"}",
  );
}

/// The entry ceiling a declared length meets is the project's, on the option that
/// already bounds what a fold carries.
///
/// Lowered, it refuses a length the default folds; raised, it folds one the
/// default refuses. Both halves matter: the first is what an author moves to
/// make a build report sooner, and the second is what a project generating real
/// arrays needs so the bound is a ceiling rather than a wall.
#[test]
fn the_entry_ceiling_bounds_a_declared_length() {
  assert_refuses_under(
    "",
    "content: Array(10).fill('a').length,",
    "It declares a length of 10 elements, and at most 4 are supported.",
    |module| fold_under(module, 1_000_000, 4),
  );

  let output = fold_under(
    &module("", "content: Array(20000).fill('a').length,"),
    1_000_000,
    50_000,
  );

  assert!(
    output.contains(".x7ycng{content:\"20000px\"}"),
    "expected a raised entry ceiling to fold twenty thousand declared elements, got:\n{}",
    output
  );

  // And the same on the other spelling, so one option answers both.
  let output = fold_under(
    &module("", "content: Array.from({length: 20000}).length,"),
    1_000_000,
    50_000,
  );

  assert!(
    output.contains(".x7ycng{content:\"20000px\"}"),
    "expected a raised entry ceiling to fold a declared array-like length, got:\n{}",
    output
  );
}

/// The shapes nobody writes on purpose, which are the ones a guard has to answer
/// without dying.
///
/// A declaration nested inside another, a declaration reached through a chain
/// long enough to spend most of the depth budget, and a hundred declarations in
/// one module: each is refused or folded on its own merits, and the module
/// compiles either way rather than exhausting a machine on the way to an answer.
#[test]
fn an_extreme_declaration_is_answered_rather_than_built() {
  // A length declared from a length, where the inner one is past the ceiling:
  // the receiver is walked, so the innermost declaration is what refuses.
  assert_refuses(
    "",
    "content: Array(Array(100000000).length).length,",
    CANNOT_BOUND_ARRAY,
  );

  // The same nesting under the ceiling folds, so the case above is the bound
  // answering rather than the nesting.
  assert_folds(
    "",
    "content: Array(Array(3).length).fill('z').join(''),",
    ".x19r2zee{content:\"zzz\"}",
  );

  // A hundred declarations in one module, each under the ceiling, so the bound
  // is per call and the module still compiles. The total is a million elements,
  // which is what says the ceiling bounds one fold rather than the file.
  let body = (0..100)
    .map(|index| format!("k{}: {{ zIndex: Array(10000).fill(1).length }},", index))
    .collect::<Vec<_>>()
    .join("\n");

  let output = fold(&format!(
    r#"
      import * as stylex from '@stylexjs/stylex';
      export const styles = stylex.create({{
        {}
      }});
    "#,
    body
  ));

  assert!(
    output.contains(".x18ivbjn{z-index:10000}"),
    "expected a hundred bounded declarations to fold, got:\n{}",
    output
  );

  // A declaration whose length is itself a fold past the ceiling: the count
  // resolves, so the bound reads it rather than giving up.
  assert_refuses(
    "",
    "content: Array('x'.repeat(20000).length).length,",
    CANNOT_BOUND_ARRAY,
  );
}
