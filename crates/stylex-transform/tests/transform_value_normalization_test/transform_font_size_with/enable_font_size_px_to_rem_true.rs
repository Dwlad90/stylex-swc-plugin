use crate::utils::prelude::*;
use swc_core::ecma::transforms::testing::test;

stylex_test!(
  transforms_font_size_from_px_to_rem,
  |tr| StyleXTransform::test(tr.comments.clone())
    .with_runtime_injection_option(RuntimeInjection::Boolean(true))
    .with_enable_font_size_px_to_rem(true)
    .into_pass(),
  r#"
      import stylex from 'stylex';
      const styles = stylex.create({
        foo: {
          fontSize: '24px',
        },
        bar: {
          fontSize: 18,
        },
        baz: {
          fontSize: '1.25rem',
        },
        qux: {
          fontSize: 'inherit',
        }
      });
    "#
);

stylex_test!(
  transforms_font_size_from_px_to_rem_even_with_calc,
  |tr| StyleXTransform::test(tr.comments.clone())
    .with_runtime_injection_option(RuntimeInjection::Boolean(true))
    .with_enable_font_size_px_to_rem(true)
    .into_pass(),
  r#"
      import stylex from 'stylex';
      const styles = stylex.create({
        foo: {
          fontSize: 'calc(100% - 24px)',
        },
      });
    "#
);
