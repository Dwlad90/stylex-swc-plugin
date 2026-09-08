//! What the printed expression comes to once the transported values are passed
//! to it.
//!
//! The step is asked directly rather than through a source, because both things
//! it answers for are shapes no source produces. The printed text is an arrow
//! whenever there is anything to pass, so it always evaluates to a function —
//! the refusal for one that does not stands in for a broken invariant. And a
//! throw from the call is refused by the guard before it is printed, since
//! every expression that could throw is held back.
//!
//! Both are kept rather than deleted: what makes them unreachable is the
//! printing on one side and a list of admitted shapes on the other, and either
//! is a refusal here rather than a panic or a wrong value the day it changes.

use super::*;

use std::mem::ManuallyDrop;

use super::engine_reads::assert_refused_saying;
use crate::tests::scaffolding::parse_expr;
use swc_core::common::{GLOBALS, Globals};

/// The method the fold is running, which names the throw an author reads.
const METHOD: &str = "map";

/// A key no two cases share, so nothing here reads another case's script.
fn key_numbered(parameters: u128) -> FoldKey {
  let call = match GLOBALS.set(&Globals::new(), || parse_expr("[].map(f)")) {
    Expr::Call(call) => call,
    other => panic!("`[].map(f)` parsed as {:?}", other),
  };

  FoldKey::new(&call, parameters)
}

/// What `source` comes to when `arguments` are passed to it.
///
/// One printing closure for every case, because the print is a type parameter
/// of the step: a closure per case compiles the same steps again and no one
/// compilation would then read all of them.
fn applied(
  source: &str,
  arguments: &[JsValue],
  engine: &mut Engine,
  key: FoldKey,
) -> Result<JsValue, Decline> {
  apply(
    key,
    || source.to_string(),
    arguments,
    engine,
    &Atom::from(METHOD),
  )
}

/// An engine built the way a fold builds one.
fn engine() -> ManuallyDrop<Engine> {
  match Engine::new() {
    Ok(engine) => engine,
    Err(Decline::Rule(reason)) => panic!("the engine would not start: {}", reason),
    Err(Decline::NotACandidate) => panic!("the engine was handed back"),
  }
}

/// With something to pass, the printed arrow is called with it.
#[test]
fn the_printed_arrow_is_called_with_what_the_transport_carried() {
  let mut engine = engine();
  let carried = [JsValue::from(2), JsValue::from(3)];

  match applied("(a, b) => a * b", &carried, &mut engine, key_numbered(1)) {
    Ok(value) => assert_eq!(value.as_number(), Some(6.0)),
    Err(_) => panic!("`(a, b) => a * b` refused"),
  }
}

/// With nothing to pass, the printed expression is its own answer and nothing
/// is called — which is what lets an expression that names nothing pay for no
/// function object and no frame.
#[test]
fn a_print_with_nothing_to_pass_is_its_own_answer() {
  let mut engine = engine();

  match applied("'ab'.repeat(2)", &[], &mut engine, key_numbered(2)) {
    Ok(value) => assert_eq!(
      value.as_string().map(|text| text.to_std_string_escaped()),
      Some("abab".to_string())
    ),
    Err(_) => panic!("`'ab'.repeat(2)` refused"),
  }
}

/// A print that is not a function has nothing to pass the values to, and is
/// refused under the method rather than asserted on.
#[test]
fn a_print_that_is_not_a_function_refuses_under_the_method() {
  let mut engine = engine();

  assert_refused_saying(
    applied("42", &[JsValue::from(1)], &mut engine, key_numbered(3)),
    "42",
    METHOD,
  );
}

/// A call that throws is refused in the engine's own words, since the throw is
/// an answer about the source rather than a failure of this module.
#[test]
fn a_call_that_throws_refuses_in_the_engines_words() {
  let mut engine = engine();
  let source = "(a) => { throw new TypeError('this call will not run'); }";

  assert_refused_saying(
    applied(source, &[JsValue::from(1)], &mut engine, key_numbered(4)),
    source,
    "this call will not run",
  );
}

/// A print the engine cannot read is refused before anything is passed to it,
/// so a source that will not parse never reaches the call.
///
/// The refusal is the engine's, under the method the fold is running, which is
/// the same sentence the evaluation carries for a source that throws.
#[test]
fn a_print_the_engine_cannot_read_refuses_before_the_call() {
  let mut engine = engine();

  assert_refused_saying(
    applied("(", &[JsValue::from(1)], &mut engine, key_numbered(5)),
    "(",
    METHOD,
  );
}
