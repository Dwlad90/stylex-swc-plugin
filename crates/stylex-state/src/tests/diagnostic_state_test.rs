//! The nine questions a diagnostic asks of the traversal state, asked of the
//! traversal state.
//!
//! `stylex_diagnostics` takes its state by generic bound and the only
//! implementation it can see is its own test double, so its own suite measures
//! the double. The implementation that ships is here, and every answer of it is
//! an inherent method of the same name -- so a rename that drops the inherent
//! one turns the trait body into unbounded recursion rather than into a compile
//! error. Each case below therefore goes through the trait, never through the
//! inherent method.

use swc_core::{
  atoms::Atom,
  common::{BytePos, DUMMY_SP, FileName, SourceMap, Span, sync::Lrc},
  ecma::{
    ast::{CallExpr, EsVersion, Expr, Module, ModuleItem, Stmt},
    parser::{Parser, StringInput, Syntax, TsSyntax, lexer::Lexer},
  },
};

use stylex_diagnostics::state::DiagnosticState;
use stylex_state_index::key_span_index::{CallLookup, ModuleBase};
use stylex_structures::plugin_pass::PluginPass;

use crate::state_manager::StateManager;

/// A state manager naming `filename` as the file being transformed.
fn state_for_file(filename: &str) -> StateManager {
  let mut state = StateManager::default();

  state.set_plugin_pass(PluginPass::new(None, Some(FileName::Real(filename.into()))));

  state
}

/// Parses a module into a source map of its own, so its first byte is
/// `BytePos(1)` -- the one arrangement where an offset into the file and a
/// `BytePos` are interchangeable. Every case here resolves a key against the
/// module it parsed, so nothing depends on two modules being told apart by
/// position; what a shared map does to the two is
/// `key_span_index_test`'s subject, not this module's.
fn parse(source: &str) -> Module {
  let source_map: Lrc<SourceMap> = Default::default();
  let source_file = source_map.new_source_file(FileName::Anon.into(), source.to_owned());
  let lexer = Lexer::new(
    Syntax::Typescript(TsSyntax {
      tsx: true,
      ..Default::default()
    }),
    EsVersion::EsNext,
    StringInput::from(&*source_file),
    None,
  );

  match Parser::new_from(lexer).parse_module() {
    Ok(module) => module,
    Err(error) => panic!("failed to parse `{}`: {:?}", source, error),
  }
}

/// The first call the module writes at top level.
fn first_call(module: &Module) -> CallExpr {
  module
    .body
    .iter()
    .find_map(|item| match item {
      ModuleItem::Stmt(Stmt::Expr(statement)) => match statement.expr.as_ref() {
        Expr::Call(call) => Some(call.clone()),
        _ => None,
      },
      _ => None,
    })
    .unwrap_or_else(|| panic!("the module holds no top-level call"))
}

/// Where the index says `namespace_key` of `call` is written, asked the way the
/// annotation path asks it.
fn resolve_key(
  state: &StateManager,
  module: &Module,
  call: &CallExpr,
  namespace_key: &str,
) -> Span {
  let index = DiagnosticState::key_span_index(state).expect("a memoized module has an index");
  let lookup = CallLookup::new(call, Some(ModuleBase::of(module)));

  index.resolve(&lookup.query(namespace_key))
}

#[test]
fn names_the_file_being_transformed() {
  let state = state_for_file("/app/components/Button.tsx");

  // The whole path, not the basename: `extract_filename_with_ext_from_path`
  // lives beside this and answers `Button.tsx`, so an `ends_with` here would
  // pass on either.
  assert_eq!(
    DiagnosticState::get_filename(&state),
    "/app/components/Button.tsx"
  );
}

/// Nothing is memoized before a diagnostic parses the source, so the first
/// question a frame asks is answered with nothing rather than with an empty
/// module.
#[test]
fn a_fresh_state_has_memoized_no_module() {
  let state = StateManager::default();

  assert!(DiagnosticState::get_seen_module_source_code(&state).is_none());
  assert!(DiagnosticState::key_span_index(&state).is_none());
}

/// The round trip both halves of a code frame depend on: the module it walks
/// and the text it slices come back as they went in.
#[test]
fn the_memoized_module_and_its_text_come_back_as_they_went_in() {
  let source = "create({ root: { color: 'red' } });";
  let module = parse(source);
  let mut state = StateManager::default();

  DiagnosticState::set_seen_module_source_code(&mut state, &module, Some(String::from(source)));

  let (seen, text) =
    DiagnosticState::get_seen_module_source_code(&state).expect("the module was just memoized");

  // The whole module, not its statement count: any one-statement module would
  // satisfy a length comparison.
  assert_eq!(seen, &module);
  assert_eq!(text.as_deref(), Some(source));
}

