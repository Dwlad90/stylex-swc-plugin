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
  |tr| StyleXTransform::new_test_force_runtime_injection_with_pass(
    tr.comments.clone(),
    PluginPass::default(),
    None
  ),
  stylex_create_theme_call,
  r#"
        import stylex from 'stylex';
        export const variables = stylex.createTheme(
            {
                __themeName__: 'TestTheme.stylex.js//buttonTheme',
                bgColor: 'var(--xgck17p)',
            },
            {
                bgColor: {
                    default: {
                        default: 'green',
                        '@supports (color: oklab(0 0 0))': 'oklab(0.7 -0.3 -0.4)',
                    },
                    '@media (prefers-color-scheme: dark)': {
                        default: 'lightgreen',
                        '@supports (color: oklab(0 0 0))': 'oklab(0.7 -0.2 -0.4)',
                    },
                },
            }
        );
    "#
);
