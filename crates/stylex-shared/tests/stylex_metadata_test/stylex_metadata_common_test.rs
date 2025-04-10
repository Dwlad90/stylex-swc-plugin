use stylex_shared::{StyleXTransform, shared::structures::plugin_pass::PluginPass};
use swc_core::ecma::{
  parser::{Syntax, TsSyntax},
  transforms::testing::test,
};

test!(
  Syntax::Typescript(TsSyntax {
    tsx: true,
    ..Default::default()
  }),
  |tr| { StyleXTransform::new_test_with_pass(tr.comments.clone(), PluginPass::default(), None) },
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
