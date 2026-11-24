//! Tests for the readers the merge asks about one argument of a
//! `stylex.props`-family call.

use rustc_hash::FxHashSet;
use stylex_state::state_manager::{BindingWrites, StateManager};
use swc_core::ecma::{
  ast::{Id, ObjectLit},
  visit::{Visit, VisitWith},
};

use super::{can_hoist_object, object_has_css_marker};
use crate::tests::support::{module, object};

/// A state where each of `names` is declared inside a function or a block.
///
/// The test source is parsed without the resolver, so each name has the empty
/// syntax context.
fn state_with_enclosed(names: &[&str]) -> StateManager {
  let enclosed = ids(names);

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

/// A state where each of `names` is a program-level binding that a statement
/// assigns again.
fn state_with_reassigned(names: &[&str]) -> StateManager {
  let reassigned = ids(names);

  let mut state = StateManager::default();
  state.adopt_binding_writes(BindingWrites {
    reassignments: reassigned.clone(),
    mutations: FxHashSet::default(),
    deep_mutations: FxHashSet::default(),
    declared: reassigned,
    enclosed: FxHashSet::default(),
  });
  state
}

/// The ids of `names`, each with the empty syntax context of a fixture that is
/// parsed without the resolver.
fn ids(names: &[&str]) -> FxHashSet<Id> {
  names
    .iter()
    .map(|name| (Into::into(*name), Default::default()))
    .collect()
}

/// A state where the top-level walk runs, and each of `names` is a
/// program-level binding that no earlier statement declares.
fn state_with_undeclared(names: &[&str]) -> StateManager {
  let mut state = StateManager::default();
  state.start_statement_hoists(ids(names));
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

/// A template literal or a computed key can read an enclosed name inside a
/// longer expression. Moved to program level, the object throws when the
/// module loads.
#[test]
fn keeps_an_object_that_reads_an_enclosed_binding_inside_an_expression() {
  assert!(!can_hoist("{ $$css: true, a: `x${size}` }", &["size"]));
  assert!(!can_hoist(
    "{ $$css: true, a: `x${`y${size}`}` }",
    &["size"]
  ));
  assert!(!can_hoist("{ $$css: true, a: `x${map.key}` }", &["map"]));
  assert!(!can_hoist("{ $$css: true, [map.key]: 'xa' }", &["map"]));
  assert!(!can_hoist("{ $$css: true, [`k${size}`]: 'xa' }", &["size"]));
  assert!(!can_hoist("{ $$css: true, [(key)]: 'xa' }", &["key"]));
}

/// `this`, `arguments`, `super`, `new.target`, `await` and `yield` take
/// their value from the function that holds the object. At program level they
/// read another value or do not parse, so the object stays.
#[test]
fn keeps_an_object_that_reads_a_value_of_the_enclosing_function() {
  for code in [
    "{ $$css: true, a: `x${this.k}` }",
    "{ $$css: true, [this.k]: 'xa' }",
    "{ $$css: true, a: `x${arguments[0]}` }",
    "{ $$css: true, [arguments[0]]: 'xa' }",
    "{ $$css: true, a: `x${super.k}` }",
    "{ $$css: true, a: `x${new.target}` }",
    "{ $$css: true, a: `x${await size}` }",
    "{ $$css: true, a: `x${yield size}` }",
  ] {
    let object = object_in_a_method(code);

    assert!(
      !can_hoist_object(&object, &state_with_enclosed(&[])),
      "{code}"
    );
  }
}

/// `import.meta` is the same everywhere in the module, so it does not keep the
/// object in place.
#[test]
fn hoists_an_object_that_reads_import_meta() {
  let object = object_in_a_method("{ $$css: true, a: `x${import.meta.url}` }");

  assert!(can_hoist_object(&object, &state_with_enclosed(&[])));
}

/// The object literal `code` spells, read as the value an async generator
/// method returns. `super`, `new.target` and `yield` parse only there.
fn object_in_a_method(code: &str) -> ObjectLit {
  struct FirstObject(Option<ObjectLit>);

  impl Visit for FirstObject {
    fn visit_object_lit(&mut self, object: &ObjectLit) {
      if self.0.is_none() {
        self.0 = Some(object.clone());
      }
    }
  }

  let source = format!("class C extends B {{ async *m() {{ return {code}; }} }}");
  let mut search = FirstObject(None);
  module(&source).visit_with(&mut search);

  match search.0 {
    Some(object) => object,
    None => panic!("the fixture {code} holds no object literal"),
  }
}

/// A template literal that reads only program-level names, or no name at
/// all, can move. A property name after a dot is not a read of a binding.
#[test]
fn hoists_a_template_literal_that_reads_a_module_scope_name() {
  assert!(can_hoist("{ $$css: true, a: `x${size}` }", &[]));
  assert!(can_hoist("{ $$css: true, a: `xa` }", &["size"]));
  assert!(can_hoist("{ $$css: true, a: `x${map.size}` }", &["size"]));
  assert!(can_hoist("{ $$css: true, [map.key]: 'xa' }", &["key"]));
}

/// A very long template literal is searched to its end without a failure.
#[test]
fn searches_a_very_long_template_literal() {
  let parts = "${1}".repeat(10_000);

  assert!(can_hoist(
    &format!("{{ $$css: true, a: `{parts}` }}"),
    &["size"]
  ));
  assert!(!can_hoist(
    &format!("{{ $$css: true, a: `{parts}${{size}}` }}"),
    &["size"]
  ));
}

/// A program-level binding that the statement holding the object, or a later
/// one, declares has no value yet where the hoisted object goes.
#[test]
fn keeps_an_object_that_reads_a_binding_declared_later() {
  let state = state_with_undeclared(&["later"]);

  for code in [
    "{ $$css: true, __: later }",
    "{ $$css: true, later }",
    "{ $$css: true, a: `x${later}` }",
    "{ $$css: true, [later]: 'xa' }",
  ] {
    assert!(!can_hoist_object(&object(code), &state), "{code}");
  }

  assert!(can_hoist_object(
    &object("{ $$css: true, __: earlier }"),
    &state
  ));
}

/// A hoisted object reads its values once, when the module loads. A binding
/// that a statement assigns again can hold another value when the call runs.
#[test]
fn keeps_an_object_that_reads_a_reassigned_binding() {
  let state = state_with_reassigned(&["v"]);

  assert!(!can_hoist_object(&object("{ $$css: true, __: v }"), &state));
  assert!(!can_hoist_object(&object("{ $$css: true, v }"), &state));
  assert!(can_hoist_object(&object("{ $$css: true, __: w }"), &state));
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
