use std::sync::atomic::{AtomicUsize, Ordering};

use colored::{Color, Colorize};
use env_logger::fmt::Formatter;

use crate::constants::STYLEX_PREFIX;

// Shared ANSI escape sequences – single source of truth for the whole log module.
pub const ANSI_RED: &str = "\x1B[31m";
pub const ANSI_YELLOW: &str = "\x1B[33m";
pub const ANSI_GREEN: &str = "\x1B[32m";
pub const ANSI_BLUE: &str = "\x1B[34m";
pub const ANSI_WHITE: &str = "\x1B[37m";
pub const ANSI_CYAN: &str = "\x1B[36m";
pub const ANSI_BOLD: &str = "\x1B[1m";
pub const ANSI_DIM: &str = "\x1B[2m";
pub const ANSI_RESET: &str = "\x1B[0m";

pub const ANSI_ORANGE: &str = "\x1B[38;5;208m";

static MAX_MODULE_WIDTH: AtomicUsize = AtomicUsize::new(0);

fn max_target_width(target: impl AsRef<str>) -> usize {
  let target = target.as_ref();
  let max_width = MAX_MODULE_WIDTH.load(Ordering::Relaxed);

  if max_width < target.len() {
    MAX_MODULE_WIDTH.store(target.len(), Ordering::Relaxed);
    target.len()
  } else {
    max_width
  }
}

pub fn log_formatter(f: &mut Formatter, record: &log::Record) -> std::io::Result<()> {
  use std::io::Write;

  let target = record.target();
  let max_width = max_target_width(target);

  let level_color = match record.level() {
    log::Level::Error => Color::Red,
    log::Level::Warn => Color::Yellow,
    log::Level::Info => Color::Green,
    log::Level::Debug => Color::Blue,
    log::Level::Trace => Color::White,
  };

  writeln!(
    f,
    "{} {} {:width$} > {}",
    record.level().to_string().color(level_color).bold(),
    STYLEX_PREFIX.bright_blue().bold(),
    target.bold(),
    record.args(),
    width = max_width
  )
}
