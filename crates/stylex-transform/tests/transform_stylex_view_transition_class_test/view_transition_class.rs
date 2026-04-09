use crate::utils::prelude::*;
use swc_core::ecma::transforms::testing::{test, test_transform};

stylex_test!(
  basic_object,
  |tr| StyleXTransform::test(tr.comments.clone()).into_pass(),
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
  |tr| StyleXTransform::test(tr.comments.clone()).into_pass(),
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
  |tr| StyleXTransform::test(tr.comments.clone()).into_pass(),
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
  |tr| StyleXTransform::test(tr.comments.clone()).into_pass(),
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

#[test]
#[ignore]
fn using_contextual_styles() {
  test_transform(
    ts_syntax(),
    Option::None,
    |tr| StyleXTransform::test(tr.comments.clone()).into_pass(),
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
    r#""#,
  )
}
