use stylex_swc_plugin::{
    shared::structures::{
        named_import_source::RuntimeInjection, plugin_pass::PluginPass,
        stylex_options::StyleXOptionsParams,
    },
    ModuleTransformVisitor,
};
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
        let mut config = StyleXOptionsParams::default();

        config.runtime_injection = Option::Some(RuntimeInjection::Boolean(false));

        ModuleTransformVisitor::new_test_styles(
            tr.comments.clone(),
            PluginPass::default(),
            Option::Some(config),
        )
    },
    stylex_metadata_is_correctly_set,
    r#"
        import stylex from 'stylex';
        const styles = stylex.create({
        foo: {
            color: 'red',
            height: 5,
            ':hover': {
                start: 10,
            },
            '@media (min-width: 1000px)': {
                end: 5
            }
        },
        });

        const name = stylex.keyframes({
            from: {
                start: 0,
            },
            to: {
                start: 100,
            }
        });
    "#
);
