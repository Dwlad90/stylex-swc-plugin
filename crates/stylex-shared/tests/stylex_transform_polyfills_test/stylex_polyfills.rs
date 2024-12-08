use stylex_shared::{shared::structures::plugin_pass::PluginPass, StyleXTransform};
use swc_core::ecma::{
  parser::{Syntax, TsSyntax},
  transforms::testing::test,
};

test!(
  Syntax::Typescript(TsSyntax {
    tsx: true,
    ..Default::default()
  }),
  |tr| {
    StyleXTransform::new_test_force_runtime_injection_with_pass(
      tr.comments.clone(),
      PluginPass::default(),
      None,
    )
  },
  line_clamp,
  r#"
    import stylex from 'stylex';
    const styles = stylex.create({ x: { lineClamp: 3 } });
  "#
);

test!(
  Syntax::Typescript(TsSyntax {
    tsx: true,
    ..Default::default()
  }),
  |tr| {
    StyleXTransform::new_test_force_runtime_injection_with_pass(
      tr.comments.clone(),
      PluginPass::default(),
      None,
    )
  },
  pointer_events,
  r#"
    import stylex from 'stylex';
    const styles = stylex.create({
      a: { pointerEvents: 'auto' },
      b: { pointerEvents: 'box-none' },
      c: { pointerEvents: 'box-only' },
      d: { pointerEvents: 'none' }
    });
  "#
);

test!(
  Syntax::Typescript(TsSyntax {
    tsx: true,
    ..Default::default()
  }),
  |tr| {
    StyleXTransform::new_test_force_runtime_injection_with_pass(
      tr.comments.clone(),
      PluginPass::default(),
      None,
    )
  },
  scrollbar_width,
  r#"
    import stylex from 'stylex';
    const styles = stylex.create({ x: { scrollbarWidth: 'none' } });
  "#
);
