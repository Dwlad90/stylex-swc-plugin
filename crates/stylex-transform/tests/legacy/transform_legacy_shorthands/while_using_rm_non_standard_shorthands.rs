use crate::utils::prelude::*;
use swc_core::ecma::transforms::testing::test;

stylex_test!(
  stylex_call_with_exported_short_form_properties,
  |tr| {
    StyleXTransform::test(tr.comments.clone())
      .with_runtime_injection_option(RuntimeInjection::Boolean(true))
      .with_style_resolution(StyleResolution::LegacyExpandShorthands)
      .with_enable_logical_styles_polyfill(true)
      .with_runtime_injection()
      .into_pass()
  },
  r#"
    import stylex from 'stylex';
    export const styles = stylex.create({
        foo: {
            padding: 5
        }
    });
    stylex(styles.foo);
    "#
);

stylex_test!(
  stylex_call_with_short_form_property_collisions,
  |tr| {
    StyleXTransform::test(tr.comments.clone())
      .with_runtime_injection_option(RuntimeInjection::Boolean(true))
      .with_style_resolution(StyleResolution::LegacyExpandShorthands)
      .with_enable_logical_styles_polyfill(true)
      .with_runtime_injection()
      .into_pass()
  },
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
  stylex_call_with_short_form_property_collisions_with_null,
  |tr| {
    StyleXTransform::test(tr.comments.clone())
      .with_runtime_injection_option(RuntimeInjection::Boolean(true))
      .with_style_resolution(StyleResolution::LegacyExpandShorthands)
      .with_enable_logical_styles_polyfill(true)
      .with_runtime_injection()
      .into_pass()
  },
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
  |tr| {
    StyleXTransform::test(tr.comments.clone())
      .with_runtime_injection_option(RuntimeInjection::Boolean(true))
      .with_style_resolution(StyleResolution::LegacyExpandShorthands)
      .with_enable_logical_styles_polyfill(true)
      .with_runtime_injection()
      .into_pass()
  },
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
  |tr| {
    StyleXTransform::test(tr.comments.clone())
      .with_runtime_injection_option(RuntimeInjection::Boolean(true))
      .with_style_resolution(StyleResolution::LegacyExpandShorthands)
      .with_enable_logical_styles_polyfill(true)
      .with_runtime_injection()
      .into_pass()
  },
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
