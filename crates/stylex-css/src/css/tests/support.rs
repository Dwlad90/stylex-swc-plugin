//! Fixtures shared by the CSS test modules.
//!
//! Value normalization is asserted from more than one module — the general
//! coverage in `common_test`, and the harness-verdicted coverage in
//! `value_normalization_parity_test` — and both need the same option objects
//! and the same way of reading a rejection. Kept here so a change to how the
//! compiler is configured or how it reports a rejection lands in one place.

use std::any::Any;

use stylex_structures::stylex_state_options::StyleXStateOptions;

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
