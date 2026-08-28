//! A callback that knows how many times it runs.
//!
//! Both amplification rules used to give up in the same place and for the same
//! reason. A length written into a callback body bounds one evaluation, and the
//! body is evaluated once per element of a receiver the guard had nowhere to put
//! a count for -- so the answer was a blanket refusal whatever the source said,
//! and `['a','b'].map(x => x.repeat(3))` was refused over a product of six.
//!
//! The count is now carried. `admit_call` walks the receiver before the
//! arguments, so by the time the walk descends into a callback that receiver has
//! been admitted and, where it is a literal array or a resolved name, counted --
//! and both rules become the arithmetic they already were one level up: the
//! receiver's elements times the length the source declares, against the
//! ceiling that unit spends.
//!
//! **Two things ride on the same measurement.** How often the body runs, and how
//! wide the element it is handed is -- because `x.repeat(3)` needs `x`'s own
//! length and `x` is a name no module can resolve. Both come off one reading of
//! one receiver, so they cannot come to disagree.
//!
//! **What keeps its refusal is the honest remainder.** A receiver whose element
//! count cannot be read: a call, whose answer is bounded per link so reading it
//! is what would let two allowed counts multiply. A method whose callback does
//! not run once per element, or is not handed the element first. A name a *block*
//! declared, which holds whatever the body built rather than an element. And a
//! declared length the guard cannot read at all, which is unreadable in the
//! length rather than in the repeats.
//!
//! Every folding case below is measured output of `@stylexjs/babel-plugin` 0.19.0
//! under the same options, so each asserts agreement with the reference compiler
//! rather than with this compiler's own previous answer.

use crate::utils::{
  prelude::*,
  transform::{assert_folds, assert_refuses, base_style_module as module, stringify_js},
};

/// The first line the string rule's two refusals open with, so a case cannot be
/// satisfied by some later, unrelated rule firing.
const CANNOT_BOUND: &str = "Cannot bound the string 'repeat' would build.";

/// The line the blanket refusal opens with, which is the one the remainder keeps.
const UNMEASURED: &str = "would build inside a callback";

/// Compile with the two allocation ceilings set, the way an author moves them.
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
// The shapes the ticket is about
// ──────────────────────────────────────────────

/// The two examples the ticket names. Each is a small array whose element count
/// is written out one call away, and the product -- two elements times a length
/// of two or three -- is four orders of magnitude inside either ceiling.
#[test]
fn a_bounded_product_inside_a_callback_folds() {
  assert_folds(
    "",
    "content: ['a','b'].map(x => x.repeat(3)).join('-'),",
    ".x3d7avo{content:\"aaa-bbb\"}",
  );

  assert_folds(
    "",
    "content: ['a','b'].map(x => Array(2).fill(x).join('')).join('-'),",
    ".xlpoh5y{content:\"aa-bb\"}",
  );
}

/// The same on the other spelling of a declared length, and on the two padding
/// methods, so the rule is the arithmetic rather than one method's special case.
#[test]
fn every_amplifying_spelling_inside_a_callback_folds() {
  let cases = [
    (
      "content: ['a','b'].map(x => Array.from({length: 3}).length).join('-'),",
      ".xzy23d7{content:\"3-3\"}",
    ),
    (
      "content: ['1','22'].map(x => x.padStart(4, '0')).join('-'),",
      ".x17vu0lq{content:\"0001-0022\"}",
    ),
    (
      "content: ['1'].map(x => x.padEnd(2, '0')).join(''),",
      ".x17uujby{content:\"10\"}",
    ),
  ];

  for (body, rule) in cases {
    assert_folds("", body, rule);
  }
}

/// A receiver reached through a name is counted as the literal it holds, which is
/// the same claim the rest of this guard makes about every other position: giving
/// a value a name does not change whether the call on it folds.
#[test]
fn a_named_receiver_is_counted() {
  assert_folds(
    "const parts = ['aa','bb'];",
    "content: parts.map(x => x.repeat(2)).join('-'),",
    ".x1idind9{content:\"aaaa-bbbb\"}",
  );
}

