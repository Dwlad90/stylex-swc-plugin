//! Evaluating one expression written as source, and asserting what came back.
//!
//! Every suite that asks the evaluator about author-written syntax needs the
//! same four steps — parse, evaluate under fresh globals, decide whether the
//! answer is a fold or a refusal, and say which value it is. One copy, because
//! two suites asserting "this refuses" through separately maintained helpers is
//! how one of them comes to accept a refusal with no reason.
//!
//! [`on_a_thread_of`] is here for the same reason and is the one thing here that
//! is not about source: a case whose subject is stack has to say how much of it
//! there is, and two suites need to.
//!
//! [`folded_in_a_module_binding`] is the other thing two suites need: an
//! expression that resolves no name is printed with no parameters, so a case
//! about what the transport carries has to evaluate against a module that binds
//! something.

use super::*;
use stylex_structures::stylex_options::StyleXOptions;
use swc_core::{
  common::{DUMMY_SP, FileName, GLOBALS, Globals, SourceFile, SourceMap, SyntaxContext, sync::Lrc},
  ecma::{
    ast::{BindingIdent, Module},
    parser::{EsSyntax, Parser, StringInput, Syntax, lexer::Lexer},
    visit::{Visit, VisitWith},
  },
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
  evaluate_parsed(&parse_expr(source), max_evaluation_depth)
}

/// Evaluates an expression that was parsed out of something larger, so a suite
/// reading a whole file asks the same evaluator under the same globals as one
/// writing its subject out as source.
pub(crate) fn evaluate_expr(expr: &Expr) -> Box<EvaluateResult> {
  evaluate_parsed(expr, None)
}

fn evaluate_parsed(expr: &Expr, max_evaluation_depth: Option<usize>) -> Box<EvaluateResult> {
  let globals = Globals::new();

  GLOBALS.set(&globals, || {
    let mut options = StyleXOptions::default();

    if let Some(depth) = max_evaluation_depth {
      options.core.max_evaluation_depth = depth;
    }

    let mut traversal_state = StateManager::new(options);
    let fns = FunctionMap::default();

    evaluate(expr, &mut traversal_state, &fns)
  })
}

pub(crate) fn parse_expr(source: &str) -> Expr {
  let file = anonymous_file(source);

  match parser_for(&file).parse_expr() {
    Ok(expr) => *expr,
    Err(error) => panic!("failed to parse `{}`: {:?}", source, error),
  }
}

fn anonymous_file(source: &str) -> Lrc<SourceFile> {
  let source_map: Lrc<SourceMap> = Default::default();

  source_map.new_source_file(FileName::Anon.into(), source.to_string())
}

/// A parser over one file, in the syntax every suite here reads.
///
/// One copy for the same reason the assertions have one: an expression and the
/// module it was written in have to be read under the same syntax, or a suite
/// comes to disagree with another about what the author wrote.
fn parser_for(file: &SourceFile) -> Parser<Lexer<'_>> {
  Parser::new_from(Lexer::new(
    Syntax::Es(EsSyntax {
      jsx: true,
      ..Default::default()
    }),
    Default::default(),
    StringInput::from(file),
    None,
  ))
}

/// Every call expression written anywhere in a module, in source order.
///
/// For a suite whose subject is a whole file rather than one expression -- the
/// benchmark fixture that exists to fold nothing has to be read as the module
/// it is, since one call it never lists would change what it measures.
pub(crate) fn call_expressions(source: &str) -> Vec<Expr> {
  #[derive(Default)]
  struct Calls(Vec<Expr>);

  impl Visit for Calls {
    fn visit_call_expr(&mut self, call: &CallExpr) {
      self.0.push(Expr::Call(call.clone()));
      call.visit_children_with(self);
    }
  }

  let mut calls = Calls::default();
  parse_module(source).visit_with(&mut calls);

  calls.0
}

