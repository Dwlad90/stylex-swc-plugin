//! The losing operand of `||`, `&&` and `??` inside an engine fold.
//!
//! The evaluator runs both operands of these operators. So a compiler
//! function on the side that loses still files its rule. `file` stands for
//! such a function: it records a filed rule and answers its argument.

use super::evaluate;
use super::source_evaluation::*;
use stylex_state::{
  functions::{FunctionMap, FunctionType},
  state_manager::StateManager,
};
use stylex_structures::stylex_options::StyleXOptions;
use swc_core::common::{GLOBALS, Globals};
use swc_core::ecma::ast::Expr;

/// A function that records a filed rule, as `keyframes` does, and answers its
/// argument.
fn files_a_rule(expr: Expr, state: &mut StateManager) -> Expr {
  state.note_nested_rule_filed();

  expr
}

/// The function fold, with `file` bound to [`files_a_rule`].
fn with_file() -> FunctionMap {
  let mut fns = a_function_fold();

  fns.identifiers.insert(
    "file".into(),
    Box::new(folded_entry(
      FunctionType::StylexExprFn(files_a_rule),
      false,
    )),
  );

  fns
}

/// The evaluation of `source` against [`with_file`], after `declarations`
/// were recorded, and how many rules the build state recorded.
fn evaluated_with_filings(
  declarations: &[(&str, &str)],
  source: &str,
) -> (Box<crate::evaluate_result::EvaluateResult>, u64) {
  GLOBALS.set(&Globals::new(), || {
    let mut state = StateManager::new(StyleXOptions::default());

    for (name, init) in declarations {
      state.push_declaration(a_declaration_of(name, init));
    }

    let result = evaluate(&parse_expr(source), &mut state, &with_file());

    (result, state.nested_rules_filed())
  })
}

/// The text `source` folds to, and how many rules it filed.
#[track_caller]
fn folded_with_filings(source: &str) -> (String, u64) {
  let (result, filed) = evaluated_with_filings(&[], source);

  (folded_text_of(result, source), filed)
}

/// The left operand decides, and the right operand still runs once.
#[test]
fn a_losing_operand_files_its_rule() {
  for (source, expected) in [
    ("['x' || file('a')].join()", "x"),
    ("[false && file('a'), 'y'].join()", "false,y"),
    ("['x' ?? file('a')].join()", "x"),
    ("('x' || file('a')).concat('')", "x"),
    ("'y'.concat('x' || file('a'))", "yx"),
  ] {
    assert_eq!(
      folded_with_filings(source),
      (expected.into(), 1),
      "{source}"
    );
  }
}

/// In a callback the engine decides each operator, and the right operand runs
/// whichever side wins.
#[test]
fn a_losing_operand_in_a_callback_files_its_rule() {
  for (source, expected) in [
    ("['x'].map((v) => v || file('a')).join()", "x"),
    ("[''].map((v) => v || file('a')).join()", "a"),
    ("[0].map((v) => v && file('a')).join()", "0"),
    ("[1].map((v) => v && file('a')).join()", "a"),
    ("['x'].map((v) => v ?? file('a')).join()", "x"),
    ("[null].map((v) => v ?? file('a')).join()", "a"),
  ] {
    assert_eq!(
      folded_with_filings(source),
      (expected.into(), 1),
      "{source}"
    );
  }
}

/// A right operand that throws has no effect where the left one decides, and
/// is the refusal where the right one is the answer.
#[test]
fn a_throw_counts_only_on_the_side_that_wins() {
  assert_eq!(
    folded_with_filings("['x'].map((v) => v || file(v.a.b)).join()"),
    ("x".into(), 0)
  );

  let source = "[''].map((v) => v || file(v.a.b)).join()";
  let (result, filed) = evaluated_with_filings(&[], source);

  assert_refused(&result, source);
  assert_eq!(filed, 0);
}

/// A losing operand the walk refuses is left out, as the evaluator ignores a
/// refusal on the side it does not use. In a callback the walk cannot tell
/// which side wins, so the refusal is the answer.
#[test]
fn a_losing_operand_the_walk_refuses() {
  assert_eq!(
    folded_with_filings("['x' || file(unknown)].join()"),
    ("x".into(), 0)
  );

  let source = "['x'].map((v) => v || file(unknown)).join()";
  let (result, _) = evaluated_with_filings(&[], source);

  assert_refused(&result, source);
}

/// Only the operator whose right operand calls a compiler function changes,
/// and each operator is found by its position among the others. An operand
/// that decides and calls a compiler function is not read before the engine
/// runs, so its rule is filed once.
#[test]
fn only_the_recorded_operator_changes() {
  for (source, expected) in [
    ("['x' || 'y', 'z' || file('a')].join()", "x,z"),
    ("['a' + 'b', 'x' || file('c')].join()", "ab,x"),
    ("['x' || ('y' || file('a'))].join()", "x"),
    ("[('x' || file('a')) || 'y'].join()", "x"),
    ("[file('a') || 'y'].join()", "a"),
    ("[file('a') ? 'b' : 'c'].join()", "b"),
  ] {
    assert_eq!(
      folded_with_filings(source),
      (expected.into(), 1),
      "{source}"
    );
  }
}

/// A losing operand inside a callback that crosses as a declaration runs too.
#[test]
fn a_losing_operand_in_a_named_callback_files_its_rule() {
  let source = "['x', ''].map(mk).join()";
  let (result, filed) = evaluated_with_filings(&[("mk", "(v) => v || file('a')")], source);

  assert_eq!(folded_text_of(result, source), "x,a");
  assert_eq!(filed, 2);
}
