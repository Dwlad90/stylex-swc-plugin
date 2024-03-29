use stylex_swc_plugin::{
    shared::structures::{plugin_pass::PluginPass, stylex_options::StyleXOptionsParams},
    ModuleTransformVisitor,
};
use swc_core::{
    common::FileName,
    ecma::{
        parser::{Syntax, TsConfig},
        transforms::testing::test,
    },
};

test!(
    Syntax::Typescript(TsConfig {
        tsx: true,
        ..Default::default()
    }),
    |tr| ModuleTransformVisitor::new_test(
        tr.comments.clone(),
        PluginPass {
            cwd: Option::None,
            filename: FileName::Real("/stylex/packages/TestTheme.stylex.js".into()),
        },
        Option::None
    ),
    transforms_variables_object,
    r#"
        import stylex from 'stylex';
        export const buttonTheme = stylex.defineVars({
            bgColor: {
                default: 'blue',
                '@media (prefers-color-scheme: dark)': 'lightblue',
                '@media print': 'white',
            },
            bgColorDisabled: {
                default: 'grey',
                '@media (prefers-color-scheme: dark)': 'rgba(0, 0, 0, 0.8)',
            },
            cornerRadius: 10,
            fgColor: {
                default: 'pink',
            },
        });
    "#
);

test!(
    Syntax::Typescript(TsConfig {
        tsx: true,
        ..Default::default()
    }),
    |tr| ModuleTransformVisitor::new_test(
        tr.comments.clone(),
        PluginPass {
            cwd: Option::None,
            filename: FileName::Real("/stylex/packages/TestTheme.stylex.js".into()),
        },
        Option::None
    ),
    transforms_variables_object_with_import_asterisk,
    r#"
        import * as foo from 'stylex';
        export const buttonTheme = foo.defineVars({
            bgColor: {
                default: 'blue',
                '@media (prefers-color-scheme: dark)': 'lightblue',
                '@media print': 'white',
            },
            bgColorDisabled: {
                default: 'grey',
                '@media (prefers-color-scheme: dark)': 'rgba(0, 0, 0, 0.8)',
            },
            cornerRadius: 10,
            fgColor: {
                default: 'pink',
            },
        });
    "#
);

test!(
    Syntax::Typescript(TsConfig {
        tsx: true,
        ..Default::default()
    }),
    |tr| ModuleTransformVisitor::new_test(
        tr.comments.clone(),
        PluginPass {
            cwd: Option::None,
            filename: FileName::Real("/stylex/packages/TestTheme.stylex.js".into()),
        },
        Option::None
    ),
    transforms_variables_object_with_named_import,
    r#"
        import {defineVars} from 'stylex';
        export const buttonTheme = defineVars({
            bgColor: {
                default: 'blue',
                '@media (prefers-color-scheme: dark)': 'lightblue',
                '@media print': 'white',
            },
            bgColorDisabled: {
                default: 'grey',
                '@media (prefers-color-scheme: dark)': 'rgba(0, 0, 0, 0.8)',
            },
            cornerRadius: 10,
            fgColor: {
                default: 'pink',
            },
        });
    "#
);

test!(
    Syntax::Typescript(TsConfig {
        tsx: true,
        ..Default::default()
    }),
    |tr| ModuleTransformVisitor::new_test(
        tr.comments.clone(),
        PluginPass {
            cwd: Option::None,
            filename: FileName::Real("/stylex/packages/TestTheme.stylex.js".into()),
        },
        Option::None
    ),
    transforms_referenced_local_variables_object,
    r#"
        import stylex from 'stylex';
        const defaultButtonTokens = {
            bgColor: {
                default: 'blue',
                '@media (prefers-color-scheme: dark)': 'lightblue',
                '@media print': 'white',
            },
            bgColorDisabled: {
                default: 'grey',
                '@media (prefers-color-scheme: dark)': 'rgba(0, 0, 0, 0.8)',
            },
            cornerRadius: 10,
            fgColor: {
                default: 'pink',
            },
        };
        export const buttonTheme = stylex.defineVars(defaultButtonTokens);
    "#
);

test!(
    Syntax::Typescript(TsConfig {
        tsx: true,
        ..Default::default()
    }),
    |tr| ModuleTransformVisitor::new_test(
        tr.comments.clone(),
        PluginPass {
            cwd: Option::None,
            filename: FileName::Real("/stylex/packages/TestTheme.stylex.js".into()),
        },
        Some(StyleXOptionsParams {
            dev: Some(true),
            ..StyleXOptionsParams::default()
        })
    ),
    transforms_variables_object_and_add_stylex_inject_in_dev_mode,
    r#"
        import stylex from 'stylex';
        export const buttonTheme = stylex.defineVars({
            bgColor: {
                default: 'blue',
                '@media (prefers-color-scheme: dark)': 'lightblue',
                '@media print': 'white',
            },
            bgColorDisabled: {
                default: 'grey',
                '@media (prefers-color-scheme: dark)': 'rgba(0, 0, 0, 0.8)',
            },
            cornerRadius: 10,
            fgColor: {
                default: 'pink',
            },
        });
    "#
);
