use crate::utils::prelude::*;

fn stylex_transform(
  comments: TestComments,
  customize: impl FnOnce(TestBuilder) -> TestBuilder,
) -> impl Pass {
  build_test_transform(comments, |b| customize(b.with_runtime_injection()))
}

stylex_test_transform!(
  #[ignore],
  line_clamp,
  |tr| stylex_transform(tr.comments.clone(), |b| b),
  r#"
            import * as stylex from '@stylexjs/stylex';
            export const styles = stylex.create({ x: { lineClamp: 3 } });
        "#,
  r#""#
);

stylex_test_transform!(
  #[ignore],
  pointer_events,
  |tr| stylex_transform(tr.comments.clone(), |b| b),
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
  |tr| stylex_transform(tr.comments.clone(), |b| b),
  r#"
            import * as stylex from '@stylexjs/stylex';
            export const styles = stylex.create({ x: { scrollbarWidth: 'none' } });
        "#,
  r#""#
);
