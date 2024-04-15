use std::collections::HashMap;

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

#[test]
#[should_panic(expected = "stylex.create calls must be bound to a bare variable.")]
fn must_be_bound_to_a_variable() {
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
            stylex.createTheme({__themeName__: 'x568ih9'}, {});
        "#,
        r#""#,
        false,
    )
}

#[test]
#[should_panic(expected = "stylex() should have 1 argument.")]
fn it_must_have_two_arguments_no_args() {
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
            const variables = stylex.createTheme();
        "#,
        r#""#,
        false,
    )
}

#[test]
#[should_panic(expected = "stylex() should have 1 argument.")]
fn it_must_have_two_arguments_one_args() {
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
            const variables = stylex.createTheme({});
        "#,
        r#""#,
        false,
    )
}

#[test]
#[should_panic(expected = "Only static values are allowed inside of a stylex.create() call.")]
fn it_must_have_two_arguments_fn_args() {
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
            const variables = stylex.createTheme(genStyles(),{});
        "#,
        r#""#,
        false,
    )
}

#[test]
#[should_panic(expected = "Can only override variables theme created with stylex.defineVars().")]
fn it_must_have_two_arguments_empty_object_args() {
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
            const variables = stylex.createTheme({},{});
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
    it_must_have_two_arguments_valid,
    r#"
        import stylex from 'stylex';
        export const variables = stylex.createTheme(
            {__themeName__: 'x568ih9', labelColor: 'var(--labelColorHash)'},
            {}
        );
    "#
);

#[test]
#[should_panic(expected = "Only static values are allowed inside of a stylex.create() call.")]
fn variable_keys_must_be_a_static_value() {
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
            const variables = stylex.createTheme(
                {__themeName__: 'x568ih9', labelColor: 'var(--labelColorHash)'},
                {[labelColor]: 'red',}
            );
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
    values_must_be_static_number_or_string_in_stylex_create_theme_v1,
    r#"
        import stylex from 'stylex';
        export const variables = stylex.createTheme(
            {__themeName__: 'x568ih9', cornerRadius: 'var(--cornerRadiusHash)'},
            {cornerRadius: 5,}
        );
    "#
);

test!(
    Default::default(),
    |tr| {
        ModuleTransformVisitor::new_test_styles(
            tr.comments.clone(),
            PluginPass::default(),
            Option::None,
        )
    },
    values_must_be_static_number_or_string_in_stylex_create_theme_v2,
    r#"
        import stylex from 'stylex';
        export const variables = stylex.createTheme(
            {__themeName__: 'x568ih9', labelColor: 'var(--labelColorHash)'},
            {labelColor: 'red',}
        );
    "#
);

#[test]
#[should_panic(expected = "Only static values are allowed inside of a stylex.create() call.")]
fn values_must_be_static_number_or_string_in_stylex_create_theme_var() {
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
            const variables = stylex.createTheme(
                {__themeName__: 'x568ih9', labelColor: 'var(--labelColorHash)'},
                {labelColor: labelColor,}
            );
        "#,
        r#""#,
        false,
    )
}


#[test]
#[should_panic(expected = "Only static values are allowed inside of a stylex.create() call.")]
fn values_must_be_static_number_or_string_in_stylex_create_theme_fn() {
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
            const variables = stylex.createTheme(
                {__themeName__: 'x568ih9', labelColor: 'var(--labelColorHash)'},
                {labelColor: labelColor(),}
            );
        "#,
        r#""#,
        false,
    )
}
