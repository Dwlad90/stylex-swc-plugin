//! A callback measures the receiver it has.
//!
//! The element count a callback is priced against belongs to the receiver, so it
//! is the same count whatever method reads it. It used to belong to a list of ten
//! method names instead: a callback on anything else was unmeasured and refused
//! the moment its body amplified, and a receiver that was itself a call was
//! unmeasured whatever the method. Zero-padding over a `split` refused; so did a
//! repeat counted by an element, and one counted by an index.
//!
//! Three readings replace the list, and all three come off one measurement of one
//! receiver so they cannot come to disagree:
//!
//! - **The count.** A receiver that is a call resolves like any other, because
//!   the walk admits it *before* the count is taken -- so whatever it answers is
//!   already inside both ceilings and the fold is about to build it anyway. The
//!   length rule keeps refusing a call, and for the opposite reason: it is asked
//!   in front of the receiver's own bound rather than behind it.
//! - **The element.** What one element renders to, for the parameter the method
//!   hands it -- first for almost everything, second for a reducer.
//! - **The index.** The largest index a receiver of that length has, which bounds
//!   a count written as `i + 1` the way an element bounds one written as `n`.
//!
//! What is left refusing is a comparator, the one callback the language runs more
//! often than its receiver is long. Every other method is measured, including one
//! nobody has written down -- which is the point: an unlisted method now folds
//! rather than refusing.
//!
//! Every folding case below is measured output of `@stylexjs/babel-plugin` 0.19.0
//! under the same options, so each asserts agreement with the reference compiler
//! rather than with this compiler's own previous answer.

use crate::utils::{
  prelude::*,
  transform::{
    assert_folds, assert_folds_with, assert_refuses, assert_refuses_under, base_style_module,
    stringify_js,
  },
};

/// The line the blanket refusal opens with, which is what a callback nothing
/// counted still gets.
const UNMEASURED: &str = "would build inside a callback";

/// Compile with both allocation ceilings set, the way an author moves them.
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
// The four shapes the ticket is about
// ──────────────────────────────────────────────

/// Zero-padding over a `split` and over an `Object.keys`, which are the two
/// spellings of a receiver that is a call, and a repeat counted by an element and
/// by an index, which are the two spellings of a count only the receiver settles.
#[test]
fn the_four_shapes_the_ticket_names_fold() {
  assert_folds(
    "",
    "content: 'ab'.split('').map(x => x.padStart(2, '0')).join(''),",
    ".x1a1yyj{content:\"0a0b\"}",
  );

  assert_folds(
    "const a = [1, 2];",
    "content: a.map(n => 'x'.repeat(n)).join('-'),",
    ".xmq1om5{content:\"x-xx\"}",
  );

  assert_folds(
    "const a = ['p', 'q'];",
    "content: a.map((s, i) => s.repeat(i + 1)).join('-'),",
    ".x1b13cjn{content:\"p-qq\"}",
  );

  assert_folds(
    "const o = { a: 1, b: 2 };",
    "content: Object.keys(o).map(k => k.padStart(3, '0')).join('-'),",
    ".x1gndu4b{content:\"00a-00b\"}",
  );
}

// ──────────────────────────────────────────────
// The methods the list never held
// ──────────────────────────────────────────────

/// A reducer is handed the accumulator first and the element second, so the width
/// belongs to the second parameter -- which is why it could not simply be added
/// to the list, and why the element's position is read rather than assumed.
#[test]
fn a_reducer_measures_its_element_in_the_second_parameter() {
  assert_folds(
    "const a = ['p', 'q'];",
    "content: a.reduce((acc, s) => acc + s.repeat(2), ''),",
    ".xffwxmf{content:\"ppqq\"}",
  );

  assert_folds(
    "const a = ['p', 'q'];",
    "content: a.reduceRight((acc, s) => acc + s.repeat(2), ''),",
    ".xc76y22{content:\"qqpp\"}",
  );
}

