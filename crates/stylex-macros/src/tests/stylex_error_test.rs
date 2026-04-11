//! Tests for StyleXError formatting, SuppressPanicStderr guard,
//! ANSI stripping, and panic message extraction.

use crate::stylex_error::{
  SuppressPanicStderr, format_panic_message, is_panic_stderr_suppressed,
};

/// SuppressPanicStderr RAII guard should toggle the thread-local flag.
#[test]
fn suppress_panic_stderr_guard_toggles_flag() {
  assert!(!is_panic_stderr_suppressed());
  {
    let _guard = SuppressPanicStderr::new();
    assert!(is_panic_stderr_suppressed());
  }
  assert!(!is_panic_stderr_suppressed());
}

/// SuppressPanicStderr::default() should behave identically to ::new().
#[test]
fn suppress_panic_stderr_default_works() {
  assert!(!is_panic_stderr_suppressed());
  {
    let _guard = SuppressPanicStderr::default();
    assert!(is_panic_stderr_suppressed());
  }
  assert!(!is_panic_stderr_suppressed());
}

/// strip_ansi is private, but tested indirectly via format_panic_message.
/// Here we test the format_panic_message function which internally uses strip_ansi.

/// A String payload without [StyleX] prefix should be wrapped with the prefix.
#[test]
fn format_panic_message_from_string_payload() {
  let msg = String::from("something went wrong");
  let payload: Box<dyn std::any::Any + Send> = Box::new(msg);
  let result = format_panic_message(&payload);
  assert_eq!(result, "[StyleX] something went wrong");
}

/// A &str payload should be converted and wrapped.
#[test]
fn format_panic_message_from_str_payload() {
  let msg: &str = "static error";
  let payload: Box<dyn std::any::Any + Send> = Box::new(msg);
  let result = format_panic_message(&payload);
  assert_eq!(result, "[StyleX] static error");
}

/// A payload already containing [StyleX] should be returned as-is (not double-wrapped).
#[test]
fn format_panic_message_with_existing_prefix() {
  let msg = String::from("[StyleX] already prefixed");
  let payload: Box<dyn std::any::Any + Send> = Box::new(msg);
  let result = format_panic_message(&payload);
  assert_eq!(result, "[StyleX] already prefixed");
}

/// An unknown type (not String or &str) should produce a generic "Unknown error" message.
#[test]
fn format_panic_message_unknown_type() {
  let payload: Box<dyn std::any::Any + Send> = Box::new(42i32);
  let result = format_panic_message(&payload);
  assert!(result.contains("[StyleX]"));
  assert!(result.contains("Unknown error"));
}
