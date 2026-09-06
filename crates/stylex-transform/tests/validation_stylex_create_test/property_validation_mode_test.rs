use crate::utils::prelude::*;
use stylex_enums::property_validation_mode::PropertyValidationMode;

fn stylex_transform(
  comments: TestComments,
  customize: impl FnOnce(TestBuilder) -> TestBuilder,
) -> impl Pass {
  build_test_transform(comments, |b| customize(b.with_runtime_injection()))
}

// Test default behavior (silent mode)
stylex_test!(
  does_not_throw_by_default_for_disallowed_properties_silent_mode,
  r#"
    import * as stylex from '@stylexjs/stylex';
    export const styles = stylex.create({
      root: {
        border: '1px solid red',
      },
    });
  "#
);

// Test throw mode
stylex_test_panic!(
  throws_error_when_property_validation_mode_is_throw,
  "is not supported",
  |tr| stylex_transform(tr.comments.clone(), |b| b
    .with_property_validation_mode(PropertyValidationMode::Throw)),
  r#"
    import * as stylex from '@stylexjs/stylex';
    const styles = stylex.create({
      root: {
        border: '1px solid red',
      },
    });
  "#
);

// Test warn mode
stylex_test!(
  does_not_throw_when_property_validation_mode_is_warn,
  |tr| stylex_transform(tr.comments.clone(), |b| b
    .with_property_validation_mode(PropertyValidationMode::Warn)),
  r#"
    import * as stylex from '@stylexjs/stylex';
    export const styles = stylex.create({
      root: {
        border: '1px solid red',
      },
    });
  "#
);

// Test silent mode explicitly
stylex_test!(
  does_not_throw_when_property_validation_mode_is_silent,
  |tr| stylex_transform(tr.comments.clone(), |b| b
    .with_property_validation_mode(PropertyValidationMode::Silent)),
  r#"
    import * as stylex from '@stylexjs/stylex';
    export const styles = stylex.create({
      root: {
        border: '1px solid red',
      },
    });
  "#
);

// Test with background property
stylex_test!(
  works_with_background_property,
  |tr| stylex_transform(tr.comments.clone(), |b| b
    .with_property_validation_mode(PropertyValidationMode::Silent)),
  r#"
    import * as stylex from '@stylexjs/stylex';
    export const styles = stylex.create({
      root: {
        background: 'red',
      },
    });
  "#
);

// Test with animation property
stylex_test!(
  works_with_animation_property,
  |tr| stylex_transform(tr.comments.clone(), |b| b
    .with_property_validation_mode(PropertyValidationMode::Silent)),
  r#"
    import * as stylex from '@stylexjs/stylex';
    export const styles = stylex.create({
      root: {
        animation: 'spin 1s',
      },
    });
  "#
);

// Test throw mode with background
stylex_test_panic!(
  throws_for_background_in_throw_mode,
  "is not supported",
  |tr| stylex_transform(tr.comments.clone(), |b| b
    .with_property_validation_mode(PropertyValidationMode::Throw)),
  r#"
    import * as stylex from '@stylexjs/stylex';
    const styles = stylex.create({
      root: {
        background: 'red',
      },
    });
  "#
);

// Test throw mode with animation
stylex_test_panic!(
  throws_for_animation_in_throw_mode,
  "is not supported",
  |tr| stylex_transform(tr.comments.clone(), |b| b
    .with_property_validation_mode(PropertyValidationMode::Throw)),
  r#"
    import * as stylex from '@stylexjs/stylex';
    const styles = stylex.create({
      root: {
        animation: 'spin 1s',
      },
    });
  "#
);

// ── Every rejecting shorthand, not just the three spelled one word ──
//
// `border`, `background` and `animation` are the shorthands whose property
// name is a single lowercase word. They agreed with the rejection table while
// every multi-word name silently missed it and reached the stylesheet as a
// shorthand rule, which is precisely what `property-specificity` exists to
// prevent: a later `borderTopWidth` cannot reliably override `border-top`.

stylex_test!(
  every_rejecting_shorthand_declares_nothing_in_silent_mode,
  |tr| stylex_transform(tr.comments.clone(), |b| b
    .with_property_validation_mode(PropertyValidationMode::Silent)),
  r#"
    import * as stylex from '@stylexjs/stylex';
    export const styles = stylex.create({
      root: {
        all: 'unset',
        animation: 'spin 1s',
        background: 'red',
        border: '1px solid red',
        borderInline: '1px solid red',
        borderBlock: '1px solid red',
        borderTop: 'none',
        borderInlineEnd: '1px solid red',
        borderRight: '1px solid red',
        borderBottom: '1px solid red',
        borderInlineStart: '1px solid red',
        borderLeft: '1px solid red',
      },
    });
  "#
);

