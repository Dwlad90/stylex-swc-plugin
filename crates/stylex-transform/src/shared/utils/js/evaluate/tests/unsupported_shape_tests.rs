//! The panic/deopt split: an input shape the evaluator does not fold is an
//! ordinary answer, and answering it must never abort the build.
//!
//! Every case here reached `stylex_panic_with_context!` before the split, and
//! every one of them is an expression an author can write. The three logical
//! operators evaluate their right operand under a forked confidence — see
//! `nodes/logical_expression.rs` — so any of these in that position used to
//! take the whole compilation with it, which is what
//! [#1265](https://github.com/Dwlad90/stylex-swc-plugin/issues/1265) reported.
//!
//! The suite is deliberately two-sided: refusing everything would pass a
//! "nothing panics" test while quietly stopping the compiler from folding
//! anything, so each refusal group is paired with the folds it must not have
//! broken.

use super::*;
use stylex_structures::stylex_options::StyleXOptions;
use swc_core::{
  common::{FileName, GLOBALS, Globals, SourceMap, sync::Lrc},
  ecma::parser::{EsSyntax, Parser, StringInput, Syntax, lexer::Lexer},
};

/// Parses one expression, evaluates it, and reports what the evaluator made of
/// it. Panics propagate, which is the point: a test that reaches one fails.
fn evaluate_source(source: &str) -> Box<EvaluateResult> {
  let expr = parse_expr(source);
  let globals = Globals::new();

  GLOBALS.set(&globals, || {
    let mut traversal_state = StateManager::new(StyleXOptions::default());
    let fns = FunctionMap::default();

    evaluate(&expr, &mut traversal_state, &fns)
  })
}

/// Asserts the source refuses to fold, and does so as a deopt rather than by
/// aborting. The reason has to be there: `stylex.create()` turns it into the
/// author-facing diagnostic, so a refusal with no reason is a regression in
/// what a build error says.
#[track_caller]
fn assert_deopts(source: &str) {
  let result = evaluate_source(source);

  assert!(
    !result.confident,
    "expected `{}` to refuse to fold, got {:?}",
    source, result.value
  );

  assert!(
    result.reason.is_some(),
    "expected `{}` to record a deopt reason",
    source
  );
}

/// Asserts the source folds to a value. Guards the refusals above from being
/// satisfied by an evaluator that folds nothing at all.
#[track_caller]
fn assert_folds(source: &str) -> Expr {
  let result = evaluate_source(source);

  assert!(
    result.confident,
    "expected `{}` to fold, got a deopt: {:?}",
    source, result.reason
  );

  match result.value {
    Some(EvaluateResultValue::Expr(expr)) => expr,
    other => panic!(
      "expected `{}` to fold to an expression, got {:?}",
      source, other
    ),
  }
}

#[track_caller]
fn assert_folds_to_string(source: &str, expected: &str) {
  match assert_folds(source) {
    Expr::Lit(Lit::Str(strng)) => assert_eq!(
      convert_atom_to_string(&strng.value),
      expected,
      "wrong folded string for `{}`",
      source
    ),
    other => panic!("expected `{}` to fold to a string, got {:?}", source, other),
  }
}

#[track_caller]
fn assert_folds_to_number(source: &str, expected: f64) {
  match assert_folds(source) {
    Expr::Lit(Lit::Num(num)) => {
      assert_eq!(num.value, expected, "wrong folded number for `{}`", source)
    },
    other => panic!("expected `{}` to fold to a number, got {:?}", source, other),
  }
}

fn parse_expr(source: &str) -> Expr {
  let source_map: Lrc<SourceMap> = Default::default();
  let source_file = source_map.new_source_file(FileName::Anon.into(), source.to_string());

  let lexer = Lexer::new(
    Syntax::Es(EsSyntax {
      jsx: true,
      ..Default::default()
    }),
    Default::default(),
    StringInput::from(&*source_file),
    None,
  );

  match Parser::new_from(lexer).parse_expr() {
    Ok(expr) => *expr,
    Err(error) => panic!("failed to parse `{}`: {:?}", source, error),
  }
}

// ==================== the reported input ====================

/// The shape from #1265, reduced to the expression that aborted the build.
#[test]
fn a_string_method_the_evaluator_does_not_fold_refuses_rather_than_aborting() {
  assert_deopts("\"documentation\".startsWith(lowerQuery)");
}

/// The same call in the position that made it reachable. `1 > 0` is a
/// confident, truthy left side, so `&&` consults the right operand — which is
/// exactly the fork the panic escaped from.
#[test]
fn an_unfoldable_right_operand_of_and_refuses_rather_than_aborting() {
  assert_deopts("1 > 0 && \"documentation\".startsWith(lowerQuery)");
}

