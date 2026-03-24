//! Error handling macros for consistent error handling across the workspace.
//!
//! These macros provide standardized patterns for handling common error cases
//! when working with expressions, conversions, and evaluations.

use crate::macros::stylex_error::StyleXError;
use colored::Colorize;

/// Macro to unwrap a Result or panic with the error message.
/// This is a cleaner replacement for `.unwrap_or_else(|error| panic!("{}", error))`.
///
/// # Usage
/// ```ignore
/// let value = unwrap_or_panic!(expr_to_num(&arg, state, traversal_state, fns));
/// // Or with additional context:
/// let value = unwrap_or_panic!(expr_to_num(&arg, state, traversal_state, fns), "Converting to number");
/// ```
///
/// # Arguments
/// - `$result`: A Result type to unwrap
/// - `$context` (optional): Additional context string to prepend to the error message
#[macro_export]
macro_rules! unwrap_or_panic {
  ($result:expr) => {
    $result.unwrap_or_else(|error| {
      $crate::macros::panic_macros::__stylex_panic($crate::macros::panic_macros::stylex_err(
        format!("{}", error),
      ))
    })
  };
  ($result:expr, $context:expr) => {
    $result.unwrap_or_else(|error| {
      $crate::macros::panic_macros::__stylex_panic($crate::macros::panic_macros::stylex_err(
        format!("{}: {}", $context, error),
      ))
    })
  };
}

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
    $crate::macros::panic_macros::__stylex_panic(
      $crate::macros::panic_macros::stylex_err(
        format!($($arg)*)
      )
    )
  };
}

#[macro_export]
macro_rules! stylex_panic_with_file {
  ($($arg:tt)*) => {
    $crate::macros::panic_macros::__stylex_panic(
      $crate::macros::panic_macros::stylex_err_with_file(
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
    $crate::macros::panic_macros::__stylex_unimplemented(
      $crate::macros::panic_macros::stylex_err(
        format!($($arg)*)
      )
    )
  };
}

/// Like `unreachable!()` but produces `[StyleX] [UNREACHABLE] <message>`.
#[macro_export]
macro_rules! stylex_unreachable {
  ($($arg:tt)*) => {
    $crate::macros::panic_macros::__stylex_unreachable(
      $crate::macros::panic_macros::stylex_err(
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
      $crate::macros::panic_macros::StyleXError {
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
      $crate::macros::panic_macros::StyleXError {
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
      $crate::macros::panic_macros::__stylex_panic($crate::macros::panic_macros::stylex_err(
        format!("{}", error),
      ))
    })
  };
  ($result:expr, $context:expr) => {
    $result.unwrap_or_else(|error| {
      $crate::macros::panic_macros::__stylex_panic($crate::macros::panic_macros::stylex_err(
        format!("{}: {}", $context, error),
      ))
    })
  };
}