fn parse_module(source: &str) -> Module {
  let file = anonymous_file(source);

  match parser_for(&file).parse_module() {
    Ok(module) => module,
    Err(error) => panic!("failed to parse the module: {:?}", error),
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

/// Asserts the refusal says what it says — the sentence an author reads, not
/// merely that there was one.
///
/// A refusal with no reason is already caught by [`assert_deopts`]; what this
/// catches is a refusal whose reason names the wrong rule, which reads to an
/// author exactly like the right one and is why every rule the fold applies
/// pins its own words here.
#[track_caller]
pub(crate) fn assert_deopt_reason_contains(source: &str, expected: &str) {
  assert_deopt_reason(*evaluate_source(source), source, expected);
}

/// The same, with the depth ceiling raised for a source whose subject is depth.
#[track_caller]
pub(crate) fn assert_deopt_reason_contains_with_ceiling(
  source: &str,
  expected: &str,
  max_evaluation_depth: usize,
) {
  assert_deopt_reason(
    *evaluate_source_with_ceiling(source, max_evaluation_depth),
    source,
    expected,
  );
}

#[track_caller]
fn assert_deopt_reason(result: EvaluateResult, source: &str, expected: &str) {
  assert_deopt_result(&result, source);

  match result.reason {
    Some(reason) => assert!(
      reason.contains(expected),
      "expected the refusal of `{}` to say `{}`, got {:?}",
      source,
      expected,
      reason
    ),
    None => panic!("expected `{}` to record a deopt reason", source),
  }
}

/// Asserts the source folds to an object, and to one carrying exactly these own
/// keys in this order.
///
/// The keys rather than the whole object because order is the half a test can
/// get wrong by accident: two objects with the same properties in different
/// orders are different values to the language, and only one of them hashes the
/// class name the reference implementation hashes.
#[track_caller]
pub(crate) fn assert_folds_to_object_keys(source: &str, expected: &[&str]) {
  match assert_folds(source) {
    Expr::Object(object) => {
      let keys: Vec<String> = object
        .props
        .iter()
        .map(|prop| match prop {
          PropOrSpread::Prop(prop) => match prop.as_ref() {
            Prop::KeyValue(key_value) => convert_key_value_to_str(key_value),
            other => panic!(
              "expected `{}` to fold to key-value props, got {:?}",
              source, other
            ),
          },
          PropOrSpread::Spread(_) => {
            panic!("expected `{}` to fold to an object with no spread", source)
          },
        })
        .collect();

      assert_eq!(keys, expected, "wrong own-key order for `{}`", source);
    },
    other => panic!(
      "expected `{}` to fold to an object, got {:?}",
      source, other
    ),
  }
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

/// The same, for a source whose value is not an expression — an array folds to
/// the evaluator's own list rather than to an array literal, and a case whose
/// subject is *that it folded* should not have to know which.
#[track_caller]
pub(crate) fn assert_folds_to_a_value(source: &str) -> EvaluateResultValue {
  assert_folds_to_a_value_result(*evaluate_source(source), source)
}

/// The same, with the depth ceiling raised for a source whose subject is depth.
#[track_caller]
pub(crate) fn assert_folds_to_a_value_with_ceiling(
  source: &str,
  max_evaluation_depth: usize,
) -> EvaluateResultValue {
  assert_folds_to_a_value_result(
    *evaluate_source_with_ceiling(source, max_evaluation_depth),
    source,
  )
}

#[track_caller]
fn assert_folds_to_a_value_result(result: EvaluateResult, source: &str) -> EvaluateResultValue {
  assert!(
    result.confident,
    "expected `{}` to fold, got a deopt: {:?}",
    source, result.reason
  );

  match result.value {
    Some(value) => value,
    None => panic!("expected `{}` to fold to a value, got none", source),
  }
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

/// The folded string, for a caller comparing a fold against something other
/// than a literal it wrote out — the differential pass beside the coercions.
#[track_caller]
pub(crate) fn folded_string(source: &str) -> String {
  match assert_folds(source) {
    Expr::Lit(Lit::Str(strng)) => convert_atom_to_string(&strng.value),
    other => panic!("expected `{}` to fold to a string, got {:?}", source, other),
  }
}

/// The folded number, likewise. `NaN` is among the values it answers, because
/// `NaN` is a value here rather than a refusal.
#[track_caller]
pub(crate) fn folded_number(source: &str) -> f64 {
  match assert_folds(source) {
    Expr::Lit(Lit::Num(number)) => number.value,
    other => panic!("expected `{}` to fold to a number, got {:?}", source, other),
  }
}

/// Asserts the source folds to `null`.
///
/// Its own assertion because `null` carries no value to compare — the variant
/// is the whole of it, and the neighbouring `undefined` has no literal at all
/// and refuses instead.
#[track_caller]
pub(crate) fn assert_folds_to_null(source: &str) {
  match assert_folds(source) {
    Expr::Lit(Lit::Null(_)) => {},
    other => panic!("expected `{}` to fold to null, got {:?}", source, other),
  }
}

/// Asserts the source folds to an array holding exactly `expected`, in order.
///
/// A folded array is the evaluator's own list rather than an array literal —
/// the same shape an array the author wrote evaluates to — so it is read
/// through [`assert_folds_to_a_value`]. Every other array case asserts a
/// `length` or a `join` the engine had already applied, so none of them would
/// notice elements arriving in the wrong order, or a conversion that dropped
/// one.
#[track_caller]
pub(crate) fn assert_folds_to_strings(source: &str, expected: &[&str]) {
  match assert_folds_to_a_value(source) {
    EvaluateResultValue::Vec(items) => {
      let folded = items
        .iter()
        .map(|item| match item {
          EvaluateResultValue::Expr(Expr::Lit(Lit::Str(strng))) => {
            convert_atom_to_string(&strng.value)
          },
          other => panic!("expected `{}` to hold strings, got {:?}", source, other),
        })
        .collect::<Vec<String>>();

      assert_eq!(folded, expected, "wrong folded array for `{}`", source);
    },
    other => panic!("expected `{}` to fold to an array, got {:?}", source, other),
  }
}

/// Asserts the source folds to a boolean.
///
/// A predicate method — `startsWith`, `includes`, `hasOwnProperty` — folds to
/// one, so a test can say which boolean rather than routing it through a
/// conditional and asserting the branch it picked.
#[track_caller]
pub(crate) fn assert_folds_to_boolean(source: &str, expected: bool) {
  match assert_folds(source) {
    Expr::Lit(Lit::Bool(truth)) => assert_eq!(
      truth.value, expected,
      "wrong folded boolean for `{}`",
      source
    ),
    other => panic!(
      "expected `{}` to fold to a boolean, got {:?}",
      source, other
    ),
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

/// Asserts the source folds to `NaN`.
///
/// Its own assertion because `NaN != NaN`, so the equality
/// [`assert_folds_to_number`] makes can never hold for it.
#[track_caller]
pub(crate) fn assert_folds_to_nan(source: &str) {
  match assert_folds(source) {
    Expr::Lit(Lit::Num(num)) => {
      assert!(
        num.value.is_nan(),
        "expected `{}` to fold to NaN, got {}",
        source,
        num.value
      )
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

/// The string a folded value holds, for a case reading a value it evaluated
/// itself rather than one an assertion helper folded for it.
#[track_caller]
pub(crate) fn folded_text(value: &EvaluateResultValue) -> String {
  match value {
    EvaluateResultValue::Expr(Expr::Lit(Lit::Str(text))) => convert_atom_to_string(&text.value),
    other => panic!("expected a folded string, got {other:?}"),
  }
}

/// `const <name> = <init>`, as the module-wide collector would have recorded it.
fn declarator_of(name: &str, init: Expr) -> VarDeclarator {
  let id = Ident {
    span: DUMMY_SP,
    sym: name.into(),
    optional: false,
    ctxt: SyntaxContext::empty(),
  };

  VarDeclarator {
    span: DUMMY_SP,
    name: Pat::Ident(BindingIdent { id, type_ann: None }),
    init: Some(Box::new(init)),
    definite: false,
  }
}

/// Folds `source` against a module holding one declaration, which is the only
/// way to reach a printed parameter: an expression that resolves no name is
/// printed with none.
pub(crate) fn folded_in_a_module_binding(name: &str, init: &str, source: &str) -> String {
  let globals = Globals::new();

  GLOBALS.set(&globals, || {
    let mut traversal_state = StateManager::new(StyleXOptions::default());

    traversal_state.push_declaration(declarator_of(name, parse_expr(init)));

    let result = evaluate(
      &parse_expr(source),
      &mut traversal_state,
      &FunctionMap::default(),
    );

    assert!(
      result.confident,
      "`{source}` refused with `{init}` bound to `{name}`"
    );

    match result.value.as_ref() {
      Some(value) => folded_text(value),
      None => panic!("`{source}` answered no value with `{init}` bound to `{name}`"),
    }
  })
}

/// A thread small enough that a case has to be given room it did not start with.
///
/// Under a megabyte the runtime's own guard-page arithmetic starts to matter, so
/// this is the smallest honest floor rather than the smallest number that fits.
pub(crate) const SMALL_THREAD: usize = 1024 * 1024;

/// An array literal nested `levels` deep around a string.
///
/// The shape every case about how deep the printer and the parser go is written
/// in, so a case can say which depth it is about without also saying how a
/// bracket is spelled.
pub(crate) fn nested_literal(levels: usize) -> String {
  "[".repeat(levels) + "'x'" + &"]".repeat(levels)
}

/// A thread large enough for the stages either side of the fold — SWC's parse of
/// the source and the drop of the tree it answered — to run on input nested as
/// deeply as the compiler will carry.
///
/// For a case whose subject is what the fold does with such input rather than
/// what those stages cost: both recurse on the bare thread stack and neither is
/// the fold's, so a case measuring the fold has to be given room for them.
pub(crate) const LARGE_THREAD: usize = 256 * 1024 * 1024;

/// Runs `case` on a thread of `stack` bytes and hands back what it answered.
///
/// For a case whose subject is how much stack something needs. A test thread's
/// own size is not something a case can state, and the failure it is measuring
/// is an abort rather than an assertion, so the size has to be written down
/// beside the case that depends on it.
///
/// A panic inside `case` is resumed here rather than swallowed, so an assertion
/// that failed on the thread reads as a failure of the test that started it.
pub(crate) fn on_a_thread_of<R: Send + 'static>(
  stack: usize,
  case: impl FnOnce() -> R + Send + 'static,
) -> R {
  let started = std::thread::Builder::new().stack_size(stack).spawn(case);

  match started {
    Ok(thread) => match thread.join() {
      Ok(answer) => answer,
      Err(panic) => std::panic::resume_unwind(panic),
    },
    Err(error) => panic!("could not start the thread the case runs on: {}", error),
  }
}
