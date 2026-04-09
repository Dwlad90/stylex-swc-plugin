use crate::utils::prelude::*;
use stylex_enums::property_validation_mode::PropertyValidationMode;

fn stylex_transform(
  comments: TestComments,
  customize: impl FnOnce(TestBuilder) -> TestBuilder,
) -> impl Pass {
  build_test_transform(comments, |b| customize(b.with_runtime_injection()))
}

// Test default behavior (silent mode)
stylex_test_transform!(
  does_not_throw_by_default_for_disallowed_properties_silent_mode,
  r#"
    import * as stylex from '@stylexjs/stylex';
    const styles = stylex.create({
      root: {
        border: '1px solid red',
      },
    });
  "#,
  r#"
    import * as stylex from '@stylexjs/stylex';
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
stylex_test_transform!(
  does_not_throw_when_property_validation_mode_is_warn,
  |tr| stylex_transform(tr.comments.clone(), |b| b
    .with_property_validation_mode(PropertyValidationMode::Warn)),
  r#"
    import * as stylex from '@stylexjs/stylex';
    const styles = stylex.create({
      root: {
        border: '1px solid red',
      },
    });
  "#,
  r#"
    import * as stylex from '@stylexjs/stylex';
  "#
);

// Test silent mode explicitly
stylex_test_transform!(
  does_not_throw_when_property_validation_mode_is_silent,
  |tr| stylex_transform(tr.comments.clone(), |b| b
    .with_property_validation_mode(PropertyValidationMode::Silent)),
  r#"
    import * as stylex from '@stylexjs/stylex';
    const styles = stylex.create({
      root: {
        border: '1px solid red',
      },
    });
  "#,
  r#"
    import * as stylex from '@stylexjs/stylex';
  "#
);

// Test with background property
stylex_test_transform!(
  works_with_background_property,
  |tr| stylex_transform(tr.comments.clone(), |b| b
    .with_property_validation_mode(PropertyValidationMode::Silent)),
  r#"
    import * as stylex from '@stylexjs/stylex';
    const styles = stylex.create({
      root: {
        background: 'red',
      },
    });
  "#,
  r#"
    import * as stylex from '@stylexjs/stylex';
  "#
);

// Test with animation property
stylex_test_transform!(
  works_with_animation_property,
  |tr| stylex_transform(tr.comments.clone(), |b| b
    .with_property_validation_mode(PropertyValidationMode::Silent)),
  r#"
    import * as stylex from '@stylexjs/stylex';
    const styles = stylex.create({
      root: {
        animation: 'spin 1s',
      },
    });
  "#,
  r#"
    import * as stylex from '@stylexjs/stylex';
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
