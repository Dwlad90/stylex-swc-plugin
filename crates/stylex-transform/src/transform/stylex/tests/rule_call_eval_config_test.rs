//! The function map a rule call folds its argument with.
//!
//! The suite under `tests/` proves the map holds the right names, because a
//! missing name stops a call folding. It cannot prove the map is built once:
//! delete the memo and every one of those cases still passes. So the two
//! questions the memo owes are asked here, against the builder itself.

use std::rc::Rc;

use stylex_state::{
  functions::RuleCallHelpers,
  state_manager::{ImportKind, StateManager},
};
use stylex_structures::{
  named_import_source::ImportSources, stylex_state_options::StyleXStateOptions,
};
use swc_core::atoms::Atom;

use crate::transform::stylex::visitor_utils::rule_call_eval_config;

/// A module that imports the StyleX namespace and names `firstThatWorks` and
/// `keyframes` through it, which is what the two maps register.
fn state_with_stylex_imports() -> StateManager {
  let mut state = StateManager::for_test(None, StyleXStateOptions::default());

  state.insert_stylex_import(ImportSources::Regular("stylex".to_string()));
  state.insert_stylex_api_import(ImportKind::FirstThatWorks, Atom::from("firstThatWorks"));
  state.insert_stylex_api_import(ImportKind::Keyframes, Atom::from("keyframes"));

  state
}

/// The second call of a module reads the map the first one built. The same
/// object, not an equal one -- an equal one would mean the build ran again.
#[test]
fn the_map_is_built_once_for_the_module() {
  let mut state = state_with_stylex_imports();

  let first = rule_call_eval_config(&mut state, RuleCallHelpers::FirstThatWorks);
  let second = rule_call_eval_config(&mut state, RuleCallHelpers::FirstThatWorks);

  assert!(Rc::ptr_eq(&first, &second));
}

/// The two sets are built and kept apart. `viewTransitionClass` folds a
/// `keyframes` call inside a step, and the other two calls must refuse one.
#[test]
fn the_two_helper_sets_answer_different_maps() {
  let mut state = state_with_stylex_imports();

  let fallback_only = rule_call_eval_config(&mut state, RuleCallHelpers::FirstThatWorks);
  let with_keyframes =
    rule_call_eval_config(&mut state, RuleCallHelpers::FirstThatWorksAndKeyframes);

  assert!(!Rc::ptr_eq(&fallback_only, &with_keyframes));

  assert!(
    fallback_only
      .identifiers
      .contains_key(&Atom::from("firstThatWorks"))
  );
  assert!(
    !fallback_only
      .identifiers
      .contains_key(&Atom::from("keyframes"))
  );

  assert!(
    with_keyframes
      .identifiers
      .contains_key(&Atom::from("firstThatWorks"))
  );
  assert!(
    with_keyframes
      .identifiers
      .contains_key(&Atom::from("keyframes"))
  );
}

/// Each set keeps its own map, so asking for the other one in between does not
/// replace the first answer.
#[test]
fn one_set_does_not_replace_the_other() {
  let mut state = state_with_stylex_imports();

  let first = rule_call_eval_config(&mut state, RuleCallHelpers::FirstThatWorks);

  rule_call_eval_config(&mut state, RuleCallHelpers::FirstThatWorksAndKeyframes);

  let again = rule_call_eval_config(&mut state, RuleCallHelpers::FirstThatWorks);

  assert!(Rc::ptr_eq(&first, &again));
}

/// A module that imports no StyleX name still gets a map, and it registers
/// nothing. The first call of such a module must not be answered by a build
/// that reads names it does not have.
#[test]
fn a_module_with_no_stylex_import_gets_an_empty_map() {
  let mut state = StateManager::for_test(None, StyleXStateOptions::default());

  let function_map = rule_call_eval_config(&mut state, RuleCallHelpers::FirstThatWorks);

  assert!(function_map.identifiers.is_empty());
  assert!(function_map.member_expressions.is_empty());
  assert!(!function_map.disable_imports);
}
