//! A method on a style object, and the sentence the author is handed for it.
//!
//! A method has no compile-time value, so both compilers refuse it. What this
//! module is about is *which* refusal, because the two arms that raise it held
//! each other's reason and neither one could say so.
//!
//! `deopt` writes a reason only while `confident` is still true. The spread arm
//! ran with `confident` already cleared by the nested evaluation, so the
//! constant it carried was never applied to anything; the method arm ran with
//! `confident` still true and read a reason out of the state that nothing had
//! written yet, so it always resolved to the bare fallback. Crossed, the pair
//! looked plausible from either side -- each arm named a real message -- and the
//! only way to see it was to ask what the author actually read.
//!
//! The reference compiler reports `Unsupported object method.` here
//! (`utils/evaluate-path.js:759-761`, 0.19.0). A getter is a different node and
//! always reached that message correctly, which is why it is pinned beside the
//! method: the bug was reachable through one of two neighbouring shapes.

use crate::utils::prelude::*;

fn stylex_transform(
  comments: TestComments,
  customize: impl FnOnce(TestBuilder) -> TestBuilder,
) -> impl Pass {
  build_test_transform(comments, customize)
}

stylex_test_panic!(
  a_method_on_a_style_object_names_the_method,
  "Unsupported object method",
  |tr| stylex_transform(tr.comments.clone(), |b| b),
  r#"
    import * as stylex from '@stylexjs/stylex';
    export const styles = stylex.create({
      a: { color() { return 'red'; } },
    });
  "#
);

stylex_test_panic!(
  a_getter_on_a_style_object_names_the_method_too,
  "Unsupported object method",
  |tr| stylex_transform(tr.comments.clone(), |b| b),
  r#"
    import * as stylex from '@stylexjs/stylex';
    export const styles = stylex.create({
      a: { get color() { return 'red'; } },
    });
  "#
);
