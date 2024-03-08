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
                ':invalpwdijad': {
                backgroundColor: 'red',
                color: 'blue',
                },
            },
        });
    "#
);
