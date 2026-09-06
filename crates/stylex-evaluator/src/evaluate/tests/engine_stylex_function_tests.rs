//! `firstThatWorks` run by the compile-time engine.
//!
//! It is the one StyleX function the engine may call: it answers from its
//! arguments alone and touches no state, so there is nothing to stop the engine
//! running it -- and running it there is what lets a callback fold, since
//! `parts.map((part) => firstThatWorks(part, 'serif'))` is one call per element
//! and that is the engine's job.
//!
//! Which name reaches it is a question about what the module imported rather
//! than about the expression, so every case here records the import a collector
//! would have recorded. Both spellings are covered, because a named import
//! carries the function itself and a namespace import carries an object the
//! call is read off, and the two are separate lookups.
//!
//! What the engine answers has to be what the evaluator's own path answers for
//! the same arguments -- both run the same ordering -- so each case is written
//! as the value the reference implementation writes.

use super::source_evaluation::*;
use stylex_state::{
  functions::FunctionMap,
  state_manager::{ImportKind, StateManager},
};
use stylex_structures::named_import_source::ImportSources;

/// The name a named import binds the function under.
const IMPORTED: &str = "firstThatWorks";

/// The name a namespace import binds the whole surface under.
const NAMESPACE: &str = "stylex";

/// Records both imports, which is what a module writing either spelling would
/// have left in the state.
fn importing_both(state: &mut StateManager) {
  state.insert_stylex_api_import(ImportKind::FirstThatWorks, IMPORTED.into());
  state.insert_stylex_import(ImportSources::Regular(NAMESPACE.to_string()));
}

/// What `source` folds to, as text.
#[track_caller]
fn folded(source: &str) -> String {
  folded_text_of(
    evaluated_in_a_state(importing_both, &FunctionMap::default(), source),
    source,
  )
}

/// The named spelling and the namespace spelling are the same call and answer
/// the same value.
#[test]
fn both_spellings_of_the_call_answer_the_same_chain() {
  let named = folded(&format!("[{IMPORTED}('var(--accent)', 'blue')].join('')"));
  let through_the_namespace = folded(&format!(
    "[{NAMESPACE}.{IMPORTED}('var(--accent)', 'blue')].join('')"
  ));

  assert_eq!(named, "var(--accent, blue)");
  assert_eq!(named, through_the_namespace);
}

/// A call naming no variable at all has no chain to build, so its arguments are
/// reversed -- highest priority last, as CSS reads a repeated declaration.
#[test]
fn a_call_naming_no_variable_answers_its_arguments_reversed() {
  assert_eq!(
    folded(&format!("{IMPORTED}('a', 'b', 'c').join('-')")),
    "c-b-a"
  );
}

/// A chain with values behind it answers the chain first and then those values,
/// which is the shape the reference implementation answers with.
#[test]
fn a_chain_with_values_behind_it_answers_both() {
  assert_eq!(
    folded(&format!(
      "{IMPORTED}('color-mix(in srgb, currentColor 20%, transparent)', 'var(--accent)', 'blue').join('|')"
    )),
    "var(--accent, blue)|color-mix(in srgb, currentColor 20%, transparent)"
  );
}

/// An argument that is not a string keeps its own form, so a number among the
/// fallbacks reaches CSS the way the language writes it rather than as text the
/// call invented.
#[test]
fn an_argument_that_is_not_a_string_keeps_its_own_form() {
  assert_eq!(folded(&format!("{IMPORTED}(1, 2).join('-')")), "2-1");
  assert_eq!(
    folded(&format!("[{IMPORTED}('var(--size)', 0)].join('')")),
    "var(--size, 0)"
  );
}

/// The call is what a callback runs, which is the whole reason the function
/// travels into the engine: one call per element, none of them this compiler's
/// to make.
#[test]
fn the_call_runs_inside_a_callback() {
  assert_eq!(
    folded(&format!(
      "['var(--a)', 'var(--b)'].map((name) => {IMPORTED}(name, 'serif')).join('|')"
    )),
    "var(--a, serif)|var(--b, serif)"
  );
}

/// A name the module never imported is not this function, and a property of the
/// namespace that is not on the callable surface is not either -- so neither is
/// run by the engine.
#[test]
fn a_name_the_module_did_not_import_is_not_the_function() {
  for source in [
    String::from("notImported('var(--a)', 'blue')"),
    format!("{NAMESPACE}.notCallable('var(--a)', 'blue')"),
  ] {
    assert_refused(
      &evaluated_in_a_state(importing_both, &FunctionMap::default(), &source),
      &source,
    );
  }
}
