//! Which of the three callee shapes a call is read as, and what happens to a
//! call that is none of them.
//!
//! `admit_call` matches the callee and hands the call to the helper for its
//! shape: an applied global, a name, or a method. Only the third carries rules
//! of its own, so the match itself is where a call can be sent to the wrong set
//! of rules -- and a call sent to the wrong helper does not fail loudly. It
//! folds, to whatever the other shape's rules answer.
//!
//! So each shape is pinned by an answer only its own helper produces, and the
//! shapes that match none of them are pinned as refusals. The distinctions are
//! deliberately awkward ones: a name in parentheses is still a name, a
//! parenthesised member is still a method, and a computed key that reads as a
//! literal is still not one.

use super::source_evaluation::*;

// ==================== the applied globals ====================

/// The conversions are folded by being called, and each answers something no
/// other shape would: `Array(3)` is a length rather than an element, which is
/// the arm that separates it from every other global.
#[test]
fn an_applied_global_is_answered_by_the_language() {
  assert_folds_to_string("String(1)", "1");
  assert_folds_to_number("Number('2')", 2.0);
  assert_folds_to_number("Array(3).length", 3.0);
  assert_folds_to_number("Object.keys(Object({ a: 1 })).length", 1.0);
}

/// Parentheses are unwrapped before the callee is matched, so a wrapped global
/// reaches the same arm rather than falling to the member arm below.
#[test]
fn parentheses_around_an_applied_global_do_not_change_which_arm_reads_it() {
  assert_folds_to_string("(String)(1)", "1");
  assert_folds_to_string("((String))(1)", "1");
}

// The third arm, a call reached through a name, resolves a module binding --
// which a lone expression has none of, so it cannot be reached from here. It is
// pinned where a module is in scope, in
// `transform_stylex_create_test/refusals_a_module_reaches.rs` and
// `globals_written_as_a_value.rs`.

// ==================== a method call ====================

/// A method has a receiver, and the receiver's own rules are what separates this
/// arm: a locale-sensitive name refuses where the identical call on any other
/// name folds.
#[test]
fn a_method_call_reads_the_rules_its_receiver_carries() {
  assert_folds_to_string("'AB'.toLowerCase()", "ab");
  assert_deopts("'AB'.toLocaleLowerCase()");
}

/// Parentheses are unwrapped for the two arms that read a name, and not for the
/// one that reads a member -- so a wrapped method reaches no arm and is handed
/// back, where the same call unwrapped folds.
///
/// Recorded as it is rather than as it might be. A call handed back is refused,
/// never mis-folded, so the asymmetry costs an expression nobody writes and
/// cannot name a class the reference compiler does not define. Widening it is a
/// change to what compiles, which is not a thing a test should decide quietly.
#[test]
fn parentheses_around_a_method_take_it_out_of_the_arm_that_reads_receivers() {
  assert_folds_to_string("'AB'.toLowerCase()", "ab");

  for wrapped in ["('AB'.toLowerCase)()", "(('AB'.toLowerCase))()"] {
    assert_deopts(wrapped);
  }
}

/// A chain hides its middle links, so the rules run per link rather than at the
/// outermost call only.
#[test]
fn every_link_of_a_chain_is_read_as_its_own_call() {
  assert_folds_to_string("'AB'.toLowerCase().trim()", "ab");
  assert_deopts("'AB'.toLocaleLowerCase().trim()");
}

// ==================== the shapes that are none of the three ====================

/// A computed key is a lookup nothing here resolves, and reading it as a literal
/// would be the guard resolving a name it has no binding for. Both spellings
/// refuse, including the one a reader would call obviously static.
#[test]
fn a_computed_method_name_is_not_a_method_this_module_reads() {
  assert_deopts("'AB'['toLowerCase']()");
  assert_deopts("'AB'[['toLowerCase'][0]]()");
}

/// A callee that is neither a global, a name, nor a member reaches no arm at
/// all, and a call that reaches no arm is handed back rather than folded.
#[test]
fn a_callee_of_no_shape_reaches_no_arm() {
  for source in [
    "(function () { return 'a'; })()",
    "(() => 'a')()",
    "(0, String)(1)",
    "(true ? String : Number)(1)",
  ] {
    assert_deopts(source);
  }
}

// ==================== the extremes ====================

/// A chain long past anything an author writes still runs the rules once per
/// link rather than losing them, and answers the same value at every length.
///
/// Under a raised ceiling, because a chain this long is deeper than the shipped
/// default admits -- which is the depth ceiling answering rather than the
/// dispatch, and is a different claim, pinned in `evaluation_depth_budget.rs`.
#[test]
fn a_very_long_chain_reads_every_link_without_losing_a_rule() {
  let ceiling = 1_024;

  let folding = format!("'ab'{}", ".trim()".repeat(64));
  assert_folds_to_string_with_ceiling(&folding, "ab", ceiling);

  // The one refusing link is buried under sixty-four that fold, and is still
  // what the whole chain answers for -- so no link is skipped for depth.
  let refusing = format!("'ab'.toLocaleUpperCase(){}", ".trim()".repeat(64));
  assert_deopt_reason_contains_with_ceiling(&refusing, "toLocaleUpperCase", ceiling);
}

/// The same chain past the shipped ceiling answers for its depth rather than
/// folding, so the two limits are not confused for one another.
#[test]
fn a_chain_deeper_than_the_default_ceiling_is_refused_for_its_depth() {
  let deep = format!("'ab'{}", ".trim()".repeat(64));

  assert_deopt_reason_contains(&deep, "too deeply nested");
}

/// Parentheses nest without bound in the source, and unwrapping them is what
/// decides the arm -- so a deeply wrapped callee must still reach the arm its
/// innermost spelling names.
#[test]
fn deeply_nested_parentheses_still_resolve_to_one_shape() {
  let depth = 32;
  let wrapped = format!("{}String{}(1)", "(".repeat(depth), ")".repeat(depth));

  assert_folds_to_string(&wrapped, "1");
}
