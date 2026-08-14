//! A call to a JavaScript global folds inside `keyframes` too.
//!
//! An animation step is not a special case an author has to remember: the same
//! coercions that fold inside `create` fold in a step, in a step's key, and in
//! the declaration that names the resulting animation. The expected animation
//! names and rule text are measured output of `@stylexjs/babel-plugin@0.19.0`
//! for the same input.

use crate::utils::prelude::*;

fn stylex_transform(
  comments: TestComments,
  customize: impl FnOnce(TestBuilder) -> TestBuilder,
) -> impl Pass {
  build_test_transform(comments, customize)
}

// `@keyframes x2up61p-B{from{color:red;}to{color:blue;}}` — the same animation
// a step written with plain string literals produces.
stylex_test!(
  string_folds_in_a_keyframes_step,
  |tr| stylex_transform(tr.comments.clone(), |b| b.with_runtime_injection()),
  r#"
    import * as stylex from '@stylexjs/stylex';
    export const name = stylex.keyframes({
      from: { color: String('red') },
      to: { color: String('blue') },
    });
  "#
);

// A numeric string coerces in a step exactly as it does in a rule, so
// `Number('1e0')` is the `1` an author would otherwise have written.
stylex_test!(
  number_folds_in_a_keyframes_step,
  |tr| stylex_transform(tr.comments.clone(), |b| b.with_runtime_injection()),
  r#"
    import * as stylex from '@stylexjs/stylex';
    export const name = stylex.keyframes({
      from: { opacity: Number('0') },
      to: { opacity: Number('1e0') },
    });
  "#
);

// A step's own key folds too, so a coerced `from` names the same step the
// literal does and the animation hashes identically.
stylex_test!(
  a_coerced_keyframes_step_key_folds,
  |tr| stylex_transform(tr.comments.clone(), |b| b.with_runtime_injection()),
  r#"
    import * as stylex from '@stylexjs/stylex';
    export const name = stylex.keyframes({
      [String('from')]: { color: 'red' },
      to: { color: 'blue' },
    });
  "#
);

// The animation name a coerced step produces is the one the declaration
// referring to it records — `.xx2qnu0{animation-name:x2up61p-B}` — and the
// declaration around it coerces on its own terms.
stylex_test!(
  a_coerced_keyframes_animation_is_named_by_a_coerced_declaration,
  |tr| stylex_transform(tr.comments.clone(), |b| b.with_runtime_injection()),
  r#"
    import * as stylex from '@stylexjs/stylex';
    const name = stylex.keyframes({
      from: { color: String('red') },
      to: { color: String('blue') },
    });
    export const styles = stylex.create({
      root: { animationName: name, animationDuration: String('1s') },
    });
  "#
);

// A declared `String` is an ordinary function in a step as well, so it is
// called rather than folded.
stylex_test!(
  a_locally_declared_string_shadows_the_global_in_a_step,
  |tr| stylex_transform(tr.comments.clone(), |b| b.with_runtime_injection()),
  r#"
    import * as stylex from '@stylexjs/stylex';
    const String = () => 'shadowed';
    export const name = stylex.keyframes({
      from: { color: String('red') },
      to: { color: 'blue' },
    });
  "#
);
