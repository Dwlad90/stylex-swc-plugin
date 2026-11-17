//! Tests for the readers the merge asks about one argument of a
//! `stylex.props`-family call.

use rustc_hash::FxHashSet;
use stylex_state::state_manager::{BindingWrites, StateManager};
use swc_core::ecma::ast::Id;

use super::{can_hoist_object, object_has_css_marker};
use crate::tests::support::object;

/// A state where each of `names` is declared inside a function or a block.
///
/// The test source is parsed without the resolver, so each name has the empty
/// syntax context.
fn state_with_enclosed(names: &[&str]) -> StateManager {
  let enclosed: FxHashSet<Id> = names
    .iter()
    .map(|name| (Into::into(*name), Default::default()))
    .collect();

  let mut state = StateManager::default();
  state.adopt_binding_writes(BindingWrites {
    reassignments: FxHashSet::default(),
    mutations: FxHashSet::default(),
    deep_mutations: FxHashSet::default(),
    declared: enclosed.clone(),
    enclosed,
  });
  state
}

/// Whether the object in `code` can be hoisted, where each of `enclosed` is
/// bound below program level.
fn can_hoist(code: &str, enclosed: &[&str]) -> bool {
  can_hoist_object(&object(code), &state_with_enclosed(enclosed))
}

/// A compiled style carries the marker under a plain name, which is how the
/// merge tells one from an object the author wrote.
#[test]
fn finds_the_marker_under_a_plain_name() {
  assert!(object_has_css_marker(&object(
    "{ color: 'xa', $$css: true }"
  )));
}

/// The same key written as a string names the same property.
#[test]
fn finds_the_marker_under_a_string_key() {
  assert!(object_has_css_marker(&object("{ '$$css': true }")));
}

/// The marker says the object is compiled. An object that does not carry it is
/// the author's own and is left where it stands.
#[test]
fn finds_no_marker_where_none_is_written() {
  assert!(!object_has_css_marker(&object("{ color: 'red' }")));
  assert!(!object_has_css_marker(&object("{}")));
}

/// The marker is the value `true`. Any other value is a property that happens
/// to share the name.
#[test]
fn finds_no_marker_under_another_value() {
  assert!(!object_has_css_marker(&object("{ $$css: false }")));
  assert!(!object_has_css_marker(&object("{ $$css: 'true' }")));
}

/// A key spelled in a way that names nothing at compile time is not the marker,
/// even where it would fold to it.
#[test]
fn finds_no_marker_under_a_computed_key() {
  assert!(!object_has_css_marker(&object("{ ['$$css']: true }")));
}

/// A spread and a shorthand method are not key-value pairs, so neither carries
/// the marker.
#[test]
fn finds_no_marker_in_a_property_that_is_not_a_key_value_pair() {
  assert!(!object_has_css_marker(&object("{ ...compiled }")));
  assert!(!object_has_css_marker(&object(
    "{ get $$css() { return true } }"
  )));
}

/// Literal values, template literals included, need nothing from the scope.
#[test]
fn hoists_an_object_of_literals() {
  assert!(can_hoist(
    "{ $$css: true, color: 'xa', size: 1, on: null }",
    &[]
  ));
  assert!(can_hoist("{ $$css: true, color: `xa` }", &[]));
}

/// A name that a function or a block binds does not exist at program level.
#[test]
fn keeps_an_object_that_reads_an_enclosed_binding() {
  assert!(!can_hoist("{ $$css: true, __: className }", &["className"]));
  assert!(!can_hoist("{ $$css: true, className }", &["className"]));
  assert!(!can_hoist("{ $$css: true, [key]: 'xa' }", &["key"]));
  assert!(!can_hoist(
    "{ $$css: true, __: (className) }",
    &["className"]
  ));
}

/// A global or a program-level name is visible at program level.
#[test]
fn hoists_an_object_that_reads_a_module_scope_name() {
  assert!(can_hoist("{ $$css: true, __: className }", &[]));
  assert!(can_hoist("{ $$css: true, className }", &[]));
  assert!(can_hoist("{ $$css: true, [key]: 'xa' }", &[]));
  assert!(can_hoist("{ $$css: true, ['key']: 'xa' }", &["key"]));
  assert!(can_hoist("{ $$css: true, __: undefined }", &["className"]));
}

/// A value that is neither a literal nor a name can read anything, so the
/// object stays.
#[test]
fn keeps_an_object_with_a_value_that_is_not_a_literal_or_a_name() {
  assert!(!can_hoist("{ $$css: true, __: a.b }", &[]));
  assert!(!can_hoist("{ $$css: true, __: f() }", &[]));
  assert!(!can_hoist("{ $$css: true, __: { color: 'xa' } }", &[]));
}

/// A spread or a method is not a plain property, so the object stays.
#[test]
fn keeps_an_object_with_a_property_that_is_not_plain() {
  assert!(!can_hoist("{ $$css: true, ...rest }", &[]));
  assert!(!can_hoist(
    "{ $$css: true, get color() { return 'xa' } }",
    &[]
  ));
}
