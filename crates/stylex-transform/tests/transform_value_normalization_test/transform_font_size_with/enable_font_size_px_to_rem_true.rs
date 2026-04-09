use crate::utils::prelude::*;

fn stylex_transform(comments: TestComments, customize: impl FnOnce(TestBuilder) -> TestBuilder) -> impl Pass {
  build_test_transform(comments, |b| {
    customize(
      b.with_runtime_injection_option(RuntimeInjection::Boolean(true))
        .with_enable_font_size_px_to_rem(true),
    )
  })
}

stylex_test!(
  transforms_font_size_from_px_to_rem,
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
  transforms_font_size_from_px_to_rem_even_with_calc,
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
