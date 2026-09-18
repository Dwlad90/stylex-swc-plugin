//! Tests for the reader that records which style namespaces a bailed-out
//! `stylex.props` call still needs at runtime.

use std::rc::Rc;

use stylex_enums::style_vars_to_keep::NonNullProps;
use stylex_state::{functions::FunctionMap, state_manager::StateManager};
use swc_core::ecma::ast::Expr;

use crate::shared::utils::core::member_expression::member_expression;
use crate::tests::support::expr;

/// Reads `code` as one member expression and hands back what the reader
/// recorded.
///
/// The counters start where the walk starts them: at the first argument, with
/// no bail-out point behind it and no namespace named yet.
fn namespaces_of(code: &str) -> NonNullProps {
  read(code, NonNullProps::Vec(vec![]), None, 0)
}

fn read(
  code: &str,
  mut non_null_props: NonNullProps,
  mut bail_out_index: Option<i32>,
  mut index: i32,
) -> NonNullProps {
  let member = match expr(code) {
    Expr::Member(member) => member,
    other => panic!("the fixture {code} is not a member expression: {other:?}"),
  };

  let mut state = StateManager::default();

  member_expression(
    &member,
    &mut index,
    &mut bail_out_index,
    &mut non_null_props,
    &mut state,
    &Rc::new(FunctionMap::default()),
  );

  non_null_props
}

fn named(names: &[&str]) -> NonNullProps {
  NonNullProps::Vec(names.iter().map(|name| (*name).into()).collect())
}

/// Every property the namespace declares is a name the runtime still needs.
#[test]
fn names_every_property_the_namespace_declares() {
  assert_eq!(
    namespaces_of("({ root: { color: 'red', margin: '1px' } }).root"),
    named(&["color", "margin"])
  );
}

/// A property declared as absent declares nothing, so the runtime needs no name
/// for it.
#[test]
fn leaves_out_a_property_that_declares_nothing() {
  assert_eq!(
    namespaces_of("({ root: { color: 'red', margin: null } }).root"),
    named(&["color"])
  );
}

/// A key the printer has to quote names its property like any other. Read
/// through `as_ident` it answered nothing at all, so the namespace behind it
/// was missing from the keep list and the null sweep dropped a name the runtime
/// still reads. The reference lists the keys of the folded object, which names
/// a quoted key as readily as a bare one.
#[test]
fn names_a_property_whose_key_has_to_be_quoted() {
  assert_eq!(
    namespaces_of("({ root: { '--my-color': 'red', color: 'blue' } }).root"),
    named(&["--my-color", "color"])
  );
}

/// The same rule for the three other shapes a key is written in. A number key
/// names a string property spelled the way JavaScript spells the number, and a
/// computed key naming a literal is the key that literal spells.
#[test]
fn names_a_property_whose_key_is_not_written_as_a_name() {
  assert_eq!(
    namespaces_of("({ root: { 0: 'red', 1e21: 'blue', ['--x']: 'green' } }).root"),
    named(&["0", "1e+21", "--x"])
  );
}

/// A quoted key that declares nothing is still left out, so the reader is
/// narrowing on the value and not on how the key was written.
#[test]
fn leaves_out_a_quoted_key_that_declares_nothing() {
  assert_eq!(
    namespaces_of("({ root: { '--my-color': null, color: 'blue' } }).root"),
    named(&["color"])
  );
}

#[test]
fn names_nothing_of_an_empty_namespace() {
  assert_eq!(namespaces_of("({ root: {} }).root"), named(&[]));
}

/// A reader that already needs every namespace has nothing left to narrow, so
/// it reads no further.
#[test]
fn a_reader_that_needs_everything_reads_no_further() {
  assert_eq!(
    read(
      "({ root: { color: 'red' } }).root",
      NonNullProps::True,
      None,
      0
    ),
    NonNullProps::True
  );
}

/// Past the point the walk bailed out at, the call can hold anything, so every
/// namespace is needed.
#[test]
fn everything_past_the_bail_out_point_is_needed() {
  assert_eq!(
    read("({ root: { color: 'red' } }).root", named(&[]), Some(0), 1),
    NonNullProps::True
  );
}

/// An expression that does not fold tells the reader nothing about which
/// namespaces are needed, so every one of them is.
#[test]
fn an_expression_that_does_not_fold_needs_everything() {
  assert_eq!(namespaces_of("unknown.root"), NonNullProps::True);
}

/// A spread, a key that is not a name, and a property with no value of its own
/// never reach the reader: the evaluator merges the first away and refuses the
/// other two, so a namespace that holds one folds to nothing at all.
#[test]
fn a_namespace_the_evaluator_cannot_fold_needs_everything() {
  assert_eq!(
    namespaces_of("({ root: { method() { return 1 } } }).root"),
    NonNullProps::True
  );
  assert_eq!(
    namespaces_of("({ root: { get color() { return 1 } } }).root"),
    NonNullProps::True
  );
}

/// A spread is merged before the reader sees it, so the names it brought are
/// the names it records.
#[test]
fn reads_the_names_a_spread_brought() {
  assert_eq!(
    namespaces_of("({ root: { ...{ color: 'red' }, margin: '1px' } }).root"),
    named(&["color", "margin"])
  );
}

/// A key that is not a plain name is still a name the runtime needs, because
/// the evaluator names every key it folds.
#[test]
fn reads_a_key_that_is_not_a_plain_name() {
  assert_eq!(
    namespaces_of("({ root: { 'background-color': 'red' } }).root"),
    named(&["background-color"])
  );
}
