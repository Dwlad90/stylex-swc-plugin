use crate::utils::prelude::*;
use swc_core::ecma::transforms::testing::{test, test_transform};

#[test]
#[ignore]
fn line_clamp() {
  test_transform(
    ts_syntax(),
    Option::None,
    |tr| {
      StyleXTransform::test(tr.comments.clone())
        .with_runtime_injection()
        .into_pass()
    },
    r#"
            import * as stylex from '@stylexjs/stylex';
            export const styles = stylex.create({ x: { lineClamp: 3 } });
        "#,
    r#""#,
  )
}

#[test]
#[ignore]
fn pointer_events() {
  test_transform(
    ts_syntax(),
    Option::None,
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
    r#""#,
  )
}

#[test]
#[ignore]
fn scrollbar_width() {
  test_transform(
    ts_syntax(),
    Option::None,
    |tr| {
      StyleXTransform::test(tr.comments.clone())
        .with_runtime_injection()
        .into_pass()
    },
    r#"
            import * as stylex from '@stylexjs/stylex';
            export const styles = stylex.create({ x: { scrollbarWidth: 'none' } });
        "#,
    r#""#,
  )
}