/// `||` consults its right operand only when the left is falsy.
#[test]
fn an_unfoldable_right_operand_of_or_refuses_rather_than_aborting() {
  assert_deopts("\"\" || \"documentation\".startsWith(lowerQuery)");
}

/// `??` consults its right operand when the left is nullish.
#[test]
fn an_unfoldable_right_operand_of_nullish_refuses_rather_than_aborting() {
  assert_deopts("null ?? \"documentation\".startsWith(lowerQuery)");
}

/// The property this suite is really about: whatever the evaluator cannot fold,
/// putting it on the right of one of the three logical operators must not
/// change the answer from "refused" to "aborted". Written as a sweep so a
/// newly added unfoldable shape is covered by construction rather than by
/// someone remembering to add three more tests.
#[test]
fn every_unfoldable_shape_survives_every_logical_operand_position() {
  const UNFOLDABLE: &[&str] = &[
    "\"documentation\".startsWith(q)",
    "\"abc\".normalize()",
    "[\"a\", \"b\"].reduce(f)",
    "[\"a\", \"b\"].at(0)",
    "(5).toFixed(2)",
    "true.toString()",
    "/re/.test(\"a\")",
    "Math.sin(1)",
    "Math.pow(\"a\", 2)",
    "Object.assign({}, {})",
    "Object.fromEntries(1)",
    "({}).hasOwnProperty(\"a\")",
    "tag`x`",
    "-({})",
    "({ ...1 })",
  ];

  for shape in UNFOLDABLE {
    assert_deopts(shape);
    assert_deopts(&format!("1 > 0 && {}", shape));
    assert_deopts(&format!("\"\" || {}", shape));
    assert_deopts(&format!("null ?? {}", shape));
  }
}

/// A left operand that decides the fold on its own is not made unconfident by
/// an unfoldable right one — the short-circuit still holds after the split.
#[test]
fn an_unconsulted_unfoldable_operand_does_not_refuse_the_fold() {
  assert_folds_to_string("\"blue\" || \"documentation\".startsWith(q)", "blue");
  assert_folds_to_string("\"\" && \"documentation\".startsWith(q)", "");
  assert_folds_to_string("\"blue\" ?? \"documentation\".startsWith(q)", "blue");
}

// ==================== receivers with no folded methods ====================

#[test]
fn a_method_call_on_a_receiver_kind_with_no_folds_refuses() {
  for source in [
    "(5).toFixed(2)",
    "(5.5).toPrecision(2)",
    "true.toString()",
    "false.valueOf()",
    "null?.toString()",
    "/re/.test(\"a\")",
    "/re/.exec(\"a\")",
    "(() => 1).call()",
    "({}).hasOwnProperty(\"a\")",
    "({}).constructor()",
  ] {
    assert_deopts(source);
  }
}

/// The two receivers whose methods this evaluator does fold keep folding.
#[test]
fn the_folded_string_and_array_methods_still_fold() {
  assert_folds_to_string("\"abc\".concat(\"d\")", "abcd");
  assert_folds_to_number("\"abc\".charCodeAt(0)", 97.0);
  assert_folds_to_string("[\"a\", \"b\"].join(\"-\")", "a-b");
}

// ==================== unicode and escapes ====================

/// A method the evaluator does not fold is refused whatever the receiver
/// holds. The receivers here are the ones a UTF-16 mistake would surface on:
/// an astral pair, a combining sequence, an escaped quote, a NUL.
#[test]
fn an_unfoldable_method_on_a_unicode_receiver_refuses_rather_than_aborting() {
  for receiver in [
    "\"\\u{1F600}a\"",
    "\"e\\u0301\"",
    "\"a\\\"b\"",
    "\"a\\u0000b\"",
    "\"\\uD83D\"",
  ] {
    assert_deopts(&format!("{}.normalize()", receiver));
    assert_deopts(&format!("1 > 0 && {}.padStart(4)", receiver));
  }
}

/// `charCodeAt` counts UTF-16 code units, and the folded path has to keep
/// saying so after the refusals around it moved.
#[test]
fn char_code_at_still_reads_utf16_code_units() {
  assert_folds_to_number("\"\\u{1F600}a\".charCodeAt(0)", 55357.0);
  assert_folds_to_number("\"\\u{1F600}a\".charCodeAt(1)", 56832.0);
  assert_folds_to_number("\"\\u{1F600}a\".charCodeAt(2)", 97.0);
}

