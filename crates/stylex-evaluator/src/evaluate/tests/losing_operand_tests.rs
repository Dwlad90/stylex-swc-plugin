//! The losing operand of `||`, `&&` and `??` inside an engine fold.
//!
//! The evaluator runs both operands of these operators. So a compiler
//! function on the side that loses still files its rule. `file` stands for
//! such a function: it records a filed rule and answers its argument.

use super::evaluate;
use super::source_evaluation::*;
use stylex_constants::constants::evaluation_errors::unfoldable_function;
use stylex_state::{
  functions::{FunctionMap, FunctionType},
  state_manager::StateManager,
};
use stylex_structures::stylex_options::StyleXOptions;
use swc_core::common::{DUMMY_SP, GLOBALS, Globals, Span};
use swc_core::ecma::ast::Expr;
use swc_core::ecma::visit::{VisitMut, VisitMutWith};

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
/// refusal on the side it does not use. In a callback only the engine can tell
/// which side wins, so the refusal is a put-off refusal, and it is the answer
/// only where the right side wins.
#[test]
fn a_losing_operand_the_walk_refuses() {
  assert_eq!(
    folded_with_filings("['x' || file(unknown)].join()"),
    ("x".into(), 0)
  );
  assert_eq!(
    folded_with_filings("['x'].map((v) => v || file(unknown)).join()"),
    ("x".into(), 0)
  );

  let source = "[''].map((v) => v || file(unknown)).join()";
  let (result, filed) = evaluated_with_filings(&[], source);

  assert_refused(&result, source);
  assert_eq!(filed, 0);
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

/// A function that the transport cannot take. The walk refuses a call of it.
const UNCARRIED: (&str, &str) = ("f", "(a) => { return a; }");

/// Where the left operand decides, a put-off refusal has no effect. This is
/// so in a callback, and where the left operand calls a compiler function:
/// in both, only the engine can tell whether the left operand decides.
#[test]
fn a_put_off_refusal_has_no_effect_where_the_left_operand_decides() {
  let (name, init) = UNCARRIED;

  for (source, expected) in [
    (r#"["a"].map(() => true || f("x")).join("")"#, "true"),
    (r#"["", 0].map((v) => v && f("x")).join()"#, ",0"),
    (r#"["a"].map(() => "v" ?? f("x")).join("")"#, "v"),
    (r#"["a"].map(() => true || /x/).join("")"#, "true"),
    (
      r#"["a"].map(() => true || (false || f("x"))).join("")"#,
      "true",
    ),
    (
      r#"["a"].map(() => { return true || f("x"); }).join("")"#,
      "true",
    ),
  ] {
    assert_eq!(folded_in_a_module_binding(name, init, source), expected);
  }

  let (result, filed) = evaluated_with_filings(&[UNCARRIED], "[file('a') || f('x')].join()");

  assert_eq!(folded_text_of(result, "file || f"), "a");
  assert_eq!(filed, 1);
}

/// A declaration that the walk refuses is not a put-off refusal, because it
/// is printed as a parameter default, which runs before any operator decides.
/// The name that reads it holds the put-off refusal.
#[test]
fn a_refused_declaration_puts_off_the_name_that_reads_it() {
  assert_eq!(
    folded_in_a_module_binding(
      "g",
      "(__sxRead) => 'x'",
      r#"["a"].map(() => true || g()).join("")"#
    ),
    "true"
  );
}

/// A put-off refusal in the left operand of an operator that runs its losing
/// operand is the answer, but the right operand runs first, so its rule is
/// filed.
#[test]
fn a_put_off_refusal_on_the_left_still_runs_the_right_operand() {
  for source in [
    "['a'].map(() => true || (missing || file('a'))).join()",
    "[true || (missing || file('a'))].join()",
  ] {
    assert_eq!(folded_with_filings(source), ("true".into(), 1), "{source}");
  }
}

/// Where the left operand does not decide, the fold gives the put-off
/// refusal. A left operand of `??` that is falsy and not nullish decides
/// nothing, and the refusal of the right operand comes first.
#[test]
fn a_put_off_refusal_applies_where_the_left_operand_does_not_decide() {
  let (name, init) = UNCARRIED;

  for source in [
    r#"["a"].map(() => false || f("x")).join("")"#,
    r#"["a", ""].map((v) => v || f("x")).join("")"#,
    r#"["a"].map(() => 0 ?? f("x")).join("")"#,
  ] {
    assert_refused_in_a_module_binding(name, init, source, &unfoldable_function(name));
  }

  let (result, _) = evaluated_with_filings(&[UNCARRIED], "[file('') || f('x')].join()");

  assert_refused_with(&result, "file('') || f", &unfoldable_function(name));
}

/// A part that is not a candidate gives the call back where the left operand
/// does not decide, and the evaluator refuses it.
#[test]
fn a_put_off_part_that_is_not_a_candidate_gives_the_call_back() {
  assert_deopt_reason_contains(r#"["a"].map(() => false || /x/).join("")"#, "map");
}

/// The parts of a losing side in front of a put-off refusal run, so a rule
/// there is filed. A part after it does not run, as the evaluator stops at the
/// first part that it cannot evaluate.
#[test]
fn a_losing_side_runs_up_to_a_put_off_refusal() {
  for (source, expected, filings) in [
    ("[true || [file('a'), f('x')]].join()", "true", 1),
    (
      "['a'].map(() => true || [file('a'), f('x')]).join()",
      "true",
      1,
    ),
    (
      "['a'].map(() => true || [file('a'), missing]).join()",
      "true",
      1,
    ),
    (
      "['a'].map(() => true || [f('x'), file('a')]).join()",
      "true",
      0,
    ),
    (
      "['a', ''].map((v) => true || (v ? f('x') : file('a'))).join()",
      "true,true",
      1,
    ),
  ] {
    let (result, filed) = evaluated_with_filings(&[UNCARRIED], source);

    assert_eq!(folded_text_of(result, source), expected);
    assert_eq!(filed, filings, "{source}");
  }
}

/// A left operand of `??` that is falsy and not nullish decides nothing. A
/// throw of the right operand is then the refusal, in front of the reason of
/// the logical node.
#[test]
fn a_throw_of_the_right_operand_comes_before_a_falsy_left_operand_of_nullish() {
  assert_deopt_reason_contains(
    "['a'].map((v) => 0 ?? v.x.y).join()",
    "TypeError: cannot convert 'null' or 'undefined' to object",
  );
}

/// A throw of the engine that is not the position of a put-off refusal is a throw in the
/// engine's own words, also where the fold holds a put-off refusal.
#[test]
fn a_throw_in_a_fold_with_a_put_off_refusal_keeps_its_words() {
  let (name, init) = UNCARRIED;
  let source = r#"["a"].map(() => (true || f("x")) + "a".repeat(-1)).join("")"#;
  let reason = evaluated_in_a_module_binding(name, init, source).reason;

  assert!(
    reason
      .as_deref()
      .is_some_and(|reason| reason.contains("RangeError: String.prototype.repeat")),
    "{reason:?}"
  );
}

/// A refused part with no span has no stable name for the printer, so the
/// fold hands the call back to the dispatch below.
#[test]
fn a_refused_part_with_no_span_is_handed_back() {
  let source = r#"["a"].map(() => true || [1, missing]).join("")"#;

  let result = GLOBALS.set(&Globals::new(), || {
    let mut expr = parse_expr(source);

    expr.visit_mut_with(&mut NoSpans);

    evaluate(
      &expr,
      &mut StateManager::new(StyleXOptions::default()),
      &with_file(),
    )
  });

  assert_refused(&result, source);
}

/// Sets every span it visits to the dummy span.
struct NoSpans;

impl VisitMut for NoSpans {
  fn visit_mut_span(&mut self, span: &mut Span) {
    *span = DUMMY_SP;
  }
}
