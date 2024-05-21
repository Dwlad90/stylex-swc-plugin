use stylex_swc_plugin::{shared::structures::plugin_pass::PluginPass, ModuleTransformVisitor};
use swc_core::ecma::{
  parser::{Syntax, TsConfig},
  transforms::testing::test,
};

test!(
  Syntax::Typescript(TsConfig {
    tsx: true,
    ..Default::default()
  }),
  |tr| {
    ModuleTransformVisitor::new_test(tr.comments.clone(), &PluginPass::default(), Option::None)
  },
  stylex_metadata_is_correctly_set,
  r#"
        import stylex from 'stylex';
        export const styles = stylex.create({
          foo: {
              color: 'red',
              height: 5,
              ':hover': {
                  start: 10,
              },
              '@media (min-width: 1000px)': {
                  end: 5
              }
          },
        });

        export const name = stylex.keyframes({
            from: {
                start: 0,
            },
            to: {
                start: 100,
            }
        });
    "#
);
