use crate::utils::prelude::*;

/// File-level transform: legacy deprecated tests use explicit RuntimeInjection::Boolean(true)
fn legacy_transform(
  comments: std::rc::Rc<swc_core::common::comments::SingleThreadedComments>,
) -> impl swc_core::ecma::ast::Pass {
  build_test_transform(comments, |b| {
    b.with_runtime_injection_option(RuntimeInjection::Boolean(true))
      .with_runtime_injection()
  })
}

stylex_test!(
  transforms_nested_pseudo_class_to_css,
  |tr| legacy_transform(tr.comments.clone()),
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
  |tr| legacy_transform(tr.comments.clone()),
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
  |tr| legacy_transform(tr.comments.clone()),
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
  |tr| legacy_transform(tr.comments.clone()),
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
  |tr| legacy_transform(tr.comments.clone()),
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
  |tr| legacy_transform(tr.comments.clone()),
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
  |tr| legacy_transform(tr.comments.clone()),
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
  |tr| legacy_transform(tr.comments.clone()),
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
  |tr| legacy_transform(tr.comments.clone()),
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
  |tr| build_test_transform(tr.comments.clone(), |b| {
    b.with_runtime_injection_option(RuntimeInjection::Boolean(true))
      .with_style_resolution(StyleResolution::LegacyExpandShorthands)
      .with_runtime_injection()
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
