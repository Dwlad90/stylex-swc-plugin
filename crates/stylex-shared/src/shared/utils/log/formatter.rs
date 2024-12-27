use std::sync::atomic::{AtomicUsize, Ordering};

use env_logger::fmt::Formatter;

static MAX_MODULE_WIDTH: AtomicUsize = AtomicUsize::new(0);

fn max_target_width(target: &str) -> usize {
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

  let level = match record.level() {
    log::Level::Error => "31", // Red
    log::Level::Warn => "33",  // Yellow
    log::Level::Info => "32",  // Green
    log::Level::Debug => "34", // Blue
    log::Level::Trace => "37", // White
  };

  writeln!(
    f,
    "\x1B[{}m{} \x1B[0m\x1B[1m{:width$}\x1B[0m > {}",
    level,
    record.level(),    // Log level
    target,            // Target
    record.args(),     // Log message
    width = max_width  // Apply the maximum width
  )
}
