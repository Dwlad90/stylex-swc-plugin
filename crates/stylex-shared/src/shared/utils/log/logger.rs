use env_logger::{Builder, WriteStyle};
use log::LevelFilter;
use std::{panic, sync::Once};

use crate::shared::utils::log::stylex_error::STYLEX_PREFIX;

use super::{formatter::log_formatter, stylex_error::is_panic_stderr_suppressed};

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

      // StyleX errors are already emitted via log::error!() inside stylex_panic()
      // / stylex_unimplemented() / stylex_unreachable() – skip them here to avoid
      // printing the same message a second time.
      if !msg.contains(STYLEX_PREFIX) {
        eprintln!("{}{}", STYLEX_PREFIX, msg);
      }
    }));
  });
}
