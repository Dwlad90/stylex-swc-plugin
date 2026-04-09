use crate::utils::prelude::*;
use swc_core::ecma::transforms::testing::test;

stylex_test!(
  position_try_object,
  |tr| StyleXTransform::test(tr.comments.clone()).into_pass(),
  r#"
    import * as stylex from '@stylexjs/stylex';
    export const name = stylex.positionTry({
      positionAnchor: '--anchor',
      top: '0',
      left: '0',
      width: '100px',
      height: '100px'
    });
  "#
);

stylex_test!(
  local_constants_used_in_position_try_object,
  |tr| StyleXTransform::test(tr.comments.clone()).into_pass(),
  r#"
    import * as stylex from '@stylexjs/stylex';
    const SIZE = '100px';
    export const name = stylex.positionTry({
      positionAnchor: '--anchor',
      top: '0',
      left: '0',
      width: SIZE,
      height: SIZE
    });
  "#
);

stylex_test!(
  position_try_value_used_within_create,
  |tr| StyleXTransform::test(tr.comments.clone()).into_pass(),
  r#"
    import * as stylex from '@stylexjs/stylex';
    const SIZE = '100px';
    const name = stylex.positionTry({
      top: '0',
      left: '0',
      width: SIZE,
      height: SIZE
    });
    export const styles = stylex.create({
      root: {
        positionTryFallbacks: name,
      }
    });
  "#
);

stylex_test!(
  position_try_object_used_inline,
  |tr| StyleXTransform::test(tr.comments.clone()).into_pass(),
  r#"
    import * as stylex from '@stylexjs/stylex';
    export const styles = stylex.create({
      root: {
        positionTryFallbacks: stylex.positionTry({
          positionAnchor: '--anchor',
          top: '0',
          left: '0',
          width: '100px',
          height: '100px'
        }),
      },
    });
  "#
);
