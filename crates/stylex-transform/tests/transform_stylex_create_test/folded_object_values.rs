//! A fold whose result is an object, used as a style value.
//!
//! The fold used to hand back a syntax node and to answer nothing at all when
//! the result was a plain object, so an expression that computed a nested value
//! object deopted even though the engine had produced it correctly. It answers
//! the evaluator's own value type now, which is what lets a folded object reach
//! every place an object the author wrote reaches — including the condition
//! positions a style value is mostly made of.
//!
//! Every class name and rule text below is measured output of
//! `@stylexjs/babel-plugin` 0.19.0 under the same options.

use crate::utils::{
  prelude::*,
  transform::{base_style_module, fold_module as fold},
};

fn module(body: &str) -> String {
  base_style_module("", body)
}

/// The shape the divergence was reported as: a value object computed by a fold
/// rather than written out, reaching the same two rules a written one reaches.
#[test]
fn a_folded_object_is_a_condition_object() {
  let output = fold(&module(
    "color: ['red'].reduce((o, v) => ({ default: v, ':hover': 'blue' }), {}),",
  ));

  assert!(output.contains(".x1e2nbdu{color:red}"));
  assert!(output.contains(".x17z2mba:hover{color:blue}"));
}

/// Nesting inside the folded object too, so what crosses the bridge is the
/// whole value and not a flattened one.
#[test]
fn a_folded_object_carries_its_nested_conditions() {
  let output = fold(&module(
    "color: [1].reduce(() => ({ default: 'red', '@media (min-width: 100px)': { default: 'blue' } }), {}),",
  ));

  assert!(output.contains(".x1e2nbdu{color:red}"));
  assert!(output.contains("@media (min-width: 100px){.x18tmubq.x18tmubq{color:blue}}"));
}

/// An array a fold produced is the value an array the author wrote is, so a
/// folded fallback list expands into one declaration per entry.
#[test]
fn a_folded_array_is_a_fallback_list() {
  let output = fold(&module("fontFamily: 'Inter,serif'.split(','),"));

  assert!(output.contains(".x1jd9j1i{font-family:Inter;font-family:serif}"));
}

/// A folded object reaching a position that validates it is refused by that
/// position, not by the fold — which is the whole point of letting it across.
/// The reference compiler refuses the same input, in its own words.
#[test]
#[should_panic(expected = "Invalid pseudo or at-rule")]
fn a_folded_object_whose_keys_are_not_conditions_is_refused_where_it_lands() {
  fold(&module("content: ({ a: 1 }).valueOf(),"));
}
