use std::{rc::Rc, sync::Arc};

use stylex_structures::stylex_options::ModuleResolution;
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

/// A transform for the cases where a theme import must *resolve*.
///
/// Resolving one takes both a real filename and `haste` resolution, and every
/// case that reads a theme -- a name shadowed by a dynamic parameter, a theme
/// reference read where a style value belongs -- needs it for the same reason:
/// otherwise the case is about the path rather than about what it asks. One
/// function rather than a name per caller, so the files asking cannot drift into
/// asking under different options.
#[allow(dead_code)]
pub(crate) fn theme_import_transform(comments: TestComments) -> impl Pass {
  theme_import_transform_with(comments, |b| b)
}

/// The theme-import transform with one further option applied.
///
/// For a case that needs the same module resolution and runtime injection but
/// differs in one setting -- media query ordering, say -- so that the shared
/// half stays in one place and the difference is the only thing the test says.
#[allow(dead_code)]
pub(crate) fn theme_import_transform_with<F>(comments: TestComments, customize: F) -> impl Pass
where
  F: FnOnce(TestBuilder) -> TestBuilder,
{
  build_test_transform(comments, move |b| {
    customize(
      b.with_filename(FileName::Real("MyComponent.js".into()))
        .with_unstable_module_resolution(ModuleResolution::haste(None))
        .with_runtime_injection(),
    )
  })
}

/// The theme-import transform with the evaluator's ceiling raised.
///
/// For the cases that measure how deep a fold can go: the shipped default is
/// sized for hand-written styles, and a test that walks hundreds of levels has
/// to say so rather than quietly depend on the default being generous.
#[allow(dead_code)]
pub(crate) fn deep_theme_import_transform(comments: TestComments, depth: usize) -> impl Pass {
  build_test_transform(comments, move |b| {
    b.with_filename(FileName::Real("MyComponent.js".into()))
      .with_unstable_module_resolution(ModuleResolution::haste(None))
      .with_max_evaluation_depth(depth)
      .with_runtime_injection()
  })
}

/// The same, for a module that *declares* a theme rather than importing one.
///
/// A `defineVars` call hashes its own filename, so a file not named
/// `*.stylex.js` refuses for the filename before the value under test is ever
/// read -- which is how a value question comes to be measured as a path
/// question.
#[allow(dead_code)]
pub(crate) fn theme_module_transform(comments: TestComments) -> impl Pass {
  build_test_transform(comments, |b| {
    b.with_filename(FileName::Real("vars.stylex.js".into()))
      .with_unstable_module_resolution(ModuleResolution::haste(None))
      .with_runtime_injection()
  })
}

/// Compile one module under the theme-import transform and hand back what it
/// printed, rules included.
///
/// The four files whose subject is what the evaluator folds each need exactly
/// this — compile a whole module, then assert on the class names and rule text
/// upstream was measured to produce. Kept here rather than copied per file so
/// they cannot drift into compiling under different options and reporting the
/// difference as a divergence in the value under test.
#[allow(dead_code)]
pub(crate) fn fold_module(input: &str) -> String {
  stringify_js(input, ts_syntax(), |tr| {
    theme_import_transform(tr.comments.clone())
  })
}

/// The rule a `stylex.create` module of one style is expected to emit,
/// asserted on what it printed.
///
/// The three things a fold case is — the bindings, the declaration, the measured
/// rule — read once here rather than per file, so two files whose subject is the
/// same fold cannot come to assert it differently.
#[track_caller]
#[allow(dead_code)]
pub(crate) fn assert_folds(decls: &str, body: &str, rule: &str) {
  let output = fold_module(&base_style_module(decls, body));

  assert!(
    output.contains(rule),
    "expected `{}` with `{}` to emit `{}`, got:\n{}",
    body,
    decls,
    rule,
    output
  );
}

/// The sentence a refusal has to carry, so a case cannot be satisfied by a
/// refusal for some later, wrong reason.
///
/// A `should_panic` attribute answers one case per function and says nothing
/// about which rule fired when a file has several; this reads the panic's own
/// message, so a list of refusals stays a list.
#[track_caller]
#[allow(dead_code)]
pub(crate) fn assert_refuses(decls: &str, body: &str, sentence: &str) {
  assert_refuses_under(decls, body, sentence, fold_module);
}

/// The same, with the compile step handed in.
///
/// A file whose subject is an *option* has to compile under that option, so the
/// reading of the refusal cannot be tied to the default transform. Everything
/// else about a refusal case is the same, which is why this is one function
/// rather than a second copy of it.
#[track_caller]
#[allow(dead_code)]
pub(crate) fn assert_refuses_under(
  decls: &str,
  body: &str,
  sentence: &str,
  compile: impl FnOnce(&str) -> String + std::panic::UnwindSafe,
) {
  let module = base_style_module(decls, body);
  let refusal = std::panic::catch_unwind(|| compile(&module));

  let Err(payload) = refusal else {
    panic!("expected `{}` with `{}` to refuse", body, decls);
  };

  // A panic payload is whichever of the two string types the caller raised, and
  // both reach here: `panic!("{}", …)` carries a `String` and a literal message
  // a `&str`.
  let said = match payload.downcast_ref::<String>() {
    Some(message) => message.clone(),
    None => match payload.downcast_ref::<&str>() {
      Some(message) => (*message).to_string(),
      None => panic!("the refusal of `{}` carried no message", body),
    },
  };

  assert!(
    said.contains(sentence),
    "expected `{}` with `{}` to refuse with `{}`, got `{}`",
    body,
    decls,
    sentence,
    said
  );
}

/// Compile one module with the character ceiling set to `characters`, the way an
/// author moves it.
///
/// A file whose subject is that ceiling has to move it through the option rather
/// than assert the default from the inside, and two files now have that subject --
/// a string the evaluator grows and the join an array's `ToString` performs -- so
/// the compile step lives here rather than once per file.
#[allow(dead_code)]
pub(crate) fn fold_module_under(input: &str, characters: usize) -> String {
  stringify_js(input, ts_syntax(), move |tr| {
    theme_import_transform_with(tr.comments.clone(), move |builder| {
      builder.with_max_folded_characters(characters)
    })
  })
}

/// The rule `body` is expected to emit under a character ceiling of `characters`.
#[track_caller]
#[allow(dead_code)]
pub(crate) fn assert_folds_under(decls: &str, body: &str, rule: &str, characters: usize) {
  let output = fold_module_under(&base_style_module(decls, body), characters);

  assert!(
    output.contains(rule),
    "expected `{}` with `{}` to emit `{}` under a ceiling of {}, got:\n{}",
    body,
    decls,
    rule,
    characters,
    output
  );
}

/// One `stylex.create` module of a single `base` style: `decls` above it,
/// `body` as that style's declarations.
///
/// `decls` is where a case that needs a binding puts it, and empty where the
/// case is about a value written out. A file whose subject is the *shape* of
/// the create call rather than one style's value writes its own, since this
/// one fixes the style name.
#[allow(dead_code)]
pub(crate) fn base_style_module(decls: &str, body: &str) -> String {
  format!(
    r#"
      import * as stylex from '@stylexjs/stylex';
      {}
      export const styles = stylex.create({{
        base: {{ {} }},
      }});
    "#,
    decls, body
  )
}