/// Past the end is `NaN` in JavaScript, which this evaluator does not carry as
/// a folded value — so it refuses. It used to abort.
#[test]
fn char_code_at_past_the_end_refuses_rather_than_aborting() {
  assert_deopts("\"abc\".charCodeAt(10)");
  assert_deopts("\"abc\".charCodeAt(-1)");
  assert_deopts("\"abc\".charCodeAt(\"x\")");
  assert_deopts("1 > 0 && \"abc\".charCodeAt(99)");
}

// ==================== the globals ====================

#[test]
fn an_unfolded_math_method_refuses_and_the_folded_ones_still_fold() {
  for source in [
    "Math.sin(1)",
    "Math.hypot(1, 2)",
    "Math.random()",
    "Math.max()",
    "Math.min()",
    "Math.round()",
    "Math.pow(2)",
    "Math.pow(\"a\", 2)",
    "Math.abs({})",
    "Math.max(1, {})",
  ] {
    assert_deopts(source);
  }

  assert_folds_to_number("Math.pow(2, 3)", 8.0);
  assert_folds_to_number("Math.max(1, 5, 3)", 5.0);
  assert_folds_to_number("Math.min(1, 5, 3)", 1.0);
  assert_folds_to_number("Math.abs(-2)", 2.0);
  assert_folds_to_number("Math.round(1.5)", 2.0);
  assert_folds_to_number("Math.round(-1.5)", -1.0);
  assert_folds_to_number("Math.ceil(1.1)", 2.0);
  assert_folds_to_number("Math.floor(1.9)", 1.0);
}

#[test]
fn an_unfolded_object_method_refuses_and_the_folded_ones_still_fold() {
  for source in [
    "Object.assign({}, {})",
    "Object.freeze({})",
    "Object.fromEntries(1)",
    "Object.fromEntries([1])",
    "Object.fromEntries([[{}, 1]])",
    "Object.keys()",
  ] {
    assert_deopts(source);
  }

  assert_folds("Object.keys({ a: 1, b: 2 })");
  assert_folds("Object.values({ a: 1, b: 2 })");
  assert_folds("Object.entries({ a: 1, b: 2 })");
  assert_folds("Object.fromEntries([[\"a\", 1]])");
}

/// A spread argument to a callable global was already a refusal; it stays one.
#[test]
fn a_spread_argument_refuses() {
  assert_deopts("String(...[\"a\", \"b\"])");
  assert_deopts("Math.max(...[1, 2])");
  assert_deopts("Object.keys(...[{}])");
}

// ==================== object and member shapes ====================

#[test]
fn an_object_shape_with_no_compile_time_value_refuses() {
  for source in [
    "({ ...1 })",
    "({ ...(() => 1) })",
    "({ get a() { return 1 } }).a",
    "({ a: 1 })[/re/]",
    "({ a: 1 })[{}]",
  ] {
    assert_deopts(source);
  }
}

/// The two object lookups that answer rather than refuse keep answering: a key
/// the object carries, and a key it does not — the latter is `undefined`,
/// which is what lets `token.missing ?? fallback` fold.
#[test]
fn object_member_lookups_that_answer_still_answer() {
  assert_folds_to_number("({ a: 1 }).a", 1.0);
  assert_folds_to_string("({ a: 1 }).missing ?? \"red\"", "red");
  assert_folds_to_number("({ a: 1 })[\"a\"]", 1.0);
}

#[test]
fn an_array_index_that_is_not_a_number_refuses() {
  assert_deopts("[1, 2][{}]");
  assert_deopts("[1, 2][/re/]");
}

// ==================== expression kinds the evaluator has no fold for ====

#[test]
fn an_expression_kind_with_no_fold_refuses() {
  for source in [
    "tag`x`",
    "String.raw`x`",
    "(function () {})()",
    "class {}",
    "new Date()",
    "(async () => 1)()",
    "1n + 1n",
  ] {
    assert_deopts(source);
  }
}

#[test]
fn a_unary_operator_over_a_value_with_no_numeric_reading_refuses() {
  for source in ["-({})", "+({})", "~({})", "-[1, 2, 3]"] {
    assert_deopts(source);
  }

  assert_folds_to_number("-5", -5.0);
  assert_folds_to_number("+\"5\"", 5.0);
  assert_folds_to_string("typeof \"a\"", "string");
  assert_folds_to_string("typeof 1", "number");
  assert_folds_to_string("typeof undefined", "undefined");
  assert_folds_to_string("typeof (1, {})", "object");
}