/// A module may be memoized without its text -- the frame then has a tree to
/// place a key in and no source to quote.
#[test]
fn a_module_can_be_memoized_without_its_source_text() {
  let module = parse("create({ root: {} });");
  let mut state = StateManager::default();

  DiagnosticState::set_seen_module_source_code(&mut state, &module, None);

  let (_, text) =
    DiagnosticState::get_seen_module_source_code(&state).expect("the module was just memoized");

  assert_eq!(text.as_deref(), None);
}

/// The span cache answers only for a key something put there.
#[test]
fn the_span_cache_answers_for_the_key_it_was_given() {
  let mut state = StateManager::default();
  let span = Span {
    lo: BytePos(11),
    hi: BytePos(30),
  };

  assert_eq!(DiagnosticState::cached_span(&state, 7), None);

  DiagnosticState::insert_cached_span(&mut state, 7, span);

  assert_eq!(DiagnosticState::cached_span(&state, 7), Some(span));
  assert_eq!(DiagnosticState::cached_span(&state, 8), None);
}

/// A second answer for the same key replaces the first, which is what lets a
/// re-resolved namespace correct a cached position.
#[test]
fn the_span_cache_keeps_the_last_answer_for_a_key() {
  let mut state = StateManager::default();

  DiagnosticState::insert_cached_span(&mut state, 7, DUMMY_SP);
  DiagnosticState::insert_cached_span(
    &mut state,
    7,
    Span {
      lo: BytePos(1),
      hi: BytePos(2),
    },
  );

  assert_eq!(
    DiagnosticState::cached_span(&state, 7).map(|span| span.lo),
    Some(BytePos(1))
  );
}

/// The index is built from the memoized module, on the first question that
/// needs it.
#[test]
fn the_key_span_index_places_a_namespace_of_the_memoized_module() {
  let source = "create({ root: { color: 'red' } });";
  let module = parse(source);
  let call = first_call(&module);
  let mut state = StateManager::default();

  DiagnosticState::set_seen_module_source_code(&mut state, &module, Some(String::from(source)));

  assert!(!resolve_key(&state, &module, &call, "root").is_dummy());
  assert!(resolve_key(&state, &module, &call, "absent").is_dummy());
}

/// The rule the diagnostics' own test double encodes, asserted against the
/// implementation that ships: the index is built from the memoized module, so
/// replacing that module has to drop it. Kept as a cache across a replacement,
/// it would place a key at a position in the file that is no longer open.
#[test]
fn replacing_the_memoized_module_drops_the_index_built_from_it() {
  let first_source = "create({ root: { color: 'red' } });";
  let first = parse(first_source);
  let first_call_expr = first_call(&first);
  let second_source = "create({ other: { color: 'blue' } });";
  let second = parse(second_source);
  let second_call_expr = first_call(&second);

  let mut state = StateManager::default();

  DiagnosticState::set_seen_module_source_code(
    &mut state,
    &first,
    Some(String::from(first_source)),
  );
  // Built here, so the replacement below has something to drop.
  assert!(!resolve_key(&state, &first, &first_call_expr, "root").is_dummy());

  DiagnosticState::set_seen_module_source_code(
    &mut state,
    &second,
    Some(String::from(second_source)),
  );

  assert!(
    !resolve_key(&state, &second, &second_call_expr, "other").is_dummy(),
    "the index has to be rebuilt from the module that replaced the first"
  );
  assert!(
    resolve_key(&state, &second, &first_call_expr, "root").is_dummy(),
    "a key of the replaced module must not still resolve"
  );
}

/// A build that refuses nothing answers without hashing anything, which is what
/// `has_framed_declarations` is for.
#[test]
fn a_state_that_refused_nothing_has_no_framed_declaration() {
  let state = StateManager::default();

  assert!(!DiagnosticState::has_framed_declarations(&state));
  assert_eq!(DiagnosticState::framed_declaration(&state, 7), None);
}

#[test]
fn a_framed_declaration_comes_back_under_the_key_it_was_recorded_against() {
  let mut state = StateManager::default();

  DiagnosticState::frame_declaration(&mut state, 7, Atom::from("Button"));

  assert!(DiagnosticState::has_framed_declarations(&state));
  assert_eq!(
    DiagnosticState::framed_declaration(&state, 7),
    Some(&Atom::from("Button"))
  );
  assert_eq!(DiagnosticState::framed_declaration(&state, 8), None);
}

/// Two refusals on different expressions are framed against their own
/// bindings: the key is the expression, so one must not answer for the other.
#[test]
fn two_refusals_keep_their_own_framed_declarations() {
  let mut state = StateManager::default();

  DiagnosticState::frame_declaration(&mut state, 7, Atom::from("Button"));
  DiagnosticState::frame_declaration(&mut state, 8, Atom::from("Card"));

  assert_eq!(
    DiagnosticState::framed_declaration(&state, 7),
    Some(&Atom::from("Button"))
  );
  assert_eq!(
    DiagnosticState::framed_declaration(&state, 8),
    Some(&Atom::from("Card"))
  );
}
