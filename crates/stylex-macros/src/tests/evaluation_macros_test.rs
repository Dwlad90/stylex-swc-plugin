//! Tests for the evaluation macros.
//!
//! The macros are type-agnostic — they only call the functions they are given
//! and return from the function they expand in — so the stubs below stand in
//! for the evaluator with plain types. That keeps the contract under test
//! where it belongs: what is injected is called, with which arguments, and
//! what the expansion does to the control flow around it.
//!
//! The two state kinds are separate types on purpose. A refusal records itself
//! on the evaluation state and a panic builds its code frame from the
//! traversal state, so a single stub for both would hide the one distinction
//! that tells the two constructs apart.

/// Stands in for the evaluation state a refusal is recorded on.
#[derive(Default)]
struct EvalState {
  reason: Option<String>,
  calls: usize,
}

/// Stands in for the state manager a conversion reads and a code frame is
/// built from.
#[derive(Default)]
struct TraversalState {
  reads: usize,
}

/// Stands in for the evaluator's `deopt`: records the refusal and answers with
/// no value, exactly as the real one does.
fn record_deopt(expr: &str, state: &mut EvalState, reason: &str) -> Option<String> {
  state.calls += 1;
  state.reason = Some(format!("{expr}: {reason}"));

  None
}

/// Stands in for `convert_expr_to_str`. Anything but `"opaque"` has a string
/// form. It counts its read so a test can prove the traversal state reached
/// it rather than the evaluation state.
fn convert(expr: &str, traversal_state: &mut TraversalState, _fns: &()) -> Option<String> {
  traversal_state.reads += 1;

  match expr {
    "opaque" => None,
    other => Some(other.to_uppercase()),
  }
}

// ==================== deopt_unsupported ====================

fn refuse(expr: &str, state: &mut EvalState) -> Option<u8> {
  deopt_unsupported!(record_deopt, expr, state, "no static value");
}

/// The refusal is recorded on the evaluation state and the enclosing function
/// answers with no value.
#[test]
fn deopt_unsupported_records_the_reason_and_returns_none() {
  let mut state = EvalState::default();

  assert_eq!(refuse("fn()", &mut state), None);
  assert_eq!(state.reason.as_deref(), Some("fn(): no static value"));
  assert_eq!(state.calls, 1);
}

fn refuse_before_the_tail(state: &mut EvalState) -> Option<u8> {
  for step in 0..3u8 {
    if step == 1 {
      deopt_unsupported!(record_deopt, "loop", state, "refused mid-loop");
    }
  }

  Some(42)
}

/// The expansion returns from the *function*, not from the loop, so nothing
/// after the refusal runs.
#[test]
fn deopt_unsupported_leaves_the_function_from_inside_a_loop() {
  let mut state = EvalState::default();

  assert_eq!(refuse_before_the_tail(&mut state), None);
  assert_eq!(state.calls, 1);
}

fn refuse_in_a_match_arm(expr: &str, state: &mut EvalState) -> Option<u8> {
  let value = match expr {
    "one" => 1,
    other => deopt_unsupported!(record_deopt, other, state, "unknown shape"),
  };

  Some(value)
}

/// The expansion stands where a value is expected, because it never produces
/// one.
#[test]
fn deopt_unsupported_holds_a_value_position() {
  let mut state = EvalState::default();

  assert_eq!(refuse_in_a_match_arm("one", &mut state), Some(1));
  assert_eq!(state.calls, 0);

  assert_eq!(refuse_in_a_match_arm("two", &mut state), None);
  assert_eq!(state.reason.as_deref(), Some("two: unknown shape"));
}

// ==================== expr_to_str_or_deopt ====================

fn to_string_or_refuse(
  expr: &str,
  state: &mut EvalState,
  traversal_state: &mut TraversalState,
) -> Option<String> {
  let converted = expr_to_str_or_deopt!(
    convert,
    record_deopt,
    expr,
    state,
    traversal_state,
    &(),
    "expression is not a string"
  );

  Some(converted)
}

/// A conversion that succeeds gives its string, reads the traversal state and
/// records nothing on the evaluation state.
#[test]
fn expr_to_str_or_deopt_gives_the_converted_string() {
  let mut state = EvalState::default();
  let mut traversal_state = TraversalState::default();

  let converted = to_string_or_refuse("red", &mut state, &mut traversal_state);

  assert_eq!(converted.as_deref(), Some("RED"));
  assert_eq!(traversal_state.reads, 1);
  assert_eq!(state.calls, 0);
}

/// A conversion that fails refuses the same way `deopt_unsupported!` does, on
/// the evaluation state rather than on the one it read.
#[test]
fn expr_to_str_or_deopt_refuses_when_the_conversion_fails() {
  let mut state = EvalState::default();
  let mut traversal_state = TraversalState::default();

  assert_eq!(
    to_string_or_refuse("opaque", &mut state, &mut traversal_state),
    None
  );
  assert_eq!(
    state.reason.as_deref(),
    Some("opaque: expression is not a string")
  );
  assert_eq!(traversal_state.reads, 1);
}

// ==================== stylex_panic_with_context ====================

/// Stands in for `wrap_in_paren_ref`.
fn wrap(expr: &str) -> String {
  format!("({expr})")
}

/// Stands in for the reporter, without the panic, so a test can read what the
/// macro handed it.
fn report(wrapped: &str, expr: &str, msg: &str, state: &mut TraversalState) -> String {
  state.reads += 1;

  format!("{wrapped}|{expr}|{msg}")
}

/// Both forms of the expression reach the reporter — the wrapped one for the
/// code frame, the original one for the position it points at — and the state
/// it reads is the traversal state, not the evaluation state.
#[test]
fn stylex_panic_with_context_reports_both_forms_of_the_expression() {
  let mut traversal_state = TraversalState::default();

  let reported = stylex_panic_with_context!(
    wrap,
    report,
    "x + y",
    &mut traversal_state,
    "broken invariant"
  );

  assert_eq!(reported, "(x + y)|x + y|broken invariant");
  assert_eq!(traversal_state.reads, 1);
}

/// Stands in for the reporter that really does end the process.
fn report_and_panic(wrapped: &str, _expr: &str, msg: &str, _state: &mut TraversalState) -> ! {
  panic!("[StyleX] {msg} at {wrapped}")
}

/// A reporter that never returns is accepted where a value is expected,
/// because the expansion never produces one.
#[test]
#[should_panic(expected = "[StyleX] broken invariant at (x + y)")]
fn stylex_panic_with_context_carries_a_diverging_reporter() {
  let mut traversal_state = TraversalState::default();

  let _unreachable: u8 = stylex_panic_with_context!(
    wrap,
    report_and_panic,
    "x + y",
    &mut traversal_state,
    "broken invariant"
  );
}
