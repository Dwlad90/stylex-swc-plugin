use crate::utils::prelude::*;

fn stylex_transform(
  comments: TestComments,
  customize: impl FnOnce(TestBuilder) -> TestBuilder,
) -> impl Pass {
  build_test_transform(comments, customize)
}

stylex_test!(
  basic_object,
  |tr| stylex_transform(tr.comments.clone(), |b| b),
  r#"
    import * as stylex from '@stylexjs/stylex';
    export const cls = stylex.viewTransitionClass({
      group: {
        transitionProperty: 'none',
      },
      imagePair: {
        borderRadius: 16,
      },
      old: {
        animationDuration: '0.5s',
      },
      new: {
        animationTimingFunction: 'ease-out',
      },
    });
  "#
);

stylex_test!(
  local_variables_used_in_view_transition_class,
  |tr| stylex_transform(tr.comments.clone(), |b| b),
  r#"
    import * as stylex from '@stylexjs/stylex';
    const animationDuration = '1s';
    export const cls = stylex.viewTransitionClass({
      old: { animationDuration },
      new: { animationDuration },
      group: { animationDuration },
      imagePair: { animationDuration },
    });
  "#
);

stylex_test!(
  using_keyframes,
  |tr| stylex_transform(tr.comments.clone(), |b| b),
  r#"
    import * as stylex from '@stylexjs/stylex';
    export const fadeIn = stylex.keyframes({
      from: {opacity: 0},
      to: {opacity: 1},
    });
    export const fadeOut = stylex.keyframes({
      from: {opacity: 1},
      to: {opacity: 0},
    });
    export const cls = stylex.viewTransitionClass({
      old: {
        animationName: fadeOut,
        animationDuration: '1s',
      },
      new: {
        animationName: fadeIn,
        animationDuration: '1s',
      },
    });
  "#
);

stylex_test!(
  using_inline_keyframes,
  |tr| stylex_transform(tr.comments.clone(), |b| b),
  r#"
    import * as stylex from '@stylexjs/stylex';
    export const cls = stylex.viewTransitionClass({
      old: {
        animationName: stylex.keyframes({
          from: {opacity: 1},
          to: {opacity: 0},
        }),
        animationDuration: '1s',
      },
      new: {
        animationName: stylex.keyframes({
          from: {opacity: 0},
          to: {opacity: 1},
        }),
        animationDuration: '1s',
      },
    });
  "#
);

stylex_test_transform!(
  #[ignore],
  using_contextual_styles,
  |tr| stylex_transform(tr.comments.clone(), |b| b),
  r#"
    import * as stylex from 'stylex';
    export const cls = stylex.viewTransitionClass({
      group: {
        animationDuration: {
          default: '1s',
          '@media (min-width: 800px)': '2s'
        }
      },
    });
  "#,
  r#""#
);

// An explicitly empty `classNamePrefix` is honoured rather than replaced by
// the default, so the view-transition class carries no prefix. The input
// repeats `basic_object` so the two snapshots differ only by that prefix.
stylex_test!(
  view_transition_class_with_empty_class_name_prefix,
  |tr| stylex_transform(tr.comments.clone(), |b| b.with_class_name_prefix("")),
  r#"
    import * as stylex from '@stylexjs/stylex';
    export const cls = stylex.viewTransitionClass({
      group: {
        transitionProperty: 'none',
      },
      imagePair: {
        borderRadius: 16,
      },
      old: {
        animationDuration: '0.5s',
      },
      new: {
        animationTimingFunction: 'ease-out',
      },
    });
  "#
);

// The same rule as every other at-rule body assembled from pairs: a value that
// spells no CSS text declares nothing, so `animation-duration:` never reaches
// the body, and the class name is the one an empty body produces.
//
// `x1od172d` for the first two is measured output of
// `@stylexjs/babel-plugin` 0.19.0 for the `null` spelling, the one it handles
// deliberately.
stylex_test!(
  a_value_that_spells_nothing_declares_nothing,
  |tr| stylex_transform(tr.comments.clone(), |b| b),
  r#"
    import * as stylex from '@stylexjs/stylex';
    export const nullish = stylex.viewTransitionClass({
      group: { animationDuration: null },
    });
    export const blank = stylex.viewTransitionClass({
      group: { animationDuration: ' ' },
    });
    export const withSibling = stylex.viewTransitionClass({
      group: { animationDuration: null, opacity: 0.5 },
    });
  "#
);
