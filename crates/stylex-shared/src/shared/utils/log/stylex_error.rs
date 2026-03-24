//! Structured error types and formatting for StyleX diagnostics.
//!
//! Provides `StyleXError` and helper functions/macros that produce clean,
//! branded `[StyleX] <message>` output on both stderr and NAPI boundaries.

use colored::*;
use std::fmt;

/// The branded prefix for all StyleX diagnostics.
pub const STYLEX_PREFIX: &str = "[StyleX]";

/// Structured error for all user-facing StyleX diagnostics.
///
/// `Display` produces:
/// ```text
/// [StyleX] key > path > message
///   --> file:line
/// [Stack trace]: source_location    (whenever source_location is set)
/// ```
#[derive(Debug, Clone)]
pub struct StyleXError {
  pub message: String,
  pub file: Option<String>,
  pub key_path: Option<Vec<String>>,
  pub line: Option<usize>,
  pub col: Option<usize>,
  pub source_location: Option<String>,
}

impl fmt::Display for StyleXError {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    // Colored [StyleX] prefix
    write!(f, "{} ", "[StyleX]".bright_blue().bold())?;

    // Key path breadcrumbs (if any)
    if let Some(ref keys) = self.key_path {
      for key in keys {
        write!(f, "{} > ", key.dimmed().cyan())?;
      }
    }

    // Main error message
    write!(f, "{}", self.message.red())?;

    // File location (when available)
    if let Some(ref file) = self.file {
      match (self.line, self.col) {
        (Some(line), Some(col)) => {
          write!(f, "\n  --> {file}:{line}:{col}")?;
        }
        (Some(line), None) => {
          write!(f, "\n  --> {file}:{line}")?;
        }
        _ => {
          write!(f, "\n  --> {file}")?;
        }
      }
    }

    // Stack trace (printed whenever source_location is set)
    if let Some(ref src) = self.source_location
      && log::log_enabled!(log::Level::Info)
    {
      write!(f, "\n{}: {src}", "[Stack trace]".dimmed().yellow(),)?;
    }

    Ok(())
  }
}

impl std::error::Error for StyleXError {}

// ---------------------------------------------------------------------------
// Constructor helpers
// ---------------------------------------------------------------------------

/// Create a simple `StyleXError` with just a message.
pub fn stylex_err(message: impl Into<String>) -> StyleXError {
  StyleXError {
    message: message.into(),
    file: None,
    key_path: None,
    line: None,
    col: None,
    source_location: None,
  }
}

/// Create a `StyleXError` with a message and file context.
pub fn stylex_err_with_file(message: impl Into<String>, file: impl Into<String>) -> StyleXError {
  StyleXError {
    message: message.into(),
    file: Some(file.into()),
    key_path: None,
    line: None,
    col: None,
    source_location: None,
  }
}

// ---------------------------------------------------------------------------
// Internal diverging functions (called by macros — not for direct use)
//
// Macros must call these via `$crate::…` paths because Rust macros cannot
// invoke other `#[macro_export]` macros by `$crate::macro_name!()`.
// ---------------------------------------------------------------------------

#[doc(hidden)]
#[track_caller]
pub fn __stylex_panic(mut err: StyleXError) -> ! {
  let caller = std::panic::Location::caller();

  if err.source_location.is_none() {
    err.source_location = Some(format!("{}:{}", caller.file(), caller.line()));
  }

  panic!("{}", err)
}

#[doc(hidden)]
#[track_caller]
pub fn __stylex_unimplemented(mut err: StyleXError) -> ! {
  let caller = std::panic::Location::caller();
  if err.source_location.is_none() {
    err.source_location = Some(format!("{}:{}", caller.file(), caller.line()));
  }
  err.message = format!("{} {}", "[UNIMPLEMENTED]".dimmed().magenta(), err.message);

  panic!("{}", err)
}

#[doc(hidden)]
#[track_caller]
pub fn __stylex_unreachable(mut err: StyleXError) -> ! {
  let caller = std::panic::Location::caller();
  if err.source_location.is_none() {
    err.source_location = Some(format!("{}:{}", caller.file(), caller.line()));
  }
  err.message = format!("{} {}", "[UNREACHABLE]".dimmed().blue(), err.message);

  panic!("{}", err)
}

// ---------------------------------------------------------------------------
// Convenience macros
// ---------------------------------------------------------------------------

/// Like `panic!()` but produces `[StyleX] <message>`.
///
/// Usage:
/// ```ignore
/// stylex_panic!("border is not supported");
/// stylex_panic!("Invalid value: {}", val);
/// ```
#[macro_export]
macro_rules! stylex_panic {
  ($($arg:tt)*) => {
    $crate::shared::utils::log::stylex_error::__stylex_panic(
      $crate::shared::utils::log::stylex_error::stylex_err(
        format!($($arg)*)
      )
    )
  };
}

#[macro_export]
macro_rules! stylex_panic_with_file {
  ($($arg:tt)*) => {
    $crate::shared::utils::log::stylex_error::__stylex_panic(
      $crate::shared::utils::log::stylex_error::stylex_err_with_file(
        format!($($arg)*),
        file!()
      )
    )
  };
}

/// Like `unimplemented!()` but produces `[StyleX] [UNIMPLEMENTED] <message>`.
#[macro_export]
macro_rules! stylex_unimplemented {
  ($($arg:tt)*) => {
    $crate::shared::utils::log::stylex_error::__stylex_unimplemented(
      $crate::shared::utils::log::stylex_error::stylex_err(
        format!($($arg)*)
      )
    )
  };
}

