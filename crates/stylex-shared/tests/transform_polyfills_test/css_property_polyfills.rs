use stylex_shared::{StyleXTransform, shared::structures::plugin_pass::PluginPass};
use swc_core::ecma::{
  parser::{Syntax, TsSyntax},
  transforms::testing::{test, test_transform},
};

#[test]
#[ignore]
fn line_clamp() {
  test_transform(
    Syntax::Typescript(TsSyntax {
      tsx: true,
      ..Default::default()
    }),
    Option::None,
    |tr| {
      StyleXTransform::new_test_force_runtime_injection_with_pass(
        tr.comments.clone(),
        PluginPass::default(),
        None,
      )
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
    Syntax::Typescript(TsSyntax {
      tsx: true,
      ..Default::default()
    }),
    Option::None,
    |tr| {
      StyleXTransform::new_test_force_runtime_injection_with_pass(
        tr.comments.clone(),
        PluginPass::default(),
        None,
      )
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
    Syntax::Typescript(TsSyntax {
      tsx: true,
      ..Default::default()
    }),
    Option::None,
    |tr| {
      StyleXTransform::new_test_force_runtime_injection_with_pass(
        tr.comments.clone(),
        PluginPass::default(),
        None,
      )
    },
    r#"
            import * as stylex from '@stylexjs/stylex';
            export const styles = stylex.create({ x: { scrollbarWidth: 'none' } });
        "#,
    r#""#,
  )
}

test!(
  Syntax::Typescript(TsSyntax {
    tsx: true,
    ..Default::default()
  }),
  |tr| StyleXTransform::new_test_force_runtime_injection_with_pass(
    tr.comments.clone(),
    PluginPass::default(),
    None
  ),
  non_standard_end_aka_inset_inline_end,
  r#"
        import * as stylex from '@stylexjs/stylex';
        export const styles = stylex.create({ x: { end: 5 } });
    "#
);

test!(
  Syntax::Typescript(TsSyntax {
    tsx: true,
    ..Default::default()
  }),
  |tr| StyleXTransform::new_test_force_runtime_injection_with_pass(
    tr.comments.clone(),
    PluginPass::default(),
    None
  ),
  non_standard_margin_end_aka_margin_inline_end,
  r#"
        import * as stylex from '@stylexjs/stylex';
        export const styles = stylex.create({ x: { marginEnd: 0 } });
    "#
);

test!(
  Syntax::Typescript(TsSyntax {
    tsx: true,
    ..Default::default()
  }),
  |tr| StyleXTransform::new_test_force_runtime_injection_with_pass(
    tr.comments.clone(),
    PluginPass::default(),
    None
  ),
  non_standard_margin_horizontal_aka_margin_inline,
  r#"
        import * as stylex from '@stylexjs/stylex';
        export const styles = stylex.create({ x: { marginHorizontal: 0 } });
    "#
);

test!(
  Syntax::Typescript(TsSyntax {
    tsx: true,
    ..Default::default()
  }),
  |tr| StyleXTransform::new_test_force_runtime_injection_with_pass(
    tr.comments.clone(),
    PluginPass::default(),
    None
  ),
  non_standard_margin_start_aka_margin_inline_start,
  r#"
        import * as stylex from '@stylexjs/stylex';
        export const styles = stylex.create({ x: { marginStart: 0 } });
    "#
);

test!(
  Syntax::Typescript(TsSyntax {
    tsx: true,
    ..Default::default()
  }),
  |tr| StyleXTransform::new_test_force_runtime_injection_with_pass(
    tr.comments.clone(),
    PluginPass::default(),
    None
  ),
  non_standard_margin_vertical_aka_margin_block,
  r#"
        import * as stylex from '@stylexjs/stylex';
        export const styles = stylex.create({ x: { marginVertical: 0 } });
    "#
);

test!(
  Syntax::Typescript(TsSyntax {
    tsx: true,
    ..Default::default()
  }),
  |tr| StyleXTransform::new_test_force_runtime_injection_with_pass(
    tr.comments.clone(),
    PluginPass::default(),
    None
  ),
  non_standard_padding_end_aka_padding_inline_end,
  r#"
        import * as stylex from '@stylexjs/stylex';
        export const styles = stylex.create({ x: { paddingEnd: 0 } });
    "#
);

test!(
  Syntax::Typescript(TsSyntax {
    tsx: true,
    ..Default::default()
  }),
  |tr| StyleXTransform::new_test_force_runtime_injection_with_pass(
    tr.comments.clone(),
    PluginPass::default(),
    None
  ),
  non_standard_padding_horizontal_aka_padding_inline,
  r#"
        import * as stylex from '@stylexjs/stylex';
        export const styles = stylex.create({ x: { paddingHorizontal: 0 } });
    "#
);

test!(
  Syntax::Typescript(TsSyntax {
    tsx: true,
    ..Default::default()
  }),
  |tr| StyleXTransform::new_test_force_runtime_injection_with_pass(
    tr.comments.clone(),
    PluginPass::default(),
    None
  ),
  non_standard_padding_start_aka_padding_inline_start,
  r#"
        import * as stylex from '@stylexjs/stylex';
        export const styles = stylex.create({ x: { paddingStart: 0 } });
    "#
);

test!(
  Syntax::Typescript(TsSyntax {
    tsx: true,
    ..Default::default()
  }),
  |tr| StyleXTransform::new_test_force_runtime_injection_with_pass(
    tr.comments.clone(),
    PluginPass::default(),
    None
  ),
  non_standard_padding_vertical_aka_padding_block,
  r#"
        import * as stylex from '@stylexjs/stylex';
        export const styles = stylex.create({ x: { paddingVertical: 0 } });
    "#
);

test!(
  Syntax::Typescript(TsSyntax {
    tsx: true,
    ..Default::default()
  }),
  |tr| StyleXTransform::new_test_force_runtime_injection_with_pass(
    tr.comments.clone(),
    PluginPass::default(),
    None
  ),
  non_standard_start_aka_inset_inline_start,
  r#"
        import * as stylex from '@stylexjs/stylex';
        export const styles = stylex.create({ x: { start: 5 } });
    "#
);
