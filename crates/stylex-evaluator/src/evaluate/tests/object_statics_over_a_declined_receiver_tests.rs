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
//!
//! **Where this walk reads more than the reference implementation does.** An
//! array literal it reads from the syntax is read past anything it cannot fold
//! -- a hole, an element that refused, an element that resolved to nothing --
//! where the reference implementation refuses the whole declaration. Measured
//! against it, one case at a time, and recorded as ticket 48 of
//! `.scratch/split-transform-crate`. Each case below that pins such an answer
//! says so.

use super::source_evaluation::*;
use crate::evaluate_result::EvaluateResult;
use stylex_constants::constants::evaluation_errors::OBJECT_METHOD;
use stylex_constants::constants::messages::{ILLEGAL_PROP_ARRAY_VALUE, NULLISH_TO_OBJECT};

/// The three spellings of the same question, so every case asks all three and a
/// walk that answered one of them differently is visible.
const QUESTIONS: [&str; 3] = ["keys", "values", "entries"];

/// The list `Object.<question>(<receiver>)` answers, as its length.
#[track_caller]
fn counted(question: &str, receiver: &str) -> f64 {
  let source = format!("Object.{question}({receiver}).length");

  number_of(evaluated_against_a_function_fold(&source), &source)
}

/// What `source` folds to over a state whose memo already answers nothing for
/// `(() => 1) + 1`.
///
/// A receiver holding a value that resolved to nothing needs two doors open at
/// once. The memo is the one thing that answers nothing while the walk stays
/// confident, and the function fold is what makes the engine decline the call.
fn folded_after_the_memo(source: &str) -> Box<EvaluateResult> {
  evaluated_after_against(UNRESOLVED_MEMO_WARM, &a_function_fold(), source)
}

/// The same as [`counted`], over that state.
#[track_caller]
fn counted_after_the_memo(question: &str, receiver: &str) -> f64 {
  number_after_the_memo(&format!("Object.{question}({receiver}).length"))
}

/// The number `source` folds to over that state, for a case reading into the
/// answer rather than counting it.
#[track_caller]
fn number_after_the_memo(source: &str) -> f64 {
  number_of(folded_after_the_memo(source), source)
}

/// The first key `Object.keys(<receiver>)` answers over that state.
///
/// A count alone would pass on a walk that kept the wrong key, because dropping
/// one element of two leaves one either way.
#[track_caller]
fn first_key_after_the_memo(receiver: &str) -> String {
  let source = format!("Object.keys({receiver})[0]");

  folded_text_of(folded_after_the_memo(&source), &source)
}

/// The number `result` folded to.
#[track_caller]
fn number_of(result: Box<EvaluateResult>, source: &str) -> f64 {
  match folded_value_of(result, source)
    .as_expr()
    .and_then(|expr| expr.as_lit())
  {
    Some(swc_core::ecma::ast::Lit::Num(number)) => number.value,
    other => panic!("expected `{}` to count, got {:?}", source, other),
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
///
/// The reference implementation folds no array holding a hole at all, so it
/// refuses these three. See the module doc.
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
    assert_eq!(counted_past_the_fold(question, "[[[own]]]"), 0.0);
  }
}

/// A property carried by a getter has no value the walk can read, so the whole
/// receiver refuses rather than answering a list with the property missing.
///
/// The sentence comes from the object walk rather than from this one: an object
/// literal is evaluated before the receiver is read, and a getter is refused
/// there. So an [evaluator-written
/// object](../../../CONTEXT.md#evaluator-written-object) is all this reader can
/// be handed, and every property of one is a key and a value.
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

/// A callback has no object form of its own, so the receiver reads as one with
/// no own keys -- the same answer a number gets, and for the same reason. An
/// arrow is the one such value an author can write down, and the empty list is
/// what the reference implementation answers for one.
#[test]
fn a_callback_receiver_answers_the_empty_list() {
  for question in QUESTIONS {
    assert_eq!(counted_past_the_fold(question, "(() => 1)"), 0.0);
  }
}

/// A three-level array is written down whole, which is the one shape that reads
/// a nested array out of another nested array. The count and the value at the
/// bottom are both the reference implementation's answers for the same array.
#[test]
fn a_three_level_array_is_one_key_holding_its_own_levels() {
  for question in QUESTIONS {
    assert_eq!(counted_past_the_fold(question, "[[['a']]]"), 1.0);
  }

  // The levels rather than the count, because a walk that flattened them would
  // answer the same one key.
  let source = "Object.values(sx.missing ?? [[['a']]])[0][0][0]";

  assert_eq!(
    folded_text_of(evaluated_against_a_function_fold(source), source),
    "a"
  );
}

