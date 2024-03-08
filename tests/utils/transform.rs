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

pub(crate) fn parse_js(source_code: &str) -> Module {
    let cm: Arc<SourceMap> = Default::default();
    let handler = Handler::with_tty_emitter(ColorConfig::Auto, true, false, Some(cm.clone()));

    // This is the JavaScript code you want to parse.
    let fm = cm.new_source_file(FileName::Custom("input.js".into()), source_code.into());

    let lexer = Lexer::new(
        Syntax::default(),
        EsVersion::EsNext,
        StringInput::from(&*fm),
        None,
    );

    let mut parser = Parser::new_from(lexer);

    match parser.parse_module() {
        Ok(module) => {
            // Do something with the parsed module.
            module.fold_with(&mut ModuleTransformVisitor::new_test_styles(
                PluginCommentsProxy,
                Option::None,
            ))
        }
        Err(err) => {
            handler
                .struct_err(format!("An error occurred: {:#?}", err).as_str())
                .emit();

            panic!("{:#?}", err)
        }
    }
}

fn module_to_string(module: &Module, cm: Arc<SourceMap>) -> String {
    let mut buf = Vec::new();
    {
        let mut emitter = Emitter {
            cfg: Default::default(),
            comments: None,
            cm: cm.clone(),
            wr: Box::new(JsWriter::new(cm.clone(), "\n    ", &mut buf, None)),
        };
        emitter.emit_module(&module).unwrap();
    }
    String::from_utf8(buf).unwrap()
}

pub(crate) fn stringify_js(module: &Module) -> String {
    let cm: Arc<SourceMap> = Default::default();

    format!("{}", module_to_string(module, cm))
}
