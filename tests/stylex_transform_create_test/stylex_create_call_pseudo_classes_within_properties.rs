use std::cell::RefCell;
use std::io::Write;
use std::{rc::Rc, sync::Arc};

use stylex_swc_plugin::ModuleTransformVisitor;
// use swc_ecma_codegen::{text_writer::JsWriter, Emitter};

use swc_core::ecma::codegen::text_writer::JsWriter;
use swc_core::ecma::codegen::Emitter;
use swc_core::{
    common::{
        comments::Comments,
        errors::{ColorConfig, Handler},
        FileName, Globals, SourceMap,
    },
    ecma::{
        ast::{EsVersion, Module},
        parser::{lexer::Lexer, Parser, StringInput, Syntax, TsConfig},
        transforms::testing::test,
        visit::FoldWith,
    },
    plugin::proxies::PluginCommentsProxy,
};

use crate::utils::transform::{parse_js, stringify_js};

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
