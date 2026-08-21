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

// A boolean is the one value that spells nothing without being blank or absent,
// and it reaches the drop through the shorthand expansion rather than through
// the value normalizer. Pinned separately because it is the only input that
// exercises that arm of the expansion -- `create` refuses a boolean before the
// expansion runs, and a keyframes step drops one by its own route.
stylex_test!(
  a_boolean_value_declares_nothing,
  |tr| stylex_transform(tr.comments.clone(), |b| b),
  r#"
    import * as stylex from '@stylexjs/stylex';
    export const boolean = stylex.positionTry({ positionAnchor: '--a', top: true });
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

// A theme reference read as a fallback value. The reference implementation emits
// the at-rule with the declaration missing -- `@position-try --x {}` -- and
// says nothing; refused here, as every other shape with no value form in this
// position already is. A decided divergence, recorded as
// `modules-1266-theme-reference-in-a-position-try`.
stylex_test_panic!(
  a_theme_reference_read_as_a_position_try_value_is_refused,
  "Only static values are allowed inside of a positionTry() call.",
  |tr| theme_import_transform(tr.comments.clone()),
  r#"
    import * as stylex from '@stylexjs/stylex';
    import { zIndex } from 'zIndex.stylex.js';

    export const name = stylex.positionTry({ top: zIndex });
  "#
);

// A folded function map read as a fallback value. The reference implementation
// emits the at-rule with the declaration missing; refused here, for the reason
// a theme reference in this position is. A decided divergence, recorded as
// `modules-1266-a-folded-function-map-in-a-position-try`.
stylex_test_panic!(
  a_folded_function_map_read_as_a_position_try_value_is_refused,
  "Only static values are allowed inside of a positionTry() call.",
  r#"
    import * as stylex from '@stylexjs/stylex';

    export const name = stylex.positionTry({ positionAnchor: '--a', top: stylex });
  "#
);
