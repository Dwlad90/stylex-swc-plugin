use crate::utils::prelude::*;

fn stylex_transform(
  comments: TestComments,
  customize: impl FnOnce(TestBuilder) -> TestBuilder,
) -> impl Pass {
  build_test_transform(comments, customize)
}

stylex_test!(
  position_try_object,
  |tr| stylex_transform(tr.comments.clone(), |b| b),
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
  |tr| stylex_transform(tr.comments.clone(), |b| b),
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
  |tr| stylex_transform(tr.comments.clone(), |b| b),
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
  |tr| stylex_transform(tr.comments.clone(), |b| b),
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

// Legacy shorthand expansion + the logical-styles polyfill is the only mode in
// which a `@position-try` property's key actually flips (e.g. `inset-inline-start`
// -> `left` for LTR, `-> right` for RTL). This exercises the `[key, value]` tuple
// serialization in `construct_position_try_obj`, where the flipped key differs from
// the outer property key, so the LTR/RTL strings diverge and a distinct RTL rule is
// emitted.
stylex_test!(
  position_try_object_legacy_logical_rtl,
  |tr| stylex_transform(tr.comments.clone(), |b| {
    b.with_style_resolution(StyleResolution::LegacyExpandShorthands)
      .with_enable_logical_styles_polyfill(true)
      .with_runtime_injection()
  }),
  r#"
    import * as stylex from '@stylexjs/stylex';
    export const styles = stylex.create({
      root: {
        positionTryFallbacks: stylex.positionTry({
          insetInlineStart: '10px',
          top: '0',
        }),
      },
    });
  "#
);
