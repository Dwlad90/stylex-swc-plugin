use std::sync::Arc;

use stylex_swc_plugin::shared::structures::plugin_pass::PluginPass;
use stylex_swc_plugin::ModuleTransformVisitor;
// use swc_ecma_codegen::{text_writer::JsWriter, Emitter};

use swc_core::common::{chain, DUMMY_SP};
use swc_core::ecma::ast::{
  CallExpr, Decl, Expr, ImportSpecifier, ModuleDecl, ModuleItem, Stmt, VarDecl, VarDeclKind,
  VarDeclarator,
};


use swc_core::ecma::transforms::base::{fixer, hygiene};
use swc_core::ecma::transforms::testing::{HygieneVisualizer, Tester};
use swc_core::ecma::utils::{quote_ident, quote_str, DropSpan, ExprFactory};
use swc_core::ecma::visit::{as_folder, noop_visit_mut_type, Fold, VisitMut};
use swc_core::{
  common::{
    errors::{ColorConfig, Handler},
    FileName, SourceMap,
  },
  ecma::{
    ast::{EsVersion, Module},
    parser::{lexer::Lexer, Parser, StringInput, Syntax},
    visit::FoldWith,
  },
  plugin::proxies::PluginCommentsProxy,
};

pub(crate) fn _parse_js(source_code: &str) -> Module {
  if std::env::var("INSTA_UPDATE").is_err() {
    std::env::set_var("INSTA_UPDATE", "no");
  }

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
        PluginPass::default(),
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

// fn module_to_string(module: &Module, cm: Arc<SourceMap>) -> String {
//     Tester::print(&mut self, module, comments)
//     let mut buf = Vec::new();
//     {
//         let mut emitter = Emitter {
//             cfg: Default::default(),
//             cm: cm.clone(),
//             wr: Box::new(JsWriter::new(cm.clone(), "\n", &mut buf, None)),
//             comments: Option::None,
//         };

//         emitter.emit_module(module).unwrap();
//     }
//     let s = String::from_utf8_lossy(&buf);
//     s.to_string()
// }

struct RegeneratorHandler;

impl VisitMut for RegeneratorHandler {
  noop_visit_mut_type!();

  fn visit_mut_module_item(&mut self, item: &mut ModuleItem) {
    if let ModuleItem::ModuleDecl(ModuleDecl::Import(import)) = item {
      if &*import.src.value != "regenerator-runtime" {
        return;
      }

      let s = import.specifiers.iter().find_map(|v| match v {
        ImportSpecifier::Default(rt) => Some(rt.local.clone()),
        _ => None,
      });

      let s = match s {
        Some(v) => v,
        _ => return,
      };

      let init = Box::new(Expr::Call(CallExpr {
        span: DUMMY_SP,
        callee: quote_ident!("require").as_callee(),
        args: vec![quote_str!("regenerator-runtime").as_arg()],
        type_args: Default::default(),
      }));

      let decl = VarDeclarator {
        span: DUMMY_SP,
        name: s.into(),
        init: Some(init),
        definite: Default::default(),
      };
      *item = ModuleItem::Stmt(Stmt::Decl(Decl::Var(Box::new(VarDecl {
        span: import.span,
        kind: VarDeclKind::Var,
        declare: false,
        decls: vec![decl],
      }))))
    }
  }
}

fn make_tr<F, P>(op: F, tester: &mut Tester<'_>) -> impl Fold
where
  F: FnOnce(&mut Tester<'_>) -> P,
  P: Fold,
{
  chain!(op(tester), as_folder(RegeneratorHandler))
}

pub(crate) fn stringify_js<F, P>(input: &str, syntax: Syntax, tr: F) -> String
where
  F: FnOnce(&mut Tester) -> P,
  P: Fold,
{
  let res = Tester::run(|tester| {
    let tr = make_tr(tr, tester);

    let actual = tester.apply_transform(tr, "input.js", syntax, input)?;

    match ::std::env::var("PRINT_HYGIENE") {
      Ok(ref s) if s == "1" => {
        let hygiene_src = tester.print(
          &actual.clone().fold_with(&mut HygieneVisualizer),
          &tester.comments.clone(),
        );
        println!("----- Hygiene -----\n{}", hygiene_src);
      }
      _ => {}
    }

    let actual = actual
      .fold_with(&mut as_folder(DropSpan {
        preserve_ctxt: true,
      }))
      .fold_with(&mut hygiene::hygiene())
      .fold_with(&mut fixer::fixer(Some(&tester.comments)));

    Result::Ok(tester.print(&actual, &tester.comments.clone()))
  });

  res

  // let cm: Arc<SourceMap> = Default::default();

  // let tester = Tester {
  //     cm,
  //     handler: todo!(),
  //     comments: Option::None,
  // };

  // tester.print(module, comments)
}
