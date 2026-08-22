use std::{
  fs,
  panic::AssertUnwindSafe,
  path::{Path, PathBuf},
  sync::{
    Arc,
    atomic::{AtomicUsize, Ordering},
  },
};

use swc_core::atoms::Atom;
use swc_core::common::{BytePos, DUMMY_SP, FileName, GLOBALS, Globals, Span, SyntaxContext};
use swc_core::ecma::ast::{
  CallExpr, Callee, Expr, ExprOrSpread, Ident, ImportDecl, ImportNamedSpecifier, ImportSpecifier,
  Module, ModuleDecl, ModuleItem, Str,
};

use crate::shared::{
  structures::{key_span_index::CallLookup, state_manager::StateManager},
  utils::log::build_code_frame_error::{
    CodeFrame, build_code_frame_error_and_panic, frame_declaration_of,
    get_key_span_from_source_code, get_span_from_source_code, print_module,
  },
};
use stylex_ast::ast::{
  convertors::create_string_expr,
  factories::{create_key_value_prop, create_nested_object_prop, create_object_expression},
};

static TEST_FILE_COUNTER: AtomicUsize = AtomicUsize::new(0);

/// Writes a fixture whose content contains multi-byte characters, so any byte
/// offset taken from a foreign source map is likely to land inside a character
/// instead of on a char boundary.
fn write_multibyte_fixture(name: &str) -> PathBuf {
  let id = TEST_FILE_COUNTER.fetch_add(1, Ordering::Relaxed);
  let dir = std::env::temp_dir().join(format!(
    "stylex_code_frame_error_tests_{}_{}",
    std::process::id(),
    id
  ));
  if let Err(error) = fs::create_dir_all(&dir) {
    panic!("failed to create temp fixture directory: {error}");
  }

  // Two-byte characters ("λ" is U+03BB) after the 3-byte "// " prefix, so
  // every even offset >= 4 falls inside a character.
  let source = format!(
    "// {}\nexport const styles = {{ root: {{ color: 'red' }} }};\n",
    "λ".repeat(700)
  );

  let path = dir.join(name);
  if let Err(error) = fs::write(&path, source) {
    panic!("failed to write temp fixture: {error}");
  }

  path
}

fn write_fixture(name: &str, source: &str) -> PathBuf {
  let id = TEST_FILE_COUNTER.fetch_add(1, Ordering::Relaxed);
  let dir = std::env::temp_dir().join(format!(
    "stylex_code_frame_error_tests_{}_{}",
    std::process::id(),
    id
  ));

  if let Err(error) = fs::create_dir_all(&dir) {
    panic!("failed to create temp fixture directory: {error}");
  }

  let path = dir.join(name);

  if let Err(error) = fs::write(&path, source) {
    panic!("failed to write temp fixture: {error}");
  }

  path
}