/// Every method whose callback runs once per element, so the list is asserted
/// rather than trusted.
#[test]
fn each_per_element_method_counts_its_receiver() {
  let cases = [
    (
      "content: ['a','b'].flatMap(x => [x.repeat(2)]).join('-'),",
      ".xlpoh5y{content:\"aa-bb\"}",
    ),
    (
      "content: ['a','b'].filter(x => x.repeat(2) !== 'zz').join('-'),",
      ".x1t42mo{content:\"a-b\"}",
    ),
    (
      "content: (['a','b'].some(x => x.repeat(2) === 'aa') ? 'yes' : 'no'),",
      ".xs2tsc8{content:\"yes\"}",
    ),
    (
      "content: (['a','b'].every(x => x.repeat(2).length === 2) ? 'y' : 'n'),",
      ".x1t2hdmn{content:\"y\"}",
    ),
    (
      "content: ['a','bb'].find(x => x.repeat(2).length === 4),",
      ".xxmvk1m{content:\"bb\"}",
    ),
    (
      "content: ['a','bb'].findLast(x => x.repeat(2).length === 2),",
      ".x16319ns{content:\"a\"}",
    ),
  ];

  for (body, rule) in cases {
    assert_folds("", body, rule);
  }
}

// ──────────────────────────────────────────────
// The product is a product
// ──────────────────────────────────────────────

/// Neither factor alone reaches the ceiling and together they pass it, which is
/// the whole of what carrying the count buys over reading the argument.
#[test]
fn the_product_refuses_what_neither_factor_would() {
  // Two elements, two characters each, two hundred thousand copies: 800000 is
  // inside the default million and one more element is not.
  assert_folds(
    "",
    "content: ['ab','cd'].map(x => x.repeat(200000)).join('').length,",
    ".xsijj8u{content:\"800000px\"}",
  );

  assert_refuses(
    "",
    "content: ['ab','cd','ef'].map(x => x.repeat(200000)).join('').length,",
    CANNOT_BOUND,
  );

  // The same on the entry ceiling: a declaration of 9999 elements is one under
  // the default, and three of them are three times past it.
  assert_folds(
    "",
    "content: ['a'].map(x => Array(9999).fill(x).length).join('-'),",
    ".xtnz4dz{content:\"9999\"}",
  );

  assert_refuses(
    "",
    "content: ['a','b','c'].map(x => Array(9999).fill(x).length).join('-'),",
    "Cannot bound the array 'Array' would build.",
  );
}

/// The refusal names the product, because a count of 9999 against a limit of
/// 10000 reads as being inside it and the number that is not appears nowhere in
/// what the author wrote.
#[test]
fn the_refusal_names_the_evaluations_and_the_total() {
  assert_refuses(
    "",
    "content: ['a','b','c'].map(x => Array(9999).fill(x).length).join('-'),",
    "3 evaluations and 29997 elements in all",
  );

  assert_refuses(
    "",
    "content: ['ab','cd','ef'].map(x => x.repeat(200000)).join(''),",
    "3 evaluations and 1200000 characters in all",
  );

  // Outside a callback there is one evaluation and nothing to say about it, so
  // the clause the two cases above assert is not there to confuse the reading.
  assert_refuses(
    "",
    "content: 'x'.repeat(1000001),",
    "It asks for 1000001 characters, and at most 1000000 are supported.",
  );
}

/// A callback inside a callback multiplies rather than resets, so the bound is
/// the product of both receivers.
#[test]
fn nesting_multiplies_both_receivers() {
  assert_folds(
    "",
    "content: ['aa','bb'].map(x => ['c','d'].map(y => x.repeat(2)).join('')).join('-'),",
    ".x1l2nazr{content:\"aaaaaaaa-bbbbbbbb\"}",
  );

  // Two receivers of two elements each, and a width of two: 250000 copies comes
  // to exactly the default million, and a third element on either receiver is
  // past it. The outer one is the one that would reset if nesting replaced the
  // count rather than multiplying it.
  assert_folds(
    "",
    "content: ['a','b'].map(x => ['c','d'].map(y => x.repeat(250000)).join('')).join('').length,",
    ".x1va9z7q{content:\"1000000px\"}",
  );

  assert_refuses(
    "",
    "content: ['a','b','c'].map(x => ['c','d'].map(y => x.repeat(250000)).join('')).join(''),",
    CANNOT_BOUND,
  );

  assert_refuses(
    "",
    "content: ['a','b'].map(x => ['c','d','e'].map(y => x.repeat(250000)).join('')).join(''),",
    CANNOT_BOUND,
  );
}

