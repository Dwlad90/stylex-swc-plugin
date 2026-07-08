use std::{
  fs,
  panic::AssertUnwindSafe,
  path::{Path, PathBuf},
  sync::Arc,
};

use swc_core::common::{BytePos, DUMMY_SP, FileName, GLOBALS, Globals, Span, SyntaxContext};
use swc_core::ecma::ast::{
  Expr, Ident, ImportDecl, ImportNamedSpecifier, ImportSpecifier, Module, ModuleDecl, ModuleItem,
  Str,
};

use crate::shared::{
  structures::state_manager::StateManager,
  utils::log::build_code_frame_error::{
    CodeFrame, build_code_frame_error_and_panic, get_span_from_source_code, print_module,
  },
};

/// Writes a fixture whose content contains multi-byte characters, so any byte
/// offset taken from a foreign source map is likely to land inside a character
/// instead of on a char boundary.
fn write_multibyte_fixture(name: &str) -> PathBuf {
  let dir = std::env::temp_dir().join("stylex_code_frame_error_tests");
  fs::create_dir_all(&dir).unwrap();

  // Two-byte characters ("λ" is U+03BB) after the 3-byte "// " prefix, so
  // every even offset >= 4 falls inside a character.
  let source = format!(
    "// {}\nexport const styles = {{ root: {{ color: 'red' }} }};\n",
    "λ".repeat(700)
  );

  let path = dir.join(name);
  fs::write(&path, source).unwrap();

  path
}

fn state_for_fixture(path: &Path) -> StateManager {
  let mut state = StateManager::default();
  state.plugin_pass.filename = FileName::Real(path.to_path_buf());
  state
}

/// An expression that does not exist in the fixture, carrying a span from a
/// foreign source map (e.g. the compiler's own parse) whose byte offsets are
/// meaningless for the code-frame source map.
fn unmatched_expression_with_foreign_span() -> Expr {
  Expr::Ident(Ident::new(
    "identifier_not_present_in_fixture".into(),
    Span::new(BytePos(17), BytePos(27)),
    SyntaxContext::empty(),
  ))
}

#[test]
fn unmatched_expression_yields_dummy_span() {
  let path = write_multibyte_fixture("unmatched_expression.tsx");
  let mut state = state_for_fixture(&path);
  let target = unmatched_expression_with_foreign_span();

  let (_code_frame, span) = GLOBALS.set(&Globals::default(), || {
    get_span_from_source_code(&target, &target, &mut state).unwrap()
  });

  assert!(
    span.is_dummy(),
    "an expression that cannot be located in the source must not leak its \
     foreign span (got {:?}); foreign byte offsets can land inside multi-byte \
     characters and panic on source-map lookups",
    span
  );
}

/// Recreates the production failure: the shared code-frame source map holds a
/// multi-byte source, and a module carrying spans from the compiler's own
/// source map gets printed against it. The codegen samples snippets for
/// non-dummy list spans (`span_to_snippet`), so a foreign span whose offsets
/// land inside a multi-byte character panics unless spans are dropped first.
#[test]
fn print_module_ignores_foreign_spans_over_multibyte_sources() {
  let code_frame = CodeFrame::new();

  // A large single-character run so mid-character offsets are dense: after the
  // 2-byte "//" prefix every character occupies two bytes, meaning one of any
  // two consecutive offsets is not a char boundary wherever the file lands in
  // the shared source map.
  let multibyte_source = format!("//{}", "λ".repeat(300_000));
  code_frame.source_map.new_source_file(
    Arc::new(FileName::Custom("foreign_span_fixture.tsx".to_string())),
    multibyte_source,
  );

  // Named import/export specifier lists are the codegen's trailing-comma
  // snippet-sampling case (`ListFormat::NamedImportsOrExportsElements`), with
  // the import declaration's own span as the sampled range.
  let import_with_span = |lo: u32, hi: u32| {
    ModuleItem::ModuleDecl(ModuleDecl::Import(ImportDecl {
      span: Span::new(BytePos(lo), BytePos(hi)),
      specifiers: vec![ImportSpecifier::Named(ImportNamedSpecifier {
        span: DUMMY_SP,
        local: Ident::new("exampleExport".into(), DUMMY_SP, SyntaxContext::empty()),
        imported: None,
        is_type_only: false,
      })],
      src: Box::new(Str {
        span: DUMMY_SP,
        value: "example-package".into(),
        raw: None,
      }),
      type_only: false,
      with: None,
      phase: Default::default(),
    }))
  };

  // Two consecutive parities: whichever the fixture's start position is, one
  // of these spans starts or ends inside a character.
  let module = Module {
    span: DUMMY_SP,
    body: vec![
      import_with_span(100_000, 100_010),
      import_with_span(100_001, 100_011),
    ],
    shebang: None,
  };

  let printed = print_module(&code_frame, module, None);

  assert!(
    printed.contains("exampleExport"),
    "printing must succeed and include the module contents, got: {}",
    printed
  );
}

#[test]
fn panic_reports_real_message_for_multibyte_source() {
  let path = write_multibyte_fixture("panic_real_message.tsx");
  let mut state = state_for_fixture(&path);
  let target = unmatched_expression_with_foreign_span();

  let error_message = "A style value must be static";

  let panic_payload = std::panic::catch_unwind(AssertUnwindSafe(|| {
    GLOBALS.set(&Globals::default(), || {
      build_code_frame_error_and_panic(&target, &target, error_message, &mut state)
    })
  }))
  .unwrap_err();

  let message = match panic_payload.downcast_ref::<String>() {
    Some(message) => message.clone(),
    None => match panic_payload.downcast_ref::<&str>() {
      Some(message) => (*message).to_string(),
      None => String::from("<non-string panic payload>"),
    },
  };

  assert!(
    message.contains(error_message),
    "panic must surface the original StyleX error, got: {}",
    message
  );
  assert!(
    !message.contains("char boundary"),
    "panic must not be replaced by a char-boundary slicing panic, got: {}",
    message
  );
}
