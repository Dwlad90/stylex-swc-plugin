use stylex_swc_plugin::{
  shared::structures::{plugin_pass::PluginPass, stylex_options::StyleXOptionsParams},
  ModuleTransformVisitor,
};
use swc_core::ecma::{
  parser::{Syntax, TsSyntax},
  transforms::testing::test,
};

test!(
  Syntax::Typescript(TsSyntax {
    tsx: true,
    ..Default::default()
  }),
  |tr| ModuleTransformVisitor::new_test(
    tr.comments.clone(),
    &PluginPass::default(),
    Some(&mut StyleXOptionsParams {
      runtime_injection: Some(true),
      ..StyleXOptionsParams::default()
    })
  ),
  keeps_used_styles,
  r#"
      import stylex from 'stylex';
      const styles = stylex.create({
        default: {
          backgroundColor: 'red',
          color: 'blue',
        }
      });
      styles;
    "#
);

test!(
  Syntax::Typescript(TsSyntax {
    tsx: true,
    ..Default::default()
  }),
  |tr| ModuleTransformVisitor::new_test(
    tr.comments.clone(),
    &PluginPass::default(),
    Some(&mut StyleXOptionsParams {
      runtime_injection: Some(true),
      ..StyleXOptionsParams::default()
    })
  ),
  removes_unused_styles,
  r#"
      import stylex from 'stylex';
      const styles = stylex.create({
        default: {
          backgroundColor: 'red',
          color: 'blue',
        }
      });
    "#
);
