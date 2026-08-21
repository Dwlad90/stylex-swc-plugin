//! Reading an element out of an array by index.
//!
//! Both array receivers used to refuse one. An array literal a fold produced
//! read a slot only where the index was written as a numeric literal, and
//! answered no value at all past the end -- a confident `None` the callers read
//! as "nothing to see". An array the evaluator holds as its own value refused
//! every index, so `const A = ['1px']; A[0]` stopped a build the reference
//! implementation compiles.
//!
//! The language is the contract: a canonical digit key names a slot, every
//! other key is an ordinary property name, and a slot past the end is
//! `undefined`. So the two array receivers now answer the same thing about the
//! same key, and both answer what a key an object does not carry already did.
//!
//! `length` and everything a member access refuses live in
//! `member_length_tests.rs`; a hole's own refusal is in `array_hole_tests.rs`.

use super::source_evaluation::*;

// ==================== the fold this file is about ====================

/// The reported shape: an array bound to a name, indexed at the value position.
/// This is the `Vec` receiver -- the evaluator's own value, with no literal to
/// read a slot off.
#[test]
fn an_evaluated_array_answers_an_index() {
  assert_folds_to_string("(0 ? [] : [\"1px\"])[0]", "1px");
  assert_folds_to_string("(1 ? [\"1px\", \"2px\"] : [])[1]", "2px");
}

/// The array-literal receiver, which is what a fold answers.
#[test]
fn an_array_a_fold_produced_answers_an_index() {
  assert_folds_to_string("Object.keys({ a: 1, b: 2 })[1]", "b");
  assert_folds_to_string("Object.values({ a: \"1px\" })[0]", "1px");
}

/// Written as a literal, which is the shortest spelling of either receiver.
#[test]
fn an_array_literal_answers_an_index() {
  assert_folds_to_string("[\"1px\", \"2px\"][0]", "1px");
  assert_folds_to_string("[\"1px\", \"2px\"][1]", "2px");
  assert_folds_to_number("[1, 2, 3][2]", 3.0);
}

/// A string key of the same digits names the same slot: `list[0]` and
/// `list["0"]` are one element in the language.
#[test]
fn a_string_written_index_names_the_same_slot() {
  assert_folds_to_string("[\"1px\", \"2px\"][\"0\"]", "1px");
  assert_folds_to_string("(0 ? [] : [\"1px\"])[\"0\"]", "1px");
  assert_folds_to_string("Object.keys({ a: 1 })[\"0\"]", "a");
}

/// A float that stringifies to a whole number is that index, because the key a
/// member read asks for is the string the number coerces to.
#[test]
fn an_index_written_as_a_whole_float_names_its_slot() {
  assert_folds_to_string("[\"1px\", \"2px\"][1.0]", "2px");
}

/// The second row of the reported shape: an index read feeding another array,
/// which is where the refusal used to arrive one level in.
#[test]
fn an_index_read_nests() {
  assert_folds_to_string("[[\"1px\", \"2px\"][0], \"3px\"][0]", "1px");
  assert_folds_to_string("[[[\"1px\"]]][0][0][0]", "1px");
}

// ==================== past the end, and the keys that are not indices ====

/// An index past the end is `undefined`, not a refusal and not a missing value.
#[test]
fn an_index_past_the_end_answers_undefined() {
  assert_folds_to_undefined("[\"1px\"][7]");
  assert_folds_to_undefined("[\"1px\"][\"7\"]");
  assert_folds_to_undefined("(0 ? [] : [\"1px\"])[7]");
  assert_folds_to_undefined("Object.keys({ a: 1 })[7]");
  assert_folds_to_undefined("[][0]");
}

/// So an index past the end takes a fallback, which is the whole reason
/// answering `undefined` beats answering nothing.
#[test]
fn an_index_past_the_end_takes_a_fallback() {
  assert_folds_to_string("[\"1px\"][7] ?? \"2px\"", "2px");
  assert_folds_to_string("[\"1px\"][0] ?? \"2px\"", "1px");
}

/// A key of digits that is not the canonical spelling of its number is an
/// ordinary property name: `["a"]["00"]` is `undefined` where `["a"]["0"]` is
/// the first element. Reading it as a slot is the one way a digit test can be
/// confidently wrong.
#[test]
fn a_non_canonical_digit_key_is_not_an_index() {
  assert_folds_to_undefined("[\"1px\"][\"00\"]");
  assert_folds_to_undefined("[\"1px\"][\"007\"]");
  assert_folds_to_undefined("(0 ? [] : [\"1px\"])[\"00\"]");
  assert_folds_to_undefined("Object.keys({ a: 1 })[\"00\"]");
}

/// A key that is numeric-looking but names no slot at all -- negative,
/// fractional, `NaN`, `Infinity` -- is a property no array carries.
#[test]
fn a_numeric_key_that_names_no_slot_answers_undefined() {
  for source in [
    "[\"1px\"][-1]",
    "[\"1px\"][1.5]",
    "[\"1px\"][\"NaN\"]",
    "[\"1px\"][\"-0\"]",
    "[\"1px\"][\"1e3\"]",
    "(0 ? [] : [\"1px\"])[-1]",
    "(0 ? [] : [\"1px\"])[1.5]",
  ] {
    assert_folds_to_undefined(source);
  }
}

