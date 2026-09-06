//! What a member read answers, receiver kind by receiver kind.
//!
//! A read is not a call, so nothing here goes through the compile-time engine:
//! the evaluator answers every one of these itself. What it has to get right is
//! the parting between three answers -- the value under the key, `undefined`
//! for a key the receiver does not carry, and a refusal for a lookup it cannot
//! read at all.
//!
//! `undefined` rather than a refusal is the half that matters most: it is what
//! lets `token.missing ?? fallback` fold, where a refusal would send the whole
//! declaration to the runtime. A refusal rather than `undefined` is the other
//! half: answering `undefined` for a lookup the evaluator could not read would
//! write a value the source does not describe.

use std::rc::Rc;

use super::source_evaluation::*;
use indexmap::IndexMap;
use stylex_ast::ast::convertors::create_string_expr;
use stylex_constants::constants::evaluation_errors::{
  OBJECT_METHOD, UNEXPECTED_MEMBER_LOOKUP, unreadable_index, unsupported_expression,
};
use stylex_constants::constants::messages::{PROPERTY_NOT_FOUND, THEME_IMPORT_KEY_AS_OBJECT_KEY};
use stylex_state::{
  evaluate_result_value::EvaluateResultValue,
  functions::{FunctionConfigType, FunctionMap, FunctionType},
  theme_ref::ThemeRef,
  types::FunctionConfigMap,
};
use stylex_structures::stylex_env::{EnvEntry, JSFunction};

// ==================== a key the receiver does not carry ====================

/// Every receiver that carries keys answers `undefined` for one it does not,
/// which is the language's own answer and what a fallback beside the read then
/// folds against.
#[test]
fn a_key_the_receiver_does_not_carry_is_undefined() {
  for source in [
    "'abc'.foo",
    "[1, 2].foo",
    "[1, 2][5]",
    "({ a: 1 }).b",
    "({ ...({ a: 1 }) }).b",
  ] {
    assert_folds_to_undefined(source);
  }

  assert_folds_to_string("({ a: 1 }).b ?? 'red'", "red");
}

/// Reading a property off the `undefined` a missing key answered throws in the
/// language, so a second read has to refuse rather than answer `undefined`
/// again -- which would fold a chain that cannot exist.
#[test]
fn a_read_off_an_absent_key_refuses_rather_than_answering_undefined_twice() {
  assert_deopt_reason_contains("({}).a.b", UNEXPECTED_MEMBER_LOOKUP);
}

// ==================== a lookup the evaluator cannot read ====================

/// An index into a string is a single UTF-16 code unit, which can be an
/// unpaired surrogate that no Rust string holds. Refused rather than
/// approximated, and the refusal names the index that was asked for.
#[test]
fn an_index_into_a_string_is_refused_by_the_index_it_names() {
  assert_deopt_reason_contains("'abc'[0]", &unreadable_index("0"));
  assert_deopt_reason_contains("'abc'[2]", &unreadable_index("2"));
}

/// A computed key that folded to something with no name reads no property.
/// The refusal names the object rather than the key, because the key is a
/// value and the object is where the author has to look.
#[test]
fn a_computed_key_with_no_name_reads_no_property() {
  for source in ["({ a: 1 })[['a']]", "({ a: 1 })[Object.keys({ b: 1 })]"] {
    assert_deopt_reason_contains(source, PROPERTY_NOT_FOUND);
  }
}

/// A property carried by a getter, a setter or a method has no value to read,
/// so the whole lookup refuses -- including for a key the object does carry
/// plainly, because a receiver that cannot be read whole cannot be read at all.
#[test]
fn a_property_that_is_not_a_value_refuses_the_whole_lookup() {
  assert_deopt_reason_contains("({ get a() { return 1; } }).b", OBJECT_METHOD);
}

/// A receiver kind the evaluator reads no properties from names itself, so the
/// author sees which of the two halves of the expression stopped the fold.
#[test]
fn a_receiver_with_no_properties_names_its_own_kind() {
  for (source, kind) in [
    ("(1).foo", "NumericLiteral"),
    ("true.foo", "BooleanLiteral"),
    ("null.foo", "NullLiteral"),
    ("({ a: 1 }).a.b", "NumericLiteral"),
  ] {
    assert_deopt_reason_contains(source, &unsupported_expression(kind));
  }
}

/// A receiver the evaluator holds in a representation of its own, with no keys
/// to read -- an arrow folds to a callback rather than to an expression.
#[test]
fn a_receiver_the_evaluator_holds_its_own_way_refuses() {
  assert_deopt_reason_contains("(() => 1).foo", UNEXPECTED_MEMBER_LOOKUP);
}

// ==================== the compiler's own values as receivers ====================

/// A namespace nested inside a namespace answers the inner namespace, so a
/// member chain over the fold resolves at every link.
#[test]
fn a_namespace_inside_a_namespace_answers_the_inner_one() {
  let fns = map_binding("outer", nested_namespace());
  let value = folded_value_of(evaluated_against(&fns, "outer.inner"), "outer.inner");

  assert!(
    matches!(value, EvaluateResultValue::FunctionConfigMap(_)),
    "expected the inner namespace, got {:?}",
    value
  );
}

/// The `env` object answers itself, so the member read below it is what
/// resolves an entry.
#[test]
fn the_env_object_answers_itself_and_then_its_entries() {
  let fns = map_binding("sx", namespace_holding_the_env_object());

  assert!(
    matches!(
      folded_value_of(evaluated_against(&fns, "sx.env"), "sx.env"),
      EvaluateResultValue::EnvObject(_)
    ),
    "the env object answers itself"
  );

  assert_eq!(
    folded_text_of(
      evaluated_against(&fns, "sx.env.breakpoint"),
      "sx.env.breakpoint"
    ),
    "40rem"
  );
}

