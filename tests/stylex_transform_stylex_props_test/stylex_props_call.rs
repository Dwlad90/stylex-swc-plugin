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
    |tr| ModuleTransformVisitor::new_test_styles(
        tr.comments.clone(),
        PluginPass::default(),
        Option::None
    ),
    empty_stylex_props_call,
    r#"
        import stylex from 'stylex';
        stylex.props();
    "#
);

test!(
    Syntax::Typescript(TsConfig {
        tsx: true,
        ..Default::default()
    }),
    |tr| ModuleTransformVisitor::new_test_styles(
        tr.comments.clone(),
        PluginPass::default(),
        Option::None
    ),
    basic_stylex_call,
    r#"
        import stylex from 'stylex';
        const styles = stylex.create({
            red: {
                color: 'red',
            }
        });
        stylex.props(styles.red);
    "#
);

test!(
    Syntax::Typescript(TsConfig {
        tsx: true,
        ..Default::default()
    }),
    |tr| ModuleTransformVisitor::new_test_styles(
        tr.comments.clone(),
        PluginPass::default(),
        Option::None
    ),
    stylex_call_with_number,
    r#"
        import stylex from 'stylex';
        const styles = stylex.create({
            0: {
                color: 'red',
            },
            1: {
                backgroundColor: 'blue',
            }
        });
        stylex.props([styles[0], styles[1]]);
    "#
);

test!(
    Syntax::Typescript(TsConfig {
        tsx: true,
        ..Default::default()
    }),
    |tr| ModuleTransformVisitor::new_test_styles(
        tr.comments.clone(),
        PluginPass::default(),
        Option::None
    ),
    stylex_call_with_computed_number,
    r#"
        import stylex from 'stylex';
        const styles = stylex.create({
            [0]: {
                color: 'red',
            },
            [1]: {
                backgroundColor: 'blue',
            }
        });
        stylex.props([styles[0], styles[1]]);
    "#
);

test!(
    Syntax::Typescript(TsConfig {
        tsx: true,
        ..Default::default()
    }),
    |tr| ModuleTransformVisitor::new_test_styles(
        tr.comments.clone(),
        PluginPass::default(),
        Option::None
    ),
    stylex_call_with_computed_string,
    r#"
        import stylex from 'stylex';
        const styles = stylex.create({
            'default': {
                color: 'red',
            }
        });
        stylex.props(styles['default']);
    "#
);

test!(
    Syntax::Typescript(TsConfig {
        tsx: true,
        ..Default::default()
    }),
    |tr| ModuleTransformVisitor::new_test_styles(
        tr.comments.clone(),
        PluginPass::default(),
        Option::None
    ),
    stylex_call_with_multiple_namespaces,
    r#"
        import {create, props} from 'stylex';
        const styles = create({
            default: {
                color: 'red',
            },
        });
        const otherStyles = create({
            default: {
                backgroundColor: 'blue',
            }
        });
        props([styles.default, otherStyles.default]);
    "#
);

test!(
    Syntax::Typescript(TsConfig {
        tsx: true,
        ..Default::default()
    }),
    |tr| ModuleTransformVisitor::new_test_styles(
        tr.comments.clone(),
        PluginPass::default(),
        Option::None
    ),
    stylex_call_within_variable_declarations,
    r#"
        import stylex from 'stylex';
        const styles = stylex.create({
            foo: { color: 'red' }
        });
        export const a = function() {
            return stylex.props(styles.foo);
        }
    "#
);

test!(
    Syntax::Typescript(TsConfig {
        tsx: true,
        ..Default::default()
    }),
    |tr| ModuleTransformVisitor::new_test_styles(
        tr.comments.clone(),
        PluginPass::default(),
        Option::None
    ),
    stylex_call_with_styles_variable_assignment,
    r#"
        import stylex from 'stylex';
        const styles = stylex.create({
            foo: {
                color: 'red',
            },
            bar: {
                backgroundColor: 'blue',
            }
        });
        stylex.props([styles.foo, styles.bar]);
        export const foo = styles;
    "#
);

test!(
    Syntax::Typescript(TsConfig {
        tsx: true,
        ..Default::default()
    }),
    |tr| ModuleTransformVisitor::new_test_styles(
        tr.comments.clone(),
        PluginPass::default(),
        Option::None
    ),
    stylex_call_within_export_declarations,
    r#"
        import stylex from 'stylex';
        const styles = stylex.create({
            foo: { color: 'red' }
        });
        export default function MyExportDefault() {
            return stylex.props(styles.foo);
        }
        export function MyExport() {
            return stylex.props(styles.foo);
        }
    "#
);

