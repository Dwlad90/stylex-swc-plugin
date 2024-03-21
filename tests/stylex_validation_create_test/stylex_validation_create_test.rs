use stylex_swc_plugin::{shared::structures::plugin_pass::PluginPass, ModuleTransformVisitor};
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
            import stylex from "@stylexjs/stylex";

            stylex.create({});
        "#,
        r#""#,
        false,
    )
}

#[test]
#[should_panic(expected = "stylex.create calls must be bound to a bare variable.")]
fn must_be_called_at_top_level() {
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
            import stylex from "@stylexjs/stylex";

            if (bar) {
                const styles = stylex.create({});
            }
        "#,
        r#""#,
        false,
    )
}

#[test]
#[should_panic(expected = "stylex.create() can only accept a style object.")]
fn its_only_argument_must_be_a_single_object_non_object() {
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
            import stylex from "@stylexjs/stylex";

            const styles = stylex.create(genStyles());
        "#,
        r#""#,
        false,
    )
}

#[test]
#[should_panic(expected = "stylex() should have 1 argument.")]
fn its_only_argument_must_be_a_single_object_argument() {
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
            import stylex from "@stylexjs/stylex";

            const styles = stylex.create();
        "#,
        r#""#,
        false,
    )
}

#[test]
#[should_panic(expected = "stylex() should have 1 argument.")]
fn its_only_argument_must_be_a_single_object_illegal_argument_length() {
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
            import stylex from "@stylexjs/stylex";

            const styles = stylex.create({}, {});
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
    its_only_argument_must_be_a_single_object_correct_argument_length,
    r#"
        import s from "@stylexjs/stylex";

        const c = s.create({
            base: {
                backgroundColor: 'red',
            },
        });
    "#
);

#[test]
#[should_panic(expected = "A stylex namespace must be an object.")]
fn namespace_values_must_be_an_object() {
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
            import stylex from "@stylexjs/stylex";

            const styles = stylex.create({
                namespace: false,
            });
        "#,
        r#""#,
        false,
    )
}

#[test]
#[should_panic(expected = "Only static values are allowed inside of a stylex.create() call.")]
fn namespace_keys_must_be_a_static_value() {
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
            import stylex from "@stylexjs/stylex";

            const styles = stylex.create({
                [root]: {
                    backgroundColor: 'red',
                }
            });
        "#,
        r#""#,
        false,
    )
}

#[test]
#[should_panic(expected = "Only static values are allowed inside of a stylex.create() call.")]
fn properties_must_be_a_static_value() {
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
            import stylex from "@stylexjs/stylex";

            const styles = stylex.create({
                root: {
                    [backgroundColor]: 'red',
                }
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
    namespace_values_can_be_an_empty_object,
    r#"
        import s from "@stylexjs/stylex";

        const c = s.create({
            namespace: {},
        });
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
    values_can_be_static_arrays_of_number_or_string_in_stylex_create,
    r#"
    import stylex from "@stylexjs/stylex";

    const styles = stylex.create({
        root: {
            padding: 5,
        },
        rootRoot: {
            backgroundColor: 'red',
        },
        default: {
            transitionDuration: [500],
        },
        defaultDefault: {
            transitionDuration: ['0.5s', '1s'],
        }
    });
    "#
);

#[test]
#[should_panic(expected = "A style array value can only contain strings or numbers.")]
fn values_must_be_static_arrays_of_number_or_string_in_stylex_create() {
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
            import stylex from "@stylexjs/stylex";

            const styles = stylex.create({
                default: {
                    transitionDuration: [[], {}],
                },
            });
        "#,
        r#""#,
        false,
    )
}

#[test]
#[should_panic(expected = "A style value can only contain an array, string or number.")]
fn values_must_be_static_arrays_of_number_or_string_in_stylex_create_illegal_prop() {
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
            import stylex from "@stylexjs/stylex";

            const styles = stylex.create({
                default: {
                    color: true,
                },
            });
        "#,
        r#""#,
        false,
    )
}

#[test]
#[should_panic(expected = "Only static values are allowed inside of a stylex.create() call.")]
fn values_must_be_static_arrays_of_number_or_string_in_stylex_create_non_static_value_var() {
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
            import stylex from "@stylexjs/stylex";

            const styles = stylex.create({
                default: {
                    backgroundColor: backgroundColor,
                },
            });
        "#,
        r#""#,
        false,
    )
}

#[test]
#[should_panic(expected = "Only static values are allowed inside of a stylex.create() call.")]
fn values_must_be_static_arrays_of_number_or_string_in_stylex_create_non_static_value_fn() {
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
            import stylex from "@stylexjs/stylex";

            const styles = stylex.create({
                default: {
                    backgroundColor: generateBg(),
                },
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
    values_can_reference_local_bindings_in_stylex_create,
    r#"
    import stylex from "@stylexjs/stylex";

    const bgRed = 'red';
    const bgGray = '#eee';

    const styles = stylex.create({
        var:{
            backgroundColor: bgRed,
        },
        varBg:{
            backgroundColor: bgGray,
        }
    });
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
    values_can_be_pure_complex_expressions_in_stylex_create,
    r#"
    import stylex from "@stylexjs/stylex";

    const borderRadius = 2;

    const styles = stylex.create({
        root:{
            borderRadius: borderRadius * 2,
        },
    });
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
    values_can_be_template_literal_expressions_in_stylex_create,
    r#"
    import stylex from "@stylexjs/stylex";

    const borderSize = 2;

    const styles = stylex.create({
        root:{
            borderRadius: `${borderSize * 2}px`,
        },
    });
    "#
);

// test!(
//     Default::default(),
//     |tr| { ModuleTransformVisitor::new_test_styles(tr.comments.clone(), PluginPass::default(), Option::None) },
//     pseudo_classes_must_start_with_colon_character,
//     r#"
//     import stylex from "@stylexjs/stylex";

//     const styles = stylex.create({
//         root: {
//             borderRadius: '2rem',
//             padding: '1rem',
//         },
//     });
//     "#
// );

test!(
    Default::default(),
    |tr| {
        ModuleTransformVisitor::new_test_styles(
            tr.comments.clone(),
            PluginPass::default(),
            Option::None,
        )
    },
    pseudo_classes_must_start_with_colon_character,
    r#"
    import stylex from "@stylexjs/stylex";

    const borderSize = 2;

    const styles = stylex.create({
        default: {
            ':hover': {},
        },
    });
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
    pseudo_classes_invalid_pseudo,
    r#"
        import stylex from "@stylexjs/stylex";

        const styles = stylex.create({
            default: {
                'color': {
                    default: 'black',
                    ':hover': 'blue'
                },
            },
        });
    "#
);

#[test]
#[should_panic(expected = "Invalid pseudo or at-rule.")]
fn pseudo_classes_throw_invalid_pseudo() {
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
            import stylex from "@stylexjs/stylex";

            const styles = stylex.create({
                default: {
                    'color': {
                        default: 'black',
                        '&:hover': 'blue'
                    },
                },
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
    pseudo_classes_cannot_be_nested,
    r#"
        import stylex from "@stylexjs/stylex";

        const styles = stylex.create({
            default: {
                ':hover': {
                    ':active': {},
                },
            },
        });
    "#
);
