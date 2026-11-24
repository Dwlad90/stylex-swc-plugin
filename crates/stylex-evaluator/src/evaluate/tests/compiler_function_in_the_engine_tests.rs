//! A compiler function called inside a callback that the engine runs.
//!
//! `own` stands for a function such as `keyframes`: it answers the argument it
//! was handed. The engine calls it once for each time the callback runs, with
//! the build state of the fold. A callback that never runs does not call it.

use super::source_evaluation::*;
use super::{engine_fold::NO_ENGINE_FORM, evaluate};
use rustc_hash::FxHashMap;
use stylex_constants::constants::evaluation_errors::uncoercible_value;
use stylex_state::{
  functions::{FunctionConfigType, FunctionMap, FunctionType},
  state_manager::{ImportKind, StateManager},
};
use stylex_structures::{named_import_source::ImportSources, stylex_options::StyleXOptions};
use swc_core::common::{DUMMY_SP, GLOBALS, Globals};
use swc_core::{
  atoms::Atom,
  ecma::ast::{Expr, Ident, Lit},
};

/// The text `source` folds to against the function fold.
#[track_caller]
fn folded(source: &str) -> String {
  folded_text_of(evaluated_against_a_function_fold(source), source)
}

/// The text `source` folds to against `fns`.
#[track_caller]
fn folded_against(fns: &FunctionMap, source: &str) -> String {
  folded_text_of(evaluated_against(fns, source), source)
}

/// The reason `source` refused with against `fns`.
#[track_caller]
fn refusal_against(fns: &FunctionMap, source: &str) -> String {
  let result = evaluated_against(fns, source);

  assert_refused(&result, source);

  result.reason.unwrap_or_default()
}

/// The function fold, with one more name bound to `fn_ptr`.
fn with_a_function(name: &str, fn_ptr: FunctionType) -> FunctionMap {
  let mut fns = a_function_fold();

  fns
    .identifiers
    .insert(name.into(), Box::new(folded_entry(fn_ptr, false)));

  fns
}

/// A function that records a filed rule, as `keyframes` does, and answers its
/// argument.
fn files_a_rule(expr: Expr, state: &mut StateManager) -> Expr {
  state.note_nested_rule_filed();

  expr
}

/// A function that starts a nested fold, which calls a compiler function of
/// its own.
fn folds_again(_: Expr, state: &mut StateManager) -> Expr {
  let source = "['x', 'y'].map((v) => own(v.toUpperCase())).join('')";
  let result = evaluate(&parse_expr(source), state, &a_function_fold());

  folded_value_of(result, source)
    .as_expr()
    .cloned()
    .unwrap_or_else(|| panic!("the nested fold answered no expression"))
}

/// Evaluates `source` against `fns`, and answers the text and how many rules
/// the build state recorded.
#[track_caller]
fn folded_with_filings(fns: &FunctionMap, sources: &[&str]) -> (Vec<String>, u64) {
  GLOBALS.set(&Globals::new(), || {
    let mut state = StateManager::new(StyleXOptions::default());

    let texts = sources
      .iter()
      .map(|source| folded_text_of(evaluate(&parse_expr(source), &mut state, fns), source))
      .collect();

    (texts, state.nested_rules_filed())
  })
}

#[test]
fn a_call_in_a_callback_folds() {
  assert_eq!(folded("['a'].map(() => own('b')).join()"), "b");
}

#[test]
fn a_call_reads_the_callback_parameter() {
  assert_eq!(folded("['a', 'b'].map((c) => own(c)).join('-')"), "a-b");
}

#[test]
fn a_call_through_a_namespace_map_folds() {
  assert_eq!(folded("['a'].map(() => sx.create('b')).join()"), "b");
}

/// A call outside a callback runs in the engine too.
#[test]
fn a_call_in_an_array_folds() {
  assert_eq!(folded("[own('a'), sx.create('b')].join('-')"), "a-b");
}

/// An array argument reaches the function as an array literal.
#[test]
fn an_array_argument_reaches_the_function() {
  assert_eq!(folded("['a'].map(() => own(['b', 'c'])).join()"), "b,c");
}

#[test]
fn a_call_through_the_member_entries_of_an_import_folds() {
  let mut fns = FunctionMap::default();
  let mut members = FxHashMap::default();

  members.insert(
    "kf".into(),
    Box::new(folded_entry(
      FunctionType::StylexExprFn(|expr, _| expr),
      false,
    )),
  );
  fns
    .member_expressions
    .insert(ImportSources::Regular("ns".to_string()), Box::new(members));

  assert_eq!(
    folded_against(&fns, "['a'].map(() => ns.kf('b')).join()"),
    "b"
  );
}

