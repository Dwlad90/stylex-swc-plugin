//! The operand a short-circuiting form never evaluates, as the fold's guard
//! walks it.
//!
//! Why the walk has to stop where the language stops is
//! [Dead operand](../../../../../../CONTEXT.md#dead-operand). Every case here
//! puts a leaf the guard *would* refuse on the side that never runs, which makes
//! the fold succeeding the proof that the walk never got there. `constructor` is
//! the leaf of choice because its refusal has a sentence of its own, so a case
//! that lost the laziness fails loudly instead of merely deopting.
//!
//! Every value below is `@stylexjs/babel-plugin@0.19.0`'s own for the same
//! source, and the three places it is not are marked where they stand. The
//! reference implementation is *eager* here — it evaluates both sides of a
//! logical expression under forked states so a dead one may fail without
//! deopting the whole — where this compiler's logical node is lazy. Same answers
//! wherever the reference has one; the difference shows only where evaluating a
//! branch nothing runs is what breaks it.

use super::source_evaluation::*;

// ==================== the side that never runs ====================

/// `&&` behind a falsy left side, which is the reported shape. `false` is what
/// the operator answers and what `join` then writes, exactly as the language
/// does.
#[test]
fn and_never_reaches_its_right_operand_behind_a_falsy_guard() {
  assert_folds_to_string("['a', false && 'x'.constructor].join('-')", "a-false");
  assert_folds_to_string("['a', '' && 'x'.constructor].join('-')", "a-");
  assert_folds_to_string("['a', 0 && 'x'.constructor].join('-')", "a-0");
  // `join` renders `null` as the empty string, which is why this row reads like
  // the empty one above rather than like `a-null`.
  assert_folds_to_string("['a', null && 'x'.constructor].join('-')", "a-");
}

/// `||` behind a truthy left side.
#[test]
fn or_never_reaches_its_right_operand_behind_a_truthy_guard() {
  assert_folds_to_string("['a', 'b' || 'x'.constructor].join('-')", "a-b");
  assert_folds_to_string("['a', 1 || 'x'.constructor].join('-')", "a-1");
  assert_folds_to_string("['a', true || 'x'.constructor].join('-')", "a-true");
}

/// `??` behind a left side that is neither `null` nor `undefined` *and* is
/// truthy — the only left side this operator settles on its own.
#[test]
fn nullish_never_reaches_its_right_operand_behind_a_set_guard() {
  assert_folds_to_string("['a', 'b' ?? 'x'.constructor].join('-')", "a-b");
  assert_folds_to_string("['a', 1 ?? 'x'.constructor].join('-')", "a-1");
}

/// The conditional form, on each of its arms.
#[test]
fn a_conditional_never_reaches_the_arm_it_does_not_take() {
  assert_folds_to_string("[true ? 'a' : 'x'.constructor].join('')", "a");
  assert_folds_to_string("[false ? 'x'.constructor : 'a'].join('')", "a");
  assert_folds_to_string("['' ? 'x'.constructor : 'a'].join('')", "a");
  assert_folds_to_string("[[] ? 'a' : 'x'.constructor].join('')", "a");
}

/// A guard that is itself a call, so the decision is made from a value the
/// evaluator had to fold before it could be read.
#[test]
fn a_guard_the_evaluator_has_to_fold_first_still_decides() {
  assert_folds_to_string(
    "['a', 'abc'.startsWith('z') && 'x'.constructor].join('-')",
    "a-false",
  );
  assert_folds_to_string(
    "['a', [1, 2].length > 1 || 'x'.constructor].join('-')",
    "a-true",
  );
}

/// Short circuits nested through each other: the innermost dead operand is
/// reached only by deciding the two above it, and none of them is walked.
#[test]
fn nested_short_circuits_are_decided_from_the_outside_in() {
  assert_folds_to_string(
    "[((false && 'x'.constructor) || (true ? 'a' : 'x'.constructor)) || 'x'.constructor].join('')",
    "a",
  );
}

// ==================== what the dead side may contain ====================

/// A dead operand that would have *thrown* rather than been refused. The
/// language does not throw here either, and the fold agrees with it.
///
/// The reference implementation aborts the build on the first of these with
/// `Cannot read properties of null (reading 'x')`, which is its eagerness
/// reaching a branch no build runs. Not reproduced: laziness is the language's
/// own answer, and a crash is not a behaviour worth matching.
#[test]
fn a_dead_operand_that_would_throw_does_not() {
  assert_folds_to_string("['a', false && null.x].join('-')", "a-false");
  assert_folds_to_string("[true ? 'a' : undefined.x].join('')", "a");
}

