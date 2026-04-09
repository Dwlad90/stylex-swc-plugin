use crate::utils::prelude::*;

fn stylex_transform(
  comments: TestComments,
  customize: impl FnOnce(TestBuilder) -> TestBuilder,
) -> impl Pass {
  build_test_transform(comments, |b| {
    customize(
      b.with_runtime_injection_option(RuntimeInjection::Boolean(true))
        .with_style_resolution(StyleResolution::LegacyExpandShorthands)
        .with_enable_logical_styles_polyfill(false)
        .with_runtime_injection(),
    )
  })
}

stylex_test!(
  padding_with_longhand_property_collisions,
  |tr| stylex_transform(tr.comments.clone(), |b| b),
  r#"
    import stylex from 'stylex';
    const styles = stylex.create({
      foo: {
        padding: 5,
        paddingEnd: 10,
      },

      bar: {
        padding: 2,
        paddingStart: 10,
      },
    });
    stylex(styles.foo, styles.bar);
  "#
);

stylex_test!(
  padding_with_null_longhand_property_collisions,
  |tr| stylex_transform(tr.comments.clone(), |b| b),
  r#"
    import stylex from 'stylex';
    const styles = stylex.create({
      foo: {
        padding: 5,
        paddingEnd: 10,
      },

      bar: {
        padding: 2,
        paddingStart: null,
      },
    });
    stylex(styles.foo, styles.bar);
  "#
);

stylex_test!(
  border_color_basic_shorthand,
  |tr| stylex_transform(tr.comments.clone(), |b| b),
  r#"
    import stylex from 'stylex';
    const styles = stylex.create({
      foo: {
        borderColor: 'red blue green yellow'
      }
    });
    stylex(styles.foo);
  "#
);

stylex_test!(
  border_width_basic_shorthand,
  |tr| stylex_transform(tr.comments.clone(), |b| b),
  r#"
    import stylex from 'stylex';
    const styles = stylex.create({
      foo: {
        borderWidth: '1px 2px 3px 4px'
      }
    });
    stylex(styles.foo);
  "#
);