/// The accumulator itself is a value the body built rather than one the receiver
/// held, so nothing bounds its width and a repeat of it is refused -- the count
/// being one is what makes the same expression fold, since a repeat of one builds
/// nothing the receiver was not already paid for.
#[test]
fn a_reducer_bounds_its_element_and_not_its_accumulator() {
  assert_folds(
    "const a = ['p', 'q'];",
    "content: a.reduce((acc, s) => acc.repeat(1) + s, ''),",
    ".xx4y88w{content:\"pq\"}",
  );

  assert_refuses(
    "const a = ['p', 'q'];",
    "content: a.reduce((acc, s) => acc.repeat(2) + s, ''),",
    "Cannot bound the string 'repeat' would build.",
  );
}

/// A comparator runs once per comparison, which for any sort worth using is more
/// often than the array is long. Nothing here counts comparisons, so the count is
/// withheld and a body that amplifies inside one keeps the refusal every callback
/// used to get. Upstream folds the second of these, which is what the exception
/// costs.
#[test]
fn a_comparator_folds_but_is_never_counted() {
  assert_folds(
    "",
    "content: ['b','a'].sort((x, y) => (x < y ? -1 : 1)).join('-'),",
    ".x1t42mo{content:\"a-b\"}",
  );

  assert_folds(
    "const a = ['b', 'a'];",
    "content: a.toSorted((x, y) => (x < y ? -1 : 1)).join('-'),",
    ".x1t42mo{content:\"a-b\"}",
  );

  assert_refuses(
    "",
    "content: ['b','a'].sort((x, y) => x.padStart(4, '0') < y ? -1 : 1).join('-'),",
    UNMEASURED,
  );
}

/// `Array.from` runs its mapper once per element of the value it iterates, which
/// is an argument rather than a receiver -- the one static that reaches a
/// callback at all, and the reason the count is read there too.
///
/// Three sources, because `from` accepts three shapes and each settles the count
/// a different way: an array holds its elements, a string is iterated by code
/// point, and an array-like declares a length it does not hold and hands the
/// mapper `undefined` for every one of them.
#[test]
fn the_mapper_array_from_takes_is_measured_against_its_source() {
  assert_folds(
    "const a = ['b', 'a'];",
    "content: Array.from(a, s => s.repeat(2)).join('-'),",
    ".x1kaxd41{content:\"bb-aa\"}",
  );

  assert_folds(
    "",
    "content: Array.from('ab', (c, i) => c.repeat(i + 1)).join('-'),",
    ".x53ohii{content:\"a-bb\"}",
  );

  assert_folds(
    "",
    "content: Array.from({ length: 3 }, (_, i) => 'x'.repeat(i)).join('-'),",
    ".x1eoao1j{content:\"-x-xx\"}",
  );
}

/// A method the guard has never heard of is measured like every other, which is
/// the whole of what deleting the list bought. None of these was on it.
#[test]
fn a_method_the_list_never_held_is_measured_like_any_other() {
  assert_folds(
    "const a = ['p', 'q'];",
    "content: a.every(s => s.padStart(2, '0').length === 2) ? 'yes' : 'no',",
    ".xs2tsc8{content:\"yes\"}",
  );

  assert_folds(
    "const a = ['p', 'q'];",
    "content: a.findLast(s => s.padEnd(2, '!') === 'q!'),",
    ".xdgu3iy{content:\"q\"}",
  );

  assert_folds(
    "const a = ['p', 'q'];",
    "content: a.flatMap(s => [s.repeat(2)]).join('-'),",
    ".xxeajk5{content:\"pp-qq\"}",
  );
}

// ──────────────────────────────────────────────
// Receivers a count is now read off
// ──────────────────────────────────────────────