#[test]
fn a_conditional_whose_test_has_no_compile_time_truthiness_refuses() {
  assert_deopts("(runtimeFlag ? \"a\" : \"b\")");
  assert_folds_to_string("(1 ? \"a\" : \"b\")", "a");
  assert_folds_to_string("(0 ? \"a\" : \"b\")", "b");
}

// ==================== boundaries and malformed input ====================

/// Deep nesting is the shape most likely to turn a refusal into a stack
/// overflow, because each operand recurses. A hundred levels is far past
/// anything a stylesheet contains and well inside the default stack.
#[test]
fn a_deeply_nested_refusal_stays_a_refusal() {
  let deep = std::iter::repeat_n("1 > 0 && ", 100).collect::<String>();

  assert_deopts(&format!("{}\"abc\".normalize()", deep));
}

#[test]
fn a_deeply_nested_fold_still_folds() {
  let deep = std::iter::repeat_n("1 > 0 && ", 100).collect::<String>();

  assert_folds_to_string(&format!("{}\"red\"", deep), "red");
}

/// A chain long enough to matter for the argument-collecting loops the split
/// rewrote, in both directions.
#[test]
fn a_long_argument_list_folds_and_a_long_one_with_a_bad_argument_refuses() {
  let numbers = (0..256)
    .map(|n| n.to_string())
    .collect::<Vec<_>>()
    .join(", ");

  assert_folds_to_number(&format!("Math.max({})", numbers), 255.0);
  assert_deopts(&format!("Math.max({}, {{}})", numbers));
}

/// Array holes are absent values rather than a broken invariant; reading one
/// must not abort. Indexing an array literal refuses whether the slot is a
/// hole or not — see the note in `nodes/member_expression.rs` on why the `Vec`
/// representation is not indexed — so what is asserted here is that both
/// answers are refusals rather than aborts.
#[test]
fn array_holes_do_not_abort() {
  assert_deopts("[, 1][0]");
  assert_deopts("[, 1][1]");
  assert_deopts("1 > 0 && [, 1].at(0)");
  assert_folds_to_string("[\"a\", 1].join(\"-\")", "a-1");
}

/// An identifier with no binding is the evaluator's oldest refusal and the one
/// every other refusal has to keep behaving like.
#[test]
fn an_unresolved_identifier_still_refuses() {
  assert_deopts("someRuntimeValue");
  assert_deopts("someRuntimeValue.length");
  assert_deopts("someRuntimeValue()");
  assert_deopts("1 > 0 && someRuntimeValue");
}

/// A block-bodied arrow has no compile-time value — `nodes/arrow_function.rs`
/// folds only expression bodies — so an arithmetic operand that is one refuses.
/// Pinned in every operand position because the binary paths read a missing
/// operand differently depending on which coercion claimed the operator.
#[test]
fn an_operand_with_no_value_refuses_rather_than_aborting() {
  assert_deopts("(() => { return 1 }) + 1");
  assert_deopts("1 + (() => { return 1 })");
  assert_deopts("(() => { return 1 }) * 2");
  assert_deopts("1 > 0 && (() => { return 1 }) + 1");
}

/// A receiver element the evaluator holds but cannot write down leaves the
/// whole receiver unreadable, so `Object.keys`/`values`/`entries` refuse.
///
/// Answering the short list instead is the failure this suite exists to
/// prevent, and it is the one a refusal can slide into unnoticed: the fold
/// still succeeds, the build still passes, and the stylesheet gets a value the
/// source never described. `Object.keys([x => x])` has one own key in
/// JavaScript, so `[]` would be wrong rather than merely incomplete.
#[test]
fn an_unreadable_receiver_element_refuses_rather_than_shortening_the_list() {
  for receiver in ["[x => x]", "[[x => x]]", "[1, x => x]"] {
    assert_deopts(&format!("Object.keys({})", receiver));
    assert_deopts(&format!("Object.values({})", receiver));
    assert_deopts(&format!("Object.entries({})", receiver));
    assert_deopts(&format!("1 > 0 && Object.keys({})", receiver));
  }
}

/// The receivers around it still fold, including the two that are absent for
/// opposite reasons: a hole has no own key, and a non-object has none either —
/// `Object.keys(5)` is `[]` in JavaScript and must not be mistaken for the
/// refusal above.
#[test]
fn a_readable_object_method_receiver_still_folds() {
  assert_folds("Object.keys([1, 2])");
  assert_folds("Object.values([1, 2])");
  assert_folds("Object.entries([1, 2])");
  assert_folds("Object.keys([, 1])");
  assert_folds("Object.keys([[1, 2]])");
  assert_folds("Object.keys(5)");
  assert_folds("Object.keys(\"ab\")");
}
