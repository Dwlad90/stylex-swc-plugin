use crate::utils::prelude::*;

stylex_test!(
  keyframes_object,
  r#"
    import * as stylex from '@stylexjs/stylex';
    export const name = stylex.keyframes({
      from: {
        color: 'red',
      },
      to: {
        color: 'blue',
      }
    });
  "#
);

stylex_test!(
  local_variables_used_in_keyframes_object,
  r#"
    import * as stylex from '@stylexjs/stylex';
    const COLOR = 'red';
    export const name = stylex.keyframes({
      from: {
        color: COLOR,
      },
      to: {
        color: 'blue',
      }
    });
  "#
);

stylex_test!(
  template_literals_used_in_keyframes_object,
  r#"
    import * as stylex from '@stylexjs/stylex';
    const COLOR = 'red';
    const name = stylex.keyframes({
      from: {
        color: COLOR,
      },
      to: {
        color: 'blue',
      }
    });
    export const styles = stylex.create({
      root: {
        animationName: `${name}`,
      }
    });
  "#
);

stylex_test!(
  keyframes_object_used_inline,
  r#"
    import * as stylex from '@stylexjs/stylex';
    export const styles = stylex.create({
      root: {
        animationName: stylex.keyframes({
          from: {
            color: 'red',
          },
          to: {
            color: 'blue',
          },
        }),
      },
    });
  "#
);

stylex_test!(
  keyframes_object_rtl_polyfills_legacy,
  r#"
    import * as stylex from '@stylexjs/stylex';
    export const name = stylex.keyframes({
      from: {
        insetBlockStart: 0,
      },
      to: {
        insetBlockStart: 100,
      }
    });
  "#
);

// An explicitly empty `classNamePrefix` is honoured rather than replaced by
// the default, so the animation name carries no prefix.
stylex_test!(
  keyframes_with_empty_class_name_prefix,
  |tr| build_test_transform(tr.comments.clone(), |b| b
    .with_class_name_prefix("")
    .with_runtime_injection()),
  r#"
    import * as stylex from '@stylexjs/stylex';
    export const name = stylex.keyframes({
      from: {
        color: 'red',
      },
      to: {
        color: 'blue',
      }
    });
  "#
);
