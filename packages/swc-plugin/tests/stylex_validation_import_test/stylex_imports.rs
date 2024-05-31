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
    ModuleTransformVisitor::new_test_styles(
      tr.comments.clone(),
      &PluginPass::default(),
      None,
    )
  },
  ignore_non_stylex_imports,
  r#"
    import classnames from 'classnames';
    "#
);

test!(
  Syntax::Typescript(TsConfig {
    tsx: true,
    ..Default::default()
  }),
  |tr| {
    ModuleTransformVisitor::new_test_styles(
      tr.comments.clone(),
      &PluginPass::default(),
      None,
    )
  },
  support_named_export_of_stylex_create,
  r#"
    import stylex from 'stylex';
    export const styles = stylex.create({});
    "#
);

test!(
  Syntax::Typescript(TsConfig {
    tsx: true,
    ..Default::default()
  }),
  |tr| {
    ModuleTransformVisitor::new_test_styles(
      tr.comments.clone(),
      &PluginPass::default(),
      None,
    )
  },
  support_default_export_of_stylex_create,
  r#"
    import stylex from 'stylex';
    export default stylex.create({});
    "#
);
