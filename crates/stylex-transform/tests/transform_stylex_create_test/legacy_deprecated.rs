use crate::utils::prelude::*;
use swc_core::ecma::transforms::testing::test;

stylex_test!(
  transforms_nested_pseudo_class_to_css,
  |tr| {
    StyleXTransform::test(tr.comments.clone())
      .with_runtime_injection_option(RuntimeInjection::Boolean(true))
      .with_runtime_injection()
      .into_pass()
  },
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
  |tr| {
    StyleXTransform::test(tr.comments.clone())
      .with_runtime_injection_option(RuntimeInjection::Boolean(true))
      .with_runtime_injection()
      .into_pass()
  },
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
  |tr| {
    StyleXTransform::test(tr.comments.clone())
      .with_runtime_injection_option(RuntimeInjection::Boolean(true))
      .with_runtime_injection()
      .into_pass()
  },
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
  |tr| {
    StyleXTransform::test(tr.comments.clone())
      .with_runtime_injection_option(RuntimeInjection::Boolean(true))
      .with_runtime_injection()
      .into_pass()
  },
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
  |tr| {
    StyleXTransform::test(tr.comments.clone())
      .with_runtime_injection_option(RuntimeInjection::Boolean(true))
      .with_runtime_injection()
      .into_pass()
  },
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
  |tr| {
    StyleXTransform::test(tr.comments.clone())
      .with_runtime_injection_option(RuntimeInjection::Boolean(true))
      .with_runtime_injection()
      .into_pass()
  },
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
  |tr| {
    StyleXTransform::test(tr.comments.clone())
      .with_runtime_injection_option(RuntimeInjection::Boolean(true))
      .with_runtime_injection()
      .into_pass()
  },
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
  |tr| {
    StyleXTransform::test(tr.comments.clone())
      .with_runtime_injection_option(RuntimeInjection::Boolean(true))
      .with_runtime_injection()
      .into_pass()
  },
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
  |tr| {
    StyleXTransform::test(tr.comments.clone())
      .with_runtime_injection_option(RuntimeInjection::Boolean(true))
      .with_runtime_injection()
      .into_pass()
  },
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
  |tr| {
    StyleXTransform::test(tr.comments.clone())
      .with_runtime_injection_option(RuntimeInjection::Boolean(true))
      .with_style_resolution(StyleResolution::LegacyExpandShorthands)
      .with_runtime_injection()
      .into_pass()
  },
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