// ==================== a value that resolved to nothing ====================
//
// The third value class a receiver can hold, beside a value and an absence: an
// element the walk read confidently and found nothing for. Only the memo
// answers that way, and only a declined call reaches this walk -- so every case
// below needs both doors open at once.

/// An element that refused to fold carries no key. A regular expression is such
/// an element: the dispatch folds none in any position, and the array around it
/// is read from the syntax because the fold will not print it.
///
/// The reference implementation refuses the declaration for the regular
/// expression instead. See the module doc.
#[test]
fn an_element_that_refused_to_fold_carries_no_key() {
  for question in QUESTIONS {
    assert_eq!(counted(question, "[/re/]"), 0.0);
    assert_eq!(counted(question, "[/re/, 'a']"), 1.0);
  }
}

/// A nested array beside a hole keeps a key of its own: the array around it is
/// read from the syntax, and the nested one from the value it folded to.
#[test]
fn a_nested_array_beside_a_hole_keeps_its_key() {
  for question in QUESTIONS {
    assert_eq!(counted(question, "[, ['a']]"), 1.0);
  }
}

/// An element the walk resolved to nothing carries no key, exactly as a hole
/// does -- both are a slot with no value to name.
#[test]
fn an_element_that_resolved_to_nothing_carries_no_key() {
  for question in QUESTIONS {
    assert_eq!(counted_after_the_memo(question, "[, (() => 1) + 1]"), 0.0);
    assert_eq!(
      counted_after_the_memo(question, "[, (() => 1) + 1, 'a']"),
      1.0
    );
  }

  assert_eq!(first_key_after_the_memo("[, (() => 1) + 1, 'a']"), "2");
}

/// An index read answers the absent value itself, rather than answering no
/// value: it hands back the slot it found, and the array it read holds the
/// absent value where an element folded to nothing. So the element is read as
/// absent one step later than the case above reads it, and carries no key
/// either.
#[test]
fn an_element_read_out_of_an_array_as_nothing_carries_no_key() {
  for question in QUESTIONS {
    assert_eq!(
      counted_after_the_memo(question, "[, [(() => 1) + 1][0]]"),
      0.0
    );
    assert_eq!(
      counted_after_the_memo(question, "[, [(() => 1) + 1][0], 'a']"),
      1.0
    );
  }

  assert_eq!(first_key_after_the_memo("[, [(() => 1) + 1][0], 'a']"), "2");
}

/// The same, read out of the evaluated value rather than out of the syntax.
#[test]
fn an_evaluated_element_that_resolved_to_nothing_carries_no_key() {
  for question in QUESTIONS {
    assert_eq!(
      counted_after_the_memo(question, "sx.missing ?? [(() => 1) + 1]"),
      0.0
    );
    assert_eq!(
      counted_after_the_memo(question, "sx.missing ?? [(() => 1) + 1, 'a']"),
      1.0
    );
  }

  assert_eq!(
    first_key_after_the_memo("sx.missing ?? [(() => 1) + 1, 'a']"),
    "1"
  );
}

/// A nested array holding one carries no key either, because the array written
/// for it would be shorter than the source describes.
#[test]
fn a_nested_array_holding_nothing_resolved_carries_no_key() {
  for question in QUESTIONS {
    assert_eq!(
      counted_after_the_memo(question, "sx.missing ?? [[(() => 1) + 1], 'a']"),
      1.0
    );
  }

  assert_eq!(
    first_key_after_the_memo("sx.missing ?? [[(() => 1) + 1], 'a']"),
    "1"
  );
}

/// One level deeper the element does keep its key, and the value that resolved
/// to nothing is dropped out of the array under it -- so the array written is
/// shorter than the source describes. Recorded rather than defended: the two
/// levels answer differently for the same value. See the module doc.
#[test]
fn a_value_that_resolved_to_nothing_deeper_down_is_dropped_from_its_array() {
  for question in QUESTIONS {
    assert_eq!(
      counted_after_the_memo(question, "sx.missing ?? [[[(() => 1) + 1]], 'a']"),
      2.0
    );
  }

  assert_eq!(
    number_after_the_memo("Object.values(sx.missing ?? [[[(() => 1) + 1]], 'a'])[0][0].length"),
    0.0
  );
}