/// A chain is a receiver that is a call at every link, and each link is counted
/// off what the one before it answered.
#[test]
fn a_chain_counts_each_link_off_the_one_before_it() {
  assert_folds(
    "",
    "content: 'abc'.split('').filter(c => c !== 'b').map(c => c.repeat(2)).join(''),",
    ".x1f428ol{content:\"aacc\"}",
  );

  assert_folds(
    "",
    "content: 'abcd'.split('').map(c => c.toUpperCase()).filter(c => c < 'D')\
     .map(c => c.repeat(2)).join(''),",
    ".x1ghw8vq{content:\"AABBCC\"}",
  );

  // The amplifying call whose receiver is an amplifying call, which is where a
  // count read off the resolved value is exact where a per-link bound was not.
  assert_folds(
    "",
    "content: 'x'.repeat(4).split('').map(c => c.repeat(4)).join(''),",
    ".xv4dwug{content:\"xxxxxxxxxxxxxxxx\"}",
  );

  // And one method's result handed straight to the same method, so the second
  // count comes off what the first built.
  assert_folds(
    "const a = ['p', 'q'];",
    "content: a.map(s => s.repeat(2)).map(s => s.repeat(2)).join('-'),",
    ".xmxo0b7{content:\"pppp-qqqq\"}",
  );
}

/// The statics that answer an array, which are receivers a call reaches the same
/// way -- and a name bound to a callback, which reaches the same body one
/// resolution later.
#[test]
fn an_object_static_and_a_named_callback_are_counted_the_same_way() {
  assert_folds(
    "const o = { a: 'x', b: 'y' };",
    "content: Object.values(o).map(v => v.padEnd(3, '.')).join('-'),",
    ".xxfuxyj{content:\"x..-y..\"}",
  );

  assert_folds(
    "const o = { a: 1, b: 2, c: 3, d: 4 };",
    "content: Object.keys(o).map(k => k.repeat(2)).join(''),",
    ".xtrn3ie{content:\"aabbccdd\"}",
  );

  assert_folds(
    "const pad = s => s.padStart(3, '0');",
    "content: ['a','b'].map(pad).join('-'),",
    ".x1gndu4b{content:\"00a-00b\"}",
  );
}

/// Nesting multiplies rather than resets, which is what makes a count read off
/// each receiver a bound on the whole rather than on one level of it.
#[test]
fn nested_callbacks_multiply_the_counts_they_were_measured_at() {
  assert_folds(
    "const a = ['p', 'q'];",
    "content: a.map(s => a.map(t => 'x'.repeat(2)).join('')).join('-'),",
    ".x1audz6t{content:\"xxxx-xxxx\"}",
  );
}

// ──────────────────────────────────────────────
// Counts read off nothing, and the edges of what one is
// ──────────────────────────────────────────────

/// A receiver of no elements runs its body no times, so a length nothing else
/// would admit is admitted -- the product is zero, and the arithmetic says so
/// without a case of its own.
#[test]
fn an_empty_receiver_prices_its_callback_at_nothing() {
  assert_folds(
    "",
    "content: [].map(x => x.repeat(999999)).join('') + 'z',",
    ".x1609fvb{content:\"z\"}",
  );
}

/// An element whose width nothing can read costs the width and not the count, so
/// a body reading no element still folds at the count the receiver has.
#[test]
fn an_unreadable_element_still_answers_for_the_count() {
  assert_folds(
    "",
    "content: [{}].map(x => 'y'.repeat(2)).join('-'),",
    ".xm2b7j9{content:\"yy\"}",
  );
}

/// A parameter with a default may be handed something other than the element, so
/// the whole parameter list takes neither the width nor the count. Upstream
/// refuses this too.
#[test]
fn a_defaulted_parameter_leaves_the_callback_unmeasured() {
  assert_refuses(
    "const a = ['p', 'q'];",
    "content: a.map((s = 'z') => s.padStart(4, '0')).join('-'),",
    UNMEASURED,
  );
}

