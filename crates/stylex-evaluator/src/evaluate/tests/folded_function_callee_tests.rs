//! Calling one of the compiler's own functions.
//!
//! The [folded function map](../../../../../CONTEXT.md#folded-function-map)
//! binds `create`, `keyframes`, the `when` markers and the `env` option to
//! names before the fold starts, and a call on one of those names is applied
//! here rather than in the engine: the entries are Rust functions and most of
//! them write into the build, so none of them has a JavaScript form to cross.
//!
//! Each entry kind takes its arguments differently -- some take the expressions
//! the author wrote, some take what those expressions folded to, and one takes
//! a map read out of an object literal -- so each is applied by its own arm.
//! An arm that read the wrong argument shape would not refuse; it would call
//! the compiler's own function with the wrong value, so every kind is pinned
//! here.
//!
//! Both spellings of the call are covered, because the same entry is reachable
//! as a bare name and as a member of the namespace it was imported under, and
//! the two lookups are separate code.

use std::rc::Rc;
use std::sync::Arc;

use indexmap::IndexMap;

use super::source_evaluation::*;
use stylex_constants::constants::evaluation_errors::NON_CONSTANT;
use stylex_enums::value_with_default::ValueWithDefault;
use stylex_state::{
  evaluate_result_value::EvaluateResultValue,
  functions::{FunctionConfigType, FunctionMap, FunctionType},
  types::{FunctionConfigMap, FunctionMapIdentifiers},
};
use stylex_structures::{
  named_import_source::ImportSources,
  stylex_env::{EnvEntry, JSFunction},
};
use swc_core::ecma::ast::Expr;

use stylex_ast::ast::convertors::{convert_atom_to_string, create_string_expr};

/// The name every case calls the entry under, and the namespace it is also
/// reachable through.
const CALLED: &str = "make";
const NAMESPACE: &str = "sx";

/// A map holding `entry` under both spellings: as the bare name a named import
/// binds, and as a property of the namespace a star import binds.
fn map_holding(entry: FunctionConfigType) -> FunctionMap {
  let mut fns = FunctionMap::default();

  fns
    .identifiers
    .insert(CALLED.into(), Box::new(entry.clone()));

  let mut members = FunctionMapIdentifiers::default();

  members.insert(CALLED.into(), Box::new(entry));

  fns.member_expressions.insert(
    ImportSources::Regular(NAMESPACE.to_string()),
    Box::new(members),
  );

  fns
}

/// The text `source` folds to, called under both spellings, which have to
/// answer the same thing: an author who imported the name and an author who
/// imported the namespace wrote the same call.
#[track_caller]
fn folded_both_ways(entry: FunctionConfigType, arguments: &str) -> String {
  let fns = map_holding(entry);

  let bare = folded_call(&fns, &format!("{CALLED}({arguments})"));
  let member = folded_call(&fns, &format!("{NAMESPACE}.{CALLED}({arguments})"));

  assert_eq!(
    bare, member,
    "the two spellings of the same call answered differently"
  );

  bare
}

#[track_caller]
fn folded_call(fns: &FunctionMap, source: &str) -> String {
  folded_text_of(evaluated_against(fns, source), source)
}

#[track_caller]
fn assert_refuses_both_ways(entry: FunctionConfigType, arguments: &str, reason: &str) {
  let fns = map_holding(entry);

  for source in [
    format!("{CALLED}({arguments})"),
    format!("{NAMESPACE}.{CALLED}({arguments})"),
  ] {
    assert_refused_with(&evaluated_against(&fns, &source), &source, reason);
  }
}

/// A function of the arguments' own count, so a case can tell a call that read
/// its arguments from one that read none.
fn joined(args: Vec<Expr>) -> Expr {
  let parts = args
    .iter()
    .map(|arg| match arg.as_lit().and_then(|lit| lit.as_str()) {
      Some(text) => convert_atom_to_string(&text.value),
      None => String::from("?"),
    })
    .collect::<Vec<String>>();

  create_string_expr(&parts.join("-"))
}

// ==================== the kinds that take evaluated arguments ====================

/// A list-taking function reads every argument, in order.
#[test]
fn a_list_taking_function_reads_every_argument() {
  assert_eq!(
    folded_both_ways(
      folded_entry(FunctionType::ArrayArgs(|args, _, _| joined(args)), false),
      "'a', 'b'"
    ),
    "a-b"
  );
}

/// The same function registered to take the expressions the author wrote rather
/// than what they folded to. The arguments arrive unfolded, so an expression
/// that is not a literal reaches the function itself.
#[test]
fn a_list_taking_function_can_take_the_written_arguments() {
  assert_eq!(
    folded_both_ways(
      folded_entry(FunctionType::ArrayArgs(|args, _, _| joined(args)), true),
      "'a', 'b'"
    ),
    "a-b"
  );
  assert_eq!(
    folded_both_ways(
      folded_entry(FunctionType::ArrayArgs(|args, _, _| joined(args)), true),
      "'a' + 'b'"
    ),
    "?"
  );
}

/// A one-expression function reads the first argument and nothing else.
#[test]
fn a_one_expression_function_reads_the_first_argument() {
  assert_eq!(
    folded_both_ways(
      folded_entry(FunctionType::StylexExprFn(|expr, _| expr), false),
      "'a', 'b'"
    ),
    "a"
  );
  assert_eq!(
    folded_both_ways(
      folded_entry(FunctionType::StylexExprFn(|expr, _| expr), true),
      "'a', 'b'"
    ),
    "a"
  );
}

