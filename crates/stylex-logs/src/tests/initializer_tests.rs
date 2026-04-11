// Tests for logger initialization idempotency and panic hook behavior.
// Source: crates/stylex-logs/src/initializer.rs

use super::*;
use std::panic;
use stylex_macros::stylex_error::SuppressPanicStderr;

#[test]
fn initialize_is_idempotent_and_formats_logs() {
  initialize();
  initialize();
  log::warn!(target: "stylex_logs::initializer::tests", "formatter smoke test");
}

#[test]
fn panic_hook_handles_prefixed_and_plain_messages() {
  initialize();

  let prefixed = panic::catch_unwind(|| panic!("{} prefixed panic", STYLEX_PREFIX));
  assert!(prefixed.is_err());

  let plain = panic::catch_unwind(|| panic!("plain panic"));
  assert!(plain.is_err());
}

#[test]
fn panic_hook_handles_non_string_payload() {
  initialize();

  let unknown = panic::catch_unwind(|| panic::panic_any(1234usize));
  assert!(unknown.is_err());
}

#[test]
fn panic_hook_respects_suppression_guard() {
  initialize();
  let guard = SuppressPanicStderr::new();

  let suppressed = panic::catch_unwind(|| panic!("suppressed panic"));
  assert!(suppressed.is_err());

  drop(guard);
}
