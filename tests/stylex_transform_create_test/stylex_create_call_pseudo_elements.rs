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
    |tr| ModuleTransformVisitor::new_test_styles(tr.comments.clone(), PluginPass::default(), Option::None),
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
    Syntax::Typescript(TsConfig {
        tsx: true,
        ..Default::default()
    }),
    |tr| ModuleTransformVisitor::new_test_styles(tr.comments.clone(), PluginPass::default(), Option::None),
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
    Syntax::Typescript(TsConfig {
        tsx: true,
        ..Default::default()
    }),
    |tr| ModuleTransformVisitor::new_test_styles(tr.comments.clone(), PluginPass::default(), Option::None),
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
