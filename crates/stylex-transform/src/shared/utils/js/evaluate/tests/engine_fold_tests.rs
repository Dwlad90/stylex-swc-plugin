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

// ==================== mutation ====================

/// A mutating method folds, at every link of a chain, exactly as it does under
/// the reference implementation.
///
/// It was refused here on the reasoning that matching would carry mutation into
/// an otherwise pure evaluation. Measured, that reasoning does not hold: the
/// reference implementation does not refuse mutating methods at all — it folds
/// them on any receiver not reachable by name, and disqualifies the **binding**
/// instead. The engine therefore only ever mutates a temporary nothing can name
/// afterwards, which is unobservable, and the rule that does the work lives in
/// binding resolution rather than here. `resolution_order` pins that half.
#[test]
fn a_mutating_array_method_folds_at_any_position_in_a_chain() {
  assert_folds_to_number("[\"a\", \"b\"].push(\"c\")", 3.0);
  assert_folds_to_string("[\"b\", \"a\"].sort().join(\",\")", "a,b");
  assert_folds_to_string("\"b,a\".split(\",\").sort().join(\",\")", "a,b");
  assert_folds_to_string("[\"a\", \"b\"].reverse().join(\"-\")", "b-a");
  assert_folds_to_string("[1, 2, 3].splice(1).join(\"-\")", "2-3");
  assert_folds_to_string("[\"a\"].pop()", "a");
  assert_folds_to_string("[\"a\", \"b\"].shift()", "a");
  assert_folds_to_number("[\"a\"].unshift(\"b\")", 2.0);
  assert_folds_to_string("[3, 1, 2].sort().reverse().join(\"\")", "321");

  // The receiver is a fresh value each time, so a fold is not carried into the
  // next one: the same expression folded twice answers the same thing.
  assert_folds_to_string("[\"b\", \"a\"].sort().join(\",\")", "a,b");
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
/// value this evaluator carries. What it declines, it names with the
/// language's own `typeof` — so a kind nobody here thought of is still named,
/// rather than falling into a catch-all this module would have to maintain.
#[test]
fn a_result_with_no_literal_form_refuses() {
  // `undefined` is what a read past the end answers.
  assert_deopt_reason_contains("\"abc\".at(99)", "folded undefined");
  assert_deopt_reason_contains("[1, 2].at(99)", "folded undefined");

  // A function, which a method read off a value is. Read through a callback
  // parameter, because `bind` — the other way to reach one — is refused before
  // the conversion is asked anything.
  assert_deopt_reason_contains("[\"a\"].reduce((a, b) => b.trim, 0)", "folded function");

  // An iterator is an object, and not a plain one. `typeof` says only
  // `object`, so the sentence that says which objects fold is what tells an
  // author theirs is not one of them.
  assert_deopt_reason_contains("[1, 2].entries()", "folded object");
  assert_deopt_reason_contains(
    "\"abc\".split(\"\").values()",
    "plain objects can be folded",
  );
}

/// A plain object crosses back and reaches the same places an object the author
/// wrote reaches, which is what makes a fold's result as usable as a value
/// somebody typed. The reference implementation folds each of these to the same
/// declaration.
#[test]
fn a_plain_object_result_crosses_back() {
  assert_folds_to_object_keys("({ a: 1 }).valueOf()", &["a"]);
  assert_folds_to_object_keys(
    "[\"red\"].reduce((o, v) => ({ default: v, \":hover\": \"blue\" }), {})",
    &["default", ":hover"],
  );

  // An empty one, and one whose properties are themselves folded values.
  assert_folds_to_object_keys("({}).valueOf()", &[]);
  assert_folds_to_object_keys(
    "[1].reduce((o, v) => ({ list: [v, v + 1], nested: { deep: v } }), {})",
    &["list", "nested"],
  );
}

/// Own-key order is the language's, and is produced by the same ordering an
/// object the author wrote goes through: integer-like keys ascending first,
/// then the rest in insertion order. Asserted rather than assumed, because two
/// implementations of own-key order agreeing today says nothing about tomorrow.
#[test]
fn an_object_result_carries_the_own_key_order_the_language_gives_it() {
  assert_folds_to_object_keys(
    "[1].reduce((o) => ({ b: 1, 2: 2, a: 3, 1: 4 }), {})",
    &["1", "2", "b", "a"],
  );
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
#[test]
fn the_object_prototype_methods_fold() {
  assert_folds_to_boolean("({ a: 1 }).hasOwnProperty(\"a\")", true);
  assert_folds_to_boolean("({ a: 1 }).hasOwnProperty(\"b\")", false);
  assert_folds_to_boolean("({ a: 1 }).propertyIsEnumerable(\"a\")", true);
  assert_folds_to_boolean("({ a: 1 }).isPrototypeOf({})", false);
  assert_folds_to_string("({ a: 1 }).toString()", "[object Object]");
  assert_folds_to_string("({ \"a-b\": 1, 2: 3 }).toString()", "[object Object]");
}

// ==================== the boundaries around a folded value ====================

/// Exactly at the bound still folds, so the refusal
/// `engine_fold_refusals::an_object_result_past_the_bound_names_the_bound`
/// pins is the count and not the shape. The two halves sit at different seams
/// because only one of them has a sentence to assert.
#[test]
fn an_object_result_at_the_bound_still_folds() {
  assert_folds(&object_of(10_000));
}

/// One object literal of `count` properties, as a receiver whose method answers
/// the object itself.
fn object_of(count: usize) -> String {
  let props: Vec<String> = (0..count)
    .map(|index| format!("k{}:{}", index, index))
    .collect();

  format!("({{{}}}).valueOf()", props.join(","))
}

/// A value can be nested deeper on the way *out* than any expression the guard
/// admits on the way in, because a loop inside the engine builds it rather than
/// syntax the author wrote. The conversion recurses on the bare thread stack,
/// so it is bounded for the reason the input is bounded — and says so with the
/// same sentence.
#[test]
fn a_value_the_engine_nested_past_the_bound_refuses_rather_than_overflowing_a_stack() {
  let nest = |levels: usize| {
    format!(
      "\"x\".repeat({}).split(\"\").reduce((a, c) => [a], [])",
      levels
    )
  };

  assert_deopt_reason_contains(&nest(40), "too deeply nested");
  assert_deopt_reason_contains(&nest(400), "too deeply nested");

  // Well inside the bound, so the refusals above are the depth and not the
  // shape: the same expression at four levels folds.
  assert_folds_to_a_value(&nest(4));
}

/// A key an object literal cannot spell as an identifier survives being carried
/// back, because it is carried as the key node the object evaluation uses and
/// not re-parsed as source.
#[test]
fn a_folded_object_carries_keys_no_identifier_could_spell() {
  assert_folds_to_object_keys(
    "({ \"a-b\": 1, \"\": 2, \"ä ü\": 3 }).valueOf()",
    &["a-b", "", "ä ü"],
  );
  assert_folds_to_object_keys(
    "({ \"a\\nb\": 1, \"a\\\"b\": 2 }).valueOf()",
    &["a\nb", "a\"b"],
  );
  assert_folds_to_object_keys(
    "({ \"@media (min-width: 1px)\": 1 }).valueOf()",
    &["@media (min-width: 1px)"],
  );
}

/// The kinds of input that are not a fold at all have to leave the existing
/// dispatch in charge rather than raise a refusal of their own, because that
/// dispatch is what folds a call to a global — `String(1)` — and what answers
/// for a receiver the bridge cannot carry.
#[test]
fn a_shape_the_guard_never_recognised_leaves_the_existing_path_in_charge() {
  assert_folds_to_string("String(1)", "1");
  assert_folds_to_a_value("Object.keys([, 1])");
}

// ==================== the boundaries, at their own value ====================

/// `Function` is two named property reads away from any literal, and a body
/// reached that way is arbitrary code running inside the compiler: it answers a
/// different number on every build and it can assign to a prototype the next
/// fold will read. The reference implementation folds all of this; matching it
/// would mean a class name that is not a function of the source.
#[test]
fn a_property_that_reaches_the_function_constructor_refuses() {
  assert_deopts("\"\".constructor.constructor(\"return 1\").call()");
  assert_deopts("[1].constructor.constructor(\"return Date.now()\").call()");
  assert_deopts("\"a\".trim.call(\"  b  \")");
  assert_deopts("\"a\".trim.apply(\"  b  \")");
  assert_deopts("[1].map(\"\".trim.bind(\"  b  \")).join(\"\")");
}

/// The engine reuses one context per thread, so a fold that writes to a
/// prototype would be read by every later fold in the build — including one in
/// another file. The guard is what makes the reuse safe, so the property is
/// asserted through the guard rather than by trusting it.
#[test]
fn a_fold_cannot_write_to_a_prototype_the_next_fold_reads() {
  assert_folds_to_string("\"  x  \".trim()", "x");
  assert_deopts(
    "\"\".constructor.constructor(\
     \"String.prototype.trim = function () { return 'poisoned'; }; return 1\").call()",
  );
  assert_folds_to_string("\"  x  \".trim()", "x");
}

/// An amplifying call is bounded by an argument written into the source, which
/// bounds one evaluation. A callback runs once per element, so the same written
/// bound is multiplied by a length the guard never measured.
#[test]
fn an_amplifying_call_inside_a_callback_refuses_however_small_its_argument() {
  assert_deopts("\"x\".repeat(4).split(\"\").map(c => c.repeat(4)).join(\"\")");
  assert_deopts("[\"1\", \"2\"].map(x => x.padStart(2, \"0\")).join(\"\")");
  assert_deopts("[\"1\"].map(x => x.padEnd(2, \"0\")).join(\"\")");

  // The same call outside a callback is still bounded by its argument alone.
  assert_folds_to_string("\"1\".padStart(2, \"0\")", "01");
}

/// The amplification bound at its own value. Every other case is orders of
/// magnitude away from it, so the comparison could be the wrong one and they
/// would all still answer the same way.
#[test]
fn the_amplified_length_bound_admits_its_own_value_and_refuses_one_past_it() {
  assert_folds_to_number("\"x\".repeat(1000000).length", 1_000_000.0);
  assert_deopts("\"x\".repeat(1000001)");
  assert_deopts("\"x\".padStart(1000001)");
}

/// The bound on the input says nothing about the depth of the answer: two
/// elements per level is never wide enough to trip the width bound, and each
/// element is one level deeper. Overflowing there aborts rather than unwinds,
/// which is the one failure a fold allowed to decline must not have.
#[test]
fn a_result_nested_deeper_than_the_bound_refuses_rather_than_overflowing_a_stack() {
  let deep = format!(
    "[{}].reduce((a, b) => [a, b]).length",
    (0..2000)
      .map(|index| index.to_string())
      .collect::<Vec<_>>()
      .join(",")
  );

  assert_deopts(&deep);

  // Shallow enough to represent still folds, so the refusal is the depth.
  assert_folds_to_number("[0, 1, 2].reduce((a, b) => [a, b]).length", 2.0);
}

/// A conditional carries its own value when all three of its parts do, which is
/// the one nesting shape the walk accepts that no other case here reaches.
#[test]
fn a_receiver_written_as_a_conditional_folds_and_one_that_needs_the_scope_refuses() {
  assert_folds_to_string("(1 > 0 ? \"ab\" : \"c\").trim()", "ab");
  assert_deopts("(runtimeFlag ? \"a\" : \"b\").trim()");
  assert_deopts("(1 > 0 ? runtimeValue : \"b\").trim()");
}

/// An object receiver is a key and a value written out. A shorthand reads the
/// scope, and a method or an accessor is a function body the guard does not
/// model, so all three are refused by their shape rather than by what they hold.
#[test]
fn an_object_receiver_written_as_anything_but_a_key_and_a_value_refuses() {
  assert_deopts("({ shorthand }).hasOwnProperty(\"shorthand\")");
  assert_deopts("({ method() { return 1; } }).hasOwnProperty(\"method\")");
  assert_deopts("({ get a() { return 1; } }).hasOwnProperty(\"a\")");
}

/// The two mutating names that are neither a `push` nor a `sort`: they mutate
/// in place and answer the receiver, so a chain hides them as completely — and
/// they fold, for the reason every other mutating name folds, which
/// `a_mutating_array_method_folds_at_any_position_in_a_chain` argues.
#[test]
fn the_remaining_mutating_array_methods_fold_too() {
  assert_folds_to_string("[\"a\", \"b\"].fill(\"c\").join(\"\")", "cc");
  assert_folds_to_string("[\"a\", \"b\"].copyWithin(0).join(\"\")", "ab");
}

/// The array a fold hands back, asserted as the array. Every other array case
/// here reads a `length` or a `join` that the engine answered before the
/// conversion ran, so none of them would notice elements arriving in the wrong
/// order or one going missing on the way out.
#[test]
fn a_fold_whose_result_is_an_array_carries_every_element_in_order() {
  assert_folds_to_strings("\"a,b\".split(\",\")", &["a", "b"]);
  assert_folds_to_strings(
    "[\"1px\", \"solid\"].concat([\"red\"])",
    &["1px", "solid", "red"],
  );
  assert_folds_to_strings("[\"a\", \"b\", \"c\"].slice(1)", &["b", "c"]);
}

/// `null` is a value the evaluator carries, and the only one of JavaScript's
/// empty answers with a literal to carry it in: `undefined` next door has none
/// and refuses. Asserted apart from the rest because the two leave the
/// conversion through different arms.
#[test]
fn a_fold_whose_result_is_null_carries_null_and_undefined_still_refuses() {
  assert_folds_to_null("[null].at(0)");
  assert_deopts("[undefined].at(0)");
}

/// A callee that is not an expression at all — `import` is the one an author
/// writes — never reaches the member check, so its refusal is its own arm.
#[test]
fn a_call_whose_callee_is_not_an_expression_refuses() {
  assert_deopts("import(\"./a\").then(x => x)");
}

/// The level the guard stops accepting at, with the evaluator's own ceiling
/// raised past it so the guard is what answers.
///
/// Both sides of the bound are pinned under the raised ceiling, so what the
/// refusal measures is the depth and not the shape — the older path would fold
/// the second of these if the guard handed it back, which is the fold this
/// bound costs and `Depth` argues.
#[test]
fn nesting_one_past_the_bound_refuses_under_a_ceiling_that_admits_it() {
  let admitted = format!("{}[\"a\"]{}.join(\"\")", "[".repeat(30), "]".repeat(30));
  let refused = format!("{}[\"a\"]{}.join(\"\")", "[".repeat(31), "]".repeat(31));

  assert_folds_to_string_with_ceiling(&admitted, "a", 512);
  assert_deopts_with_ceiling(&refused, 512);
}

/// Depth is answered as a refusal rather than as "not mine", and that costs a
/// fold. The cost is deliberate, and both halves of it are measured here
/// because the reasoning is not obvious from either half alone.
///
/// The bound is this module's own — the engine's parser recurses on the bare
/// thread stack — so it does not move when a project raises the evaluator's
/// ceiling. Under a raised ceiling the older path *would* fold the input below,
/// so answering "not mine" instead of refusing would hand it back and fold it.
/// That is the fold this refusal costs.
///
/// It is taken because the two ceilings no longer carry the same number, and a
/// bound this module owns has to answer in this module's words rather than let
/// which sentence an author reads depend on which of two disagreeing ceilings
/// they crossed. Handing it back is at least safe: the nested array that
/// reached the older `join` refuses rather than panicking, which
/// `engine_fold_refusals::a_nested_array_reaching_the_older_join_refuses_
/// rather_than_panicking` pins. Ticket 11 owns unifying the two ceilings.
#[test]
fn depth_refuses_rather_than_handing_a_deep_expression_back_to_the_older_path() {
  let deep = "1 > 0 && ".repeat(40);

  assert_deopts_with_ceiling(&format!("\"a\".concat({}\"b\")", deep), 512);

  // The same shape inside the bound folds under the same ceiling, so what the
  // refusal above measures is the depth and not the shape.
  assert_folds_to_string_with_ceiling(
    &format!("\"a\".concat({}\"b\")", "1 > 0 && ".repeat(4)),
    "ab",
    512,
  );
}

// ==================== what the guard costs ====================

/// The engine is built on the first fold that needs one and never before, which
/// is what makes the fold free for a file that folds nothing.
///
/// Asserted from a thread holding no engine, because the claim is about an input
/// the guard declined leaving the slot empty — a test that ran after another
/// fold on the same thread would find the engine that fold built and pass for
/// the wrong reason.
///
/// Each input below is declined for a different reason, so the claim covers the
/// three ways a call leaves the guard: not a candidate, refused by a name rule,
/// and refused by a rule that had to read a binding first.
#[test]
fn input_with_no_foldable_call_builds_no_engine() {
  for source in [
    "\"a\" + \"b\"",
    "someString.trim()",
    "\"abc\"[\"trim\"]()",
    "\"i\".toLocaleUpperCase(\"tr\")",
    "(1.5).toFixed(1)",
    "String(1)",
  ] {
    super::engine_fold::forget_engine();

    let _ = evaluate_source(source);

    assert!(
      !super::engine_fold::holds_an_engine(),
      "`{}` built an engine",
      source
    );
  }

  // And the guard that makes the above mean something: a call the fold does take
  // builds one.
  super::engine_fold::forget_engine();

  assert_folds_to_string("\"  4px  \".trim()", "4px");
  assert!(super::engine_fold::holds_an_engine());
}
