use stylex_swc_plugin::{shared::structures::plugin_pass::PluginPass, ModuleTransformVisitor};
use swc_core::ecma::{
    parser::{Syntax, TsConfig},
    transforms::testing::test,
};

#[test]
#[should_panic(
    expected = "Only named parameters are allowed in Dynamic Style functions. Destructuring, spreading or default values are not allowed."
)]
fn dynamic_style_function_only_accepts_named_parameters_default_value() {
    swc_core::ecma::transforms::testing::test_transform(
        Syntax::Typescript(TsConfig {
            tsx: true,
            ..Default::default()
        }),
        |tr| {
            ModuleTransformVisitor::new_test_styles(
                tr.comments.clone(),
                PluginPass::default(),
                Option::None,
            )
        },
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

#[test]
#[should_panic(
    expected = "Only named parameters are allowed in Dynamic Style functions. Destructuring, spreading or default values are not allowed."
)]
fn dynamic_style_function_only_accepts_named_parameters_default_string_value() {
    swc_core::ecma::transforms::testing::test_transform(
        Syntax::Typescript(TsConfig {
            tsx: true,
            ..Default::default()
        }),
        |tr| {
            ModuleTransformVisitor::new_test_styles(
                tr.comments.clone(),
                PluginPass::default(),
                Option::None,
            )
        },
        r#"
            import stylex from 'stylex';
            const styles = stylex.create({
                dynamic: (color = 'red') => ({
                    color,
                }),
            });
        "#,
        r#""#,
        false,
    )
}

#[test]
#[should_panic(
    expected = "Only named parameters are allowed in Dynamic Style functions. Destructuring, spreading or default values are not allowed."
)]
fn dynamic_style_function_only_accepts_named_parameters_object_arg() {
    swc_core::ecma::transforms::testing::test_transform(
        Syntax::Typescript(TsConfig {
            tsx: true,
            ..Default::default()
        }),
        |tr| {
            ModuleTransformVisitor::new_test_styles(
                tr.comments.clone(),
                PluginPass::default(),
                Option::None,
            )
        },
        r#"
            import stylex from 'stylex';
            const styles = stylex.create({
                dynamic: ({ color }) => ({
                    color,
                }),
            });
        "#,
        r#""#,
        false,
    )
}

#[test]
#[should_panic(
    expected = "Only named parameters are allowed in Dynamic Style functions. Destructuring, spreading or default values are not allowed."
)]
fn dynamic_style_function_only_accepts_named_parameters_rest_arg() {
    swc_core::ecma::transforms::testing::test_transform(
        Syntax::Typescript(TsConfig {
            tsx: true,
            ..Default::default()
        }),
        |tr| {
            ModuleTransformVisitor::new_test_styles(
                tr.comments.clone(),
                PluginPass::default(),
                Option::None,
            )
        },
        r#"
            import stylex from 'stylex';
            const styles = stylex.create({
                dynamic: (...rest) => ({
                    color,
                }),
            });
        "#,
        r#""#,
        false,
    )
}

test!(
    Default::default(),
    |tr| {
        ModuleTransformVisitor::new_test_styles(
            tr.comments.clone(),
            PluginPass::default(),
            Option::None,
        )
    },
    dynamic_style_function_only_accepts_named_parameters_valid,
    r#"
    import stylex from "@stylexjs/stylex";
    const styles = stylex.create({
        dynamic: (backgroundColor) => ({
            backgroundColor,
        }),
    });
    "#
);
