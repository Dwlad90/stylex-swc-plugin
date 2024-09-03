use stylex_swc_plugin::{shared::structures::plugin_pass::PluginPass, ModuleTransformVisitor};
use swc_core::{
  common::FileName,
  ecma::{
    parser::{Syntax, TsSyntax},
    transforms::testing::test,
  },
};

test!(
  Syntax::Typescript(TsSyntax {
    tsx: true,
    ..Default::default()
  }),
  |tr| {
    ModuleTransformVisitor::new_test(
      tr.comments.clone(),
      &PluginPass {
        cwd: None,
        filename: FileName::Real("/stylex/packages/TestTheme.stylex.js".into()),
      },
      None,
    )
  },
  stylex_number_property_is_supported,
  r#"
        import stylex from 'stylex';
        export const vars = stylex.defineVars({
          xl: 4,
          "2xl": 8,
        });
    "#
);