/// A one-expression function with no argument at all has nothing to read, and
/// says so rather than calling the compiler's own function with a value it
/// invented.
#[test]
fn a_one_expression_function_with_no_argument_refuses() {
  assert_refuses_both_ways(
    folded_entry(FunctionType::StylexExprFn(|expr, _| expr), false),
    "",
    "StyleX expression function requires an expression argument.",
  );
  assert_refuses_both_ways(
    folded_entry(FunctionType::StylexExprFn(|expr, _| expr), true),
    "",
    "StyleX expression function requires at least one argument.",
  );
}

/// A `when` function takes a selector and an optional marker, so the second
/// argument is not required and its absence is not a refusal.
#[test]
fn a_when_function_takes_a_selector_and_an_optional_marker() {
  let entry = || {
    folded_entry(
      FunctionType::StylexWhenFn(|pseudo, marker, _| {
        let selector = folded_or(&pseudo, "?");
        let observed = marker.map_or(String::from("default"), |value| folded_or(&value, "?"));

        create_string_expr(&format!("{selector}/{observed}"))
      }),
      false,
    )
  };

  assert_eq!(folded_both_ways(entry(), "':hover'"), ":hover/default");
  assert_eq!(folded_both_ways(entry(), "':hover', 'own'"), ":hover/own");
}

/// A `when` function with no selector at all refuses, because the selector is
/// the argument the function exists to read.
#[test]
fn a_when_function_with_no_selector_refuses() {
  assert_refuses_both_ways(
    folded_entry(
      FunctionType::StylexWhenFn(|_, _, _| create_string_expr("")),
      false,
    ),
    "",
    "stylex.when functions require a selector argument.",
  );
}

/// A type function reads a map out of the object literal it was handed, and a
/// bare literal is read as that map's `default` entry -- the two spellings
/// `stylex.types.length('1px')` and `stylex.types.length({ default: '1px' })`
/// have.
#[test]
fn a_type_function_reads_an_object_or_a_bare_literal() {
  let entry = || {
    folded_entry(
      FunctionType::StylexTypeFn(Rc::new(|value| {
        create_string_expr(&describe_value_with_default(&value))
      })),
      false,
    )
  };

  assert_eq!(
    folded_both_ways(entry(), "{ default: '1px', hover: '2px' }"),
    "default=1px,hover=2px"
  );
  assert_eq!(folded_both_ways(entry(), "'1px'"), "default=1px");
}

/// An `env` function takes the arguments as expressions, which is what the
/// option's own closure is written against.
#[test]
fn an_env_function_takes_its_arguments_as_expressions() {
  assert_eq!(
    folded_both_ways(
      folded_entry(FunctionType::EnvFunction(JSFunction::new(joined)), false),
      "'a', 'b'"
    ),
    "a-b"
  );
}

/// A callback entry is an arrow the module wrote, applied to the call's
/// arguments exactly as a call on the name that holds it would be.
#[test]
fn a_callback_entry_is_applied_to_the_calls_arguments() {
  assert_eq!(
    folded_both_ways(
      folded_entry(
        FunctionType::Callback(Box::new(parse_expr("(a, b) => a + b"))),
        false
      ),
      "'a', 'b'"
    ),
    "ab"
  );
}

// ==================== the kinds no call can apply ====================

/// The remaining kinds take evaluated arguments, so registering one to take the
/// written arguments instead is a call this position cannot make. It refuses
/// rather than calling the function with the wrong shape.
#[test]
fn a_kind_that_cannot_take_the_written_arguments_refuses() {
  for fn_ptr in [
    FunctionType::StylexWhenFn(|_, _, _| create_string_expr("")),
    FunctionType::StylexTypeFn(Rc::new(|_| create_string_expr(""))),
    FunctionType::StylexFnsFactory(|_| Rc::new(|_| create_string_expr(""))),
    FunctionType::Callback(Box::new(parse_expr("() => 'a'"))),
    FunctionType::DefaultMarker(Arc::new(IndexMap::default())),
    FunctionType::EnvFunction(JSFunction::new(joined)),
  ] {
    assert_refuses_both_ways(folded_entry(fn_ptr, true), "'a'", NON_CONSTANT);
  }
}

/// An entry that is not a function at all -- a namespace, a marker map, the
/// `env` object -- is reached through a member read rather than called, so a
/// call on one refuses.
#[test]
fn an_entry_that_is_not_a_function_refuses_a_call() {
  for entry in [
    FunctionConfigType::Map(FunctionConfigMap::default()),
    FunctionConfigType::IndexMap(IndexMap::default()),
    FunctionConfigType::EnvObject(Rc::new(IndexMap::<String, EnvEntry>::default())),
  ] {
    assert_refuses_both_ways(entry, "'a'", NON_CONSTANT);
  }
}

// ==================== reading the values a case asserts on ====================

/// The text a folded value holds, or `fallback` where it holds none, so a case
/// can describe what a function was handed without also saying how each value
/// is represented.
fn folded_or(value: &EvaluateResultValue, fallback: &str) -> String {
  match value
    .as_expr()
    .and_then(|expr| expr.as_lit())
    .and_then(|lit| lit.as_str())
  {
    Some(text) => convert_atom_to_string(&text.value),
    None => fallback.to_string(),
  }
}

/// The map a type function was handed, written out so a case can compare it.
fn describe_value_with_default(value: &ValueWithDefault) -> String {
  match value {
    ValueWithDefault::Map(map) => map
      .iter()
      .map(|(key, entry)| format!("{key}={}", describe_value_with_default(entry)))
      .collect::<Vec<String>>()
      .join(","),
    ValueWithDefault::String(text) => text.clone(),
    ValueWithDefault::Number(number) => number.to_string(),
  }
}
