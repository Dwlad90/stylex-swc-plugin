use std::{rc::Rc, sync::Arc};

use stylex_transform::StyleXTransform;

use swc_core::{
  common::{DUMMY_SP, SyntaxContext, comments::SingleThreadedComments},
  ecma::{
    ast::{
      CallExpr, Decl, Expr, ImportSpecifier, ModuleDecl, ModuleItem, Pass, Stmt, VarDecl,
      VarDeclKind, VarDeclarator,
    },
    visit::visit_mut_pass,
  },
};

use swc_core::{
  common::{
    FileName, SourceMap,
    errors::{ColorConfig, Handler},
  },
  ecma::{
    ast::{EsVersion, Module},
    parser::{Parser, StringInput, Syntax, lexer::Lexer},
    transforms::{
      base::{fixer, hygiene},
      testing::{HygieneVisualizer, Tester},
    },
    utils::{DropSpan, ExprFactory, quote_ident, quote_str},
    visit::{FoldWith, VisitMut, VisitMutWith, noop_visit_mut_type},
  },
};
use swc_ecma_parser::TsSyntax;

pub(crate) fn _parse_js(source_code: &str) -> Module {
  if std::env::var("INSTA_UPDATE").is_err() {
    unsafe { std::env::set_var("INSTA_UPDATE", "no") };
  }

  let cm: Arc<SourceMap> = Default::default();
  let handler = Handler::with_tty_emitter(ColorConfig::Auto, true, false, Some(cm.clone()));

  let file_name = Arc::new(FileName::Custom("input.js".into()));

  // This is the JavaScript code you want to parse.
  let fm = cm.new_source_file(file_name, source_code.to_string());

  let lexer = Lexer::new(
    Syntax::default(),
    EsVersion::EsNext,
    StringInput::from(&*fm),
    None,
  );

  let mut parser = Parser::new_from(lexer);

  match parser.parse_module() {
    Ok(mut module) => {
      module.visit_mut_with(
        &mut StyleXTransform::test(Rc::new(SingleThreadedComments::default()))
          .with_runtime_injection()
          .build(),
      );
      module
    },
    Err(err) => {
      handler
        .struct_err(format!("An error occurred: {:#?}", err).as_str())
        .emit();

      panic!("{:#?}", err)
    },
  }
}

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
        ctxt: SyntaxContext::empty(),
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
        ctxt: SyntaxContext::empty(),
      }))))
    }
  }
}

#[allow(dead_code)]
pub(crate) fn stringify_js<F, P>(input: &str, syntax: Syntax, tr: F) -> String
where
  F: for<'a> FnOnce(&mut Tester<'a>) -> P,
  P: Pass,
{
  Tester::run(|tester| {
    let tr = (tr(tester), visit_mut_pass(RegeneratorHandler));
    let actual = tester.apply_transform(tr, "input.js", syntax, Option::None, input)?;

    match ::std::env::var("PRINT_HYGIENE") {
      Ok(ref s) if s == "1" => {
        let hygiene_src = tester.print(
          &actual.clone().fold_with(&mut HygieneVisualizer),
          &tester.comments.clone(),
        );
        println!("----- Hygiene -----\n{}", hygiene_src);
      },
      _ => {},
    }

    let actual = actual
      .apply(DropSpan)
      .apply(&mut hygiene::hygiene())
      .apply(&mut fixer::fixer(Some(&tester.comments)));

    let actual_str = tester.print(&actual, &tester.comments.clone());

    Result::Ok(actual_str)
  })
}

pub(crate) fn ts_syntax() -> Syntax {
  Syntax::Typescript(TsSyntax {
    tsx: true,
    ..Default::default()
  })
}

use stylex_transform::StyleXTransformBuilder;

/// Type alias for test comments (used in file-level transform functions).
pub(crate) type TestComments = Rc<SingleThreadedComments>;

/// Type alias for the builder with standard test comments.
pub(crate) type TestBuilder = StyleXTransformBuilder<TestComments>;

/// Creates a test transform with the given customizations applied.
///
/// Takes `comments` (from `tr.comments.clone()`) and a closure that
/// customizes the builder before finalization.
///
/// # Examples
/// ```ignore
/// // In a file-level function:
/// fn my_transform(tr: &Tester) -> impl Pass {
///   build_test_transform(tr.comments.clone(), |b| b.with_dev(true).with_runtime_injection())
/// }
///
/// // Inline in a macro:
/// stylex_test!(name, |tr| build_test_transform(tr.comments.clone(), |b| b), code);
/// ```
#[allow(dead_code)]
pub(crate) fn build_test_transform<F>(
  comments: Rc<SingleThreadedComments>,
  customize: F,
) -> impl Pass
where
  F: FnOnce(TestBuilder) -> TestBuilder,
{
  customize(StyleXTransform::test(comments)).into_pass()
}
