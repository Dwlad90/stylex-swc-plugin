use env_logger::{Builder, WriteStyle};
use log::LevelFilter;
use std::{panic, sync::Once};
use stylex_macros::stylex_error::is_panic_stderr_suppressed;

use crate::constants::STYLEX_PREFIX;

use super::formatter::log_formatter;

static INIT: Once = Once::new();

pub fn initialize() {
  INIT.call_once(|| {
    let _ = Builder::new()
      .format(log_formatter)
      .filter_level(LevelFilter::Warn)
      .parse_env("STYLEX_DEBUG") // Allow override via environment variable
      .write_style(WriteStyle::Always)
      .try_init();

    panic::set_hook(Box::new(|info| {
      if is_panic_stderr_suppressed() {
        return;
      }

      let msg = if let Some(s) = info.payload().downcast_ref::<String>() {
        s.clone()
      } else if let Some(s) = info.payload().downcast_ref::<&str>() {
        s.to_string()
      } else {
        format!("{} Unknown internal error", STYLEX_PREFIX).to_string()
      };

      // StyleX panics already carry the branded prefix in their Display output;
      // print them as-is.  For any other (unexpected) panic, wrap with the prefix.
      if msg.contains(STYLEX_PREFIX) {
        eprintln!("{}", msg);
      } else {
        eprintln!("{} {}", STYLEX_PREFIX, msg);
      }
    }));
  });
}

#[cfg(test)]
mod tests {
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
}
