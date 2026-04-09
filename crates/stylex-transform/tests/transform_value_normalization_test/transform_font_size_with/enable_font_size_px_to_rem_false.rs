use crate::utils::prelude::*;
use swc_core::ecma::transforms::testing::test;

stylex_test!(
  ignores_px_font_size,
  |tr| StyleXTransform::test(tr.comments.clone())
    .with_runtime_injection_option(RuntimeInjection::Boolean(true))
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
  ignores_px_font_size_with_calc,
  |tr| StyleXTransform::test(tr.comments.clone())
    .with_runtime_injection_option(RuntimeInjection::Boolean(true))
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
