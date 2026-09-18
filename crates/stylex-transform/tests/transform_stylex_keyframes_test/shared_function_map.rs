//! The function map the three nested-rule calls evaluate their argument with.
//!
//! `keyframes`, `positionTry` and `viewTransitionClass` each register the
//! helpers their argument may fold, and the map they register into is built
//! once for the module and shared between the calls that ask for the same set
//! of helpers. Two things must hold for that sharing to be sound, and both are
//! asserted here:
//!
//! - the map holds every local name the module's imports gave a helper, however
//!   many calls stand between the import and the call that reads it. A name the
//!   map does not hold does not drop one declaration; it stops the whole call
//!   folding.
//! - the two sets stay apart. `viewTransitionClass` registers `keyframes`
//!   because one of its steps may name a keyframes rule; the other two must
//!   refuse a `keyframes` call written inside them, whichever call the module
//!   wrote first.
//!
//! The two refusal cases below write a different step from the one the
//! `viewTransitionClass` above them writes. Two calls that spell the same
//! argument are one call to the evaluator's memo. An identical step would be
//! answered from the fold the first call left, and would say nothing about the
//! map the second call read.

use crate::utils::prelude::*;

// Two calls of the same kind, and the second folds its fallback list from the
// map the first built.
stylex_test!(
  two_keyframes_calls_fold_their_own_fallback_list,
  r#"
    import * as stylex from '@stylexjs/stylex';
    export const fadeIn = stylex.keyframes({
      from: { color: stylex.firstThatWorks('var(--a)', 'red') },
      to: { color: 'blue' },
    });
    export const fadeOut = stylex.keyframes({
      from: { color: 'blue' },
      to: { color: stylex.firstThatWorks('var(--b)', 'green') },
    });
  "#
);

// Two kinds of call that share one set of helpers, in one module.
stylex_test!(
  a_position_try_call_after_a_keyframes_call_folds_its_fallback_list,
  r#"
    import * as stylex from '@stylexjs/stylex';
    export const fadeIn = stylex.keyframes({
      from: { color: stylex.firstThatWorks('var(--a)', 'red') },
      to: { color: 'blue' },
    });
    export const anchor = stylex.positionTry({
      positionAnchor: '--anchor',
      top: stylex.firstThatWorks('var(--top)', '10px'),
    });
  "#
);

// Every local name an import gave a helper is registered, including a renamed
// named import read by a later call.
stylex_test!(
  a_renamed_first_that_works_folds_in_every_call,
  r#"
    import { keyframes, positionTry, firstThatWorks as fallback } from '@stylexjs/stylex';
    export const fadeIn = keyframes({
      from: { color: fallback('var(--a)', 'red') },
      to: { color: 'blue' },
    });
    export const anchor = positionTry({
      positionAnchor: '--anchor',
      top: fallback('var(--top)', '10px'),
    });
  "#
);

// The `viewTransitionClass` set folds a nested `keyframes`, and goes on doing
// so for a second call in the same module.
stylex_test!(
  two_view_transition_classes_each_fold_a_nested_keyframes,
  r#"
    import * as stylex from '@stylexjs/stylex';
    export const first = stylex.viewTransitionClass({
      old: { animationName: stylex.keyframes({ from: { opacity: 0 }, to: { opacity: 1 } }) },
    });
    export const second = stylex.viewTransitionClass({
      new: { animationName: stylex.keyframes({ from: { opacity: 1 }, to: { opacity: 0 } }) },
    });
  "#
);

// The two sets stay apart: a `keyframes` written inside a `keyframes` refuses
// although the module opened with the one call that does register `keyframes`.
stylex_test_panic!(
  a_keyframes_after_a_view_transition_class_refuses_a_nested_keyframes,
  "Only static values are allowed inside of a keyframes() call.",
  r#"
    import * as stylex from '@stylexjs/stylex';
    export const vt = stylex.viewTransitionClass({
      old: { animationName: stylex.keyframes({ from: { opacity: 0 }, to: { opacity: 1 } }) },
    });
    export const outer = stylex.keyframes({
      from: { animationName: stylex.keyframes({ from: { opacity: 0.2 }, to: { opacity: 0.8 } }) },
      to: { opacity: 1 },
    });
  "#
);

// The same, for the other call of the pair.
stylex_test_panic!(
  a_position_try_after_a_view_transition_class_refuses_a_nested_keyframes,
  "Only static values are allowed inside of a positionTry() call.",
  r#"
    import * as stylex from '@stylexjs/stylex';
    export const vt = stylex.viewTransitionClass({
      old: { animationName: stylex.keyframes({ from: { opacity: 0 }, to: { opacity: 1 } }) },
    });
    export const anchor = stylex.positionTry({
      positionAnchor: '--anchor',
      top: stylex.keyframes({ from: { opacity: 0.2 }, to: { opacity: 0.8 } }),
    });
  "#
);
