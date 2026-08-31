use swc_core::{atoms::Atom, common::Span, ecma::ast::Module};

use stylex_state_index::key_span_index::KeySpanIndex;

/// Everything a diagnostic has to read off the compiler's traversal state.
///
/// Owned here rather than in the transform so that building a code frame never
/// names the state manager, which would make the transform and the diagnostics
/// depend on each other. The transform implements the trait; this crate only
/// ever sees these nine questions.
///
/// The trait is consulted while a diagnostic is being written, never while a
/// module is being evaluated, so the dispatch it costs is not on any hot path.
pub trait DiagnosticState {
  /// The file being transformed, as the frame should name it.
  fn get_filename(&self) -> &str;

  /// The module's own source, re-parsed and memoized by an earlier diagnostic,
  /// together with the text it was parsed from.
  fn get_seen_module_source_code(&self) -> Option<(&Module, &Option<String>)>;

  /// Memoizes that re-parsed source, so the next diagnostic in the same file
  /// does not read and parse it again.
  fn set_seen_module_source_code(&mut self, module: &Module, source_code: Option<String>);

  /// Where a previous lookup put the answer for `cache_key`, if it asked.
  fn cached_span(&self, cache_key: u128) -> Option<Span>;

  /// Records the answer for `cache_key`.
  fn insert_cached_span(&mut self, cache_key: u128, span: Span);

  /// Where every style namespace key of the memoized source is written, built
  /// on first use.
  fn key_span_index(&self) -> Option<&KeySpanIndex>;

  /// Records that the refusal behind `cache_key` is about the binding `name`.
  fn frame_declaration(&mut self, cache_key: u128, name: Atom);

  /// The binding recorded against `cache_key`, if one was.
  fn framed_declaration(&self, cache_key: u128) -> Option<&Atom>;

  /// Whether any refusal recorded a declaration to frame. False for every build
  /// that refuses nothing, which lets the annotation path answer without
  /// hashing an expression.
  fn has_framed_declarations(&self) -> bool;
}
