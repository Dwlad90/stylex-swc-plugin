use stylex_shared::{shared::structures::plugin_pass::PluginPass, StyleXTransform};
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
    StyleXTransform::new_test_force_runtime_injection(
      tr.comments.clone(),
      PluginPass::default(),
      None,
    )
  },
  ignore_non_stylex_imports,
  r#"
    import classnames from 'classnames';
    "#
);

test!(
  Syntax::Typescript(TsSyntax {
    tsx: true,
    ..Default::default()
  }),
  |tr| {
    StyleXTransform::new_test_force_runtime_injection(
      tr.comments.clone(),
      PluginPass::default(),
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
  Syntax::Typescript(TsSyntax {
    tsx: true,
    ..Default::default()
  }),
  |tr| {
    StyleXTransform::new_test_force_runtime_injection(
      tr.comments.clone(),
      PluginPass::default(),
      None,
    )
  },
  support_default_export_of_stylex_create,
  r#"
    import stylex from 'stylex';
    export default stylex.create({});
    "#
);