fn compiled_create_call() -> CallExpr {
  let compiled_arg = create_object_expression(vec![
    create_nested_object_prop(
      "root",
      vec![create_key_value_prop("color", create_string_expr("red"))],
    ),
    create_nested_object_prop(
      "other",
      vec![create_key_value_prop("display", create_string_expr("flex"))],
    ),
  ]);

  CallExpr {
    span: DUMMY_SP,
    ctxt: SyntaxContext::empty(),
    callee: Callee::Expr(Box::new(Expr::Ident(Ident::new(
      "create".into(),
      DUMMY_SP,
      SyntaxContext::empty(),
    )))),
    args: vec![ExprOrSpread {
      spread: None,
      expr: Box::new(compiled_arg),
    }],
    type_args: None,
  }
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

  let span = GLOBALS.set(&Globals::default(), || {
    match get_span_from_source_code(&target, &target, &mut state) {
      Ok((_code_frame, span)) => span,
      Err(error) => panic!("failed to get source span: {error}"),
    }
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

/// `get_key_span_from_source_code` over a freshly built [`CallLookup`], which is
/// what the production caller hoists out of its namespace loop.
fn key_span_for(
  call_expr: &CallExpr,
  namespace_key: &str,
  state: &mut StateManager,
) -> Result<(CodeFrame, Span), anyhow::Error> {
  get_key_span_from_source_code(&CallLookup::new(call_expr), namespace_key, state)
}

/// When an earlier loader rewrites style values (e.g. compile-time macros),
/// the compiled AST no longer textually matches the file on disk, so
/// value-expression matching cannot locate a source position. Namespace keys
/// are untouched by such transforms, so the key-based lookup must still
/// resolve the real line number.
#[test]
fn key_lookup_finds_line_when_values_differ_from_source() {
  let source = "\
import fancyMacro from 'example-macro';

export const styles = create({
  root: {
    color: fancyMacro(2),
  },
  other: {
    display: fancyMacro('flex'),
  },
});
";
  let path = write_fixture("key_lookup.tsx", source);
  let mut state = state_for_fixture(&path);
  let call_expr = compiled_create_call();

  let result = GLOBALS.set(&Globals::default(), || {
    key_span_for(&call_expr, "other", &mut state)
  });
  let (code_frame, span) = match result {
    Ok(result) => result,
    Err(error) => panic!("failed to get source span: {error}"),
  };

  assert!(
    !span.is_dummy(),
    "the namespace key must be locatable even though the values differ"
  );
  assert_eq!(
    code_frame.get_span_line_number(span),
    7,
    "the span must point at the `other` key in the on-disk source"
  );
}

#[test]
fn key_lookup_ignores_unrelated_objects_with_matching_keys() {
  let source = "\
const unrelated = {
  root: {},
  other: {},
};

export const styles = create({
  root: {
    color: fancyMacro(2),
  },
  other: {
    display: fancyMacro('flex'),
  },
});
";
  let path = write_fixture("key_lookup_ignores_unrelated.tsx", source);
  let mut state = state_for_fixture(&path);
  let call_expr = compiled_create_call();

  let result = GLOBALS.set(&Globals::default(), || {
    key_span_for(&call_expr, "other", &mut state)
  });
  let (code_frame, span) = match result {
    Ok(result) => result,
    Err(error) => panic!("failed to get source span: {error}"),
  };

  assert_eq!(
    code_frame.get_span_line_number(span),
    10,
    "the span must point at the `other` key in the stylex create call"
  );
}

#[test]
fn key_lookup_returns_dummy_for_ambiguous_dummy_span_calls() {
  let source = "\
export const first = create({
  root: {
    color: fancyMacro(1),
  },
  other: {
    display: fancyMacro('flex'),
  },
});

export const second = create({
  root: {
    color: fancyMacro(2),
  },
  other: {
    display: fancyMacro('block'),
  },
});
";
  let path = write_fixture("key_lookup_ambiguous_dummy_span.tsx", source);
  let mut state = state_for_fixture(&path);
  let call_expr = compiled_create_call();

  let result = GLOBALS.set(&Globals::default(), || {
    key_span_for(&call_expr, "other", &mut state)
  });
  let (_code_frame, span) = match result {
    Ok(result) => result,
    Err(error) => panic!("failed to get source span: {error}"),
  };

  assert!(
    span.is_dummy(),
    "ambiguous dummy-span calls must fall back to value-expression matching"
  );
}

