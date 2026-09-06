//! A logger the tests can read back, so the reporting paths are covered by
//! assertions rather than by a coverage exclusion.
//!
//! Both the level and the captured messages are per thread, because the test
//! harness runs tests in parallel and a global level would let one test decide
//! what another one sees. `log::max_level` stays wide open; the per-thread level
//! is what [`Log::enabled`] answers from, which is what `log_enabled!` asks.

use std::cell::RefCell;

use log::{Level, LevelFilter, Log, Metadata, Record};

thread_local! {
  static LEVEL: RefCell<LevelFilter> = const { RefCell::new(LevelFilter::Warn) };
  static MESSAGES: RefCell<Vec<String>> = const { RefCell::new(Vec::new()) };
}

struct CapturingLogger;

impl Log for CapturingLogger {
  fn enabled(&self, metadata: &Metadata) -> bool {
    LEVEL.with(|level| metadata.level() <= *level.borrow())
  }

  fn log(&self, record: &Record) {
    if !self.enabled(record.metadata()) {
      return;
    }

    MESSAGES.with(|messages| messages.borrow_mut().push(record.args().to_string()));
  }

  fn flush(&self) {}
}

/// Installs the logger. Called once, from the test harness's own start-up.
pub(crate) fn install() {
  if log::set_boxed_logger(Box::new(CapturingLogger)).is_ok() {
    log::set_max_level(LevelFilter::Trace);
  }
}

/// Runs `body` with this thread logging at `level`, and hands back everything it
/// logged.
pub(crate) fn logged_at<T>(level: Level, body: impl FnOnce() -> T) -> Vec<String> {
  LEVEL.with(|current| *current.borrow_mut() = level.to_level_filter());
  MESSAGES.with(|messages| messages.borrow_mut().clear());

  body();

  LEVEL.with(|current| *current.borrow_mut() = LevelFilter::Warn);
  MESSAGES.with(|messages| messages.borrow().clone())
}
