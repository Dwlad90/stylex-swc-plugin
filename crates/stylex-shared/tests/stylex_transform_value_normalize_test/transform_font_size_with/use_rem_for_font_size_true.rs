use stylex_shared::{
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
      use_rem_for_font_size: Some(true),
      ..StyleXOptionsParams::default()
    })
  ),
  transforms_font_size_from_px_to_rem,
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
      use_rem_for_font_size: Some(true),
      ..StyleXOptionsParams::default()
    })
  ),
  transforms_font_size_from_px_to_rem_even_with_calc,
  r#"
      import stylex from 'stylex';
      const styles = stylex.create({
        foo: {
          fontSize: 'calc(100% - 24px)',
        },
      });
    "#
);
