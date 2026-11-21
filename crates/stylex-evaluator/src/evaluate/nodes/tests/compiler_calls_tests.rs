//! What the dispatch does with a StyleX call inside a call the engine fold
//! declined.

use std::cell::Cell;

use super::*;

use swc_core::common::{GLOBALS, Globals};

use crate::evaluate::source_evaluation::*;
use crate::tests::scaffolding::deep_or;

thread_local! {
  /// How often [`counted`] ran on this thread.
  static CALLS: Cell<usize> = const { Cell::new(0) };
}

/// A compiler function that counts its calls and answers its argument. It
/// files a rule as `keyframes` does, so the memo does not replay it.
fn counted(expr: Expr, state: &mut StateManager) -> Expr {
  CALLS.set(CALLS.get() + 1);
  state.note_nested_rule_filed();

  expr
}

/// A function map that holds [`counted`] under the name `kf`.
fn counting_fold() -> FunctionMap {
  let mut fns = FunctionMap::default();

  fns.identifiers.insert(
    "kf".into(),
    Box::new(folded_entry(FunctionType::StylexExprFn(counted), false)),
  );
  fns.identifiers.insert(
    "marker".into(),
    Box::new(folded_entry(
      FunctionType::DefaultMarker(std::sync::Arc::new(indexmap::IndexMap::new())),
      false,
    )),
  );

  fns
}

/// The engine declines the call as written, because `marker()` is a function
/// of the map it may not run. It declines it again with its parts written in,
/// because the value of `marker()` has no expression form. Each part is
/// evaluated once, so the compiler function runs once.
#[test]
fn a_part_is_evaluated_once_when_the_engine_declines_twice() {
  CALLS.set(0);

  let source = "f(kf('a'), marker())";
  let result = evaluated_in_a_state(
    |state| state.push_declaration(a_declaration_of("f", "(x) => x")),
    &counting_fold(),
    source,
  );

  assert_eq!(folded_text_of(result, source), "a");
  assert_eq!(CALLS.get(), 1);
}

/// A dynamic import has no callee expression, so it has no parts to write in,
/// and it refuses as before.
#[test]
fn a_dynamic_import_has_no_parts_to_fold() {
  let source = "import('a')";

  assert_refused(&evaluated_against(&counting_fold(), source), source);
}

/// A name the function map binds to a plain value, as a callback binds its
/// parameter, is not a compiler function.
#[test]
fn a_bound_parameter_is_not_a_compiler_function() {
  GLOBALS.set(&Globals::new(), || {
    let mut fns = FunctionMap::default();

    fns.identifiers.insert(
      "v".into(),
      Box::new(folded_entry(
        FunctionType::Mapper(std::rc::Rc::new(|| parse_expr("'a'"))),
        false,
      )),
    );
    fns
      .identifiers
      .insert("kf".into(), Box::new(a_folded_function()));

    let state = EvaluationState::with_functions(std::rc::Rc::new(fns));

    assert!(!is_a_compiler_function(&parse_expr("v.concat"), &state));
    assert!(is_a_compiler_function(&parse_expr("kf"), &state));
  });
}

/// A part nested far deeper than a test thread can recurse is searched on a
/// stack that grows, so the search cannot overflow.
#[test]
fn a_deep_part_is_searched_without_overflow() {
  on_a_thread_of(SMALL_THREAD, || {
    GLOBALS.set(&Globals::new(), || {
      let state = EvaluationState::with_functions(std::rc::Rc::new(counting_fold()));
      let deep = deep_or(100_000);

      assert!(!holds_a_compiler_call(&deep, &state));

      // A tree this deep drops by recursion too, so it is not dropped here.
      std::mem::forget(deep);
    });
  });
}
