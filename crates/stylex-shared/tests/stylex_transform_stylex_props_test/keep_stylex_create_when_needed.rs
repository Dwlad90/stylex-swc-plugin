use stylex_shared::{shared::structures::plugin_pass::PluginPass, ModuleTransformVisitor};
use swc_core::ecma::{
  parser::{Syntax, TsSyntax},
  transforms::testing::test,
};

test!(
  Syntax::Typescript(TsSyntax {
    tsx: true,
    ..Default::default()
  }),
  |tr| ModuleTransformVisitor::new_test_force_runtime_injection(
    tr.comments.clone(),
    &PluginPass::default(),
    None
  ),
  stylex_call_with_computed_key_access,
  r#"
        import stylex from 'stylex';
        const styles = stylex.create({
            [0]: {
                color: 'red',
            },
            [1]: {
                backgroundColor: 'blue',
            }
        });
        stylex.props(styles[variant]);
    "#
);

test!(
  Syntax::Typescript(TsSyntax {
    tsx: true,
    ..Default::default()
  }),
  |tr| ModuleTransformVisitor::new_test_force_runtime_injection(
    tr.comments.clone(),
    &PluginPass::default(),
    None
  ),
  stylex_call_with_composition_of_external_styles,
  r#"
        import stylex from 'stylex';
        const styles = stylex.create({
            default: {
                color: 'red',
            },
        });
        stylex.props([styles.default, props]);
    "#
);

test!(
  Syntax::Typescript(TsSyntax {
    tsx: true,
    ..Default::default()
  }),
  |tr| ModuleTransformVisitor::new_test_force_runtime_injection(
    tr.comments.clone(),
    &PluginPass::default(),
    None
  ),
  stylex_call_using_exported_styles_with_pseudo_selectors_and_queries,
  r#"
        import stylex from 'stylex';
        export const styles = stylex.create({
        default: {
            ':hover': {
                color: 'blue',
            },
            '@media (min-width: 1000px)': {
                backgroundColor: 'blue',
            },
        }
        });
        stylex.props(styles.default);
    "#
);
