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
  non_standard_value_end_aka_inline_end_for_clear_property,
  |tr| stylex_transform(tr.comments.clone(), |b| b),
  r#"
        import * as stylex from '@stylexjs/stylex';
        export const styles = stylex.create({ x: { clear: 'end' } });
      "#
);

stylex_test!(
  non_standard_value_start_aka_inline_start_for_clear_property,
  |tr| stylex_transform(tr.comments.clone(), |b| b),
  r#"
        import * as stylex from '@stylexjs/stylex';
        export const styles = stylex.create({ x: { clear: 'start' } });
      "#
);

stylex_test!(
  non_standard_value_end_aka_inline_end_for_float_property,
  |tr| stylex_transform(tr.comments.clone(), |b| b),
  r#"
        import * as stylex from '@stylexjs/stylex';
        export const styles = stylex.create({ x: { float: 'end' } });
      "#
);

stylex_test!(
  non_standard_value_start_aka_inline_start_for_float_property,
  |tr| stylex_transform(tr.comments.clone(), |b| b),
  r#"
        import * as stylex from '@stylexjs/stylex';
        export const styles = stylex.create({ x: { float: 'start' } });
      "#
);
