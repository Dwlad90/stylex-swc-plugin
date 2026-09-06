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
use stylex_constants::constants::evaluation_errors::OBJECT_METHOD;
use stylex_constants::constants::messages::{ILLEGAL_PROP_ARRAY_VALUE, NULLISH_TO_OBJECT};

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

// ==================== a receiver reached past a declined fold ====================
//
// A receiver the engine can read is folded there, so a case about this walk has
// to put one behind something the engine declines. `sx.missing ?? <value>` is
// that shape: the fold declines the whole expression for the name it cannot
// carry, and the value behind the `??` is what the walk then reads.

/// The list `Object.<question>` answers for a value reached past a declined
/// fold, as its length.
#[track_caller]
fn counted_past_the_fold(question: &str, receiver: &str) -> f64 {
  counted(question, &format!("sx.missing ?? {receiver}"))
}

/// A string's own properties are its indices, and an array's are its own --
/// both read out of the evaluated value rather than out of the syntax, because
/// a value reached this way has no literal left to read.
#[test]
fn an_evaluated_string_or_array_is_read_from_the_value() {
  for question in QUESTIONS {
    assert_eq!(counted_past_the_fold(question, "'abc'"), 3.0);
    assert_eq!(counted_past_the_fold(question, "['a', 'b']"), 2.0);
    assert_eq!(counted_past_the_fold(question, "[['a'], ['b']]"), 2.0);
    assert_eq!(counted_past_the_fold(question, "[['a', null]]"), 1.0);
  }
}

/// `null` written as an element is a value rather than an absence, so it keeps
/// its own key -- unlike the hole above it, which occupies a slot and carries
/// none.
#[test]
fn a_written_null_element_keeps_its_key() {
  for question in QUESTIONS {
    assert_eq!(counted_past_the_fold(question, "[null, 'a']"), 2.0);
  }
}

/// A value with no own enumerable properties answers the empty list, and an
/// array holding an element the walk cannot write down answers it too --
/// because a receiver it cannot read whole is a receiver with nothing to read.
#[test]
fn a_receiver_the_walk_cannot_read_answers_the_empty_list() {
  for question in QUESTIONS {
    assert_eq!(counted_past_the_fold(question, "1"), 0.0);
    assert_eq!(counted_past_the_fold(question, "[own]"), 0.0);
    assert_eq!(counted_past_the_fold(question, "[[own]]"), 0.0);
  }
}

/// A property carried by a getter has no value the walk can read, so the whole
/// receiver refuses rather than answering a list with the property missing.
#[test]
fn a_property_that_is_not_a_value_refuses_the_receiver() {
  for question in QUESTIONS {
    assert_refuses(
      question,
      "sx.missing ?? ({ get a() { return 1; } })",
      OBJECT_METHOD,
    );
  }
}

// ==================== an array literal the fold will not print ====================

/// An array written where it is read is taken from the syntax, because a hole
/// has no value to evaluate. An element with no compile-time form makes the
/// whole array unreadable, and the refusal says what a style array may hold.
#[test]
fn an_array_literal_with_an_unreadable_element_refuses() {
  for question in QUESTIONS {
    assert_refuses(question, "[own, 'a']", ILLEGAL_PROP_ARRAY_VALUE);
    assert_refuses(question, "[[own]]", ILLEGAL_PROP_ARRAY_VALUE);
  }
}

/// A nested array of readable elements is written down as a nested array, which
/// is the one key the outer array carries.
#[test]
fn a_nested_array_literal_is_one_readable_key() {
  for question in QUESTIONS {
    assert_eq!(counted(question, "[['a']]"), 1.0);
  }
}
