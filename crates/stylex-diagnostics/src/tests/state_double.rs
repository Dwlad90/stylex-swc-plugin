//! A stand-in for the transform's state manager, so the diagnostics can be
//! tested without the crate they are extracted from.
//!
//! It answers [`DiagnosticState`] the way the state manager answers it, and
//! nothing else. The one part that carries a rule rather than a field is the key
//! span index: it is built from the memoized module, so replacing that module
//! has to drop it.

use std::cell::OnceCell;

use swc_core::ecma::ast::Module;

use crate::{memo::DiagnosticMemo, state::DiagnosticState};
use stylex_state_index::key_span_index::KeySpanIndex;

#[derive(Default)]
pub(crate) struct StateDouble {
  filename: String,
  seen_module: Option<Module>,
  seen_source_code: Option<String>,
  key_span_index: OnceCell<KeySpanIndex>,
  memo: DiagnosticMemo,
  /// Drops the module as soon as it is stored, so the caller reads back the
  /// nothing a state that failed to memoize would hand it.
  forgets_the_module: bool,
}

impl StateDouble {
  /// A state that names `filename` as the file being transformed.
  pub(crate) fn for_file(filename: impl Into<String>) -> Self {
    Self {
      filename: filename.into(),
      ..Self::default()
    }
  }

  /// A state that never remembers the module it was given.
  pub(crate) fn forgetful() -> Self {
    Self {
      forgets_the_module: true,
      ..Self::default()
    }
  }
}

impl DiagnosticState for StateDouble {
  fn get_filename(&self) -> &str {
    &self.filename
  }

  fn get_seen_module_source_code(&self) -> Option<(&Module, Option<&str>)> {
    Some((self.seen_module.as_ref()?, self.seen_source_code.as_deref()))
  }

  fn set_seen_module_source_code(&mut self, module: &Module, source_code: Option<String>) {
    if self.forgets_the_module {
      return;
    }

    self.seen_module = Some(module.clone());
    self.seen_source_code = source_code;
    // Built from the module above, so a new module invalidates it.
    self.key_span_index = OnceCell::new();
  }

  fn key_span_index(&self) -> Option<&KeySpanIndex> {
    let module = self.seen_module.as_ref()?;

    Some(
      self
        .key_span_index
        .get_or_init(|| KeySpanIndex::build(module)),
    )
  }

  fn diagnostic_memo(&self) -> &DiagnosticMemo {
    &self.memo
  }

  fn diagnostic_memo_mut(&mut self) -> &mut DiagnosticMemo {
    &mut self.memo
  }
}