/// An `env` entry the option does not carry has no value, and an entry that is
/// a function answers the object it came from -- which is what lets the call
/// site below resolve the function rather than the read.
#[test]
fn an_env_entry_answers_by_what_the_option_configured_it_as() {
  let fns = map_binding("sx", namespace_holding_the_env_object());

  assert_refused_with(
    &evaluated_against(&fns, "sx.env.missing"),
    "sx.env.missing",
    "The property 'missing' was not found in the stylex.env configuration.",
  );

  assert!(
    matches!(
      folded_value_of(evaluated_against(&fns, "sx.env.spacing"), "sx.env.spacing"),
      EvaluateResultValue::EnvObject(_)
    ),
    "a function entry answers the object it is read off"
  );
}

/// An `env` object the build never configured names itself in the refusal,
/// rather than reading as an option that is merely empty.
#[test]
fn an_unconfigured_env_object_says_so() {
  let mut entries = FunctionConfigMap::default();

  entries.insert(
    "env".into(),
    FunctionConfigType::EnvObject(Rc::new(IndexMap::default())),
  );

  let fns = map_binding("sx", FunctionConfigType::Map(entries));

  assert_refused_with(
    &evaluated_against(&fns, "sx.env"),
    "sx.env",
    "The stylex.env object is not configured. Check that the 'env' option is set in your StyleX configuration.",
  );
}

/// The marker map is the one entry with no member surface of its own, so a
/// read on it names this compiler's shape rather than pretending to a key.
#[test]
fn a_marker_map_read_as_a_member_names_the_shape_it_is() {
  let mut entries = FunctionConfigMap::default();

  entries.insert(
    "markers".into(),
    FunctionConfigType::IndexMap(IndexMap::default()),
  );

  let fns = map_binding("sx", FunctionConfigType::Map(entries));

  assert_refused_with(
    &evaluated_against(&fns, "sx.markers"),
    "sx.markers",
    &unsupported_expression("IndexMap"),
  );
}

/// A `defineVars` group standing where an object key belongs is a shape neither
/// compiler folds. It happens when a minifier gives an imported token the name
/// of another binding, so the refusal names the collision rather than the key.
#[test]
fn a_group_used_as_an_object_key_refuses() {
  let mut fns = FunctionMap::default();
  let theme = ThemeRef::new("vars.stylex.js", "vars", "x");

  fns.identifiers.insert(
    "colors".into(),
    Box::new(folded_entry(
      FunctionType::ThemeRefMapper(Rc::new(move || theme.clone())),
      false,
    )),
  );

  assert_refused_with(
    &evaluated_against(&fns, "({ a: 1 })[colors]"),
    "({ a: 1 })[colors]",
    THEME_IMPORT_KEY_AS_OBJECT_KEY,
  );
}

/// One name bound to `entry`, which is how every case above reaches a value
/// the compiler holds rather than one a source writes.
fn map_binding(name: &str, entry: FunctionConfigType) -> FunctionMap {
  let mut fns = FunctionMap::default();

  fns.identifiers.insert(name.into(), Box::new(entry));

  fns
}

/// A namespace holding a namespace, which is the shape a nested member chain
/// reads.
fn nested_namespace() -> FunctionConfigType {
  let mut inner = FunctionConfigMap::default();

  inner.insert(FOLD_ENTRY.into(), a_folded_function());

  let mut outer = FunctionConfigMap::default();

  outer.insert("inner".into(), FunctionConfigType::Map(inner));

  FunctionConfigType::Map(outer)
}

/// A namespace holding the `env` option's object, which is how a module reaches
/// one: `stylex.env.<name>`.
fn namespace_holding_the_env_object() -> FunctionConfigType {
  let mut env: IndexMap<String, EnvEntry> = IndexMap::default();

  env.insert(
    "breakpoint".to_string(),
    EnvEntry::Expr(create_string_expr("40rem")),
  );
  env.insert(
    "spacing".to_string(),
    EnvEntry::Function(JSFunction::new(|_| create_string_expr("1px"))),
  );

  let mut entries = FunctionConfigMap::default();

  entries.insert("env".into(), FunctionConfigType::EnvObject(Rc::new(env)));

  FunctionConfigType::Map(entries)
}

// ==================== an array a fold produced ====================
//
// An array the evaluator answered is its own list; an array a *fold* answered
// is a literal. Both are arrays to the author, so both answer the same three
// ways -- and the literal is read by a separate arm, because a hole has no
// value to evaluate and only the syntax says how many slots were written.

/// The three answers a fold's own array gives, which are the three the
/// evaluator's own list gives.
#[test]
fn an_array_a_fold_produced_answers_the_same_three_ways() {
  assert_folds_to_string("Object.keys({ a: 1, b: 2 })[1]", "b");
  assert_folds_to_undefined("Object.keys({ a: 1 }).foo");
  assert_folds_to_undefined("Object.keys({ a: 1 })[7]");
}

/// A hole occupies a slot and carries no key, so an index past the hole names
/// the element the source wrote there -- and the count is the written slots
/// rather than the keys.
#[test]
fn a_hole_occupies_a_slot_without_carrying_a_key() {
  assert_folds_to_string("Object.keys([, 'a'])[0]", "1");
  assert_folds_to_number("[, 'a'].length", 2.0);
  assert_folds_to_number("['a', , 'b'].length", 3.0);
}
