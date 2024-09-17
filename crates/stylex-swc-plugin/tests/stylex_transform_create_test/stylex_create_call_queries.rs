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
  |tr| ModuleTransformVisitor::new_test_force_runtime_injection(
    tr.comments.clone(),
    &PluginPass::default(),
    None
  ),
  transforms_media_queries,
  r#"
        import stylex from 'stylex';
        const styles = stylex.create({
            default: {
                backgroundColor: 'red',
                '@media (min-width: 1000px)': {
                    backgroundColor: 'blue',
                },
                '@media (min-width: 2000px)': {
                    backgroundColor: 'purple',
                },
            },
        });
    "#
);

test!(
  Syntax::Typescript(TsSyntax {
    tsx: true,
    ..Default::default()
  }),
  |tr| ModuleTransformVisitor::new_test_force_runtime_injection(
    tr.comments.clone(),
    &PluginPass::default(),
    None
  ),
  transforms_supports_queries,
  r#"
        import stylex from 'stylex';
        const styles = stylex.create({
            default: {
                backgroundColor: 'red',
                '@supports (hover: hover)': {
                    backgroundColor: 'blue',
                },
                '@supports not (hover: hover)': {
                    backgroundColor: 'purple',
                },
            },
        });
    "#
);
