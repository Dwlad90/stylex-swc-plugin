//! How a case asks the engine what a value it built behaves like.
//!
//! A value this module hands the engine is asked the way the printed source asks
//! it — by running JavaScript against it — rather than through an accessor of the
//! compiler's own. Two suites do that, over values built by different halves of
//! the bridge, so the reading itself is written once here.
//!
//! The two refusal assertions below are here for the same reason. They read a
//! Rust answer rather than an engine value, but both suites make them, and one
//! wording for a failed case is worth more than two that drift apart.

use boa_engine::{Context, JsValue, Source};

use super::Decline;

/// What `read` — an arrow over `values` — answers, as a string.
///
/// Every step that can fail says which of them failed, because a case that read
/// nothing and a case that read the wrong thing are different mistakes.
#[track_caller]
pub(super) fn answered_by(context: &mut Context, read: &str, values: &[JsValue]) -> String {
  let compiled = match context.eval(Source::from_bytes(read)) {
    Ok(compiled) => compiled,
    Err(error) => panic!("`{}` did not compile: {}", read, error),
  };

  let Some(reader) = compiled.as_callable() else {
    panic!("`{}` is not a function", read);
  };

  let answered = match reader.call(&JsValue::undefined(), values, context) {
    Ok(answered) => answered,
    Err(error) => panic!("`{}` threw: {}", read, error),
  };

  match answered.to_string(context) {
    Ok(text) => text.to_std_string_escaped(),
    Err(error) => panic!("`{}` answered something unreadable: {}", read, error),
  }
}

/// Asserts `answer` is the rule `expected`, in the words an author reads.
///
/// Over any answer, because the halves of the bridge refuse the same way while
/// answering different things — a crossing answers nothing and a build answers
/// a value. What the case is about is the sentence either way.
#[track_caller]
pub(super) fn assert_refused_by_rule<T>(answer: Result<T, Decline>, case: &str, expected: &str) {
  assert_refusal(answer, case, |reason| {
    assert_eq!(reason, expected, "the refusal for `{}`", case)
  });
}

/// Asserts `answer` is a rule whose words hold `fragment`.
///
/// For the refusals that carry a sentence the engine wrote rather than one this
/// compiler did. The whole sentence is the engine's to change, so a case names
/// the part of it that says which failure it is.
#[track_caller]
pub(super) fn assert_refused_saying<T>(answer: Result<T, Decline>, case: &str, fragment: &str) {
  assert_refusal(answer, case, |reason| {
    assert!(
      reason.contains(fragment),
      "expected the refusal for `{}` to say `{}`, and it said `{}`",
      case,
      fragment,
      reason
    )
  });
}

/// The two arms neither assertion above is about, so that a case which did not
/// refuse says so the same way whichever assertion read it.
#[track_caller]
fn assert_refusal<T>(answer: Result<T, Decline>, case: &str, reads: impl FnOnce(&str)) {
  match answer {
    Err(Decline::Rule(reason)) => reads(&reason),
    Err(Decline::NotACandidate) => panic!("expected `{}` to refuse, and it was handed back", case),
    Ok(_) => panic!("expected `{}` to refuse, and it answered", case),
  }
}
