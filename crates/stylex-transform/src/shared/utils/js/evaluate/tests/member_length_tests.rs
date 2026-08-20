//! `length` on a string and on an array, and everything a member access on one
//! of them refuses.
//!
//! The fold this pins is small; what it replaced is why the suite is not.
//! Reading a property off a literal receiver used to re-evaluate the receiver
//! and drop the property, so `"abc".length` folded to `"abc"` and the
//! stylesheet said `content: "abc"` where the source asked for `3`. Nothing
//! errored. So the cases below are two-sided: every shape that must now fold to
//! a count, and every neighbouring shape that must refuse rather than answer a
//! receiver again.
//!
//! `length` counts UTF-16 code units, which is the one convention that agrees
//! with the language and with `char_code_at`. A byte count and a scalar count
//! each answer something else on non-ASCII input, so the unicode cases are the
//! load-bearing half of this file.

use super::source_evaluation::*;

// ==================== the fold this file is about ====================

#[test]
fn a_string_answers_its_length() {
  assert_folds_to_number("\"abc\".length", 3.0);
  assert_folds_to_number("\"\".length", 0.0);
}

/// Written as a computed key, which is the same property by another spelling.
#[test]
fn a_string_answers_its_length_through_a_computed_key() {
  assert_folds_to_number("\"abc\"[\"length\"]", 3.0);
}

#[test]
fn an_array_answers_its_length() {
  assert_folds_to_number("[\"a\", \"b\"].length", 2.0);
  assert_folds_to_number("[].length", 0.0);
  assert_folds_to_number("[\"a\", \"b\"][\"length\"]", 2.0);
}

/// A nested array is one slot, not the sum of its contents.
#[test]
fn an_array_of_arrays_counts_its_own_slots() {
  assert_folds_to_number("[[1, 2], [3]].length", 2.0);
  assert_folds_to_number("[[[[1]]]].length", 1.0);
}

/// A trailing comma is punctuation, not an element.
#[test]
fn a_trailing_comma_is_not_an_element() {
  assert_folds_to_number("[1, 2,].length", 2.0);
  assert_folds_to_number("[1,].length", 1.0);
}

/// A hole is a slot, so it counts. This is the case that decides where the
/// count is read from: a hole has no value, so the array itself refuses and only
/// the source can say how many slots were written. `[,]` is a single hole with a
/// trailing comma after it, which the language reads as one slot. What the
/// refusal says, and every shape around it, is in `tests/array_hole_tests.rs`.
#[test]
fn a_hole_is_a_slot_and_counts() {
  assert_folds_to_number("[, 1].length", 2.0);
  assert_folds_to_number("[1, , 2].length", 3.0);
  assert_folds_to_number("[,].length", 1.0);
  assert_folds_to_number("[, , ,].length", 3.0);
}

/// The receiver kinds that reach the array-literal arm rather than the evaluated
/// one: a fold that answers an `ArrayLit` value instead of a `Vec`. Also the
/// shapes an author is most likely to actually write a `length` on.
#[test]
fn an_array_a_fold_produced_answers_its_length() {
  assert_folds_to_number("Object.keys({ a: 1, b: 2 }).length", 2.0);
  assert_folds_to_number("Object.values({ a: 1 }).length", 1.0);
  assert_folds_to_number("Object.entries({ a: 1, b: 2 }).length", 2.0);
  assert_folds_to_number("[\"a\", \"b\"].join(\"-\").length", 3.0);
}

/// A spread is one written element standing for however many the spread value
/// holds, so neither the written count nor the evaluated one is the language's
/// answer. Refusing is the point: counting `[...[1, 2]]` as one slot would be a
/// confidently wrong number, which is the defect this whole file removes.
#[test]
fn an_array_carrying_a_spread_refuses_rather_than_being_miscounted() {
  for source in [
    "[...[1, 2]].length",
    "[...[1, 2], 3].length",
    "[1, ...[2, 3]].length",
    "[...\"ab\"].length",
  ] {
    assert_deopts(source);
  }
}

/// A parenthesis is not a different receiver, so it may not change the count.
///
/// The literal is read off the receiver's own AST, and an unnormalized read
/// finds a `ParenExpression` instead of an array — falling through to the
/// evaluated element count, which is blind to every spread, or to the hole's own
/// refusal. `([, 1]).length` answered `1`, and `([...[1, 2]]).length` answered
/// `1` confidently where the bare form refuses.
#[test]
fn a_parenthesised_receiver_is_counted_as_the_bare_one_is() {
  assert_folds_to_number("([1, 2]).length", 2.0);
  assert_folds_to_number("([, 1]).length", 2.0);
  assert_folds_to_number("(([, 1])).length", 2.0);
  assert_deopts("([...[1, 2]]).length");
  assert_deopts("(([...[1, 2]])).length");
}

