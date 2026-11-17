//! A rule name a StyleX call answers, used as the receiver or an argument of a
//! method call.
//!
//! The engine that folds a method call can call `keyframes` and `positionTry`,
//! with the build state of the fold, so each call writes its rule into the
//! build. A call inside a callback runs each time the callback runs, and a
//! callback that never runs writes no rule. Each part is evaluated whole, so a
//! conditional declares only the rule of the branch it takes.

use crate::utils::{prelude::*, transform::fold_module as fold};

fn stylex_transform(
  comments: TestComments,
  customize: impl FnOnce(TestBuilder) -> TestBuilder,
) -> impl Pass {
  build_test_transform(comments, customize)
}

// `.xsxtzgd{animation-name:xbjjx1r-B}`
stylex_test!(
  a_rule_name_in_an_array_joins,
  |tr| stylex_transform(tr.comments.clone(), |b| b.with_runtime_injection()),
  r#"
    import * as stylex from '@stylexjs/stylex';
    export const styles = stylex.create({
      root: {
        animationName: [stylex.keyframes({ from: { color: 'red' } })].join(', '),
      },
    });
  "#
);

// `.x13l6z9e{animation-name:xbjjx1r-B,xeuw8hg-B}`
stylex_test!(
  two_rule_names_in_an_array_join,
  |tr| stylex_transform(tr.comments.clone(), |b| b.with_runtime_injection()),
  r#"
    import * as stylex from '@stylexjs/stylex';
    export const styles = stylex.create({
      root: {
        animationName: [
          stylex.keyframes({ from: { color: 'red' } }),
          stylex.keyframes({ to: { color: 'blue' } }),
        ].join(', '),
      },
    });
  "#
);

// `.x1a8j8l9{animation-name:xbjjx1r-B,x}`
stylex_test!(
  a_rule_name_is_the_receiver_of_a_string_method,
  |tr| stylex_transform(tr.comments.clone(), |b| b.with_runtime_injection()),
  r#"
    import * as stylex from '@stylexjs/stylex';
    export const styles = stylex.create({
      root: {
        animationName: stylex.keyframes({ from: { color: 'red' } }).concat(', x'),
      },
    });
  "#
);

// `.xsxtzgd{animation-name:xbjjx1r-B}`. The callback is the engine's to run,
// and only the receiver it is called on is computed first.
stylex_test!(
  a_rule_name_passes_through_a_callback,
  |tr| stylex_transform(tr.comments.clone(), |b| b.with_runtime_injection()),
  r#"
    import * as stylex from '@stylexjs/stylex';
    export const styles = stylex.create({
      root: {
        animationName: [stylex.keyframes({ from: { color: 'red' } })]
          .map((name) => name)
          .join(','),
      },
    });
  "#
);

// `.xsxtzgd{animation-name:xbjjx1r-B}`, from a name the module binds.
stylex_test!(
  a_joined_rule_name_bound_to_a_constant,
  |tr| stylex_transform(tr.comments.clone(), |b| b.with_runtime_injection()),
  r#"
    import * as stylex from '@stylexjs/stylex';
    const fadeIn = [stylex.keyframes({ from: { color: 'red' } })].join(', ');
    export const styles = stylex.create({
      root: { animationName: fadeIn },
    });
  "#
);

// `.x1ll8nuh{position-try-fallbacks:--x17o73}`
stylex_test!(
  a_position_try_name_in_an_array_joins,
  |tr| stylex_transform(tr.comments.clone(), |b| b.with_runtime_injection()),
  r#"
    import * as stylex from '@stylexjs/stylex';
    export const styles = stylex.create({
      root: {
        positionTryFallbacks: [stylex.positionTry({ top: '0' })].join(', '),
      },
    });
  "#
);

// `.xsxtzgd{animation-name:xbjjx1r-B}`, and no rule for the branch not taken.
stylex_test!(
  a_conditional_in_the_receiver_declares_only_the_branch_it_takes,
  |tr| stylex_transform(tr.comments.clone(), |b| b.with_runtime_injection()),
  r#"
    import * as stylex from '@stylexjs/stylex';
    const c = true;
    export const styles = stylex.create({
      root: {
        animationName: [
          c
            ? stylex.keyframes({ from: { color: 'red' } })
            : stylex.keyframes({ to: { color: 'blue' } }),
        ].join(', '),
        transitionProperty: (c ? ['a'] : [stylex.keyframes({ to: { color: 'blue' } })]).join(),
      },
    });
  "#
);

