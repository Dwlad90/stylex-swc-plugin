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
  |tr| StyleXTransform::new_test_force_runtime_injection_with_pass(
    tr.comments.clone(),
    PluginPass::default(),
    None
  ),
  preserves_kebab_case_in_css_variable_names,
  r#"
        import stylex from 'stylex';
        export const styles = stylex.create({
          default: {
            '--background-color': 'red',
          },
        });
    "#
);

test!(
  Syntax::Typescript(TsSyntax {
    tsx: true,
    ..Default::default()
  }),
  |tr| StyleXTransform::new_test_force_runtime_injection_with_pass(
    tr.comments.clone(),
    PluginPass::default(),
    None
  ),
  preserves_camel_case_in_css_variable_names,
  r#"
        import stylex from 'stylex';
        const styles = stylex.create({
          default: {
            '--myCustomVar': 'red',
            '--anotherCamelVar': '10px',
          },
        });
    "#
);