/// A dead operand that would have been priced past the allocation ceiling. The
/// walk never asks what it costs, so the ceiling is never spent on it.
#[test]
fn a_dead_operand_past_the_allocation_ceiling_is_never_priced() {
  assert_folds_to_string("['a', false && 'x'.repeat(400000000)].join('-')", "a-false");
  assert_folds_to_string("[true ? 'a' : 'x'.repeat(400000000)].join('')", "a");
}

/// A dead operand nested far deeper than the evaluator's own ceiling. Depth is
/// spent per step of the walk, so a side the walk does not enter costs none of
/// it — which is the difference between this folding and refusing for depth.
#[test]
fn a_dead_operand_deeper_than_the_ceiling_is_never_entered() {
  let deep = "[".repeat(200) + "'x'" + &"]".repeat(200);

  assert_folds_to_string(&format!("['a', false && {}].join('-')", deep), "a-false");
}

// ==================== where the walk may not decide ====================

/// A left side the operator itself declines to decide — falsy but not nullish
/// under `??` — is not a short circuit the walk may read either. Both sides are
/// walked and the refusal on the dead-looking one stands, which is the
/// conservative half of the rule: a guess here would put the wrong operand in
/// the stylesheet.
///
/// The reference implementation aborts on the first of these too, with
/// `Unsupported expression: CallExpression` — the same defect
/// [#1265](https://github.com/Dwlad90/stylex-swc-plugin/issues/1265) reports.
/// Refusing without aborting is this compiler's answer and stays one.
#[test]
fn an_undecidable_guard_still_walks_both_sides() {
  assert_deopt_reason_contains("['a', 0 ?? 'x'.constructor].join('-')", "constructor");
  assert_deopt_reason_contains("['a', false ?? 'x'.constructor].join('-')", "constructor");
}

/// A guard whose value the walk cannot read at all.
#[test]
fn a_guard_the_walk_cannot_read_still_walks_both_sides() {
  assert_deopts("['a', unknown && 'x'.constructor].join('-')");
  assert_deopts("[unknown ? 'a' : 'x'.constructor].join('')");
}

/// Inside a callback the module is not what binds the names, so which side runs
/// cannot be read from it — the element is what `x` holds, and the module's
/// answer for that spelling would be a different binding entirely. Both sides
/// are walked, and the refusal on the arm a module read would have dropped
/// stands.
#[test]
fn a_short_circuit_inside_a_callback_walks_both_sides() {
  assert_deopt_reason_contains(
    "[1, 2].map(x => x && 'x'.constructor).join('')",
    "constructor",
  );

  assert_deopt_reason_contains(
    "[1, 2].map(x => (x ? 'x'.constructor : 'b')).join('')",
    "constructor",
  );
}

/// And the values a callback's own short circuit produces are the language's,
/// which is what a walk that had pruned by the module's reading would have got
/// wrong rather than merely refused.
#[test]
fn a_short_circuit_inside_a_callback_answers_per_element() {
  assert_folds_to_string("[0, 1, 2].map(x => x && x + 'px').join(' ')", "0 1px 2px");
  assert_folds_to_string("[0, 1].map(x => (x ? 'on' : 'off')).join(' ')", "off on");
}

// ==================== the side that does run ====================

/// The guard still asks about every operand it reaches, so a live side carrying
/// the same leaf refuses exactly as it always did. Paired with the cases above,
/// this is what says the fix is laziness rather than a rule that stopped being
/// applied.
///
/// The reference implementation folds the first of these, to the source text of
/// `String` itself. The escaping-property rule is the deliberate boundary
/// documented beside it — a read that walks onto the language's function graph is
/// refused here — and the point of this case is only that a *live* branch is
/// still walked.
#[test]
fn a_live_operand_is_walked_and_refused_as_before() {
  assert_deopt_reason_contains("['a', true && 'x'.constructor].join('-')", "constructor");
  assert_deopt_reason_contains("['a', '' || 'x'.constructor].join('-')", "constructor");
  assert_deopt_reason_contains("['a', null ?? 'x'.constructor].join('-')", "constructor");
  assert_deopt_reason_contains("[false ? 'a' : 'x'.constructor].join('')", "constructor");
}

/// And the operand the language keeps is the one the fold answers with, on every
/// form — the half of this that a case asserting only "it did not refuse" would
/// not see.
#[test]
fn the_live_operand_is_what_the_fold_answers_with() {
  assert_folds_to_string("['a', true && 'b'].join('-')", "a-b");
  assert_folds_to_string("['a', '' || 'b'].join('-')", "a-b");
  assert_folds_to_string("['a', null ?? 'b'].join('-')", "a-b");
  assert_folds_to_string("['a', undefined ?? 'b'].join('-')", "a-b");
  assert_folds_to_string("[false ? 'x' : 'b'].join('')", "b");
}