test!(
    Syntax::Typescript(TsConfig {
        tsx: true,
        ..Default::default()
    }),
    |tr| ModuleTransformVisitor::new_test_styles(
        tr.comments.clone(),
        PluginPass::default(),
        Option::None
    ),
    stylex_call_with_short_form_properties,
    r#"
        import stylex from 'stylex';
        const styles = stylex.create({
            foo: {
                padding: 5
            }
        });
        stylex.props(styles.foo);
    "#
);

test!(
    Syntax::Typescript(TsConfig {
        tsx: true,
        ..Default::default()
    }),
    |tr| ModuleTransformVisitor::new_test_styles(
        tr.comments.clone(),
        PluginPass::default(),
        Option::None
    ),
    stylex_call_with_exported_short_form_properties,
    r#"
        import stylex from 'stylex';
        export const styles = stylex.create({
            foo: {
                padding: 5
            }
        });
        stylex.props([styles.foo]);
    "#
);

test!(
    Syntax::Typescript(TsConfig {
        tsx: true,
        ..Default::default()
    }),
    |tr| ModuleTransformVisitor::new_test_styles(
        tr.comments.clone(),
        PluginPass::default(),
        Option::None
    ),
    stylex_call_using_styles_with_pseudo_selectors,
    r#"
        import stylex from 'stylex';
        const styles = stylex.create({
        default: {
            color: 'red',
            ':hover': {
                color: 'blue',
            }
        }
        });
        stylex.props(styles.default);
    "#
);

test!(
    Syntax::Typescript(TsConfig {
        tsx: true,
        ..Default::default()
    }),
    |tr| ModuleTransformVisitor::new_test_styles(
        tr.comments.clone(),
        PluginPass::default(),
        Option::None
    ),
    stylex_call_using_styles_with_pseudo_selectors_within_property,
    r#"
        import * as stylex from 'stylex';
        const styles = stylex.create({
            default: {
                color: {
                    default: 'red',
                    ':hover': 'blue',
                }
            }
        });
        stylex.props(styles.default);
    "#
);

test!(
    Syntax::Typescript(TsConfig {
        tsx: true,
        ..Default::default()
    }),
    |tr| ModuleTransformVisitor::new_test_styles(
        tr.comments.clone(),
        PluginPass::default(),
        Option::None
    ),
    stylex_call_using_styles_with_media_queries,
    r#"
        import stylex from 'stylex';
        const styles = stylex.create({
            default: {
                backgroundColor: 'red',
                '@media (min-width: 1000px)': {
                    backgroundColor: 'blue',
                },
                '@media (min-width: 2000px)': {
                    backgroundColor: 'purple',
                },
            },
        });
        stylex.props(styles.default);
    "#
);

test!(
    Syntax::Typescript(TsConfig {
        tsx: true,
        ..Default::default()
    }),
    |tr| ModuleTransformVisitor::new_test_styles(
        tr.comments.clone(),
        PluginPass::default(),
        Option::None
    ),
    stylex_call_using_styles_with_media_queries_within_property,
    r#"
        import stylex from 'stylex';
        const styles = stylex.create({
            default: {
                backgroundColor: {
                    default:'red',
                    '@media (min-width: 1000px)': 'blue',
                    '@media (min-width: 2000px)': 'purple',
                },
            },
        });
        stylex.props(styles.default);
    "#
);

test!(
    Syntax::Typescript(TsConfig {
        tsx: true,
        ..Default::default()
    }),
    |tr| ModuleTransformVisitor::new_test_styles(
        tr.comments.clone(),
        PluginPass::default(),
        Option::None
    ),
    stylex_call_using_styles_with_support_queries,
    r#"
        import stylex from 'stylex';
        const styles = stylex.create({
            default: {
                backgroundColor: 'red',
                '@supports (hover: hover)': {
                    backgroundColor: 'blue',
                },
                '@supports not (hover: hover)': {
                    backgroundColor: 'purple',
                },
            },
        });
        stylex.props(styles.default);
    "#
);

test!(
    Syntax::Typescript(TsConfig {
        tsx: true,
        ..Default::default()
    }),
    |tr| ModuleTransformVisitor::new_test_styles(
        tr.comments.clone(),
        PluginPass::default(),
        Option::None
    ),
    stylex_call_using_styles_with_support_queries_within_property,
    r#"
        import stylex from 'stylex';
        const styles = stylex.create({
            default: {
                backgroundColor: {
                    default:'red',
                    '@supports (hover: hover)': 'blue',
                    '@supports not (hover: hover)': 'purple',
                },
            },
        });
        stylex.props(styles.default);
    "#
);
