use crate::utils::prelude::*;

/// File-level transform: legacy tests use explicit
/// RuntimeInjection::Boolean(true). Accepts a closure for test-specific
/// overrides on top of the file baseline.
fn stylex_transform(
  comments: TestComments,
  customize: impl FnOnce(TestBuilder) -> TestBuilder,
) -> impl Pass {
  build_test_transform(comments, |b| {
    customize(
      b.with_runtime_injection_option(RuntimeInjection::Boolean(true))
        .with_runtime_injection(),
    )
  })
}

stylex_test!(
  transforms_nested_pseudo_class_to_css,
  |tr| stylex_transform(tr.comments.clone(), |b| b),
  r#"
    import stylex from 'stylex';
    const styles = stylex.create({
      default: {
        ':hover': {
          backgroundColor: 'red',
          color: 'blue',
        },
      },
    });
  "#
);

stylex_test!(
  transforms_invalid_pseudo_class,
  |tr| stylex_transform(tr.comments.clone(), |b| b),
  r#"
    import stylex from 'stylex';
    const styles = stylex.create({
      default: {
        ':invalpwdijad': {
          backgroundColor: 'red',
          color: 'blue',
        },
      },
    });
  "#
);

stylex_test!(
  transforms_valid_pseudo_classes_in_order,
  |tr| stylex_transform(tr.comments.clone(), |b| b),
  r#"
    import stylex from 'stylex';
    const styles = stylex.create({
      default: {
        ':hover': {
          color: 'blue',
        },
        ':active': {
          color: 'red',
        },
        ':focus': {
          color: 'yellow',
        },
        ':nth-child(2n)': {
          color: 'purple'
        }
      },
    });
  "#
);

stylex_test!(
  transforms_pseudo_class_with_array_value_as_fallbacks,
  |tr| stylex_transform(tr.comments.clone(), |b| b),
  r#"
    import stylex from 'stylex';
    const styles = stylex.create({
      default: {
        ':hover': {
          position: ['sticky', 'fixed'],
        }
      },
    });
  "#
);

stylex_test!(
  transforms_legacy_pseudo_class_within_a_pseudo_element,
  |tr| stylex_transform(tr.comments.clone(), |b| b),
  r#"
    import stylex from 'stylex';
    export const styles = stylex.create({
      foo: {
        '::before': {
          color: 'red',
          ':hover': {
            color: 'blue',
          },
        },
      },
    });
  "#
);

stylex_test!(
  transforms_pseudo_elements_within_legacy_pseudo_class,
  |tr| stylex_transform(tr.comments.clone(), |b| b),
  r#"
    import stylex from 'stylex';
    export const styles = stylex.create({
      foo: {
        '::before': {
          color: 'red',
        },
        ':hover': {
          '::before': {
            color: 'blue',
          },
        },
      },
    });
  "#
);

stylex_test!(
  transforms_pseudo_elements_sandwiched_within_pseudo_classes,
  |tr| stylex_transform(tr.comments.clone(), |b| b),
  r#"
    import stylex from 'stylex';
    export const styles = stylex.create({
      foo: {
        '::before': {
          color: 'red',
        },
        ':hover': {
          '::before': {
            color: {
              default: 'blue',
              ':hover': 'green',
              ':active': 'purple',
            },
          },
        },
      },
    });
  "#
);

stylex_test!(
  transforms_media_queries,
  |tr| stylex_transform(tr.comments.clone(), |b| b),
  r#"
    import stylex from 'stylex';
    const styles = stylex.create({
      default: {
        backgroundColor: 'red',
        '@media (min-width: 1000px)': {
          backgroundColor: 'blue',
        },
        '@media (min-width: 2000px)': {
          backgroundColor: 'purple',
        },
      },
    });
  "#
);

stylex_test!(
  transforms_supports_queries,
  |tr| stylex_transform(tr.comments.clone(), |b| b),
  r#"
    import stylex from 'stylex';
    const styles = stylex.create({
      default: {
        backgroundColor: 'red',
        '@supports (hover: hover)': {
          backgroundColor: 'blue',
        },
        '@supports not (hover: hover)': {
          backgroundColor: 'purple',
        },
      },
    });
  "#
);

stylex_test!(
  transforms_dynamic_shorthands_in_legacy_expand_shorthands_mode,
  |tr| stylex_transform(tr.comments.clone(), |b| {
    b.with_style_resolution(StyleResolution::LegacyExpandShorthands)
  }),
  r#"
    import stylex from 'stylex';
    export const styles = stylex.create({
      default: (margin) => ({
        backgroundColor: 'red',
        margin: {
          default: margin,
          ':hover': margin + 4,
        },
        marginTop: margin - 4,
      })
    });
  "#
);

// The shorthand expansion path echoes a numeric value from the source rather
// than reprinting it, and this is the seam where that reaches a stylesheet.
// Reprinting respelled everything the authored spelling and the shortest
// round-trip spelling disagree on, and each respelling was a different class
// name: `1E2px` became `100px`, `1.50px` became `1.5px`, `1e21px` became
// twenty-two digits, and `-0px` became `+-0px`, which is not a CSS value.
//
// Confirmed against `@stylexjs/babel-plugin@0.19.0` run over the same source
// with the same `styleResolution` — it emits these values and these class
// names.
stylex_test!(
  legacy_expanded_shorthands_echo_the_authored_number,
  |tr| stylex_transform(tr.comments.clone(), |b| {
    b.with_style_resolution(StyleResolution::LegacyExpandShorthands)
  }),
  r#"
    import stylex from 'stylex';
    export const styles = stylex.create({
      spellings: { margin: '1E2px 1.50px' },
      negativeZero: { padding: '-0px 1e21px' },
      pastTheDoubleRange: { margin: '1.7976931348623157e308px' },
      authoredSigns: { padding: '+1px +2% 0.5px 000.5px' },
      fullPrecision: { margin: '1.2345678901234567px 7%' },
    });
  "#
);
