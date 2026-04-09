use crate::utils::prelude::*;

stylex_test_transform!(
  #[ignore],
  line_clamp,
  |tr| {
    StyleXTransform::test(tr.comments.clone())
      .with_runtime_injection()
      .into_pass()
  },
  r#"
            import * as stylex from '@stylexjs/stylex';
            export const styles = stylex.create({ x: { lineClamp: 3 } });
        "#,
  r#""#
);

stylex_test_transform!(
  #[ignore],
  pointer_events,
  |tr| {
    StyleXTransform::test(tr.comments.clone())
      .with_runtime_injection()
      .into_pass()
  },
  r#"
            import * as stylex from '@stylexjs/stylex';
            export const styles = stylex.create({
              a: { pointerEvents: 'auto' },
              b: { pointerEvents: 'box-none' },
              c: { pointerEvents: 'box-only' },
              d: { pointerEvents: 'none' }
            });
        "#,
  r#""#
);

stylex_test_transform!(
  #[ignore],
  scrollbar_width,
  |tr| {
    StyleXTransform::test(tr.comments.clone())
      .with_runtime_injection()
      .into_pass()
  },
  r#"
            import * as stylex from '@stylexjs/stylex';
            export const styles = stylex.create({ x: { scrollbarWidth: 'none' } });
        "#,
  r#""#
);
