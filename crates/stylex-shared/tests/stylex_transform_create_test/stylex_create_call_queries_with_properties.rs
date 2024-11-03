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
    PluginPass::default(),
    None
  ),
  transforms_media_queries,
  r#"
        import stylex from 'stylex';
        const styles = stylex.create({
            default: {
                backgroundColor: {
                    default: 'red',
                    '@media (min-width: 1000px)': 'blue',
                    '@media (min-width: 2000px)': 'purple',
                }
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
    PluginPass::default(),
    None
  ),
  transforms_supports_queries,
  r#"
        import stylex from 'stylex';
        const styles = stylex.create({
            default: {
                backgroundColor: {
                default:'red',
                    '@supports (hover: hover)': 'blue',
                    '@supports not (hover: hover)': 'purple',
                }
            },
        });
    "#
);