/// A receiver holding no elements runs its callback no times, so the product is
/// zero and a count nothing could ever build folds.
#[test]
fn an_empty_receiver_bounds_any_count_at_all() {
  assert_folds(
    "",
    "content: [].map(x => x.repeat(999999999)).join('') || 'red',",
    ".x1f1gxam{content:\"red\"}",
  );
}

// ──────────────────────────────────────────────
// Which name holds an element
// ──────────────────────────────────────────────

/// The width is the widest element rather than the first, because which element
/// a parameter will hold is not something the guard chooses.
#[test]
fn the_width_is_the_widest_element() {
  assert_folds(
    "",
    "content: ['a','bbbb'].map(x => x.repeat(2)).join('-'),",
    ".x1un9bdt{content:\"aa-bbbbbbbb\"}",
  );

  // Counted in UTF-16 code units, which is the unit the engine's own strings are
  // measured in: one astral character is two of them, so a width of two is the
  // one the engine will build.
  assert_folds(
    "",
    "content: ['\\u{1F600}'].map(x => x.repeat(2)).join('-'),",
    ".x119eoyx{content:\"\u{1F600}\u{1F600}\"}",
  );
}

/// The parameter after the element is its index, whose value the same reading of
/// the same receiver settles: the largest index an array of two has is one. What
/// comes after that is the receiver itself, which nothing bounded.
#[test]
fn the_index_parameter_carries_the_count_the_element_did() {
  assert_folds(
    "",
    "content: ['a','b'].map((x, i) => x.padStart(i + 2, '0')).join('-'),",
    ".xmtou9f{content:\"0a-00b\"}",
  );

  // A product of the index rather than a sum, so the arithmetic is read rather
  // than one operator's special case.
  assert_folds(
    "",
    "content: ['a','b','c'].map((x, i) => x.repeat(i * 2)).join('-'),",
    ".x14kdrf{content:\"-bb-cccc\"}",
  );

  assert_refuses(
    "",
    "content: ['a','b'].map((x, i, all) => all.repeat(2)).join('-'),",
    CANNOT_BOUND,
  );

  // What those parameters *are* still crosses, so the callback itself folds --
  // it is the width that is missing rather than the binding.
  assert_folds(
    "",
    "content: ['a','b'].map((x, i, all) => all.length + x.repeat(2)).join('-'),",
    ".xerxmdk{content:\"2aa-2bb\"}",
  );
}

/// A first parameter that destructures binds every name it introduces to a part
/// of the element, and a part is no wider than the element it came out of.
#[test]
fn a_destructured_first_parameter_carries_the_width() {
  assert_folds(
    "",
    "content: [['a','b']].map(([p, q]) => p.repeat(2) + q).join('-'),",
    ".xklrm62{content:\"aab\"}",
  );
}

/// A parameter shadowing a module name is read as the element, exactly as the
/// language reads it -- and a module name the callback does *not* shadow is
/// resolved rather than bounded by an element's width.
#[test]
fn shadowing_decides_which_length_is_read() {
  assert_folds(
    "const x = 'zzzz';",
    "content: ['a','b'].map(x => x.repeat(2)).join('-'),",
    ".xlpoh5y{content:\"aa-bb\"}",
  );

  assert_folds(
    "const x = 'zzzz';",
    "content: ['a','b'].map(y => x.repeat(2)).join('-'),",
    ".xmg60u5{content:\"zzzzzzzz-zzzzzzzz\"}",
  );
}

/// The one shape that could defeat the bound, pinned because what stops it lives
/// in another subsystem.
///
/// The module here declares one element under the same spelling the callback
/// binds ten thousand under. Counting the module's `parts` where the call is made
/// on the callback's would bound `'z'.repeat(10000)` at one evaluation instead of
/// ten thousand — inside the ceiling, against a hundred million characters really
/// built.
///
/// It cannot happen, and not because the guard checks: the evaluator resolves a
/// reference by the full SWC `Id`, symbol *and* `SyntaxContext`, so the parameter
/// and the module binding are different keys and the parameter's holds no
/// initializer. That is a property of the resolver, so this passed before the
/// element count was carried and would pass with the count read carelessly. What
/// it is here to do is fail if hygiene ever stops holding, which is the only
/// warning this rule would get.
#[test]
fn a_receiver_the_callback_binds_is_never_counted_from_the_module() {
  let elements = (0..10_000)
    .map(|index| format!("'e{}'", index))
    .collect::<Vec<_>>()
    .join(",");

  for receiver in ["parts", "parts.slice()"] {
    assert_refuses(
      "const parts = ['q'];",
      &format!(
        "content: [[{}]].map(parts => {}.map(y => 'z'.repeat(10000)).length).join('-'),",
        elements, receiver
      ),
      UNMEASURED,
    );
  }
}

