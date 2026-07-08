use std::{
  fs,
  panic::AssertUnwindSafe,
  path::{Path, PathBuf},
  sync::{
    Arc,
    atomic::{AtomicUsize, Ordering},
  },
};

use swc_core::common::{BytePos, DUMMY_SP, FileName, GLOBALS, Globals, Span, SyntaxContext};
use swc_core::ecma::ast::{
  CallExpr, Callee, Expr, ExprOrSpread, Ident, ImportDecl, ImportNamedSpecifier, ImportSpecifier,
  Module, ModuleDecl, ModuleItem, Str,
};

use crate::shared::{
  structures::state_manager::StateManager,
  utils::log::build_code_frame_error::{
    CodeFrame, build_code_frame_error_and_panic, get_key_span_from_source_code,
    get_span_from_source_code, print_module,
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
    get_key_span_from_source_code(&call_expr, "other", &mut state)
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
    get_key_span_from_source_code(&call_expr, "other", &mut state)
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
    get_key_span_from_source_code(&call_expr, "other", &mut state)
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

fn key_span_candidate(
  namespace_value_overlap: usize,
  overlap: usize,
  distance_from_target: Option<u32>,
) -> super::KeySpanCandidate {
  super::KeySpanCandidate {
    namespace_value_overlap,
    overlap,
    distance_from_target,
    span: DUMMY_SP,
  }
}

#[test]
fn candidate_rank_prefers_value_overlap_then_sibling_overlap_then_proximity() {
  // Namespace-value overlap dominates every other signal.
  assert!(key_span_candidate(2, 0, Some(100)).rank() > key_span_candidate(1, 9, Some(0)).rank());

  // Sibling-key overlap breaks value-overlap ties.
  assert!(key_span_candidate(1, 3, Some(100)).rank() > key_span_candidate(1, 2, Some(0)).rank());

  // Smaller distance to the target wins a full overlap tie.
  assert!(key_span_candidate(1, 3, Some(5)).rank() > key_span_candidate(1, 3, Some(6)).rank());

  // No target position outranks any measured distance (Option: None < Some).
  assert!(key_span_candidate(1, 3, None).rank() > key_span_candidate(1, 3, Some(0)).rank());

  // Identical signals rank equal, which the finder reports as ambiguous.
  assert_eq!(
    key_span_candidate(1, 3, Some(5)).rank(),
    key_span_candidate(1, 3, Some(5)).rank()
  );
}
