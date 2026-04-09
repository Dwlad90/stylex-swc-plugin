use crate::utils::prelude::*;

fn stylex_transform(comments: TestComments, customize: impl FnOnce(TestBuilder) -> TestBuilder) -> impl Pass {
  build_test_transform(comments, |b| {
    customize(
      b.with_runtime_injection_option(RuntimeInjection::Boolean(true)),
    )
  })
}

stylex_test!(
  ignores_px_font_size,
  |tr| stylex_transform(tr.comments.clone(), |b| b),
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
  |tr| stylex_transform(tr.comments.clone(), |b| b),
  r#"
      import stylex from 'stylex';
      const styles = stylex.create({
        foo: {
          fontSize: 'calc(100% - 24px)',
        },
      });
    "#
);