/// A template literal folds to a string before the property is read, so its
/// length is the length of what it folded to and not of what was written.
#[test]
fn a_folded_string_answers_the_length_of_what_it_folded_to() {
  assert_folds_to_number("`ab${\"c\"}`.length", 3.0);
  assert_folds_to_number("(\"a\" + \"bc\").length", 3.0);
  assert_folds_to_number("\"abc\".concat(\"de\").length", 5.0);
  assert_folds_to_number("(1 ? \"abcd\" : \"x\").length", 4.0);
}

/// A length is a number, so it composes with the arithmetic the evaluator
/// already folds. This is the shape an author actually writes.
#[test]
fn a_length_composes_with_arithmetic() {
  assert_folds_to_number("\"abcd\".length * 2", 8.0);
  assert_folds_to_number("[1, 2, 3].length - 1", 2.0);
  assert_folds_to_number("\"ab\".length + \"cde\".length", 5.0);
}

/// A zero length is falsy and a non-zero one is truthy, which is what makes
/// `length` reachable from the conditions #1265 is about.
#[test]
fn a_length_reads_as_a_truth_value() {
  assert_folds_to_number("\"abc\".length && 1", 1.0);
  assert_folds_to_number("\"\".length || 2", 2.0);
  assert_folds_to_number("[1].length ?? 3", 1.0);

  // `??` refuses a falsy left operand, `0` included, because the reference
  // implementation's guard tests truthiness where it meant nullishness — see
  // `nodes/logical_expression.rs`. A zero length inherits that restriction
  // rather than folding where upstream does not.
  assert_deopts("[].length ?? 3");
}

// ==================== UTF-16, which is the whole point ====================

/// A scalar outside the basic plane is two code units. A byte count would say
/// five here and a scalar count two; the language says three.
#[test]
fn an_astral_scalar_counts_as_two_code_units() {
  assert_folds_to_number("\"\\u{1F600}a\".length", 3.0);
  assert_folds_to_number("\"\\u{1F389}\".length", 2.0);
  assert_folds_to_number("\"\\u{1F600}\\u{1F600}\".length", 4.0);
}

/// The same character written as its two surrogate halves is the same string
/// and therefore the same length.
#[test]
fn a_surrogate_pair_written_out_counts_the_same() {
  assert_folds_to_number("\"\\uD83D\\uDE00\".length", 2.0);
}

/// A lone surrogate is one code unit. It is also a string this compiler must
/// carry through without normalizing it into a replacement character, which is
/// the failure a byte-oriented count would hide.
#[test]
fn a_lone_surrogate_is_one_code_unit() {
  assert_folds_to_number("\"\\uD83D\".length", 1.0);
  assert_folds_to_number("\"\\uDE00\".length", 1.0);
  assert_folds_to_number("\"a\\uD83Db\".length", 3.0);
}

/// A non-ASCII character inside the basic plane is one code unit and several
/// bytes, which is where `str::len` and the language part company.
#[test]
fn a_multi_byte_character_is_one_code_unit() {
  assert_folds_to_number("\"é\".length", 1.0);
  assert_folds_to_number("\"日本語\".length", 3.0);
}

/// `length` does not normalize: a combining sequence counts its scalars even
/// though it renders as one character.
#[test]
fn a_combining_sequence_is_not_normalized() {
  assert_folds_to_number("\"e\\u0301\".length", 2.0);
  assert_folds_to_number("\"\\u00E9\".length", 1.0);
}

/// An escape is one character once the parser has read it, whatever its
/// spelling — a NUL, a C0 control, a line separator, an escaped quote.
#[test]
fn an_escape_counts_as_the_character_it_spells() {
  assert_folds_to_number("\"a\\u0000b\".length", 3.0);
  assert_folds_to_number("\"\\u0001\\u0002\".length", 2.0);
  assert_folds_to_number("\"a\\\"b\".length", 3.0);
  assert_folds_to_number("\"a\\\\b\".length", 3.0);
  assert_folds_to_number("\"a\\nb\".length", 3.0);
  assert_folds_to_number("\"\\u2028\".length", 1.0);
  assert_folds_to_number("\"\\u00A0\".length", 1.0);
  assert_folds_to_number("\"\\uFEFF\".length", 1.0);
}

/// A string that spells CSS syntax is still just a string to `length`. These
/// are the bodies most likely to be mistaken for something the value parser
/// should have been asked about.
#[test]
fn a_string_spelling_css_syntax_is_counted_as_text() {
  assert_folds_to_number("\"rgba(0,0,0,.5)\".length", 14.0);
  assert_folds_to_number("\"a{b:c\".length", 5.0);
  assert_folds_to_number("\"url(\\\"a\".length", 6.0);
  assert_folds_to_number("\"/* x\".length", 4.0);
  assert_folds_to_number("\"!important\".length", 10.0);
  assert_folds_to_number("\"var(--a\".length", 7.0);
}

