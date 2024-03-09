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
    transforms_invalid_pseudo_class,
    r#"
        import stylex from 'stylex';
        const styles = stylex.create({
            default: {
                backgroundColor: {':invalpwdijad': 'red'},
                color: {':invalpwdijad': 'blue'},
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
    transforms_valid_pseudo_classes_in_order,
    r#"
        import stylex from 'stylex';
        const styles = stylex.create({
            default: {
                color: {
                    ':hover': 'blue',
                    ':active':'red',
                    ':focus': 'yellow',
                    ':nth-child(2n)': 'purple',
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
    |tr| ModuleTransformVisitor::new_test_styles(tr.comments.clone(), Option::None),
    transforms_pseudo_class_with_array_value_as_fallbacks,
    r#"
        import stylex from 'stylex';
        const styles = stylex.create({
            default: {
                position: {
                    ':hover': ['sticky', 'fixed'],
                }
            },
        });
    "#
);