/// `firstThatWorks` and a compiler function on one namespace cross as one
/// object that holds both.
#[test]
fn one_namespace_carries_both_kinds_of_function() {
  let mut fns = FunctionMap::default();
  let mut members = FxHashMap::default();

  members.insert(
    "kf".into(),
    Box::new(folded_entry(
      FunctionType::StylexExprFn(|expr, _| expr),
      false,
    )),
  );
  fns.member_expressions.insert(
    ImportSources::Regular("stylex".to_string()),
    Box::new(members),
  );

  let source = "['a'].map(() => stylex.kf(stylex.firstThatWorks('x', 'y'))).join()";
  let result = evaluated_in_a_state(
    |state| state.insert_stylex_import(ImportSources::Regular("stylex".to_string())),
    &fns,
    source,
  );

  assert_eq!(folded_text_of(result, source), "y,x");

  // The namespace is also a namespace for a name the import record holds, so
  // the named `firstThatWorks` stays the function that needs no state.
  let source = "['a'].map(() => firstThatWorks('x', 'y')).join()";
  let result = evaluated_in_a_state(
    |state| state.insert_stylex_api_import(ImportKind::FirstThatWorks, "firstThatWorks".into()),
    &fns,
    source,
  );

  assert_eq!(folded_text_of(result, source), "y,x");
}

/// A callback that never runs never calls the function, so no rule is filed.
#[test]
fn an_empty_array_never_calls_the_function() {
  let fns = with_a_function("file", FunctionType::StylexExprFn(files_a_rule));

  let (texts, filed) = folded_with_filings(&fns, &["[].map(() => file('b')).join() || 'none'"]);

  assert_eq!(texts, ["none"]);
  assert_eq!(filed, 0);
}

/// Each run of the callback calls the function once, and the memo does not
/// answer a second identical expression from the first.
#[test]
fn every_call_files_its_rule() {
  let fns = with_a_function("file", FunctionType::StylexExprFn(files_a_rule));
  let source = "['a', 'b'].map((c) => file(c)).join()";

  let (texts, filed) = folded_with_filings(&fns, &[source, source]);

  assert_eq!(texts, ["a,b", "a,b"]);
  assert_eq!(filed, 4);
}

/// Two functions under one name share the compiled script, because the
/// function goes to the engine as an argument. Each fold gets its own
/// function.
#[test]
fn two_functions_under_one_name_answer_apart() {
  let source = "['a'].map(() => own('b')).join()";
  let constant = with_a_function(
    "own",
    FunctionType::StylexExprFn(|_, _| Expr::Lit(Lit::Str("z".into()))),
  );

  assert_eq!(folded(source), "b");
  assert_eq!(folded_against(&constant, source), "z");
  assert_eq!(folded(source), "b");
}

/// A compiler function that starts a fold of its own gets an engine of its
/// own, and the outer fold goes on with its engine.
#[test]
fn a_nested_fold_runs_inside_a_compiler_function() {
  let fns = with_a_function("nest", FunctionType::StylexExprFn(folds_again));

  assert_eq!(
    folded_against(&fns, "['a', 'b'].map(() => nest('b')).join('-')"),
    "XY-XY"
  );
  assert_eq!(folded("['a'].map(() => own('b')).join()"), "b");
}

/// A call with no argument has nothing to read. In a callback the refusal names
/// the function. Outside one, the dispatch below owns the call.
#[test]
fn a_call_with_no_argument_refuses() {
  let reason = refusal_against(&a_function_fold(), "['a'].map(() => own()).join()");

  assert_eq!(reason, uncoercible_value("own"));

  let reason = refusal_against(&a_function_fold(), "[own()].join()");

  assert_eq!(
    reason,
    "StyleX expression function requires an expression argument."
  );
}

/// An argument the engine cannot hand back is a throw, and the throw is the
/// refusal.
#[test]
fn an_argument_with_no_value_form_refuses() {
  let reason = refusal_against(&a_function_fold(), "['a'].map(() => own(() => 1)).join()");

  assert!(reason.contains("function"), "got {reason}");
}

#[test]
fn an_answer_with_no_engine_form_refuses() {
  let fns = with_a_function(
    "odd",
    FunctionType::StylexExprFn(|_, _| {
      Expr::Ident(Ident::new_no_ctxt("somewhere".into(), DUMMY_SP))
    }),
  );
  let reason = refusal_against(&fns, "['a'].map(() => odd('b')).join()");

  assert!(reason.contains(NO_ENGINE_FORM), "got {reason}");
}

/// A name that holds no compiler function is not called in the engine: a
/// namespace map, a function that takes its argument as written, and a
/// namespace member of another kind.
#[test]
fn a_name_without_a_compiler_function_is_not_called() {
  let mut fns = with_a_function("raw", FunctionType::StylexExprFn(|expr, _| expr));

  fns.identifiers.insert(
    "raw".into(),
    Box::new(folded_entry(
      FunctionType::StylexExprFn(|expr, _| expr),
      true,
    )),
  );

  if let Some(FunctionConfigType::Map(entries)) =
    fns.identifiers.get_mut(&Atom::from("sx")).map(Box::as_mut)
  {
    entries.insert(
      "other".into(),
      folded_entry(
        FunctionType::ArrayArgs(|_, _, _| Expr::Lit(Lit::Str("q".into()))),
        false,
      ),
    );
  }

  for source in [
    "['a'].map(() => sx('b')).join()",
    "['a'].map(() => raw('b')).join()",
    "['a'].map(() => sx.other('b')).join()",
    "['a'].map(() => sx.missing('b')).join()",
  ] {
    refusal_against(&fns, source);
  }
}

/// A function the callback binds is the callback's, whatever the map holds
/// under its name.
#[test]
fn a_parameter_shadows_a_compiler_function() {
  assert_eq!(
    folded("[(v) => v + '!'].map((own) => own('b')).join()"),
    "b!"
  );
}