// ==================== the refusals ====================

/// The defect this file exists for, stated as the thing that must not happen: a
/// property a string does not carry is not answered with the string.
///
/// It answers `undefined`, which is what the reference implementation's
/// `object[property]` gives and what the object arm already gives for a key an
/// object does not hold. `Length` and `LENGTH` are here because the property is
/// case-sensitive and a case-insensitive test would fold them to a count.
#[test]
fn a_property_a_string_does_not_carry_answers_undefined() {
  for source in [
    "\"abc\".foo",
    "\"abc\".size",
    "\"abc\".Length",
    "\"abc\".LENGTH",
    "\"abc\".toUpperCase",
    "\"abc\".constructor",
    "\"abc\".__proto__",
    "\"abc\"[\"foo\"]",
  ] {
    assert_folds_to_undefined(source);
  }

  // The answer exists so a fallback folds rather than reaching the runtime.
  assert_folds_to_string("\"abc\".foo ?? \"red\"", "red");
}

/// An index into a string refuses, and deliberately. The reference
/// implementation folds `"\u{1F600}"[0]` to a lone surrogate, which no Rust
/// string can hold — answering it would be the same class of quietly-wrong value
/// the `length` fix removed. Past the end refuses too: knowing an index is out of
/// range is the same work as reading one.
#[test]
fn an_index_into_a_string_refuses() {
  for source in [
    "\"abc\"[0]",
    "\"abc\"[2]",
    "\"abc\"[9]",
    "\"\\u{1F600}\"[0]",
    "\"abc\"[\"0\"]",
  ] {
    assert_deopts(source);
  }
}

/// A key that looks numeric but is not an index is an ordinary property name,
/// and no string or array carries one — so it answers `undefined`, as upstream
/// does. Testing the key with `parse::<f64>()` would call all of these indices
/// and refuse a fold the reference implementation makes; `"NaN"` is the one that
/// makes that unmistakable.
#[test]
fn a_numeric_looking_key_that_is_not_an_index_answers_undefined() {
  for source in [
    "\"abc\"[-1]",
    "\"abc\"[1.5]",
    "\"abc\"[\"NaN\"]",
    "\"abc\"[\"1.5\"]",
    "[1, 2][-1]",
    "[1, 2][1.5]",
  ] {
    assert_folds_to_undefined(source);
  }
}

/// A refusal names the index, so a build error says which read could not be
/// folded rather than only which node kind it was.
///
/// Swept across the three receiver kinds, because one property test per receiver
/// is how they drifted into three diagnostics for one author mistake. All three
/// now say the same thing about the same index.
#[test]
fn a_refusal_names_the_index_it_could_not_read() {
  assert_deopt_names_property("\"abc\"[7]", "7");
  assert_deopt_names_property("[1, 2][\"7\"]", "7");
  assert_deopt_names_property("Object.keys({ a: 1 })[\"7\"]", "7");
}

/// `length` is a string and array property only. A number, a boolean, a
/// regular expression and `null` each refuse — the receiver used to be
/// answered in every one of these cases.
#[test]
fn a_receiver_that_is_not_a_string_or_an_array_refuses() {
  for source in [
    "(5).length",
    "(5).foo",
    "true.length",
    "false.foo",
    "null.length",
    "/re/.length",
    "1n.length",
  ] {
    assert_deopts(source);
  }
}

/// An object literal is read by key, and a key it does not carry is
/// `undefined` rather than a refusal — which is what lets
/// `token.missing ?? fallback` fold. `length` is not special there, so an
/// object without one answers `undefined` and not `0`.
#[test]
fn an_object_without_a_length_key_answers_undefined() {
  assert_folds_to_number("({}).length ?? 3", 3.0);
  assert_folds_to_number("({ a: 1 }).length ?? 4", 4.0);
}

/// An object literal is read by key, so a `length` key is a key like any other
/// and is not confused with the count of anything.
#[test]
fn an_object_with_a_length_key_answers_the_key() {
  assert_folds_to_number("({ length: 7 }).length", 7.0);
  assert_folds_to_number("({ length: 7 })[\"length\"]", 7.0);
}

/// A property read off an array that is neither `length` nor an index answers
/// `undefined`, for the same reason and by the same rule as a string.
#[test]
fn a_property_an_array_does_not_carry_answers_undefined() {
  for source in [
    "[1, 2].foo",
    "[1, 2].size",
    "[1, 2].reverse",
    "[1, 2].constructor",
    "[1, 2][\"foo\"]",
    "Object.keys({ a: 1 }).foo",
  ] {
    assert_folds_to_undefined(source);
  }

  assert_folds_to_string("[1, 2].foo ?? \"red\"", "red");
  assert_folds_to_string("Object.keys({ a: 1 }).foo ?? \"red\"", "red");
}

