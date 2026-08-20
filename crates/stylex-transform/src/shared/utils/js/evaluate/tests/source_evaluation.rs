//! Evaluating one expression written as source, and asserting what came back.
//!
//! Every suite that asks the evaluator about author-written syntax needs the
//! same four steps — parse, evaluate under fresh globals, decide whether the
//! answer is a fold or a refusal, and say which value it is. One copy, because
//! two suites asserting "this refuses" through separately maintained helpers is
//! how one of them comes to accept a refusal with no reason.

use super::*;
use stylex_structures::stylex_options::StyleXOptions;
use swc_core::{
  common::{FileName, GLOBALS, Globals, SourceMap, sync::Lrc},
  ecma::parser::{EsSyntax, Parser, StringInput, Syntax, lexer::Lexer},
};

/// Parses one expression and evaluates it. Panics propagate, which is the
/// point: a refusal that aborts the build instead of deopting is the defect
/// [#1265](https://github.com/Dwlad90/stylex-swc-plugin/issues/1265) reported,
/// so a test that reaches one fails rather than reporting a refusal.
pub(crate) fn evaluate_source(source: &str) -> Box<EvaluateResult> {
  evaluate_source_under_ceiling(source, None)
}

/// The same, with the evaluator's depth ceiling raised.
///
/// The shipped default is sized for hand-written styles, so a case whose subject
/// is depth has to name the ceiling it wants rather than inherit one: without
/// this a test asserting that a hundred levels still fold would be asserting
/// that the default refuses them, which is a different claim and a true one.
pub(crate) fn evaluate_source_with_ceiling(
  source: &str,
  max_evaluation_depth: usize,
) -> Box<EvaluateResult> {
  evaluate_source_under_ceiling(source, Some(max_evaluation_depth))
}

fn evaluate_source_under_ceiling(
  source: &str,
  max_evaluation_depth: Option<usize>,
) -> Box<EvaluateResult> {
  let expr = parse_expr(source);
  let globals = Globals::new();

  GLOBALS.set(&globals, || {
    let mut options = StyleXOptions::default();

    if let Some(depth) = max_evaluation_depth {
      options.core.max_evaluation_depth = depth;
    }

    let mut traversal_state = StateManager::new(options);
    let fns = FunctionMap::default();

    evaluate(&expr, &mut traversal_state, &fns)
  })
}

pub(crate) fn parse_expr(source: &str) -> Expr {
  let source_map: Lrc<SourceMap> = Default::default();
  let source_file = source_map.new_source_file(FileName::Anon.into(), source.to_string());

  let lexer = Lexer::new(
    Syntax::Es(EsSyntax {
      jsx: true,
      ..Default::default()
    }),
    Default::default(),
    StringInput::from(&*source_file),
    None,
  );

  match Parser::new_from(lexer).parse_expr() {
    Ok(expr) => *expr,
    Err(error) => panic!("failed to parse `{}`: {:?}", source, error),
  }
}

/// Asserts the source refuses to fold, and does so as a deopt rather than by
/// aborting. The reason has to be there: `stylex.create()` turns it into the
/// author-facing diagnostic, so a refusal with no reason is a regression in
/// what a build error says.
#[track_caller]
pub(crate) fn assert_deopts(source: &str) {
  assert_deopt_result(&evaluate_source(source), source);
}

/// The same, with the depth ceiling raised for a source whose subject is depth.
#[track_caller]
pub(crate) fn assert_deopts_with_ceiling(source: &str, max_evaluation_depth: usize) {
  assert_deopt_result(
    &evaluate_source_with_ceiling(source, max_evaluation_depth),
    source,
  );
}

#[track_caller]
fn assert_deopt_result(result: &EvaluateResult, source: &str) {
  assert!(
    !result.confident,
    "expected `{}` to refuse to fold, got {:?}",
    source, result.value
  );

  assert!(
    result.reason.is_some(),
    "expected `{}` to record a deopt reason",
    source
  );
}

