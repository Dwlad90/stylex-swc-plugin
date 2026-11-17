//! A call to one of the compiler's own functions, nested in a method call.
//!
//! The engine cannot run such a call, so it declines the method call around
//! it. Each nested call is computed first and written in as its value, and the
//! method call is offered to the engine again. `own` stands for a function such
//! as `keyframes`: it answers the argument it was handed.

use super::source_evaluation::*;
use stylex_constants::constants::evaluation_errors::{unfoldable_call, unsupported_expression};

/// The string `source` folds to against the function fold.
#[track_caller]
fn folded(source: &str) -> String {
  folded_text(&folded_value_of(
    evaluated_against_a_function_fold(source),
    source,
  ))
}

/// The reason `source` refused with against the function fold.
#[track_caller]
fn refusal(source: &str) -> String {
  let result = evaluated_against_a_function_fold(source);

  assert!(!result.confident, "`{source}` folded: {:?}", result.value);

  result.reason.unwrap_or_default()
}

#[test]
fn a_nested_call_in_the_receiver_folds_before_the_method_runs() {
  assert_eq!(folded("[own('a'), own('b')].join('-')"), "a-b");
}

/// A callback has no value form, so it is left as written for the engine to
/// run.
#[test]
fn a_callback_is_left_to_the_engine() {
  assert_eq!(folded("[own('a')].map((name) => name + '!').join()"), "a!");
}

/// A call to one of the compiler's own functions computes its own arguments,
/// so the one nested inside it folds where that call reads it.
#[test]
fn a_nested_call_inside_a_compiler_call_folds_as_its_argument() {
  assert_eq!(folded("own([own('a')].join('-'))"), "a");
}

/// What the engine refuses once the values are written in is the refusal.
#[test]
fn the_engine_refusal_on_the_folded_call_is_the_refusal() {
  assert!(
    refusal("own('a').normalize('none')").contains("normalize"),
    "expected the engine's own refusal"
  );
}

/// A call the engine still declines with the values written in is answered by
/// the dispatch, as it was before, which names the argument it cannot read.
#[test]
fn a_call_the_engine_still_declines_falls_to_the_dispatch() {
  assert!(
    refusal("[own('a')].concat(/x/)").contains(&unsupported_expression("RegExpLiteral")),
    "expected the dispatch's refusal of the argument"
  );
}

/// A part that refuses is the refusal, and the parts after it are not
/// evaluated.
#[test]
fn a_part_that_refuses_is_the_refusal() {
  let reason = refusal("own(missing).concat(own('b'))");

  assert!(
    !reason.contains(&unfoldable_call("concat")),
    "expected the receiver's refusal, got {reason}"
  );
}

/// A conditional is evaluated whole, so the branch it does not take is never
/// evaluated and cannot refuse.
#[test]
fn a_conditional_part_evaluates_only_the_branch_it_takes() {
  assert_eq!(folded("[true ? own('a') : own(missing)].join()"), "a");
}

/// A computed key is not a shape the engine reads, so the part that holds the
/// call is folded first, and the method then folds on its value.
#[test]
fn a_part_the_engine_cannot_read_folds_first() {
  assert_eq!(folded("own({ ['k']: 'b' }).k.toUpperCase()"), "B");
}

/// The same shape. What the engine then refuses on the call that is left
/// is the refusal.
#[test]
fn the_engine_refusal_after_a_part_is_folded_is_the_refusal() {
  assert!(
    refusal("own({ ['k']: 'b' }).k.normalize('none')").contains("normalize"),
    "expected the engine's own refusal"
  );
}

/// A namespace reached through the member entries of an import is found by
/// its name, and a call through a name the module did not import is not one.
#[test]
fn a_namespace_of_the_member_entries_is_found_by_its_name() {
  let mut fns = stylex_state::functions::FunctionMap::default();
  let mut members = rustc_hash::FxHashMap::default();

  members.insert("kf".into(), Box::new(a_folded_function()));
  fns.member_expressions.insert(
    stylex_structures::named_import_source::ImportSources::Regular("ns".to_string()),
    Box::new(members),
  );

  let source = "[String(1), ns.kf('a') || rt].join()";

  assert_eq!(
    folded_text_of(evaluated_against(&fns, source), source),
    "1,a"
  );
}
