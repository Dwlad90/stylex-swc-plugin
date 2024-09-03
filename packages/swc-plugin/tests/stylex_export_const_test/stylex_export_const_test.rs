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
  |tr| { ModuleTransformVisitor::new_test(tr.comments.clone(), &PluginPass::default(), None) },
  stylex_components_is_exported,
  r#"
        import stylex from 'stylex';
        export const styles = stylex.create({
          foo: {
              color: 'red'
          },
        });
        const Component = () => {
            return <div className={styles.foo} />;
        };
        export default Component;
    "#
);