/// Asserts the refusal names the property that could not be read. The node kind
/// is the half an author can already see; which property was asked for is the
/// half that says why the declaration will not fold.
#[track_caller]
pub(crate) fn assert_deopt_names_property(source: &str, property: &str) {
  let result = evaluate_source(source);

  assert!(
    !result.confident,
    "expected `{}` to refuse to fold, got {:?}",
    source, result.value
  );

  match result.reason {
    Some(reason) => assert!(
      reason.contains(property),
      "expected the refusal of `{}` to name `{}`, got {:?}",
      source,
      property,
      reason
    ),
    None => panic!("expected `{}` to record a deopt reason", source),
  }
}

/// Asserts the source folds to a value. Guards the refusals above from being
/// satisfied by an evaluator that folds nothing at all.
#[track_caller]
pub(crate) fn assert_folds(source: &str) -> Expr {
  assert_folds_result(*evaluate_source(source), source)
}

/// The same, with the depth ceiling raised for a source whose subject is depth.
#[track_caller]
pub(crate) fn assert_folds_with_ceiling(source: &str, max_evaluation_depth: usize) -> Expr {
  assert_folds_result(
    *evaluate_source_with_ceiling(source, max_evaluation_depth),
    source,
  )
}

#[track_caller]
fn assert_folds_result(result: EvaluateResult, source: &str) -> Expr {
  assert!(
    result.confident,
    "expected `{}` to fold, got a deopt: {:?}",
    source, result.reason
  );

  match result.value {
    Some(EvaluateResultValue::Expr(expr)) => expr,
    other => panic!(
      "expected `{}` to fold to an expression, got {:?}",
      source, other
    ),
  }
}

#[track_caller]
pub(crate) fn assert_folds_to_string(source: &str, expected: &str) {
  match assert_folds(source) {
    Expr::Lit(Lit::Str(strng)) => assert_eq!(
      convert_atom_to_string(&strng.value),
      expected,
      "wrong folded string for `{}`",
      source
    ),
    other => panic!("expected `{}` to fold to a string, got {:?}", source, other),
  }
}

/// The same, with the depth ceiling raised for a source whose subject is depth.
#[track_caller]
pub(crate) fn assert_folds_to_string_with_ceiling(
  source: &str,
  expected: &str,
  max_evaluation_depth: usize,
) {
  match assert_folds_with_ceiling(source, max_evaluation_depth) {
    Expr::Lit(Lit::Str(strng)) => assert_eq!(
      convert_atom_to_string(&strng.value),
      expected,
      "wrong folded string for `{}`",
      source
    ),
    other => panic!("expected `{}` to fold to a string, got {:?}", source, other),
  }
}

/// Asserts the source folds to a number. Spelled as an exact value rather than
/// "some number", because a confident answer that is not the right one is the
/// failure mode the `length` fold exists to remove.
#[track_caller]
pub(crate) fn assert_folds_to_number(source: &str, expected: f64) {
  match assert_folds(source) {
    Expr::Lit(Lit::Num(num)) => {
      assert_eq!(num.value, expected, "wrong folded number for `{}`", source)
    },
    other => panic!("expected `{}` to fold to a number, got {:?}", source, other),
  }
}

/// The same, with the depth ceiling raised for a source whose subject is depth.
#[track_caller]
pub(crate) fn assert_folds_to_number_with_ceiling(
  source: &str,
  expected: f64,
  max_evaluation_depth: usize,
) {
  match assert_folds_with_ceiling(source, max_evaluation_depth) {
    Expr::Lit(Lit::Num(num)) => {
      assert_eq!(num.value, expected, "wrong folded number for `{}`", source)
    },
    other => panic!("expected `{}` to fold to a number, got {:?}", source, other),
  }
}

/// Asserts the source folds to `undefined` — a value the evaluator is confident
/// about, not a refusal. That answer exists so a declaration folds its fallback
/// instead of reaching the runtime.
#[track_caller]
pub(crate) fn assert_folds_to_undefined(source: &str) {
  match assert_folds(source) {
    Expr::Ident(ident) => assert_eq!(
      ident.sym.as_ref(),
      "undefined",
      "wrong folded identifier for `{}`",
      source
    ),
    other => panic!(
      "expected `{}` to fold to undefined, got {:?}",
      source, other
    ),
  }
}
