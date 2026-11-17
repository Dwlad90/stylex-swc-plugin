//! An author's arrow whose body calls a compiler function, applied by name.
//!
//! The body reaches the function through a namespace as well as through a bare
//! name, and the rule the function files reaches the build state although the
//! body runs on a copy of it.

use super::evaluate;
use super::source_evaluation::*;
use rustc_hash::FxHashMap;
use stylex_state::{
  functions::{FunctionMap, FunctionType},
  state_manager::StateManager,
};
use stylex_structures::{named_import_source::ImportSources, stylex_options::StyleXOptions};
use swc_core::common::{GLOBALS, Globals};
use swc_core::ecma::ast::Expr;

/// A function that records a filed rule, as `keyframes` does, and answers its
/// argument.
fn files_a_rule(expr: Expr, state: &mut StateManager) -> Expr {
  state.note_nested_rule_filed();

  expr
}

/// A map with `file` bound to [`files_a_rule`] and the namespace `ns` holding
/// it as `kf`.
fn with_file() -> FunctionMap {
  let mut fns = a_function_fold();
  let mut members = FxHashMap::default();

  members.insert(
    "kf".into(),
    Box::new(folded_entry(
      FunctionType::StylexExprFn(files_a_rule),
      false,
    )),
  );
  fns
    .member_expressions
    .insert(ImportSources::Regular("ns".to_string()), Box::new(members));
  fns.identifiers.insert(
    "file".into(),
    Box::new(folded_entry(
      FunctionType::StylexExprFn(files_a_rule),
      false,
    )),
  );

  fns
}

/// The text `source` folds to against `mk = <init>`, and how many rules it
/// filed.
#[track_caller]
fn folded_with_filings(init: &str, source: &str) -> (String, u64) {
  GLOBALS.set(&Globals::new(), || {
    let mut state = StateManager::new(StyleXOptions::default());

    state.push_declaration(a_declaration_of("mk", init));

    let result = evaluate(&parse_expr(source), &mut state, &with_file());

    (folded_text_of(result, source), state.nested_rules_filed())
  })
}

#[test]
fn a_call_through_a_namespace_in_the_body_files_its_rule() {
  assert_eq!(
    folded_with_filings("(c) => ns.kf(c)", "mk('a')"),
    ("a".into(), 1)
  );
  assert_eq!(
    folded_with_filings("() => ns.kf('a')", "mk()"),
    ("a".into(), 1)
  );
}

#[test]
fn a_call_through_a_name_in_the_body_files_its_rule() {
  assert_eq!(
    folded_with_filings("(c) => file(c)", "mk('a')"),
    ("a".into(), 1)
  );
}

/// Each application files its rule, and the second is not answered from the
/// memo of the first.
#[test]
fn each_application_files_its_rule() {
  assert_eq!(
    folded_with_filings("(c) => file(c)", "mk('a') + mk('a')"),
    ("aa".into(), 2)
  );
}
