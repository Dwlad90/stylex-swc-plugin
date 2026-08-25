//! What the engine-backed fold accepts, and — the part that matters — what it
//! refuses.
//!
//! An engine folds whatever JavaScript folds, so the guard in front of it is
//! the whole of this module's behaviour: everything it lets through is answered
//! by `String.prototype` and friends rather than by anything here, and
//! everything it holds back is a boundary somebody decided on. A test that only
//! checked the folds would pass while the guard let the scope, a mutation or a
//! value the evaluator cannot carry through.
//!
//! Refusals are asserted as refusals, not as absences: `assert_deopts` requires
//! the evaluator to answer "not confident" without aborting, which is the
//! property issue 02 established and this hook must not undo.

use super::source_evaluation::*;

// ==================== the reported input, now folded ====================

/// The method from #1265 with a static argument. The reported shape had a
/// runtime argument and must still refuse — that pairing is the point.
#[test]
fn a_static_method_call_folds_and_a_runtime_argument_still_refuses() {
  assert_folds_to_string("\"documentation\".startsWith(\"doc\") ? \"a\" : \"b\"", "a");
  assert_deopts("\"documentation\".startsWith(lowerQuery)");
}

// ==================== the prototype surface ====================

#[test]
fn the_string_methods_the_reference_implementation_folds_fold() {
  assert_folds_to_string("\"  4px  \".trim()", "4px");
  assert_folds_to_string("\"4pxx\".slice(0, 3)", "4px");
  assert_folds_to_string("\"4\".concat(\"px\")", "4px");
  assert_folds_to_string("\"7\".padStart(3, \"0\")", "007");
  assert_folds_to_string("\"a-b-c\".replaceAll(\"-\", \"_\")", "a_b_c");
  assert_folds_to_string("\"AbC\".toLowerCase()", "abc");
  assert_folds_to_string("\"x4px\".substring(1)", "4px");
  assert_folds_to_number("\"documentation\".indexOf(\"m\")", 4.0);
  assert_folds_to_number("\"abc\".charCodeAt(0)", 97.0);
}

#[test]
fn the_array_methods_the_reference_implementation_folds_fold() {
  assert_folds_to_string("[\"a\", \"b\"].join(\"-\")", "a-b");
  assert_folds_to_string("[1, 2].map(x => x + \"px\").join(\" \")", "1px 2px");
  assert_folds_to_number("[1, 2, 3].reduce((total, x) => total + x, 0)", 6.0);
  assert_folds_to_number("[1, 2, 3].findIndex(x => x === 2)", 1.0);
  assert_folds_to_number("[[1], [2]].flat().length", 2.0);
}

/// The shape two separate method tables cannot agree on, and the reason the
/// receiver of a call is itself a candidate.
#[test]
fn a_chain_folds_at_every_link() {
  assert_folds_to_string("[\"a\", \"b\"].map(x => x).join(\"-\")", "a-b");
  assert_folds_to_string(
    "[\"1px\", \"solid\"].concat([\"red\"]).join(\" \")",
    "1px solid red",
  );
  assert_folds_to_string("\"  a-b  \".trim().replace(\"-\", \"_\")", "a_b");
}

// ==================== mutation stays refused ====================

/// The reference implementation folds these by accident of reflecting on a real
/// array. Issue 06 holds the divergence deliberately, so the guard has to state
/// it — and at every link, because a chain hides the mutating call in the middle.
#[test]
fn a_mutating_array_method_refuses_at_any_position_in_a_chain() {
  assert_deopts("[\"a\", \"b\"].push(\"c\")");
  assert_deopts("[\"b\", \"a\"].sort()");
  assert_deopts("[\"b\", \"a\"].sort().join(\"-\")");
  assert_deopts("[\"a\", \"b\"].reverse().join(\"-\")");
  assert_deopts("[1, 2, 3].splice(1).join(\"-\")");
  assert_deopts("[\"a\"].pop()");
  assert_deopts("[\"a\"].shift()");
  assert_deopts("[\"a\"].unshift(\"b\")");
}

// ==================== the scope the engine does not have ====================

#[test]
fn a_receiver_or_argument_that_needs_the_scope_refuses() {
  assert_deopts("someString.trim()");
  assert_deopts("\"a\".concat(runtimeValue)");
  assert_deopts("[runtimeValue].join(\"-\")");
  assert_deopts("[...spread].join(\"-\")");
  assert_deopts("\"a\".concat(...args)");
}

/// A callback may read its own parameters and nothing else. A block body is
/// refused rather than analysed: statements can bind, assign and loop, and the
/// guard does not model any of that.
#[test]
fn a_callback_that_escapes_its_parameters_refuses() {
  assert_folds_to_string("[\"a\"].map(x => x + \"!\").join(\"\")", "a!");
  assert_folds_to_number(
    "[\"ab\"].map(x => x.length).reduce((a, b) => a + b, 0)",
    2.0,
  );
  assert_deopts("[\"a\"].map(x => outer).join(\"\")");
  assert_deopts("[\"a\"].map(x => { return x; }).join(\"\")");
  assert_deopts("[\"a\"].map(({ x }) => x).join(\"\")");
}