/// Like `unreachable!()` but produces `[StyleX] [UNREACHABLE] <message>`.
#[macro_export]
macro_rules! stylex_unreachable {
  ($($arg:tt)*) => {
    $crate::shared::utils::log::stylex_error::__stylex_unreachable(
      $crate::shared::utils::log::stylex_error::stylex_err(
        format!($($arg)*)
      )
    )
  };
}

/// Like `anyhow::bail!()` but wraps the error in `StyleXError`.
///
/// Returns `Err(anyhow::Error)` with the `[StyleX]` prefix.
#[macro_export]
macro_rules! stylex_bail {
  ($($arg:tt)*) => {
    return Err(anyhow::anyhow!(
      $crate::shared::utils::log::stylex_error::StyleXError {
        message: format!($($arg)*),
        file: None,
        key_path: None,
        line: None,
        col: None,
        source_location: None,
      }
    ))
  };
}

/// Like `anyhow::anyhow!()` but wraps in `StyleXError`.
///
/// Returns `anyhow::Error` (not `Result`).
#[macro_export]
macro_rules! stylex_anyhow {
  ($($arg:tt)*) => {
    anyhow::anyhow!(
      $crate::shared::utils::log::stylex_error::StyleXError {
        message: format!($($arg)*),
        file: None,
        key_path: None,
        line: None,
        col: None,
        source_location: None,
      }
    )
  };
}

/// Unwrap a `Result` or panic with a `[StyleX]` prefixed message.
///
/// Drop-in replacement for the existing `unwrap_or_panic!` macro.
///
/// Usage:
/// ```ignore
/// let val = stylex_unwrap!(result);
/// let val = stylex_unwrap!(result, "Converting value");
/// ```
#[macro_export]
macro_rules! stylex_unwrap {
  ($result:expr) => {
    $result.unwrap_or_else(|error| {
      $crate::shared::utils::log::stylex_error::__stylex_panic(
        $crate::shared::utils::log::stylex_error::stylex_err(format!("{}", error)),
      )
    })
  };
  ($result:expr, $context:expr) => {
    $result.unwrap_or_else(|error| {
      $crate::shared::utils::log::stylex_error::__stylex_panic(
        $crate::shared::utils::log::stylex_error::stylex_err(format!("{}: {}", $context, error)),
      )
    })
  };
}

// ---------------------------------------------------------------------------
// Panic-output suppression (used around `catch_unwind` at the NAPI boundary)
// ---------------------------------------------------------------------------

thread_local! {
  static SUPPRESS_PANIC_STDERR: std::cell::Cell<bool> = const { std::cell::Cell::new(false) };
}

/// Returns `true` while a [`SuppressPanicStderr`] guard is alive on this thread.
///
/// Used by the custom panic hook installed in `logger::initialize()` to avoid
/// printing anything when a panic is caught by `std::panic::catch_unwind`.
pub fn is_panic_stderr_suppressed() -> bool {
  SUPPRESS_PANIC_STDERR.with(|f| f.get())
}

/// RAII guard that suppresses panic-hook stderr output for its lifetime.
///
/// Create one immediately before `catch_unwind`; the hook will stay silent
/// until the guard is dropped (on exit from scope, including on panic).
///
/// ```rust,ignore
/// let _guard = SuppressPanicStderr::new();
/// let result = std::panic::catch_unwind(|| { /* … */ });
/// guard dropped here → suppression lifted
/// ```
pub struct SuppressPanicStderr;

impl SuppressPanicStderr {
  pub fn new() -> Self {
    SUPPRESS_PANIC_STDERR.with(|f| f.set(true));
    Self
  }
}

impl Default for SuppressPanicStderr {
  fn default() -> Self {
    Self::new()
  }
}

impl Drop for SuppressPanicStderr {
  fn drop(&mut self) {
    SUPPRESS_PANIC_STDERR.with(|f| f.set(false));
  }
}

// ---------------------------------------------------------------------------
// Utilities for the NAPI boundary
// ---------------------------------------------------------------------------

/// Strip ANSI escape sequences from a string.
fn strip_ansi(s: &str) -> String {
  let mut result = String::with_capacity(s.len());
  let mut chars = s.chars().peekable();
  while let Some(ch) = chars.next() {
    if ch == '\x1B' && chars.peek() == Some(&'[') {
      chars.next(); // consume '['
      for c in chars.by_ref() {
        if c == 'm' {
          break;
        }
      }
    } else {
      result.push(ch);
    }
  }
  result
}

/// Extract a plain-text error message from a caught panic payload.
///
/// ANSI codes are stripped before the prefix check so that colored
/// `StyleXError` payloads are detected correctly.  The returned string
/// is always plain text, safe to pass to the NAPI boundary or non-TTY logs.
///
/// If the stripped message contains `[StyleX]`, it is returned as-is.
/// Otherwise, it is wrapped as `[StyleX] Internal error: <message>`.
pub fn format_panic_message(error: &Box<dyn std::any::Any + Send>) -> String {
  // How to get stack trace from the error?
  let raw = match error.downcast_ref::<String>() {
    Some(s) => s.clone(),
    None => match error.downcast_ref::<&str>() {
      Some(s) => s.to_string(),
      None => {
        return format!("{} Unknown error during transformation", STYLEX_PREFIX);
      }
    },
  };

  if raw.contains(STYLEX_PREFIX) {
    raw
  } else {
    let plain = strip_ansi(&raw);

    format!("{} {}", STYLEX_PREFIX, plain)
  }
}