#[test]
fn panic_reports_real_message_for_multibyte_source() {
  let path = write_multibyte_fixture("panic_real_message.tsx");
  let mut state = state_for_fixture(&path);
  let target = unmatched_expression_with_foreign_span();

  let error_message = "A style value must be static";

  let panic_payload = match std::panic::catch_unwind(AssertUnwindSafe(|| {
    GLOBALS.set(&Globals::default(), || {
      build_code_frame_error_and_panic(&target, &target, error_message, &mut state)
    })
  })) {
    Ok(()) => panic!("expected build_code_frame_error_and_panic to panic"),
    Err(panic_payload) => panic_payload,
  };

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

/// A refused reference, and the ident the evaluator hands the frame for it.
fn reference(name: &str) -> Expr {
  Expr::Ident(Ident::new(name.into(), DUMMY_SP, SyntaxContext::empty()))
}

#[track_caller]
fn framed_line(target: &Expr, state: &mut StateManager) -> Option<usize> {
  let result = GLOBALS.set(&Globals::default(), || {
    get_span_from_source_code(target, target, state)
  });

  match result {
    Ok((code_frame, span)) => code_frame.try_get_span_line_number(span),
    Err(error) => panic!("failed to get source span: {error}"),
  }
}

/// A refusal about a binding is framed at the binding's declaration, which is
/// the line the author has to go and change and the line
/// `@stylexjs/babel-plugin` prints. Measured on 0.19.0: for a reassigned
/// `let c = 'red'` on line 1, upstream frames line 1 and not the read on line 3.
///
/// This is the whole point of the plumbing, and the only assertion that can see
/// it: a `stylex_test_panic!` matches the message, and the position is written
/// separately.
#[test]
fn a_refusal_about_a_binding_is_framed_at_its_declaration() {
  let source = "\
let c = 'red';
c = 'blue';
export const styles = create({ x: { color: c } });
";
  let path = write_fixture("framed_declaration.tsx", source);
  let mut state = state_for_fixture(&path);
  let target = reference("c");

  assert_eq!(
    framed_line(&target, &mut state),
    Some(3),
    "the read is framed while nothing says otherwise"
  );

  let mut state = state_for_fixture(&path);
  frame_declaration_of(&Atom::from("c"), &target, &mut state);

  assert_eq!(
    framed_line(&target, &mut state),
    Some(1),
    "a refusal about `c` must be framed at the declaration of `c`"
  );
}

/// The two answers about one expression are cached apart. Asking for the read's
/// own position after a refusal recorded a declaration — the order the debug
/// path and a refusal can arrive in within one module — must not hand back the
/// declaration's line, or a style would be annotated with another line's number.
#[test]
fn the_declaration_and_the_read_are_cached_apart() {
  let source = "\
let c = 'red';
c = 'blue';
export const styles = create({ x: { color: c } });
";
  let path = write_fixture("framed_declaration_cache.tsx", source);
  let mut state = state_for_fixture(&path);
  let target = reference("c");

  frame_declaration_of(&Atom::from("c"), &target, &mut state);

  assert_eq!(framed_line(&target, &mut state), Some(1));
  // Cached now, and asked again the same way.
  assert_eq!(framed_line(&target, &mut state), Some(1));

  let other = reference("unrelated");
  assert_eq!(
    framed_line(&other, &mut state),
    None,
    "an expression with no declaration recorded and no match in the source \
     must resolve nothing rather than inherit a cached position"
  );
}

/// The declaration search reads the module the frame re-parsed, which is not
/// always the module the reference was resolved against. A name it does not
/// declare falls back to locating the expression, rather than reporting no
/// position at all.
#[test]
fn a_name_the_source_does_not_declare_falls_back_to_the_read() {
  let source = "export const styles = create({ x: { color: c } });\n";
  let path = write_fixture("framed_declaration_missing.tsx", source);
  let mut state = state_for_fixture(&path);
  let target = reference("c");

  frame_declaration_of(&Atom::from("c"), &target, &mut state);

  assert_eq!(
    framed_line(&target, &mut state),
    Some(1),
    "the read's own line is a better answer than none"
  );
}

/// Recording one refusal's declaration must not move an unrelated diagnostic. A
/// dynamic style's refused value falls through to an inline style rather than
/// stopping the build, so a second, unrelated position can be asked for
/// afterwards.
#[test]
fn a_recorded_declaration_does_not_move_an_unrelated_diagnostic() {
  let source = "\
let c = 'red';
c = 'blue';
const other = 'blue';
export const styles = create({ x: { color: c, background: other } });
";
  let path = write_fixture("framed_declaration_unrelated.tsx", source);
  let mut state = state_for_fixture(&path);

  frame_declaration_of(&Atom::from("c"), &reference("c"), &mut state);

  assert_eq!(
    framed_line(&reference("other"), &mut state),
    Some(4),
    "the unrelated read keeps its own position"
  );
}

/// A source the parser cannot read at all: the declaration search never gets a
/// module to look in, and the lookup has to degrade to "location unknown"
/// instead of aborting a build that is already failing for another reason.
#[test]
fn an_unparseable_source_frames_nothing_and_does_not_abort() {
  let path = write_fixture(
    "framed_declaration_unparseable.tsx",
    "let c = 'red'; c = 'blue'; export const styles = create({ x: { color: c } }\n",
  );
  let mut state = state_for_fixture(&path);
  let target = reference("c");

  frame_declaration_of(&Atom::from("c"), &target, &mut state);

  let result = GLOBALS.set(&Globals::default(), || {
    get_span_from_source_code(&target, &target, &mut state)
  });

  // Either answer is acceptable -- an error, or no position -- and what is not
  // is a panic, or a span whose offsets belong to a module that failed to parse.
  if let Ok((_code_frame, span)) = result {
    assert!(
      span.is_dummy(),
      "an unparseable source cannot resolve a position, got {span:?}"
    );
  }
}

/// A file the compiler was told about but which is not on disk. There is no
/// module to search, so the frame registers a synthesized one holding just the
/// expression it was handed — and the declaration lookup finds no declaration in
/// it and falls back to that expression, rather than reporting a position from a
/// file nobody read.
#[test]
fn a_missing_source_file_falls_back_to_the_expression_it_was_handed() {
  let mut state = StateManager::default();
  state.plugin_pass.filename = FileName::Real("/nonexistent/framed_declaration.tsx".into());
  let target = reference("c");

  frame_declaration_of(&Atom::from("c"), &target, &mut state);

  assert_eq!(
    framed_line(&target, &mut state),
    Some(1),
    "the synthesized module is one line long, and the read is what is on it"
  );
}

/// Windows line endings, because a line number is a count of `\n` and a `\r`
/// left in front of one is the classic off-by-one.
#[test]
fn a_declaration_in_a_crlf_source_is_framed_on_its_own_line() {
  let path = write_fixture(
    "framed_declaration_crlf.tsx",
    "let c = 'red';\r\nc = 'blue';\r\nexport const styles = create({ x: { color: c } });\r\n",
  );
  let mut state = state_for_fixture(&path);
  let target = reference("c");

  frame_declaration_of(&Atom::from("c"), &target, &mut state);

  assert_eq!(framed_line(&target, &mut state), Some(1));
}

/// A byte-order mark ahead of the first declaration: it is one to three bytes of
/// nothing, and a position that counted it as source text would land a column
/// early.
#[test]
fn a_declaration_after_a_byte_order_mark_is_framed_on_its_own_line() {
  let path = write_fixture(
    "framed_declaration_bom.tsx",
    "\u{feff}let c = 'red';\nc = 'blue';\nexport const styles = create({ x: { color: c } });\n",
  );
  let mut state = state_for_fixture(&path);
  let target = reference("c");

  frame_declaration_of(&Atom::from("c"), &target, &mut state);

  assert_eq!(framed_line(&target, &mut state), Some(1));
}

/// A declaration past a long run of multi-byte characters, which is where a byte
/// offset used as a character offset lands inside a character and panics the
/// source-map lookup. The frame catches that panic; this asserts it never has to.
#[test]
fn a_declaration_after_multibyte_text_is_framed_on_its_own_line() {
  let source = format!(
    "// {}\nlet c = 'red';\nc = 'blue';\nexport const styles = create({{ x: {{ color: c }} }});\n",
    "λ".repeat(700)
  );
  let path = write_fixture("framed_declaration_multibyte.tsx", &source);
  let mut state = state_for_fixture(&path);
  let target = reference("c");

  frame_declaration_of(&Atom::from("c"), &target, &mut state);

  assert_eq!(framed_line(&target, &mut state), Some(2));
}

/// A module long enough that the walk passes thousands of unrelated
/// declarations before the one being asked about.
#[test]
fn a_declaration_at_the_end_of_a_long_source_is_framed() {
  let mut source = String::new();
  for index in 0..3_000 {
    source.push_str(&format!("const n{index} = {index};\n"));
  }
  source.push_str("let c = 'red';\nc = 'blue';\n");
  source.push_str("export const styles = create({ x: { color: c } });\n");

  let path = write_fixture("framed_declaration_long.tsx", &source);
  let mut state = state_for_fixture(&path);
  let target = reference("c");

  frame_declaration_of(&Atom::from("c"), &target, &mut state);

  assert_eq!(framed_line(&target, &mut state), Some(3_001));
}

/// A name that appears in the source only inside a string or a comment declares
/// nothing, so the lookup falls back to the read rather than pointing at prose.
#[test]
fn a_name_only_mentioned_in_text_falls_back_to_the_read() {
  let source = "\
// c is discussed here and nowhere declared
const note = 'c = 42';
export const styles = create({ x: { color: c } });
";
  let path = write_fixture("framed_declaration_mentioned.tsx", source);
  let mut state = state_for_fixture(&path);
  let target = reference("c");

  frame_declaration_of(&Atom::from("c"), &target, &mut state);

  assert_eq!(
    framed_line(&target, &mut state),
    Some(3),
    "prose is not a declaration; the read is the answer"
  );
}

/// The record belongs to the expression the refusal was raised on, span
/// included, because that is what the key hashes. A copy of the same identifier
/// carrying a different position does not inherit it — which is a no-op rather
/// than a wrong answer, and the reason the production path hands the frame the
/// node the refusal stored rather than rebuilding one.
#[test]
fn a_recorded_declaration_belongs_to_the_expression_it_was_raised_on() {
  let source = "\
let c = 'red';
c = 'blue';
export const styles = create({ x: { color: c } });
";
  let path = write_fixture("framed_declaration_identity.tsx", source);
  let mut state = state_for_fixture(&path);

  let raised_on = reference("c");
  frame_declaration_of(&Atom::from("c"), &raised_on, &mut state);

  assert_eq!(framed_line(&raised_on, &mut state), Some(1));

  let rebuilt = Expr::Ident(Ident::new(
    "c".into(),
    Span::new(BytePos(11), BytePos(12)),
    SyntaxContext::empty(),
  ));

  assert_eq!(
    framed_line(&rebuilt, &mut state),
    Some(3),
    "a copy of the identifier does not inherit the record; it frames the read, \
     which is what every refusal framed before any of this"
  );
}
