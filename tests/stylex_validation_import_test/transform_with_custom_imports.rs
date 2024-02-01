use colored::Style;
use stylex_swc_plugin::{ModuleTransformVisitor, StylexConfig, StylexConfigParams};
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
        let mut config = StylexConfigParams::default();

        config.import_sources = Option::Some(vec!["foo-bar".to_string()]);
        config.runtime_injection = Some(true);

        ModuleTransformVisitor::new_test_styles(tr.comments.clone(), Option::Some(config))
    },
    handles_custom_default_imports,
    r#"
        import stylex from 'foo-bar';

        const styles = stylex.create({
            default: {
                    backgroundColor: 'red',
                    color: 'blue'
                }
            });
    "#
);