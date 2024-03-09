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
    Syntax::Typescript(TsConfig {
        tsx: true,
        ..Default::default()
    }),
    |tr| ModuleTransformVisitor::new_test_styles(tr.comments.clone(), Option::None),
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
