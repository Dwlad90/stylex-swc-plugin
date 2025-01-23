use env_logger::{Builder, WriteStyle};
use log::LevelFilter;
use std::sync::Once;

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
  });
}