/// A count read as a name is bounded by what the elements are, and a value that
/// is a string is left unbounded on purpose: `'2' + 1` joins to `'21'`, so a
/// bound taken off a coercion is no bound once the source adds to it.
#[test]
fn a_count_is_bounded_by_a_number_and_not_by_a_coercion() {
  assert_folds(
    "const a = [3, 1];",
    "content: a.map(n => 'y'.repeat(n)).join('-'),",
    ".x1g081fa{content:\"yyy-y\"}",
  );

  assert_refuses(
    "const a = ['2', '3'];",
    "content: a.map(n => 'y'.repeat(n)).join('-'),",
    "Cannot bound the string 'repeat' would build.",
  );
}

/// A count may mix a name the module holds with one only the receiver bounds, so
/// each part of the sum is read where its value lives rather than the whole of it
/// in one place.
#[test]
fn a_count_may_be_part_module_and_part_receiver() {
  assert_folds(
    "const k = 2;",
    "content: ['p','q'].map((s, i) => s.repeat(k + i)).join('-'),",
    ".xwhrxy2{content:\"pp-qqq\"}",
  );
}

/// A string `Array.from` iterates is iterated by code point, so an astral
/// character is one element two UTF-16 units wide rather than two elements of
/// one -- which is the reading that keeps the count short and the width long,
/// the direction a ceiling is safe to be told.
#[test]
fn a_string_source_is_counted_by_code_point() {
  assert_folds(
    "",
    "content: Array.from('a\\u{1F600}', c => c.repeat(2)).join('-'),",
    ".xpjsjxq{content:\"aa-\u{1F600}\u{1F600}\"}",
  );
}

/// A bound is only carried through a sum or a product where every part of it is
/// a number that is not below zero, because neither operation is monotone
/// otherwise: `(-5) * (-5)` is twenty-five against a bound of nothing. A written
/// negative is a unary minus rather than a number, so it stops the reading where
/// it stands, and the call is refused rather than admitted on a bound that would
/// not have held.
#[test]
fn an_arithmetic_bound_is_only_read_over_numbers_at_or_above_zero() {
  assert_folds(
    "const a = [2, 3];",
    "content: a.map(n => 'x'.repeat(n * n)).join('-'),",
    ".xk1pwb{content:\"xxxx-xxxxxxxxx\"}",
  );

  assert_refuses(
    "const a = [-5];",
    "content: a.map(n => 'x'.repeat(n * n)).join('-'),",
    "Cannot bound the string 'repeat' would build.",
  );
}

/// A fraction is rounded up on the way into the arithmetic, because the language
/// truncates the *result* and not the parts: `0.9 * 2000000` is one million eight
/// hundred thousand characters, and a bound that truncated the `0.9` first would
/// come to nothing and admit it.
#[test]
fn a_fractional_element_is_rounded_up_before_the_arithmetic() {
  assert_folds(
    "const a = [0.9];",
    "content: a.map(n => 'x'.repeat(n * 4)).join('') + 'z',",
    ".x17qxs8g{content:\"xxxz\"}",
  );

  assert_refuses(
    "const a = [0.9];",
    "content: a.map(n => 'x'.repeat(n * 2000000)).join(''),",
    "It asks for 2000000 characters, and at most 1000000 are supported.",
  );
}

// ──────────────────────────────────────────────
// A body that really is large
// ──────────────────────────────────────────────

