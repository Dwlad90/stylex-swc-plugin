#[cfg(test)]
mod stylex_exports_tests {
  use stylex_shared::{StyleXTransform, shared::structures::plugin_pass::PluginPass};
  use swc_core::ecma::transforms::testing::test;
  use swc_ecma_parser::{Syntax, TsSyntax};

  test!(
    Syntax::Typescript(TsSyntax {
      tsx: true,
      ..Default::default()
    }),
    |tr| StyleXTransform::new_test_with_pass(tr.comments.clone(), PluginPass::default(), None),
    export_named_property,
    r#"
        import * as stylex from '@stylexjs/stylex';
        const styles = stylex.create({
          root: {
            color: 'red',
          }
        });
        export {styles}
        "#
  );

  test!(
    Syntax::Typescript(TsSyntax {
      tsx: true,
      ..Default::default()
    }),
    |tr| StyleXTransform::new_test_with_pass(tr.comments.clone(), PluginPass::default(), None),
    export_named_declaration,
    r#"
        import * as stylex from '@stylexjs/stylex';
        export const styles = stylex.create({
          root: {
            color: 'red',
          }
        });
        "#
  );

  test!(
    Syntax::Typescript(TsSyntax {
      tsx: true,
      ..Default::default()
    }),
    |tr| StyleXTransform::new_test_with_pass(tr.comments.clone(), PluginPass::default(), None),
    export_default,
    r#"
        import * as stylex from '@stylexjs/stylex';
        export default (stylex.create({
          root: {
            color: 'red',
          }
        }));
        "#
  );

  test!(
    Syntax::Typescript(TsSyntax {
      tsx: true,
      ..Default::default()
    }),
    |tr| StyleXTransform::new_test_with_pass(tr.comments.clone(), PluginPass::default(), None),
    module_export,
    r#"
        import * as stylex from '@stylexjs/stylex';
        const styles = stylex.create({
          root: {
            color: 'red',
          }
        });
        module.export = styles;
        "#
  );
}