// ==================== shapes with no static value ====================

#[test]
fn a_receiver_kind_the_evaluator_cannot_carry_refuses() {
  assert_deopts("/re/.test(\"a\")");
  assert_deopts("[\"a\"].map(x => /re/.test(x)).join(\"\")");
  assert_deopts("[, 1].join(\"-\")");
  assert_deopts("({ [key]: 1 }).hasOwnProperty(\"a\")");
  assert_deopts("({ ...rest }).hasOwnProperty(\"a\")");
  assert_deopts("\"a\"[method]()");
}

/// A computed property is a lookup the guard cannot resolve without the scope,
/// even when it is written as a literal.
#[test]
fn a_computed_method_name_refuses_even_when_it_is_a_literal() {
  assert_deopts("\"abc\"[\"trim\"]()");
  assert_deopts("[\"a\", \"b\"][\"join\"](\"-\")");
}

// ==================== the value domain ====================

/// The engine hands back everything JavaScript has, and only some of it is a
/// value this evaluator carries. An object, a function and `undefined` have no
/// literal, so the fold declines and the existing path answers.
#[test]
fn a_result_with_no_literal_form_refuses() {
  assert_deopts("({ a: 1 }).valueOf()");
  assert_deopts("[1, 2].entries()");
  assert_deopts("\"abc\".at(99)");
  assert_deopts("[1, 2].at(99)");
  assert_deopts("\"abc\".split(\"\").values()");
}

/// `NaN` and `Infinity` are numbers JavaScript produces and the reference
/// implementation folds. They are pinned because they are also invalid CSS: the
/// value that reaches the declaration is what upstream writes, and the choice to
/// keep matching it belongs to issue 06 rather than to this test.
///
/// **This contradicts `unsupported_shape_tests::char_code_at_past_the_end_
/// refuses_rather_than_aborting`, deliberately and visibly.** That test states
/// the decision this repo shipped: `NaN` is not a value the evaluator carries,
/// so the receiver refuses. The engine reaches the opposite answer, and it is
/// upstream's. Both are left standing rather than one quietly deleted, because
/// choosing between parity and a refusal an author can act on is issue 06's
/// call, and a spike that silently overwrote it would hide the decision it was
/// commissioned to inform.
#[test]
fn the_numeric_edges_fold_as_the_reference_implementation_folds_them() {
  assert_folds_to_nan("\"abc\".charCodeAt(10)");
  assert_folds_to_nan("\"abc\".charCodeAt(-1)");
  assert_folds_to_number("[1].reduce((total) => total / 0, 1)", f64::INFINITY);
  assert_folds_to_number("[-0].at(0)", 0.0);
}

#[test]
fn a_throwing_call_refuses_rather_than_aborting() {
  assert_deopts("[].reduce((a, b) => a + b)");
  assert_deopts("\"a\".repeat(-1)");
  assert_deopts("\"a\".padStart(\"x\").normalize(\"NFQ\")");
}

// ==================== unicode ====================

/// The engine carries UTF-16 and `Lit::Str` carries UTF-8, so a fold whose
/// result is an unpaired surrogate lands as the replacement character. Issue 06
/// pins that as a deliberate divergence; this asserts the compiler does it
/// rather than aborting.
#[test]
fn a_fold_whose_result_is_an_unpaired_surrogate_becomes_the_replacement_character() {
  assert_folds_to_string("\"\\u{1F600}a\".slice(1)", "\u{FFFD}a");
  assert_folds_to_string("\"\\uD83D\".concat(\"\")", "\u{FFFD}");
  assert_folds_to_number("\"\\u{1F600}\".length", 2.0);
}

#[test]
fn a_receiver_that_is_unicode_but_whole_folds_exactly() {
  assert_folds_to_string("\"café\".normalize(\"NFC\")", "café");
  assert_folds_to_string("\"ﬁ\".normalize(\"NFKC\")", "fi");
  assert_folds_to_string("\"a\\u0000b\".trim()", "a\u{0000}b");
  assert_folds_to_number("\"e\\u0301\".length", 2.0);
}

/// A quote, a backslash and a newline in the receiver survive the round trip
/// through printed source. If they did not, the engine would be handed a
/// different program than the author wrote.
#[test]
fn a_receiver_needing_escapes_survives_being_printed_back_to_source() {
  assert_folds_to_string("\"a\\\"b\".trim()", "a\"b");
  assert_folds_to_string("\"a\\\\b\".trim()", "a\\b");
  assert_folds_to_string("\"a\\nb\".trim()", "a\nb");
  assert_folds_to_number("\"a'b\".length", 3.0);
}

// ==================== reuse ====================

/// The engine is created once per thread and reused. Folding repeatedly in one
/// test proves the reuse path, and that state from one fold does not reach the
/// next.
#[test]
fn folding_many_times_reuses_one_engine_and_carries_nothing_between_folds() {
  for _ in 0..50 {
    assert_folds_to_string("\"  4px  \".trim()", "4px");
  }

  assert_folds_to_string("[\"a\"].join(\"\")", "a");
  assert_folds_to_string("\"  4px  \".trim()", "4px");
}