/// The refusal that stays: a product past the character ceiling, named in both
/// factors so an author can see which one to change and that the ceiling is the
/// thing that moved -- `maxFoldedCharacters` is the option those numbers belong
/// to.
#[test]
fn a_body_that_amplifies_past_the_character_ceiling_still_refuses() {
  assert_refuses(
    "const a = ['p', 'q'];",
    "content: a.map(s => s.repeat(999999)).join('-'),",
    "It asks for 999999 characters once per element of the receiver it is written inside, \
     which is 2 evaluations and 1999998 characters in all, and at most 1000000 are supported.",
  );

  // The same product reached through a declared length rather than through a
  // written array, so the count comes from the source `Array.from` iterates.
  assert_refuses(
    "",
    "content: Array.from({ length: 5000 }, () => 'x'.repeat(1000)).join(''),",
    "5000 evaluations and 5000000 characters in all, and at most 1000000 are supported.",
  );

  // And through nesting, where the product is of three receivers rather than one.
  assert_refuses(
    "const a = ['p', 'q'];",
    "content: a.map(x => a.map(y => a.map(z => 'q'.repeat(200000)).join('')).join('')).join(''),",
    "8 evaluations and 1600000 characters in all",
  );

  // A count an element holds is bounded by the same ceiling as one written out,
  // so a large number reaching the body through a name is refused like a large
  // number reaching it directly.
  assert_refuses(
    "const a = [1000000000];",
    "content: a.map(n => 'x'.repeat(n)).join(''),",
    "It asks for 1000000000 characters, and at most 1000000 are supported.",
  );
}

/// The same arithmetic in the other unit, which the entry ceiling spends.
#[test]
fn a_body_that_declares_too_many_elements_still_refuses() {
  assert_refuses(
    "const a = ['p', 'q', 'r'];",
    "content: a.map(() => Array(4000).fill('x').length).join('-'),",
    "It declares a length of 4000 elements once per element of the receiver it is written \
     inside, which is 3 evaluations and 12000 elements in all, and at most 10000 are supported.",
  );
}

/// A raised ceiling folds what the default refuses, and a lowered one refuses
/// what it folds -- so the number the sentence names is the option's rather than
/// this rule's.
#[test]
fn the_ceiling_the_product_is_measured_against_is_the_configured_one() {
  assert_folds_with(
    "const a = ['p', 'q'];",
    "content: a.map(s => s.repeat(3)).join('-'),",
    ".x10sldiy{content:\"ppp-qqq\"}",
    " under a character ceiling of 8",
    |module| fold_under(module, 8, 10_000),
  );

  assert_refuses_under(
    "const a = ['p', 'q'];",
    "content: a.map(s => s.repeat(5)).join('-'),",
    "2 evaluations and 10 characters in all, and at most 8 are supported",
    |module| fold_under(module, 8, 10_000),
  );
}

/// A receiver the guard cannot resolve at all leaves the callback unmeasured
/// rather than answering a count nothing measured.
#[test]
fn a_receiver_nothing_resolves_leaves_the_callback_unmeasured() {
  assert_refuses(
    "",
    "content: [...[1, 2]].map(() => 'x'.padStart(4, '0')).join('-'),",
    "Unsupported expression: SpreadElement",
  );
}

/// A receiver of two thousand elements is counted rather than sampled, so a body
/// well inside the ceiling on its own is refused on the product -- which is the
/// input this rule exists for, and it is refused before anything is built.
#[test]
fn a_large_receiver_is_counted_rather_than_estimated() {
  let elements: Vec<String> = (0..2_000).map(|index| index.to_string()).collect();

  assert_refuses(
    &format!("const a = [{}];", elements.join(",")),
    "content: a.map(() => 'x'.repeat(1000)).join(''),",
    "2000 evaluations and 2000000 characters in all, and at most 1000000 are supported",
  );
}

/// One module the whole rule reads through, so the count, the width and the index
/// are all spent by the same declaration rather than one case each.
#[test]
fn the_three_readings_answer_one_declaration_together() {
  let output = fold_under(
    &base_style_module(
      "const parts = 'a-bb-ccc'.split('-');",
      "content: parts.map((part, index) => part.padStart(index + 2, '0')).join('|'),",
    ),
    1_000_000,
    10_000,
  );

  assert!(
    output.contains("0a|0bb|0ccc"),
    "expected the three readings to fold together, got:\n{}",
    output
  );
}
