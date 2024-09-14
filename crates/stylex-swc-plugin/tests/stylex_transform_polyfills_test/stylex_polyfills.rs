use stylex_swc_plugin::{shared::structures::plugin_pass::PluginPass, ModuleTransformVisitor};
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
    ModuleTransformVisitor::new_test_styles(tr.comments.clone(), &PluginPass::default(), None)
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
    ModuleTransformVisitor::new_test_styles(tr.comments.clone(), &PluginPass::default(), None)
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
    ModuleTransformVisitor::new_test_styles(tr.comments.clone(), &PluginPass::default(), None)
  },
  scrollbar_width,
  r#"
    import stylex from 'stylex';
    const styles = stylex.create({ x: { scrollbarWidth: 'none' } });
  "#
);