/// A spread in the receiver stands for however many elements its operand holds,
/// so the written length is not the count — and a count read short is the one
/// reading that admits a call nothing bounded.
///
/// Refused where the receiver is written with one, and refused a second way where
/// it reaches the guard as a literal at all: the evaluator refuses a spread in a
/// value position before this rule is asked, and the count rule refuses it again
/// rather than trusting that ordering.
#[test]
fn a_spread_in_the_receiver_is_not_a_written_count() {
  assert_refuses(
    "const parts = ['a','b'];",
    "content: [...parts].map(x => x.repeat(2)).join('-'),",
    "SpreadElement",
  );
}

/// A name a *block* declares holds whatever the body built, which may be wider
/// than any element -- `y` here is twice as long as one -- so it carries no
/// width and the length rule finds nothing to read.
#[test]
fn a_block_declaration_carries_no_element_width() {
  assert_refuses(
    "",
    "content: ['a','b'].map(x => { const y = x + x; return y.repeat(2); }).join('-'),",
    CANNOT_BOUND,
  );

  // A length written out beside it is still bounded by the product, so what the
  // block costs is the width and not the count. Upstream refuses the whole call
  // here -- a block body is a shape its evaluator does not read -- so the class
  // name is this compiler's own.
  assert_folds(
    "",
    "content: ['a','b'].map(x => { const y = x + x; return 'z'.repeat(2) + y; }).join('-'),",
    ".x1k6budk{content:\"zzaa-zzbb\"}",
  );
}

/// A parameter with a default may be handed something else entirely, so a
/// defaulted parameter list takes neither the width nor the count. Upstream
/// refuses the whole call for the same parameter shape.
#[test]
fn a_defaulted_parameter_list_takes_neither() {
  assert_refuses(
    "",
    "content: ['a'].map((x = 'zz') => x.repeat(2)).join('-'),",
    CANNOT_BOUND,
  );
}

/// An element the guard cannot read a width for gives up on all of them, and the
/// call is left to the language rather than refused by a number: an object has no
/// width its contents decide, and `(1).repeat` is a `TypeError` wherever it is
/// written. Upstream refuses both.
#[test]
fn an_element_with_no_readable_width_leaves_the_sentence_to_the_language() {
  assert_refuses(
    "",
    "content: [{a:1}].map(x => x.repeat(3)).join('-'),",
    CANNOT_BOUND,
  );

  assert_refuses(
    "",
    "content: ['aaa', 1].map(x => x.repeat(3)).join('-'),",
    "TypeError",
  );

  // And an unreadable element costs nothing where no name is the receiver: the
  // count is still the element count, and the width was never asked for.
  assert_folds(
    "",
    "content: [{a:1}].map(x => 'y'.repeat(3)).join('-'),",
    ".x1chfc3u{content:\"yyy\"}",
  );
}

// ──────────────────────────────────────────────
// The remainder that keeps its refusal
// ──────────────────────────────────────────────

/// A comparator is the one callback the language runs more often than its
/// receiver is long, so no element count bounds it and the blanket refusal is
/// what it keeps. Upstream folds both, which is what the exception costs.
#[test]
fn a_comparator_keeps_the_blanket_refusal() {
  // A `repeat` on a parameter refuses on the width, one rule ahead of the
  // repeats, because an element of a receiver nothing measured has none.
  assert_refuses(
    "",
    "content: ['b','a'].sort((p, q) => p.repeat(2) < q.repeat(2) ? -1 : 1).join('-'),",
    CANNOT_BOUND,
  );

  // And a length the source states reaches the blanket refusal, which is where
  // the uncounted repeats are the whole of the reason.
  assert_refuses(
    "",
    "content: ['b','a'].sort((p, q) => Array(2).fill(p).length - q.length).join('-'),",
    UNMEASURED,
  );
}

