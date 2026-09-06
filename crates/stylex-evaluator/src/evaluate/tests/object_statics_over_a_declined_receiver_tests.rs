//! `Object.keys`, `Object.values` and `Object.entries` over a receiver the
//! engine never sees.
//!
//! The three statics fold in the engine on every ordinary receiver, so a call
//! that reaches this compiler's own walk is one whose *receiver* the engine
//! declined. Two of those exist: the [folded function
//! map](../../../../../CONTEXT.md#folded-function-map), which is not a
//! JavaScript value at all, and an array with a hole in it, which the fold will
//! not print.
//!
//! Both are objects with own keys upstream, so answering the empty list would
//! be CSS the source does not describe -- which is what this walk exists to
//! stop. The receivers with no `ToObject` at all still have to refuse, and with
//! the sentence the reference implementation gives.

use super::source_evaluation::*;
use stylex_constants::constants::messages::NULLISH_TO_OBJECT;

/// The three spellings of the same question, so every case asks all three and a
/// walk that answered one of them differently is visible.
const QUESTIONS: [&str; 3] = ["keys", "values", "entries"];

/// The list `Object.<question>(<receiver>)` answers, as its length.
#[track_caller]
fn counted(question: &str, receiver: &str) -> f64 {
  let source = format!("Object.{question}({receiver}).length");
  let result = evaluated_against_a_function_fold(&source);

  assert!(
    result.confident,
    "expected `{}` to fold, got a deopt: {:?}",
    source, result.reason
  );

  match result.value {
    Some(value) => match value.as_expr().and_then(|expr| expr.as_lit()) {
      Some(swc_core::ecma::ast::Lit::Num(number)) => number.value,
      other => panic!("expected `{}` to count, got {:?}", source, other),
    },
    None => panic!("expected `{}` to answer a value", source),
  }
}

#[track_caller]
fn assert_refuses(question: &str, receiver: &str, reason: &str) {
  let source = format!("Object.{question}({receiver})");
  let result = evaluated_against_a_function_fold(&source);

  assert!(
    !result.confident,
    "expected `{}` to refuse, got {:?}",
    source, result.value
  );
  assert_eq!(
    result.reason.as_deref(),
    Some(reason),
    "wrong refusal for `{}`",
    source
  );
}

/// The namespace has the keys it was registered with, and one entry of it has
/// the one key the reference implementation wraps it under. Answering the empty
/// list here spread nothing and compiled a style object the author never wrote.
#[test]
fn a_function_fold_has_the_keys_it_was_registered_with() {
  for question in QUESTIONS {
    assert_eq!(counted(question, "sx"), 1.0);
    assert_eq!(counted(question, "own"), 1.0);
  }
}

/// The keys are the names, not their positions, so the namespace answers what
/// it holds rather than an index.
#[test]
fn the_keys_of_a_function_fold_are_its_names() {
  let result = evaluated_against_a_function_fold("Object.keys(sx)[0]");

  assert!(
    result.confident,
    "expected the keys to fold, got a deopt: {:?}",
    result.reason
  );

  match result.value {
    Some(value) => assert_eq!(folded_text(&value), "create"),
    None => panic!("expected the keys to answer a value"),
  }
}

/// A hole occupies a slot and carries no key of its own, so it is absent from
/// the answer -- exactly as `Object.keys([, 1])` omits index zero.
#[test]
fn a_hole_carries_no_key() {
  for question in QUESTIONS {
    assert_eq!(counted(question, "[, 'a']"), 1.0);
    assert_eq!(counted(question, "['a', , 'b']"), 2.0);
    assert_eq!(counted(question, "[, ,]"), 0.0);
  }
}

/// A string's own properties are its indices, one per character.
#[test]
fn a_string_has_one_key_per_character() {
  for question in QUESTIONS {
    assert_eq!(counted(question, "'abc'"), 3.0);
    assert_eq!(counted(question, "''"), 0.0);
  }
}

/// A value with no own enumerable properties answers the empty list rather than
/// refusing. A function is the one that reaches this walk: it is what an entry
/// of the fold stands for, and `Object.assign({}, () => 1)` is `{}` upstream.
#[test]
fn a_value_with_no_own_keys_answers_the_empty_list() {
  for question in QUESTIONS {
    assert_eq!(counted(question, "own.fn"), 0.0);
  }
}

/// `null` and `undefined` have no `ToObject` at all, so the statics throw in
/// the language rather than answering the empty list. Refused with the sentence
/// the reference implementation gives, because folding `[]` there is CSS the
/// source does not describe.
#[test]
fn a_nullish_receiver_refuses() {
  for question in QUESTIONS {
    assert_refuses(question, "sx.missing", NULLISH_TO_OBJECT);
  }
}
