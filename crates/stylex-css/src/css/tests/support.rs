//! Fixtures shared by the CSS test modules.
//!
//! Value normalization is asserted from more than one module — the general
//! coverage in `common_test`, and the harness-verdicted coverage in
//! `value_normalization_parity_test` and `spacing_repair_parity_test` — and all
//! of them need the same option objects, the same way of reading a rejection,
//! and the same way of recording a reference-compiler verdict. Kept here so a
//! change to how the compiler is configured, how it reports a rejection, or how
//! a verdict is spelled lands in one place.

use std::{
  any::Any,
  panic::{AssertUnwindSafe, catch_unwind},
};

use stylex_structures::stylex_state_options::StyleXStateOptions;

use crate::css::common::normalize_css_property_value;

/// The compiler's own defaults, which is what almost every case runs under.
pub(super) fn default_options() -> StyleXStateOptions {
  StyleXStateOptions::default()
}

/// Defaults with font-size pixel-to-rem conversion switched on.
pub(super) fn rem_enabled_options() -> StyleXStateOptions {
  StyleXStateOptions::default().with_enable_font_size_px_to_rem(true)
}

/// The text of a rejection, from the result of a `catch_unwind` around a call
/// expected to reject.
///
/// A rejection is raised as a panic carrying a message, and asserting on that
/// message is the point: `is_err()` alone passes on *any* panic, including one
/// from an unrelated bug, so a test written that way keeps passing after the
/// behaviour it guards has gone.
///
/// Panics when the call did not reject at all, since a caller reaching for the
/// message has already decided that it should have.
pub(super) fn panic_message<T>(result: Result<T, Box<dyn Any + Send>>) -> String {
  let Err(panic) = result else {
    panic!("expected the call to be rejected, but it returned normally");
  };

  panic
    .downcast_ref::<String>()
    .map(String::as_str)
    .or_else(|| panic.downcast_ref::<&str>().copied())
    .unwrap_or_default()
    .to_string()
}

/// What the reference compiler makes of a case, as the parity harness reported
/// it.
#[derive(Clone, Copy)]
pub(super) enum Reference {
  /// The reference compiler spells the value the same way. The expectation is
  /// the compatibility contract and must survive any pipeline change.
  Same,
  /// The reference compiler spells it differently, and this is its spelling.
  /// The expectation below is what this compiler produces today; replacing the
  /// pipeline adopts the reference spelling and this case's expectation
  /// changes.
  Diverges(&'static str),
}

/// One normalization case: an input, the declaration text this compiler
/// produces for it, and the reference compiler's verdict.
pub(super) struct Case {
  pub(super) property: &'static str,
  pub(super) value: &'static str,
  pub(super) expected: &'static str,
  pub(super) reference: Reference,
}

/// A case the reference compiler spells the same way.
pub(super) const fn same(
  property: &'static str,
  value: &'static str,
  expected: &'static str,
) -> Case {
  Case {
    property,
    value,
    expected,
    reference: Reference::Same,
  }
}

/// A case the reference compiler spells as `reference_spelling`.
pub(super) const fn diverges(
  property: &'static str,
  value: &'static str,
  expected: &'static str,
  reference_spelling: &'static str,
) -> Case {
  Case {
    property,
    value,
    expected,
    reference: Reference::Diverges(reference_spelling),
  }
}

/// Runs a case table and, for every case claiming a divergence, checks that the
/// two spellings really do differ.
///
/// That second assertion catches exactly one thing, which is the thing that
/// happens next: a pipeline change that adopts the reference spelling has to
/// move `expected`, and the case then fails until it is re-verdicted rather
/// than being quietly left carrying a stale claim. It cannot notice the
/// reference compiler itself changing — only a fresh harness run does that,
/// which is why the parity modules say how to do one.
pub(super) fn check(cases: &[Case], options: &StyleXStateOptions) {
  for case in cases {
    let actual = normalize_css_property_value(case.property, case.value, options);

    assert_eq!(
      actual, case.expected,
      "normalizing `{}: {}`",
      case.property, case.value
    );

    if let Reference::Diverges(reference_spelling) = case.reference {
      assert_ne!(
        case.expected, reference_spelling,
        "`{}: {}` is recorded as diverging from the reference compiler, yet both spell it the same",
        case.property, case.value
      );
    }
  }
}

/// Asserts that every value in `values` is rejected for `property`, with a
/// diagnostic containing `expected`.
///
/// A rejection is raised as a panic, so it has no spelling for a [`Case`] to
/// compare and cannot go in a case table. It still has to be asserted on the
/// message rather than on the mere fact of a panic — `is_err()` alone passes on
/// any panic at all, including one from an unrelated bug elsewhere in the
/// pipeline, so a test written that way keeps passing after the guard it
/// watches has gone.
pub(super) fn rejects(
  property: &str,
  values: &[&str],
  expected: &str,
  options: &StyleXStateOptions,
) {
  for value in values {
    let result = catch_unwind(AssertUnwindSafe(|| {
      normalize_css_property_value(property, value, options)
    }));

    let message = panic_message(result);

    assert!(
      message.contains(expected),
      "expected `{property}: {value}` to be rejected with `{expected}`, got: {message}"
    );
  }
}
