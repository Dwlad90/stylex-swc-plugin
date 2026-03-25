use env_logger::{Builder, WriteStyle};
use log::LevelFilter;
use std::{panic, sync::Once};
use stylex_macros::stylex_error::is_panic_stderr_suppressed;

use crate::constants::STYLEX_PREFIX;

use super::formatter::log_formatter;

static INIT: Once = Once::new();

pub fn initialize() {
  INIT.call_once(|| {
    Builder::new()
      .format(log_formatter)
      .filter_level(LevelFilter::Warn)
      .parse_env("STYLEX_DEBUG") // Allow override via environment variable
      .write_style(WriteStyle::Always)
      .init();

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
