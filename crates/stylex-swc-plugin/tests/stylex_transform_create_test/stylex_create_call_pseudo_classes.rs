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
  |tr| ModuleTransformVisitor::new_test_styles(tr.comments.clone(), &PluginPass::default(), None),
  transforms_invalid_pseudo_class,
  r#"
        import stylex from 'stylex';
        const styles = stylex.create({
            default: {
                ':invalpwdijad': {
                backgroundColor: 'red',
                color: 'blue',
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
  |tr| ModuleTransformVisitor::new_test_styles(tr.comments.clone(), &PluginPass::default(), None),
  transforms_valid_pseudo_classes_in_order,
  r#"
        import stylex from 'stylex';
        const styles = stylex.create({
            default: {
                ':hover': {
                    color: 'blue',
                },
                ':active': {
                    color: 'red',
                },
                ':focus': {
                    color: 'yellow',
                },
                ':nth-child(2n)': {
                    color: 'purple'
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
  |tr| ModuleTransformVisitor::new_test_styles(tr.comments.clone(), &PluginPass::default(), None),
  transforms_pseudo_class_with_array_value_as_fallbacks,
  r#"
        import stylex from 'stylex';
        const styles = stylex.create({
            default: {
                ':hover': {
                    position: ['sticky', 'fixed'],
                }
            },
        });
    "#
);
