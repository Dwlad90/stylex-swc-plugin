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

// `positionTry()` uses the active transform options while preprocessing
// properties, but default options for LTR/RTL pair generation. Legacy logical
// polyfill settings therefore do not flip the generated `@position-try`
// declaration keys during direction generation.
stylex_test!(
  position_try_object_uses_default_direction_options,
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

// An explicitly empty `classNamePrefix` is honoured rather than replaced by
// the default, so the position-try name carries no prefix.
stylex_test!(
  position_try_with_empty_class_name_prefix,
  |tr| stylex_transform(tr.comments.clone(), |b| b.with_class_name_prefix("")),
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

// A value that spells no CSS text declares nothing, so `top:` never reaches the
// at-rule body. `null` means nothing anywhere; a blank string normalizes to the
// same nothing.
//
// The body is what the name is hashed from, so `--x1rdsnup` recurring is the
// claim that the declaration is really gone rather than emitted empty: it is the
// name a body with only the anchor produces. Measured output of
// `@stylexjs/babel-plugin` 0.19.0 for each input.
stylex_test!(
  a_value_that_spells_nothing_declares_nothing,
  |tr| stylex_transform(tr.comments.clone(), |b| b),
  r#"
    import * as stylex from '@stylexjs/stylex';
    export const nullish = stylex.positionTry({ positionAnchor: '--a', top: null });
    export const empty = stylex.positionTry({ positionAnchor: '--a', top: '' });
    export const blank = stylex.positionTry({ positionAnchor: '--a', top: ' ' });
    export const anchorOnly = stylex.positionTry({ positionAnchor: '--a' });
  "#
);

// Only the declaration that spells nothing drops; the ones around it are
// untouched.
stylex_test!(
  a_dropped_value_keeps_its_siblings,
  |tr| stylex_transform(tr.comments.clone(), |b| b),
  r#"
    import * as stylex from '@stylexjs/stylex';
    export const dropped = stylex.positionTry({
      positionAnchor: '--a',
      top: null,
      left: '10px',
    });
    export const sibling = stylex.positionTry({
      positionAnchor: '--a',
      left: '10px',
    });
  "#
);
