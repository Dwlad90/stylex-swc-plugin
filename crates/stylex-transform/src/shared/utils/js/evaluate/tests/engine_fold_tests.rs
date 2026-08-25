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
/// implementation folds, so they reach the declaration here too. Why parity
/// wins over the more useful refusal is argued once, at
/// `unsupported_shape_tests::char_code_at_past_the_end_folds_to_nan_as_the_
/// reference_implementation_does`.
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

// ==================== the boundaries around the surface ====================

/// A locale-sensitive method is refused rather than answered from the root
/// locale. The reference implementation folds all four, so refusing costs
/// parity — but the engine carries no locale data, and it answers
/// `"i".toLocaleUpperCase("tr")` as `I` where the language says `İ`. A refused
/// fold leaves the expression alone; a wrong fold writes a wrong stylesheet.
#[test]
fn a_locale_sensitive_method_refuses_rather_than_answering_from_the_root_locale() {
  assert_deopts("\"i\".toLocaleUpperCase(\"tr\")");
  assert_deopts("\"I\".toLocaleLowerCase(\"tr\")");
  assert_deopts("\"ä\".localeCompare(\"z\", \"de\")");
  assert_deopts("\"ab\".toLocaleString()");
  assert_deopts("({ a: 1 }).toLocaleString()");

  // Refused at every link of a chain, like mutation, and under a logical
  // operand, which is the position that used to abort the build.
  assert_deopts("\"ab\".toLocaleUpperCase().trim()");
  assert_deopts("\"a,b\".split(\",\").map(x => x.toLocaleUpperCase()).join(\"\")");
  assert_deopts("1 > 0 && \"i\".toLocaleUpperCase(\"tr\")");

  // `normalize` is not locale-sensitive — it reads the Unicode tables the
  // engine does carry — so it stays in scope and agrees with the language.
  assert_folds_to_string("\"ﬁ\".normalize(\"NFKC\")", "fi");
}

/// The engine bounds loops, recursion and stack, but not allocation, so a
/// single argument is enough to make a fold exhaust memory. These refuse
/// instead. Upstream folds them, so this is a deliberate divergence: a typo
/// costs a refusal rather than the machine.
#[test]
fn a_length_no_declaration_could_use_refuses_rather_than_being_built() {
  assert_deopts("\"x\".repeat(200000000)");
  assert_deopts("\"x\".padStart(50000000)");
  assert_deopts("\"x\".padEnd(50000000)");

  // Per-call bounds alone are multiplied by a chain, so an amplifying call on
  // a receiver that is itself a call refuses whatever the counts are.
  assert_deopts("\"x\".repeat(1000).repeat(1000)");
  assert_deopts("\"x\".repeat(2).padStart(4, \"y\")");

  // A count that is not written as a number cannot be bounded by reading it,
  // and a spread stands for however many arguments it holds, so neither can it.
  assert_deopts("\"x\".repeat([1000].length)");
  assert_deopts("\"x\".repeat(2 * 2)");
  assert_deopts("\"x\".repeat(...[2])");
  assert_deopts("\"x\".padStart(...[4, \"0\"])");

  // Under the bound, and the no-argument form that amplifies nothing.
  assert_folds_to_string("\"ab\".repeat(2)", "abab");
  assert_folds_to_number("\"x\".repeat(999999).length", 999_999.0);
  assert_folds_to_string("\"7\".padStart(3, \"0\")", "007");
  assert_folds_to_string("\"x\".padStart()", "x");
  assert_folds_to_string("\"x\".padEnd(4, \"-\")", "x---");
}

/// A bounded string can still become one array element per code unit, which
/// costs far more as a tree than it did as text.
#[test]
fn an_array_result_longer_than_a_declaration_could_use_refuses() {
  assert_deopts("\"x\".repeat(999999).split(\"\")");
  assert_deopts("\"x\".repeat(10001).split(\"\")");

  assert_folds_to_number("\"x\".repeat(10000).split(\"\").length", 10_000.0);
  assert_folds_to_string("[\"a\", \"b\"].slice(0, 1).join(\"\")", "a");
}

/// Nesting is not free for whoever parses it: past a hundred levels or so the
/// engine's parser overflows its stack, and an overflow inside an evaluation
/// that is allowed to fail aborts the build instead of reporting anything. The
/// guard refuses first, so the answer stays a refusal at any depth.
///
/// Asserted well past the bound as well as just over it, because the failure
/// this prevents gets *more* likely as the input gets deeper, so a test that
/// stopped at the boundary would not be testing the crash.
#[test]
fn nesting_past_the_bound_refuses_rather_than_overflowing_a_stack() {
  for levels in [33, 100, 400, 900] {
    let nested = format!(
      "{}[\"a\"]{}.join(\"\")",
      "[".repeat(levels),
      "]".repeat(levels)
    );

    assert_deopts(&nested);
  }

  // Depth reached through the other nesting shapes the walk accepts, not only
  // through an array: a chain of calls, a chain of member reads, and nested
  // objects. Each counts the same, because each is a level the printed source
  // makes the engine's parser descend through.
  assert_deopts(&format!("\"a\"{}", ".concat(\"b\")".repeat(400)));
  assert_deopts(&format!(
    "({}\"x\"{}){}.toUpperCase()",
    "{ a: ".repeat(40),
    " }".repeat(40),
    ".a".repeat(40)
  ));
  assert_deopts(&format!("(1{}).toFixed(1)", " + 1".repeat(400)));

  // Just inside the bound still folds, so the refusals above are the depth and
  // not the shape.
  let shallow = format!("{}[\"a\"]{}.join(\"\")", "[".repeat(4), "]".repeat(4));
  assert_folds_to_string(&shallow, "a");
}

/// The object prototype the reference implementation reaches by reflection.
/// `valueOf` answers the object itself, which has no literal form, so it is the
/// one that refuses — and refuses for that reason rather than by name.
#[test]
fn the_object_prototype_methods_fold() {
  assert_folds_to_boolean("({ a: 1 }).hasOwnProperty(\"a\")", true);
  assert_folds_to_boolean("({ a: 1 }).hasOwnProperty(\"b\")", false);
  assert_folds_to_boolean("({ a: 1 }).propertyIsEnumerable(\"a\")", true);
  assert_folds_to_boolean("({ a: 1 }).isPrototypeOf({})", false);
  assert_folds_to_string("({ a: 1 }).toString()", "[object Object]");
  assert_folds_to_string("({ \"a-b\": 1, 2: 3 }).toString()", "[object Object]");
  assert_deopts("({ a: 1 }).valueOf()");
}
