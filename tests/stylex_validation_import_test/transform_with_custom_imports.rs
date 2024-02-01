use stylex_swc_plugin::ModuleTransformVisitor;
use swc_core::ecma::{
    parser::{Syntax, TsConfig},
    transforms::testing::test,
};


test!(
    Syntax::Typescript(TsConfig {
        tsx: true,
        ..Default::default()
    }),
    |tr| ModuleTransformVisitor::new_test_styles(tr.comments.clone(), Option::None),
    handles_custom_default_imports,
    r#"
        import stylex from 'foo-bar';

        const styles = stylex.create({
            default: {
                    backgroundColor: 'red',
                    color: 'blue',
                    padding: 5
                }
            });
        styles;
    "#
);