// A branch that is not taken is not evaluated, so its refusal does not stop
// the fold.
stylex_test!(
  a_branch_not_taken_does_not_refuse,
  |tr| stylex_transform(tr.comments.clone(), |b| b.with_runtime_injection()),
  r#"
    import * as stylex from '@stylexjs/stylex';
    const c = true;
    export const styles = stylex.create({
      root: {
        animationName: [
          c
            ? stylex.keyframes({ from: { color: 'red' } })
            : stylex.keyframes({ from: { color: missing } }),
        ].join(', '),
      },
    });
  "#
);

// `.xobp4yc.xobp4yc:where(.x-default-marker:hover *){color:blue}`, from a
// call two members deep.
stylex_test!(
  a_when_selector_in_an_array_joins,
  |tr| stylex_transform(tr.comments.clone(), |b| b.with_runtime_injection()),
  r#"
    import * as stylex from '@stylexjs/stylex';
    export const styles = stylex.create({
      root: {
        color: {
          default: 'red',
          [[stylex.when.ancestor(':hover')].join('')]: 'blue',
        },
      },
    });
  "#
);

// `.xsxtzgd{animation-name:xbjjx1r-B}`, through a function the module wrote.
stylex_test!(
  a_rule_name_passes_through_the_module_s_own_function,
  |tr| stylex_transform(tr.comments.clone(), |b| b.with_runtime_injection()),
  r#"
    import * as stylex from '@stylexjs/stylex';
    const pick = (name) => name;
    export const styles = stylex.create({
      root: { animationName: pick(stylex.keyframes({ from: { color: 'red' } })) },
    });
  "#
);

/// A nested call that cannot be computed is the refusal, and it names the
/// value it could not read.
#[test]
#[should_panic(
  expected = "root > animationName > from > color > Referenced constant is not defined."
)]
fn a_nested_call_that_refuses_is_the_refusal() {
  fold(
    r#"
    import * as stylex from '@stylexjs/stylex';
    export const styles = stylex.create({
      root: {
        animationName: [stylex.keyframes({ from: { color: missing } })].join(', '),
      },
    });
  "#,
  );
}

// `@keyframes xbjjx1r-B{from{color:red;}}` and
// `.xsxtzgd{animation-name:xbjjx1r-B}`. The engine runs the callback, and the
// callback calls `keyframes`.
stylex_test!(
  a_keyframes_call_in_a_callback_declares_its_rule,
  |tr| stylex_transform(tr.comments.clone(), |b| b.with_runtime_injection()),
  r#"
    import * as stylex from '@stylexjs/stylex';
    export const s = stylex.create({
      root: {
        animationName: ['a'].map(() => stylex.keyframes({ from: { color: 'red' } })).join(),
      },
    });
  "#
);

// The same two rules. The frame reads the callback parameter.
stylex_test!(
  a_keyframes_call_reads_the_callback_parameter,
  |tr| stylex_transform(tr.comments.clone(), |b| b.with_runtime_injection()),
  r#"
    import * as stylex from '@stylexjs/stylex';
    export const s = stylex.create({
      root: {
        animationName: ['red'].map((c) => stylex.keyframes({ from: { color: c } })).join(),
      },
    });
  "#
);

// `.x1a5igra{animation-name:none}` and no `@keyframes` rule: the callback never
// runs.
stylex_test!(
  a_callback_that_never_runs_declares_no_rule,
  |tr| stylex_transform(tr.comments.clone(), |b| b.with_runtime_injection()),
  r#"
    import * as stylex from '@stylexjs/stylex';
    export const s = stylex.create({
      root: {
        animationName: [].map(() => stylex.keyframes({ from: { color: 'red' } })).join() || 'none',
      },
    });
  "#
);

// `@position-try --x17o73 {top:0;}` and
// `.x1ll8nuh{position-try-fallbacks:--x17o73}`.
stylex_test!(
  a_position_try_call_in_a_callback_declares_its_rule,
  |tr| stylex_transform(tr.comments.clone(), |b| b.with_runtime_injection()),
  r#"
    import * as stylex from '@stylexjs/stylex';
    export const s = stylex.create({
      root: {
        positionTryFallbacks: ['a'].map(() => stylex.positionTry({ top: '0' })).join(),
      },
    });
  "#
);

// `@keyframes xw2c1lr-B{from{opacity:0;}}` and
// `.x15ouc33{content:"xw2c1lr-B"}`. The function is a named import.
stylex_test!(
  a_named_keyframes_call_in_a_callback_declares_its_rule,
  |tr| stylex_transform(tr.comments.clone(), |b| b.with_runtime_injection()),
  r#"
    import * as stylex from '@stylexjs/stylex';
    import { keyframes } from '@stylexjs/stylex';
    const a = ['a'];
    export const styles = stylex.create({
      base: { content: a.map(x => keyframes({ from: { opacity: 0 } })).join('') },
    });
  "#
);
