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
  |tr| StyleXTransform::new_test_force_runtime_injection(
    tr.comments.clone(),
    &PluginPass::default(),
    None
  ),
  transforms_before_and_after,
  r#"
        import stylex from 'stylex';
        const styles = stylex.create({
            foo: {
                '::before': {
                    color: 'red'
                },
                '::after': {
                    color: 'blue'
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
  |tr| StyleXTransform::new_test_force_runtime_injection(
    tr.comments.clone(),
    &PluginPass::default(),
    None
  ),
  transforms_placeholder,
  r#"
        import stylex from 'stylex';
        const styles = stylex.create({
            foo: {
                '::placeholder': {
                    color: 'gray',
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
  |tr| StyleXTransform::new_test_force_runtime_injection(
    tr.comments.clone(),
    &PluginPass::default(),
    None
  ),
  transforms_thumb,
  r#"
        import stylex from 'stylex';
        const styles = stylex.create({
            foo: {
                '::thumb': {
                    width: 16,
                },
            },
        });
    "#
);
