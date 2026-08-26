//! What an author reads when a fold refuses.
//!
//! A refusal used to be silent: the fold answered nothing and the surrounding
//! evaluation reported `Unsupported expression: CallExpression`, which names the
//! syntax rather than the reason. Each rule that refuses knows exactly why it
//! fired, so the information existed and was discarded.
//!
//! Pinned here, at the highest seam there is, because the sentence is the whole
//! of what a refused build hands anybody — and because the message text is this
//! compiler's own. The comparison harness compares class name, rule text and
//! style-object shape, never message text, and it already has a verdict for two
//! compilers rejecting one input in different words. Where this compiler's
//! sentence is better than the reference compiler's, it stays better; the
//! reference compiler answers `Unsupported expression: CallExpression` for the
//! unknown method below, and `not a function` for the callback.
//!
//! Every case also carries the key path the author needs to find the value:
//! `base > content > …`. A rule that fired without one would be a sentence with
//! nowhere to apply it.

use crate::utils::{
  prelude::*,
  transform::{base_style_module, fold_module as fold},
};

fn module(body: &str) -> String {
  base_style_module("", body)
}

/// The engine carries no locale data, so it would answer from the root locale —
/// `"i".toLocaleUpperCase("tr")` is `I` here and `İ` in the language. The
/// refusal names the method, which is the half that says what to write instead.
#[test]
#[should_panic(expected = "base > content > Cannot fold 'toLocaleUpperCase' at compile time.")]
fn a_locale_sensitive_method_names_itself() {
  fold(&module("content: 'i'.toLocaleUpperCase('tr'),"));
}

/// The reference compiler throws on a method call against a number literal, so
/// both compilers reject this input. Which one, and why, is what the sentence
/// adds.
#[test]
#[should_panic(expected = "base > content > Cannot call 'toFixed' on a number literal.")]
fn a_numeric_literal_receiver_names_the_rule_rather_than_the_node() {
  fold(&module("content: (1.5).toFixed(1),"));
}

/// The engine bounds loops, recursion and stack, but not allocation, so a
/// length it cannot read is a length it will not build. The message states the
/// limit, so an author can see the number rather than guess at it.
#[test]
#[should_panic(expected = "Cannot bound the string 'repeat' would build.")]
fn an_unbounded_amplifying_length_names_the_limit() {
  fold(&module("content: 'x'.repeat(200000000),"));
}

/// The same rule, reached the other way: a count that is not written as a
/// number cannot be bounded by reading it.
#[test]
#[should_panic(expected = "Cannot bound the string 'padStart' would build.")]
fn an_unreadable_amplifying_length_reaches_the_same_rule() {
  fold(&module("content: 'x'.padStart(2 * 2, '0'),"));
}

/// A throw is an answer rather than a fault of the fold — the language throws
/// on this too — so the engine's own sentence is what the author reads, under
/// this compiler's naming of the call that produced it.
#[test]
#[should_panic(expected = "base > content > Cannot fold 'reduce' at compile time.\nTypeError")]
fn a_call_the_engine_throws_on_carries_the_engine_s_own_message() {
  fold(&module("content: [].reduce((a, b) => a + b),"));
}

/// A method that does not exist reads a property that is `undefined` and then
/// calls it, so the language's own sentence is `not a callable function` and
/// names nothing. Naming the method beside it is why the fold carries it.
#[test]
#[should_panic(expected = "base > content > Cannot fold 'unsupported' at compile time.")]
fn a_method_that_does_not_exist_is_named_by_this_compiler_rather_than_the_engine() {
  fold(&module("content: 'abc'.unsupported(),"));
}

/// A value the bridge cannot carry says which kind it was. The kind is the half
/// that says why an otherwise valid call folded to nothing usable.
#[test]
#[should_panic(expected = "Cannot carry a folded undefined back from the engine.")]
fn a_result_with_no_literal_form_names_the_kind_it_was() {
  fold(&module("content: 'abc'.at(99),"));
}

/// A bounded string can still become one array element per code unit, which
/// costs far more as a tree than it did as text.
#[test]
#[should_panic(expected = "Array length is too large to evaluate at compile time.")]
fn an_array_result_past_the_bound_names_the_bound() {
  fold(&module("fontFamily: 'x'.repeat(10001).split(''),"));
}

/// An object costs one AST node per property, exactly as an array costs one per
/// element, so it is bounded by the same number.
///
/// The input has to be written out — ten thousand properties is past what any
/// loop the guard admits can build — so this is the one rule whose case is a
/// generated source string rather than a stylesheet anybody would write. It is
/// here anyway, with the other seven, because the sentence an author reads is
/// the thing under test and this is where that is asserted.
#[test]
#[should_panic(expected = "Object is too large to evaluate at compile time.")]
fn an_object_result_past_the_bound_names_the_bound() {
  let properties: Vec<String> = (0..10_001)
    .map(|index| format!("k{index}:{index}"))
    .collect();

  fold(&module(&format!(
    "content: ({{{}}}).valueOf(),",
    properties.join(",")
  )));
}

/// Nesting is bounded because the engine's parser recurses on the bare thread
/// stack, and an overflow inside an evaluation that is allowed to fail aborts
/// the build instead of reporting anything.
#[test]
#[should_panic(expected = "Expression is too deeply nested to evaluate at compile time.")]
fn nesting_past_the_bound_names_the_depth_rule() {
  fold(&module(&format!(
    "content: 'a'{},",
    ".concat('b')".repeat(400)
  )));
}

/// A read that walks off the value that was written names the property rather
/// than the call it sat in, because the property is the whole of the reason.
/// The reference compiler folds this one; the divergence is argued at
/// `ESCAPING_PROPERTIES` and costs the answer to a call no declaration uses.
#[test]
#[should_panic(expected = "base > content > Cannot fold a read of 'constructor' at compile time.")]
fn an_escaping_property_names_the_property_rather_than_the_call() {
  fold(&module(
    "content: ''.constructor.constructor('return 1').call(),",
  ));
}

/// A length written into the source bounds one evaluation, and a callback runs
/// once per element of a receiver nothing measured — so the sentence has to say
/// that the callback is the reason, not the argument, which is inside the
/// bound.
#[test]
#[should_panic(expected = "Cannot bound the string 'padStart' would build inside a callback.")]
fn an_amplifying_call_inside_a_callback_names_the_callback_as_the_reason() {
  fold(&module(
    "content: ['1'].map(x => x.padStart(2, '0')).join(''),",
  ));
}

/// Where the refusal lands is still decided by where the call sat, which this
/// work does not change: inside a dynamic style function the same refusal
/// leaves the call for the runtime rather than failing the build.
#[test]
fn the_same_refusal_inside_a_dynamic_style_leaves_the_value_to_the_runtime() {
  let output = fold(
    r#"
      import * as stylex from '@stylexjs/stylex';
      export const styles = stylex.create({
        base: (label) => ({ content: label.toLocaleUpperCase('tr') }),
      });
    "#,
  );

  assert!(output.contains("var(--"));
}