/// A computed key the evaluator cannot read at all refuses before the receiver
/// is consulted, so a `length` fold cannot be reached by a key that is not a
/// name.
#[test]
fn a_computed_key_with_no_compile_time_value_refuses() {
  for source in [
    "\"abc\"[runtimeKey]",
    "\"abc\"[{}]",
    "\"abc\"[/re/]",
    "[1, 2][runtimeKey]",
  ] {
    assert_deopts(source);
  }
}

/// A length is not read off a receiver that never folded. The refusal comes
/// from the receiver, and it must stay a refusal rather than become a count of
/// something the evaluator guessed.
#[test]
fn a_receiver_that_did_not_fold_refuses_rather_than_being_counted() {
  for source in [
    "runtimeValue.length",
    "\"abc\".normalize().length",
    "[1, 2].reduce(f).length",
    "(runtimeFlag ? \"a\" : \"bc\").length",
  ] {
    assert_deopts(source);
  }
}

// ==================== the seam that broke ====================

/// The property #1265 is about. `&&`, `||` and `??` evaluate their right
/// operand under a forked confidence, so a refusal in that position has to stay
/// a refusal rather than abort the build. Both the folds and the refusals are
/// swept through all three, because a `length` that panicked in that position
/// would be the same defect wearing a different property name.
#[test]
fn a_length_read_survives_every_logical_operand_position() {
  for shape in [
    "\"abc\".length",
    "[1, 2].length",
    "\"\\u{1F600}a\".length",
    "\"abc\"[0]",
    "(5).length",
    "null.length",
    "[1, 2].reverse",
    "runtimeValue.length",
  ] {
    assert_deopts(&format!("runtimeFlag && {}", shape));
    assert_deopts(&format!("runtimeFlag || {}", shape));
    assert_deopts(&format!("runtimeValue ?? {}", shape));
  }

  // A confident left operand consults the right one, and the fold survives it.
  assert_folds_to_number("1 > 0 && \"abc\".length", 3.0);
  assert_folds_to_number("\"\" || [1, 2].length", 2.0);
  assert_folds_to_number("null ?? \"\\u{1F600}a\".length", 3.0);

  // A confident left operand that decides on its own is not disturbed by an
  // unreadable right one.
  assert_folds_to_number("2 || \"abc\"[0]", 2.0);
  assert_folds_to_number("0 && \"abc\"[0]", 0.0);
}

// ==================== boundaries ====================

/// A long string is counted, not sampled or truncated. Ten thousand characters
/// is far past anything a stylesheet carries and cheap enough to assert on.
#[test]
fn a_long_string_is_counted_exactly() {
  let long = "x".repeat(10_000);

  assert_folds_to_number(&format!("\"{}\".length", long), 10_000.0);
}

/// The same for a long array literal, where each element is a separate AST node
/// the evaluator walks.
#[test]
fn a_long_array_is_counted_exactly() {
  let elements = std::iter::repeat_n("1", 5_000)
    .collect::<Vec<_>>()
    .join(",");

  assert_folds_to_number(&format!("[{}].length", elements), 5_000.0);
}

/// A long astral string is where a byte count diverges furthest from a code
/// unit count: four bytes and two code units per character.
#[test]
fn a_long_astral_string_is_counted_in_code_units() {
  let long = "\u{1F600}".repeat(1_000);

  assert_folds_to_number(&format!("\"{}\".length", long), 2_000.0);
}

/// Deep nesting is the shape most likely to turn either answer into a stack
/// overflow, because each level recurses. Asserted on both sides so a
/// crash-free refusal cannot be mistaken for a working fold.
#[test]
fn a_deeply_nested_length_read_neither_overflows_nor_changes_answer() {
  let deep = std::iter::repeat_n("1 > 0 && ", 100).collect::<String>();

  assert_folds_to_number(&format!("{}\"abc\".length", deep), 3.0);
  assert_deopts(&format!("{}\"abc\"[0]", deep));
}

/// A member chain deeper than the receiver is not a length read at any depth. A
/// number has no `length`, and neither does the `undefined` a missing property
/// answers — reading one off either refuses rather than folding, because reading
/// a property of `undefined` throws in the language.
#[test]
fn a_chain_past_the_length_refuses() {
  for source in [
    "\"abc\".length.length",
    "\"abc\".length.foo",
    "[1, 2].length.length",
    "\"abc\".foo.length",
    "[1, 2].foo.length",
  ] {
    assert_deopts(source);
  }
}
