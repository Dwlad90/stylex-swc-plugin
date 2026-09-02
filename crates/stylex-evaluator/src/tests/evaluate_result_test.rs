//! What a refused evaluation answers with.
//!
//! [`EvaluateResult::refused`] exists so that the four fields a refusal never
//! carries are spelled in one place. A site that spelled one of them
//! differently -- `confident: true`, or an `inline_styles` left over from a
//! fold that did not finish -- would be a refusal the caller above reads as a
//! value, so the constructor is asserted directly rather than only through the
//! folds that call it.

use crate::{evaluate_result::EvaluateResult, tests::scaffolding::parse_expr};

#[test]
fn a_refusal_carries_the_path_and_the_reason_and_nothing_else() {
  let path = parse_expr("makeStyles()");

  let refused = EvaluateResult::refused(Some(path.clone()), Some(String::from("because")));

  assert!(!refused.confident);
  assert_eq!(refused.deopt, Some(path));
  assert_eq!(refused.reason.as_deref(), Some("because"));
  assert_eq!(refused.value, None);
  assert_eq!(refused.inline_styles, None);
  assert_eq!(refused.fns, None);
}

/// Both arguments are optional, and a refusal with neither is still a refusal
/// -- which is what a caller reads to decide between a value and an inline
/// style.
#[test]
fn a_refusal_with_nothing_to_report_is_still_a_refusal() {
  let refused = EvaluateResult::refused(None, None);

  assert!(!refused.confident);
  assert_eq!(refused.deopt, None);
  assert_eq!(refused.reason, None);
}

/// The distinction the caller reads is `confident`, and it reads it because a
/// refusal and an answer are otherwise the same six fields. Asserted against a
/// confident result that differs in nothing else, so the inequality can only
/// come from `confident` itself.
#[test]
fn a_refusal_differs_from_a_confident_answer_in_nothing_but_confidence() {
  let refused = EvaluateResult::refused(None, None);
  let confident = EvaluateResult {
    confident: true,
    ..refused.clone()
  };

  assert_ne!(refused, confident);
}