/// A receiver the guard cannot resolve at all -- a spread element, or an array
/// carrying a hole -- is refused where it stands, before any count is asked for.
#[test]
fn a_receiver_the_guard_cannot_resolve_refuses_where_it_stands() {
  assert_refuses(
    "",
    "content: ['a', , 'b'].map(x => 'y'.repeat(2)).join('-'),",
    "Could not resolve the code being evaluated.",
  );
}

/// A declared length the guard cannot read is refused inside a callback whatever
/// the element count came to, because it is the *length* that is unreadable
/// rather than the repeats: a receiver of one element still declares an array of
/// a hundred million, which folded in sixty-eight seconds before this rule.
#[test]
fn an_unreadable_declared_length_refuses_however_few_the_elements() {
  for body in [
    "content: [{length: 100000000}].map(x => Array.from(x).length).join('-'),",
    "content: [100000000].map(x => Array(x).fill(0).length).join('-'),",
  ] {
    assert_refuses("", body, UNMEASURED);
  }
}

// ──────────────────────────────────────────────
// The ceilings are still the project's
// ──────────────────────────────────────────────

/// The product meets the project's ceilings rather than numbers written in here,
/// so a project generating real values can raise the bound and one that wants to
/// hear sooner can lower it.
#[test]
fn the_product_is_compared_to_the_configured_ceilings() {
  // Lowered, the character ceiling refuses a product the default folds.
  let refused = module("", "content: ['ab','cd'].map(x => x.repeat(3)).join('-'),");
  let refusal = std::panic::catch_unwind(|| fold_under(&refused, 11, 10_000));

  assert!(
    refusal.is_err(),
    "expected a product of twelve characters to refuse under a ceiling of eleven"
  );

  // Raised, it folds one the default refuses, which is what makes the bound a
  // ceiling rather than a wall.
  let folded = module(
    "",
    "content: ['ab','cd'].map(x => x.repeat(600000)).join('').length,",
  );

  assert!(
    fold_under(&folded, 2_400_000, 10_000).contains("2400000px"),
    "expected a raised character ceiling to fold a product past the default"
  );

  // And the same on the entry ceiling, which the declared length spends.
  let entries = module(
    "",
    "content: ['a','b'].map(x => Array(9999).fill(x).length).join('-'),",
  );

  assert!(
    fold_under(&entries, 1_000_000, 20_000).contains("9999-9999"),
    "expected a raised entry ceiling to fold a declared length past the default"
  );
}

// ──────────────────────────────────────────────
// Inputs at and past the shapes an author writes
// ──────────────────────────────────────────────

/// A receiver of two hundred elements is counted like one of two, and folds to
/// the length upstream reaches -- so the count is read rather than the shape
/// being recognised.
#[test]
fn a_large_receiver_is_counted_rather_than_refused() {
  let elements = (0..200)
    .map(|index| format!("'e{}'", index))
    .collect::<Vec<_>>()
    .join(",");

  assert_folds(
    "",
    &format!(
      "content: [{}].map(x => x.repeat(2)).join('').length,",
      elements
    ),
    ".xsilc6x{content:\"1380px\"}",
  );
}

/// A product past what a `u64` holds saturates rather than wrapping, because a
/// wrapped product would come out small and admit the very call the bound exists
/// to refuse.
#[test]
fn a_product_past_the_integer_range_saturates_into_a_refusal() {
  // The largest length the language accepts, twice: past `maxFoldedEntries` by
  // six orders of magnitude and nowhere near wrapping, which is the readable
  // half of the pair.
  assert_refuses(
    "",
    "content: ['a','b'].map(x => Array(4294967295).length).join('-'),",
    "2 evaluations and 8589934590 elements in all",
  );

  // And a count so large the product of it and a two hundred element receiver
  // passes `u64::MAX` if it wrapped.
  assert_refuses(
    "",
    "content: ['a','b'].map(x => x.repeat(18446744073709551615)).join('-'),",
    CANNOT_BOUND,
  );
}

/// A receiver nested deeper than the guard descends is refused by the depth
/// budget rather than measured, so the width walk cannot recurse off the stack.
#[test]
fn a_receiver_nested_past_the_depth_budget_refuses() {
  let deep = format!("{}'a'{}", "[".repeat(400), "]".repeat(400));

  assert_refuses(
    "",
    &format!("content: [{}].map(x => 'y'.repeat(2)).join('-'),", deep),
    "too deep",
  );
}
