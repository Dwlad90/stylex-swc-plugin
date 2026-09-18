//! A nested rule call written the same way by two producers.
//!
//! `create({ x: { animationName: keyframes({ … }) } })` folds the call to its
//! name where it stands and files the rule on the state, and the producer that
//! holds the call carries it. Two producers that spell the same call each need
//! their own copy, because a name that reaches a stylesheet defining nothing is
//! a broken animation -- and because either producer may be dropped without the
//! other.
//!
//! The evaluator memo answers a second identical expression without folding it
//! again, which for a rule call would answer the name and file nothing. The
//! reference has no memo and folds every rule call, so these snapshots say the
//! rule stands in front of each producer that names it.

use crate::utils::prelude::*;

fn stylex_transform(
  comments: TestComments,
  customize: impl FnOnce(TestBuilder) -> TestBuilder,
) -> impl Pass {
  build_test_transform(comments, |b| {
    customize(
      b.with_runtime_injection_option(RuntimeInjection::Boolean(true))
        .with_runtime_injection(),
    )
  })
}

stylex_test!(
  two_creates_naming_the_same_nested_keyframes,
  |tr| stylex_transform(tr.comments.clone(), |b| b),
  r#"
    import * as stylex from '@stylexjs/stylex';
    export const a = stylex.create({
      x: { animationName: stylex.keyframes({ from: { opacity: 0 }, to: { opacity: 1 } }) },
    });
    export const b = stylex.create({
      y: { animationName: stylex.keyframes({ from: { opacity: 0 }, to: { opacity: 1 } }) },
    });
  "#
);

// The same call twice inside one producer, then once more in another. The
// second call of the first producer writes the key its first call wrote, so a
// count of the filed keys would miss it.
stylex_test!(
  one_create_naming_it_twice_then_another_once,
  |tr| stylex_transform(tr.comments.clone(), |b| b),
  r#"
    import * as stylex from '@stylexjs/stylex';
    export const a = stylex.create({
      x: { animationName: stylex.keyframes({ from: { opacity: 0 }, to: { opacity: 1 } }) },
      w: { animationName: stylex.keyframes({ from: { opacity: 0 }, to: { opacity: 1 } }) },
    });
    export const b = stylex.create({
      y: { animationName: stylex.keyframes({ from: { opacity: 0 }, to: { opacity: 1 } }) },
    });
  "#
);