// The deprecated aliases are aliases *of* a shorthand, so each must answer
// with that shorthand's rejection rather than reach the stylesheet.
stylex_test!(
  every_deprecated_border_alias_declares_nothing_in_silent_mode,
  |tr| stylex_transform(tr.comments.clone(), |b| b
    .with_property_validation_mode(PropertyValidationMode::Silent)),
  r#"
    import * as stylex from '@stylexjs/stylex';
    export const styles = stylex.create({
      root: {
        borderHorizontal: '1px solid red',
        borderVertical: '1px solid red',
        borderBlockStart: '1px solid red',
        borderBlockEnd: '1px solid red',
        borderStart: '1px solid red',
        borderEnd: '1px solid red',
      },
    });
  "#
);

stylex_test_panic!(
  throws_for_border_top_in_throw_mode,
  "borderTop is not supported",
  |tr| stylex_transform(tr.comments.clone(), |b| b
    .with_property_validation_mode(PropertyValidationMode::Throw)),
  r#"
    import * as stylex from '@stylexjs/stylex';
    const styles = stylex.create({
      root: {
        borderTop: 'none',
      },
    });
  "#
);

stylex_test_panic!(
  throws_for_border_inline_in_throw_mode,
  "borderInline is not supported",
  |tr| stylex_transform(tr.comments.clone(), |b| b
    .with_property_validation_mode(PropertyValidationMode::Throw)),
  r#"
    import * as stylex from '@stylexjs/stylex';
    const styles = stylex.create({
      root: {
        borderInline: '1px solid red',
      },
    });
  "#
);

stylex_test_panic!(
  throws_for_border_left_in_throw_mode,
  "`borderLeft` is not supported",
  |tr| stylex_transform(tr.comments.clone(), |b| b
    .with_property_validation_mode(PropertyValidationMode::Throw)),
  r#"
    import * as stylex from '@stylexjs/stylex';
    const styles = stylex.create({
      root: {
        borderLeft: '1px solid red',
      },
    });
  "#
);

// An alias reports the name of the shorthand it delegates to, not its own.
stylex_test_panic!(
  throws_for_border_block_start_naming_border_top,
  "borderTop is not supported",
  |tr| stylex_transform(tr.comments.clone(), |b| b
    .with_property_validation_mode(PropertyValidationMode::Throw)),
  r#"
    import * as stylex from '@stylexjs/stylex';
    const styles = stylex.create({
      root: {
        borderBlockStart: '1px solid red',
      },
    });
  "#
);

stylex_test_panic!(
  throws_for_all_in_throw_mode,
  "all is not supported",
  |tr| stylex_transform(tr.comments.clone(), |b| b
    .with_property_validation_mode(PropertyValidationMode::Throw)),
  r#"
    import * as stylex from '@stylexjs/stylex';
    const styles = stylex.create({
      root: {
        all: 'unset',
      },
    });
  "#
);

// A rejected shorthand must not take the longhands written beside it with it,
// and must not shadow one written after it.
stylex_test!(
  a_rejected_shorthand_leaves_its_longhand_neighbours_alone,
  |tr| stylex_transform(tr.comments.clone(), |b| b
    .with_property_validation_mode(PropertyValidationMode::Silent)),
  r#"
    import * as stylex from '@stylexjs/stylex';
    export const styles = stylex.create({
      root: {
        borderTopWidth: '1px',
        borderTop: 'none',
        borderTopStyle: 'solid',
        borderTopColor: 'red',
      },
    });
  "#
);

// The rejection is about the property name, never about the value, so an
// empty, whitespace-only, unicode, or syntactically broken value is rejected
// exactly the same way rather than reaching a value parser.
stylex_test!(
  a_rejecting_shorthand_ignores_its_value_entirely,
  |tr| stylex_transform(tr.comments.clone(), |b| b
    .with_property_validation_mode(PropertyValidationMode::Silent)),
  r#"
    import * as stylex from '@stylexjs/stylex';
    export const styles = stylex.create({
      empty: { borderTop: '' },
      blank: { borderTop: '   ' },
      unclosed: { borderTop: 'url("unterminated' },
      unbalanced: { borderTop: 'calc((1px + 2px)' },
      unicode: { borderTop: '1px solid red /* ✓ */' },
      important: { borderTop: '1px solid red !important' },
      numeric: { borderTop: 0 },
    });
  "#
);

// Under a resolution that expands these rather than rejecting them, they must
// keep expanding. The lookup key is per table, so a fix to one must not leak.
stylex_test!(
  application_order_still_expands_the_border_shorthands,
  |tr| stylex_transform(tr.comments.clone(), |b| b
    .with_style_resolution(StyleResolution::ApplicationOrder)),
  r#"
    import * as stylex from '@stylexjs/stylex';
    export const styles = stylex.create({
      root: {
        borderTop: '1px solid red',
        borderInline: '2px dashed blue',
      },
    });
  "#
);

stylex_test!(
  legacy_expand_shorthands_still_expands_the_border_shorthands,
  |tr| stylex_transform(tr.comments.clone(), |b| b
    .with_style_resolution(StyleResolution::LegacyExpandShorthands)),
  r#"
    import * as stylex from '@stylexjs/stylex';
    export const styles = stylex.create({
      root: {
        borderTop: '1px solid red',
        borderInline: '2px dashed blue',
      },
    });
  "#
);
