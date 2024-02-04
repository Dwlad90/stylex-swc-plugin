use stylex_swc_plugin::ModuleTransformVisitor;
use swc_core::ecma::{
    parser::{Syntax, TsConfig},
    transforms::testing::test,
};

#[test]
#[should_panic(expected = "Only named parameters are allowed in Dynamic Style functions. Destructuring, spreading or default values are not allowed.")]
fn must_be_bound_to_a_variable() {
    swc_core::ecma::transforms::testing::test_transform(
        Syntax::Typescript(TsConfig {
            tsx: true,
            ..Default::default()
        }),
        |tr| ModuleTransformVisitor::new_test_classname(tr.comments.clone(), Option::None),
        r#"
            import stylex from 'stylex';

            const styles = stylex.create({
                dynamic: (props = {}) => ({
                    color: props.color,
                }),
            });
        "#,
        r#""#,
        false,
    )
}