/// A key wider than any array this evaluator holds is past the end, which is
/// the same answer as any other index past the end. Spelled at and beyond
/// `usize`, because parsing it is where an overflow would otherwise decide.
#[test]
fn an_index_beyond_what_a_slot_count_can_hold_answers_undefined() {
  assert_folds_to_undefined("[\"1px\"][\"18446744073709551615\"]");
  assert_folds_to_undefined("[\"1px\"][\"18446744073709551616\"]");
  assert_folds_to_undefined("[\"1px\"][\"99999999999999999999999999999999\"]");
  assert_folds_to_undefined("(0 ? [] : [\"1px\"])[\"18446744073709551616\"]");
}

/// Reading a property off the `undefined` an out-of-range index answered
/// throws in the language, so it refuses rather than answering `undefined` a
/// second time.
#[test]
fn a_property_read_off_an_out_of_range_index_refuses() {
  assert_deopts("[\"1px\"][7].length");
  assert_deopts("(0 ? [] : [\"1px\"])[7].foo");
}

// ==================== the shapes that still refuse ====================

/// A spread stands for however many elements its value holds, so no slot after
/// it is the one the source names. The receiver refuses to fold at all, which
/// is the guard the index read sits behind rather than in front of.
#[test]
fn an_array_carrying_a_spread_refuses_an_index() {
  for source in [
    "[...[\"1px\"]][0]",
    "[\"1px\", ...[\"2px\"]][0]",
    "[...\"ab\"][0]",
  ] {
    assert_deopts(source);
  }
}

/// A hole has no value, so the receiver refuses ahead of the index -- the same
/// refusal `length` reads around rather than through.
#[test]
fn an_array_carrying_a_hole_refuses_an_index() {
  for source in ["[, \"1px\"][1]", "[\"1px\", , \"2px\"][0]", "[,][0]"] {
    assert_deopts(source);
  }
}

/// A string still refuses an index: its element is a single UTF-16 code unit,
/// which can be an unpaired surrogate no Rust string holds. The two array
/// receivers agreeing does not make a third one agree with them.
#[test]
fn a_string_still_refuses_an_index() {
  assert_deopts("\"abc\"[0]");
  assert_deopts("\"\u{1F600}\"[0]");
}

/// A computed key with no name the evaluator reads refuses rather than being
/// treated as slot zero.
#[test]
fn an_unreadable_computed_key_refuses() {
  assert_deopts("[\"1px\"][{}]");
  assert_deopts("[\"1px\"][[]]");
  assert_deopts("(0 ? [] : [\"1px\"])[{}]");
}

/// A parenthesis is not a different receiver, and neither is a nested one.
#[test]
fn a_parenthesised_receiver_is_indexed_as_the_bare_one_is() {
  assert_folds_to_string("([\"1px\", \"2px\"])[1]", "2px");
  assert_folds_to_string("(([\"1px\", \"2px\"]))[1]", "2px");
  assert_deopts("([...[\"1px\"]])[0]");
  assert_deopts("([, \"1px\"])[1]");
}

// ==================== unicode and escaped keys ====================

/// A key written with an escape is the key it spells, and a non-ASCII key names
/// no slot.
#[test]
fn an_escaped_or_non_ascii_key_names_no_slot() {
  assert_folds_to_undefined("[\"1px\"][\"\\u0030\\u0030\"]");
  assert_folds_to_undefined("[\"1px\"][\"٠\"]");
  assert_folds_to_undefined("[\"1px\"][\"０\"]");
}

/// An escaped key that spells a canonical index *is* that index, which is the
/// other half of the case above -- the digits decide, not how they were typed.
#[test]
fn an_escaped_key_that_spells_an_index_reads_it() {
  assert_folds_to_string("[\"1px\", \"2px\"][\"\\u0031\"]", "2px");
}

/// A non-ASCII element is returned unchanged: the index read carries the value
/// it found rather than re-coercing it.
#[test]
fn an_index_returns_a_non_ascii_element_unchanged() {
  assert_folds_to_string("[\"ünïcödé\"][0]", "ünïcödé");
  assert_folds_to_string("[\"\u{1F600}\"][0]", "\u{1F600}");
}

// ==================== depth and size ====================

/// A long array is indexed at either end, which is the bounds check reading the
/// slot count rather than a written literal.
#[test]
fn a_long_array_is_indexed_at_both_ends() {
  let elements = (0..256)
    .map(|index| format!("\"{}px\"", index))
    .collect::<Vec<_>>()
    .join(", ");

  assert_folds_to_string(&format!("[{}][0]", elements), "0px");
  assert_folds_to_string(&format!("[{}][255]", elements), "255px");
  assert_folds_to_undefined(&format!("[{}][256]", elements));
}

/// A chain of index reads deep enough to be interesting still folds, which is
/// the evaluator's depth ceiling being reached by the receiver rather than by
/// the key.
#[test]
fn a_chain_of_index_reads_folds() {
  let source = format!("[[\"1px\"]]{}", "[0]".repeat(2));

  assert_folds_to_string(&source, "1px");
}
